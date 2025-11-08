//! Compile Typst to HTML given paths and a [`crate::config::Config`].

use anyhow::{Context, Result, anyhow};
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use walkdir::WalkDir;

use crate::config::Config;

/// Return paths to the `.typ` files we will process.
///
/// Ignores inaccessible such files.
pub fn typ_files(config: &Config) -> impl Iterator<Item = PathBuf> {
    WalkDir::new(config.project_root.join(&config.content_root))
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|entry| entry.metadata().unwrap().is_file())
        .map(|entry| entry.path().to_path_buf())
}

pub fn compile_from_scratch(config: &Config) -> Result<()> {
    log::info!("running init command");
    if config.init.len() > 0 {
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

    log::info!("starting compilation");
    compile_batch(typ_files(&config), &config)?;

    log::info!("compiled project from scratch");

    Ok(())
}

pub fn compile_single(path: &PathBuf, config: &Config) -> Result<()> {
    log::trace!("here1 compiling {}", path.to_str().unwrap());
    if config.passthrough_copy_globs.is_match(path) {
        log::trace!("here2");
        let path_in_src = path
            .strip_prefix(config.project_root.join(&config.content_root))
            .unwrap();
        let rel_path = path_in_src.parent().unwrap();
        let parent_dir_in_dst = config.project_root.join(&config.output_root).join(rel_path);
        let file_in_dst = parent_dir_in_dst.join(path.file_name().unwrap());

        fs::create_dir_all(config.project_root.join(&config.output_root).join(rel_path))?;
        fs::copy(path, &file_in_dst)?;

        log::trace!(
            "passthroughcopied {} to {}",
            path.to_str().unwrap(),
            file_in_dst.to_str().unwrap()
        );
    } else if path.extension().is_some() && path.extension().unwrap() == "typ" {
        log::trace!("here3");

        if let Ok(_) = path.strip_prefix(config.project_root.join(&config.template_root)) {
            compile_from_scratch(&config)?;
            // need to be careful of infinite recursion, compile_everything calls us (compile)
            // should be fine because this code path should only trigger when compiling
            // on the template root.
            //
            // ... what if someone puts their template code in their src folder?
        } else if let Ok(path_in_src) =
            path.strip_prefix(config.project_root.join(&config.content_root))
        {
            let rel_path = path_in_src.parent().unwrap();
            let parent_dir_in_dst = config.project_root.join(&config.output_root).join(rel_path);
            let file_in_dst = if path.file_name().unwrap() == "index.typ" || config.literal_paths {
                let mut file_in_dst = parent_dir_in_dst.join(path.file_name().unwrap());
                file_in_dst.set_extension("html");
                file_in_dst
            } else {
                parent_dir_in_dst
                    .join(path.file_stem().unwrap())
                    .join("index.html")
            };

            fs::create_dir_all(config.project_root.join(&config.output_root).join(rel_path))
                .unwrap();
            let mut child = {
                let args = [
                    OsStr::new("c"),
                    OsStr::new(path),
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
            if config.post_processing_typ.len() > 0 {
                child = Command::new(&config.post_processing_typ[0])
                    .args(&config.post_processing_typ[1..])
                    .stdin(child.stdout.unwrap())
                    .stdout(Stdio::piped())
                    .spawn()
                    .context(anyhow!(
                        "Failed to post process. We tried to run the command {:?}",
                        &config.post_processing_typ
                    ))?;
            }

            let output = child
                .wait_with_output()
                .context("Waiting for output of typst and post-processing failed.")?;

            fs::create_dir_all(&file_in_dst.parent().ok_or(anyhow!(
                "Failed trying to create parent directory of {:?}",
                &file_in_dst
            ))?)?;
            fs::write(&file_in_dst, output.stdout)
                .context(format!("Failed to write output to {:?}", &file_in_dst))?;

            log::trace!(
                "typfile compiled {} to {}",
                path.to_str().unwrap(),
                file_in_dst.to_str().unwrap()
            );
        }
    }

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
