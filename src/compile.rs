//! Compile Typst to HTML given paths and a [`crate::config::Config`].

use anyhow::{Context, Result, anyhow};
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use walkdir::WalkDir;

use crate::config::Config;

/// Return paths to the files in source we will process.
///
/// This includes data files we ignore, stuff we pass through, typ files, everything.
/// i.e. we walk through the source dir.
/// Ignores inaccessible such files.
pub fn source_files(config: &Config) -> impl Iterator<Item = PathBuf> {
    WalkDir::new(config.project_root.join(&config.content_root))
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
        if config.passthrough_copy_globs.is_match(full_path) {
            let rel_path = full_path
                .strip_prefix(&config.project_root)?
                .strip_prefix(&config.content_root)?;
            let dst_path = PathBuf::from(&config.project_root)
                .join(&config.output_root)
                .join(rel_path);
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

        if let Ok(_) = full_path.strip_prefix(config.project_root.join(&config.template_root)) {
            log::trace!(
                "CompileOutput::from_full_path({:?}, config) computed RecompileAll",
                full_path
            );
            return Ok(Self::RecompileAll);
        } else if let Ok(path_to_typ_in_src) =
            full_path.strip_prefix(config.project_root.join(&config.content_root))
        {
            let rel_parent = path_to_typ_in_src.parent().unwrap();
            let parent_dir_in_dst = config
                .project_root
                .join(&config.output_root)
                .join(rel_parent);
            let file_in_dst =
                if full_path.file_name().unwrap() == "index.typ" || config.literal_paths {
                    let mut file_in_dst = parent_dir_in_dst.join(full_path.file_name().unwrap());
                    file_in_dst.set_extension("html");
                    file_in_dst
                } else {
                    parent_dir_in_dst
                        .join(full_path.file_stem().unwrap())
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

pub fn compile_from_scratch(config: &Config) -> Result<()> {
    if config.init.len() > 0 {
        log::info!("running init command");
        Command::new(&config.init[0])
            .args(&config.init[1..])
            .spawn()
            .context(anyhow!(
                "Couldn't init. We tried running the command {:?}",
                &config.init
            ))?
            .wait()
            .context(anyhow!(
                "We failed to finish running the command {:?}",
                &config.init
            ))?;
        log::trace!("finished init");
    }

    for file in source_files(&config) {
        if let CompileOutput::CompileToPath(path) = CompileOutput::from_full_path(&file, &config)? {
        }
    }

    log::info!("starting compilation");
    compile_batch(source_files(&config), &config)?;

    log::info!("compiled project from scratch");

    Ok(())
}

pub fn compile_single(path: &Path, config: &Config) -> Result<()> {
    log::trace!("here1 compiling {}", path.to_str().unwrap());

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
            fs::create_dir_all(&dst_path.parent().unwrap())?;

            fs::copy(path, &dst_path)
                .context(format!("Failed to write output to {:?}", &dst_path))?;

            log::trace!(
                "passthroughcopied {} to {}",
                path.to_str().unwrap(),
                dst_path.to_str().unwrap()
            );
        }
        CompileOutput::CompileToPath(dst_path) => {
            let child = {
                let args = [
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

                Command::new("typst")
                    .args(args)
                    .stdout(Stdio::piped())
                    .spawn()
                    .context(anyhow!("Failed to run Typst compiler. Maybe you don't have it installed? We ran `typst` with args: {:?}", args))?
            };

            let child = if config.post_processing_typ.len() > 0 {
                Command::new(&config.post_processing_typ[0])
                    .args(&config.post_processing_typ[1..])
                    .stdin(child.stdout.unwrap())
                    .stdout(Stdio::piped())
                    .spawn()
                    .context(anyhow!(
                        "Failed to post process. We tried to run the command {:?}",
                        &config.post_processing_typ
                    ))?
            } else {
                child
            };

            let output = child
                .wait_with_output()
                .context("Waiting for output of typst and post-processing failed.")?;

            fs::create_dir_all(&dst_path.parent().unwrap())?;
            fs::write(&dst_path, output.stdout)
                .context(format!("Failed to write output to {:?}", &dst_path))?;

            log::trace!(
                "typfile compiled {} to {}",
                path.to_str().unwrap(),
                dst_path.to_str().unwrap()
            );
        }
    };

    Ok(())
}

pub fn compile_batch(paths: impl Iterator<Item = PathBuf>, config: &Config) -> Result<()> {
    std::thread::scope(|s| {
        for path in paths {
            s.spawn(move || {
                compile_single(&path, &config).unwrap_or_else(|err| eprintln!("{:?}", err)); // TODO: this should fail, but exit(1) borks.
                log::debug!("compiled {}", path.to_str().unwrap());
            });
        }
    });

    Ok(())
}
