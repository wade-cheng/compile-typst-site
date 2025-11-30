//! The function to call to kick off the binary.

use anyhow::{Result, anyhow};
use notify_debouncer_full;
use notify_debouncer_full::DebounceEventResult;
use notify_debouncer_full::notify::{EventKind, RecursiveMode};
use std::path::PathBuf;
use std::process::Command;
use std::thread;
use std::{sync::mpsc, time::Duration};

use crate::internals::compile::{self, CompileOutput};
use crate::internals::config::Config;
use crate::internals::logging;

pub fn run(config: &Config) -> Result<()> {
    logging::init(&config);

    if Command::new("typst").status().is_err() {
        return Err(anyhow!(
            "Typst doesn't seem to be installed on your computer. \
            See https://typst.app/open-source/#download"
        ));
    }

    log::debug!("loaded configuration: {:#?}", &config);

    if config.ignore_initial {
        log::info!("ignoring initial compile from scratch");
    } else {
        compile::compile_from_scratch(&config)?;
    }

    if !(config.watch || config.serve) {
        return Ok(());
    }

    let reload_tx = if config.serve {
        let (reload_tx, reload_rx) = mpsc::channel::<()>();

        let path_to_site = config.output_root();
        thread::spawn(|| crate::internals::serve::serve(reload_rx, path_to_site));

        Some(reload_tx)
    } else {
        None
    };

    let (tx, rx) = mpsc::channel::<DebounceEventResult>();

    let mut debouncer = notify_debouncer_full::new_debouncer(Duration::from_millis(200), None, tx)?;
    debouncer.watch(&config.project_root, RecursiveMode::Recursive)?;

    for res in rx {
        let events = res.unwrap_or_else(|errs| {
            for err in errs {
                log::error!("{:?}", err);
            }

            vec![]
        });

        for event in events {
            if let EventKind::Create(_) | EventKind::Modify(_) = event.kind {
                let file_created = matches!(event.kind, EventKind::Create(_));

                let relevant_paths: Vec<PathBuf> = event
                    .event
                    .paths
                    .into_iter()
                    .filter(|path| {
                        path.strip_prefix(config.content_root()).is_ok()
                            || path.strip_prefix(config.template_root()).is_ok()
                    })
                    .collect();

                if relevant_paths.is_empty() {
                    continue;
                }

                if file_created {
                    compile::compile_from_scratch(&config)
                        .unwrap_or_else(|e| log::warn!("{:?}", e));
                    if let Some(reload_tx) = &reload_tx {
                        reload_tx.send(())?;
                    }
                } else {
                    compile::compile_batch(relevant_paths.clone().into_iter(), &config)
                        .unwrap_or_else(|e| log::warn!("{:?}", e));

                    if let Some(reload_tx) = &reload_tx {
                        for path in &relevant_paths {
                            match CompileOutput::from_full_path(path, config)? {
                                CompileOutput::Noop => (),
                                _ => reload_tx
                                    .send(())
                                    .unwrap_or_else(|e| log::error!("{:?}", e)),
                            }
                        }
                    }
                }

                // it might as well be an invariant that there is one event.event.paths
                // since we watch for Create and Modify. oh well.
                if relevant_paths.len() == 1 {
                    log::info!("recompiled path: {:?}", relevant_paths[0]);
                } else {
                    log::info!("recompiled paths: {:?}", relevant_paths);
                }
            }
        }
    }

    Ok(())
}
