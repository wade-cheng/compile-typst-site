//! Compile Typst to HTML given paths and a [`crate::config::Config`].

use anyhow::{Context as _, Result, anyhow};
use json::JsonValue;
use std::ffi::OsStr;
use std::fs;
use std::io::Read as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::mpsc::{self};
use std::time::Instant;
use walkdir::WalkDir;

use crate::internals::config::{Config, FileListing};

/// Return paths to the files in source we will process.
///
/// This includes data files we ignore, stuff we pass through, typ files, everything.
/// i.e. we walk through the source dir.
/// Ignores inaccessible such files.
pub fn source_files(config: &Config) -> impl Iterator<Item = PathBuf> {
    WalkDir::new(config.content_root())
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| entry.metadata().unwrap().is_file())
        .map(|entry| entry.path().to_path_buf())
}

pub enum CompileOutput {
    Noop,
    Passthrough(PathBuf),
    RecompileAll,
    CompileToPath(PathBuf),
}

impl CompileOutput {
    pub fn from_full_path(full_path: &Path, config: &Config) -> Result<Self> {
        if config.passthrough_copy_globs.matches_path_with(&full_path) {
            let rel_path = full_path.strip_prefix(&config.content_root())?;
            let dst_path = config.output_root().join(rel_path);
            log::trace!(
                "CompileOutput::from_full_path({:?}, config) computed Passthrough to {:?}",
                full_path,
                dst_path
            );
            return Ok(Self::Passthrough(dst_path));
        }

        if full_path.extension() != Some(&OsStr::new("typ")) {
            log::trace!(
                "CompileOutput::from_full_path({:?}, config) computed Noop",
                full_path
            );
            return Ok(Self::Noop);
        }

        if let Ok(_) = full_path.strip_prefix(config.template_root()) {
            log::trace!(
                "CompileOutput::from_full_path({:?}, config) computed RecompileAll",
                full_path
            );
            return Ok(Self::RecompileAll);
        } else if let Ok(path_to_typ_in_src) = full_path.strip_prefix(config.content_root()) {
            let rel_parent = path_to_typ_in_src.parent().context("Found no parent.")?;
            let parent_dir_in_dst = config.output_root().join(rel_parent);
            let file_in_dst = if full_path.file_name().context("Found no file name")? == "index.typ"
                || config.literal_paths
            {
                let mut file_in_dst =
                    parent_dir_in_dst.join(full_path.file_name().context("Found no file name.")?);
                file_in_dst.set_extension("html");
                file_in_dst
            } else {
                parent_dir_in_dst
                    .join(full_path.file_stem().context("Found no file stem")?)
                    .join("index.html")
            };

            log::trace!(
                "CompileOutput::from_full_path({:?}, config) computed CompileToPath to {:?}",
                full_path,
                file_in_dst
            );
            return Ok(Self::CompileToPath(file_in_dst));
        }

        unreachable!(
            "Should not be reachable: all files passed into here should be in src or templates..."
        )
    }
}

/// Return all files as a json object.
///
/// For each entry in the object,
/// - the key is the full path to the original file (that is, in src, not in _site)
/// - the value is an array
///   - empty if not IncludeData
///   - otherwise, returned from querying the file for the <data> tag of the Typst file
pub fn files_as_json(config: &Config) -> Result<String> {
    let mut json = JsonValue::new_object();

    let (tx, rx) = mpsc::channel::<(String, JsonValue)>();

    let source_files: Vec<PathBuf> = source_files(&config).collect();
    let num_messages = source_files.len();

    std::thread::scope(|s| -> Result<()> {
        for file in source_files {
            let tx = tx.clone();

            s.spawn(move || -> Result<()> {
                let key = file.to_string_lossy().to_string();
                let mut value = JsonValue::new_array();

                if let (FileListing::IncludeData, CompileOutput::CompileToPath(_)) = (
                    &config.file_listing,
                    CompileOutput::from_full_path(&file, &config)?,
                ) {
                    let args = [
                        OsStr::new("--color"),
                        OsStr::new("always"),
                        OsStr::new("query"),
                        OsStr::new(&file),
                        OsStr::new("<data>"),
                        OsStr::new("--features"),
                        OsStr::new("html"),
                        OsStr::new("--root"),
                        OsStr::new(&config.project_root),
                    ];

                    let mut query_output = Command::new("typst")
                        .args(args)
                        .args(&config.compilation_extra_args)
                        .output()
                        .context(anyhow!(
                            "Failed to query <data> in the file {}. \
                            Maybe you don't have Typst installed? \
                            https://typst.app/open-source/#download \
                            We ran `typst` with args, extra args: {:?} {:?}",
                            &file.to_string_lossy(),
                            args,
                            &config.compilation_extra_args
                        ))?;

                    if !query_output.stderr.is_empty() {
                        log::warn!(
                            target: "typst query stderr",
                            "{}",
                            String::from_utf8(std::mem::take(&mut query_output.stderr))?
                        );
                    }

                    if query_output.status.success() {
                        value = json::parse(str::from_utf8(&query_output.stdout)?)?;
                    } else {
                        log::info!("failed to query {}", &file.to_string_lossy());
                    }
                }

                tx.send((key, value))?;

                Ok(())
            });
        }

        Ok(())
    })?;

    for _ in 0..num_messages {
        let (key, value) = rx.recv()?;
        json[key] = value;
    }

    Ok(json.dump())
}

pub fn compile_from_scratch(config: &Config) -> Result<()> {
    let start = Instant::now();

    if config.init.len() > 0 {
        log::info!("running init command");
        let mut init_output = Command::new(&config.init[0])
            .args(&config.init[1..])
            .output()
            .context(anyhow!(
                "Couldn't init. We tried running the command {:?}",
                &config.init
            ))?;

        if !init_output.stderr.is_empty() {
            log::warn!(
                target: "init command stderr",
                "{}",
                String::from_utf8(std::mem::take(&mut init_output.stderr))?
            );
        }

        if !init_output.status.success() {
            return Err(anyhow!(
                "Running init command failed. Command was {:?}",
                config.init
            ));
        }
        log::trace!("finished init");
    }

    if let FileListing::Disabled = config.file_listing {
        log::trace!("not file listing");
    } else {
        let listing_path = config.project_root.join("files.json");
        log::info!(
            "generating and writing file listing to {}",
            listing_path.to_string_lossy()
        );
        fs::write(&listing_path, &files_as_json(&config)?)?;
    }

    log::info!("starting compilation");
    compile_batch(source_files(&config), &config)?; // todo in here

    log::info!(
        "compiled project from scratch in {}s",
        Instant::now().duration_since(start).as_millis() as f32 / 1000.0
    );

    Ok(())
}

pub fn compile_single(path: &Path, config: &Config) -> Result<()> {
    log::trace!("here1 compiling {}", path.to_string_lossy());

    match CompileOutput::from_full_path(path, config)? {
        CompileOutput::Noop => (),
        CompileOutput::RecompileAll => {
            compile_from_scratch(config)?
            // need to be careful of infinite recursion, compile_everything calls us (compile)
            // should be fine because this code path should only trigger when compiling
            // on the template root.
            //
            // ... what if someone puts their template code in their src folder?
        }
        CompileOutput::Passthrough(dst_path) => {
            fs::create_dir_all(
                &dst_path
                    .parent()
                    .context(anyhow!("Couldn't find parent."))?,
            )?;

            fs::copy(path, &dst_path)
                .context(format!("Failed to write output to {:?}", &dst_path))?;

            log::trace!(
                "passthroughcopied {} to {}",
                path.to_string_lossy(),
                dst_path.to_string_lossy()
            );
        }
        CompileOutput::CompileToPath(dst_path) => {
            log::trace!("compile_single:t10");
            let mut child = {
                let args = [
                    OsStr::new("--color"),
                    OsStr::new("always"),
                    OsStr::new("c"),
                    OsStr::new(&path),
                    OsStr::new("-"),
                    OsStr::new("--features"),
                    OsStr::new("html"),
                    OsStr::new("--format"),
                    OsStr::new("html"),
                    OsStr::new("--root"),
                    OsStr::new(&config.project_root),
                ];
                log::trace!("compile_single:t11");
                log::trace!(
                    "compile_single:path {:?}, trying to run typst with args, extra args: {:?} {:?}",
                    &path,
                    args,
                    &config.compilation_extra_args
                );

                Command::new("typst")
                    .args(args)
                    .args(&config.compilation_extra_args)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .context(anyhow!(
                        "Failed to run Typst compiler. \
                        Maybe you don't have it installed? \
                        https://typst.app/open-source/#download \
                        We ran `typst` with args, extra args: {:?} {:?}",
                        args,
                        &config.compilation_extra_args
                    ))?
            };

            let mut compile_stderr = child
                .stderr
                .take()
                .expect("specified Stdio::piped() for the child");
            std::thread::spawn(move || {
                let mut compile_stderr_string = String::new();
                compile_stderr
                    .read_to_string(&mut compile_stderr_string)
                    .unwrap_or_else(|_| {
                        log::error!("Typst stderr wasn't valid UTF-8.");
                        0 // dummy number to type check
                    });

                if !compile_stderr_string.is_empty() {
                    log::warn!(target: "typst compile stderr", "{compile_stderr_string}");
                }
            });

            if config.post_processing_typ.len() > 0 {
                child = Command::new(&config.post_processing_typ[0])
                    .args(&config.post_processing_typ[1..])
                    .stdin(child.stdout.context("Found no child")?)
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .spawn()
                    .context(anyhow!(
                        "Failed to post process. We tried to run the command {:?}",
                        &config.post_processing_typ
                    ))?;

                let mut pproc_stderr = child
                    .stderr
                    .take()
                    .expect("specified Stdio::piped() for the child");
                std::thread::spawn(move || {
                    let mut pproc_stderr_string = String::new();
                    pproc_stderr
                        .read_to_string(&mut pproc_stderr_string)
                        .unwrap_or_else(|_| {
                            log::error!(target: "post-processing stderr", "post_processing stderr wasn't valid UTF-8.");
                            0
                        });

                    if !pproc_stderr_string.is_empty() {
                        log::warn!(target: "post-processing stderr", "{pproc_stderr_string}");
                    }
                });
            }

            log::trace!("compile_single:t14");

            let output = child
                .wait_with_output()
                .context("Waiting for output of typst and post-processing failed.")?;

            log::trace!("compile_single:t15");

            if !output.status.success() {
                return Err(anyhow!("Compiling {} failed.", path.to_string_lossy()));
            }

            log::trace!("compile_single:t16");

            fs::create_dir_all(&dst_path.parent().context("Found no parent.")?)?;
            fs::write(&dst_path, output.stdout)
                .context(format!("Failed to write output to {:?}", &dst_path))?;

            log::trace!(
                "typfile compiled {} to {}",
                path.to_string_lossy(),
                dst_path.to_string_lossy()
            );
        }
    };

    Ok(())
}

/// Blocks until batch of paths are compiled.
///
/// Each path is compiled under a separate thread. Paths can be anywhere under src or templates.
/// Calling this function on paths outside those folders mayyy cause errors.
pub fn compile_batch(paths: impl Iterator<Item = PathBuf>, config: &Config) -> Result<()> {
    let start = Instant::now();

    std::thread::scope(|s| -> Result<()> {
        let mut paths_and_handles = vec![];
        for path in paths {
            paths_and_handles.push((
                path.clone(),
                s.spawn(move || -> Result<()> { compile_single(&path, &config) }),
            ));
        }

        for (path, handle) in paths_and_handles {
            log::debug!("trying to compile {}", path.to_str().unwrap());
            handle.join().unwrap()?;
        }

        Ok(())
    })?;

    log::info!(
        "compiled batch of files in {}s",
        Instant::now().duration_since(start).as_millis() as f32 / 1000.0
    );

    Ok(())
}
