//! Compile Typst to HTML given paths and a [`crate::config::Config`].

use anyhow::Result;
use std::fs;
use std::path::PathBuf;
use std::process::{Command, Stdio};
use walkdir::WalkDir;

use crate::config::CONFIG;

pub fn compile_from_scratch() -> Result<()> {
    log::info!("running init command");
    if CONFIG.init.len() > 0 {
        Command::new(&CONFIG.init[0])
            .args(&CONFIG.init[1..])
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
        log::trace!("finished init");
    }

    log::info!("starting compilation");
    compile_batch(
        WalkDir::new(CONFIG.project_root.join(&CONFIG.content_root))
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|entry| entry.metadata().unwrap().is_file())
            .map(|entry| entry.path().to_path_buf()),
    )?;

    log::info!("compiled project from scratch");

    Ok(())
}

pub fn compile_single(path: &PathBuf) -> Result<()> {
    log::trace!("here1 compiling {}", path.to_str().unwrap());
    if CONFIG.passthrough_copy_globs.is_match(path) {
        log::trace!("here2");
        let path_in_src = path
            .strip_prefix(CONFIG.project_root.join(&CONFIG.content_root))
            .unwrap();
        let rel_path = path_in_src.parent().unwrap();
        let parent_dir_in_dst = CONFIG.project_root.join(&CONFIG.output_root).join(rel_path);
        let file_in_dst = parent_dir_in_dst.join(path.file_name().unwrap());

        fs::create_dir_all(CONFIG.project_root.join(&CONFIG.output_root).join(rel_path))?;
        fs::copy(path, &file_in_dst)?;

        log::trace!(
            "passthroughcopied {} to {}",
            path.to_str().unwrap(),
            file_in_dst.to_str().unwrap()
        );
    } else if path.extension().is_some() && path.extension().unwrap() == "typ" {
        log::trace!("here3");

        if let Ok(_) = path.strip_prefix(CONFIG.project_root.join(&CONFIG.template_root)) {
            compile_from_scratch()?;
            // need to be careful of infinite recursion, compile_everything calls us (compile)
            // should be fine because this code path should only trigger when compiling
            // on the template root.
            //
            // ... what if someone puts their template code in their src folder?
        } else if let Ok(path_in_src) =
            path.strip_prefix(CONFIG.project_root.join(&CONFIG.content_root))
        {
            let rel_path = path_in_src.parent().unwrap();
            let parent_dir_in_dst = CONFIG.project_root.join(&CONFIG.output_root).join(rel_path);
            let mut file_in_dst = parent_dir_in_dst.join(path.file_name().unwrap());
            file_in_dst.set_extension("html");

            fs::create_dir_all(CONFIG.project_root.join(&CONFIG.output_root).join(rel_path))
                .unwrap();
            let mut child = Command::new("typst")
                .arg("c")
                .arg(path)
                .arg("-") // to stdout
                .arg("--features")
                .arg("html")
                .arg("--format")
                .arg("html")
                .arg("--root")
                .arg(&*CONFIG.project_root)
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();

            if CONFIG.post_processing_typ.len() > 0 {
                child = Command::new(&CONFIG.post_processing_typ[0])
                    .args(&CONFIG.post_processing_typ[1..])
                    .stdin(child.stdout.unwrap())
                    .stdout(Stdio::piped())
                    .spawn()
                    .unwrap();
            }

            let output = child.wait_with_output().unwrap();

            fs::write(&file_in_dst, output.stdout).unwrap();

            log::trace!(
                "typfile compiled {} to {}",
                path.to_str().unwrap(),
                file_in_dst.to_str().unwrap()
            );
        }
    }

    Ok(())
}

pub fn compile_batch(paths: impl Iterator<Item = PathBuf>) -> Result<()> {
    std::thread::scope(|s| {
        for path in paths {
            s.spawn(move || {
                compile_single(&path).unwrap_or_else(|err| eprintln!("{}", err));
                log::debug!("compiled {}", path.to_str().unwrap());
            });
        }
    });

    Ok(())
}
