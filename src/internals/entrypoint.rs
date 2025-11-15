//! The function to call to kick off the binary.

use anyhow::{Result, anyhow};
use notify_debouncer_full;
use notify_debouncer_full::DebounceEventResult;
use notify_debouncer_full::notify::{EventKind, RecursiveMode};
use std::{sync::mpsc, time::Duration};

use crate::internals::compile;
use crate::internals::config::Config;
use crate::internals::logging;

pub fn run(config: &Config) -> Result<()> {
    logging::init(&config);

    log::debug!("loaded configuration: {:#?}", &config);

    if config.ignore_initial {
        log::info!("ignoring initial compile from scratch");
    } else {
        compile::compile_from_scratch(&config)?;
    }

    if !config.watch {
        return Ok(());
    }

    let (tx, rx) = mpsc::channel::<DebounceEventResult>();

    let mut debouncer = notify_debouncer_full::new_debouncer(Duration::from_millis(200), None, tx)?;

    debouncer.watch(
        &config.project_root.join(&config.content_root),
        RecursiveMode::Recursive,
    )?;
    debouncer.watch(
        &config.project_root.join(&config.template_root),
        RecursiveMode::Recursive,
    )?;
    for res in rx {
        let events = res.map_err(|errs| {
            for err in errs {
                eprintln!("{:?}", err);
            }

            anyhow!("File watcher received errors.")
        })?;

        for event in events {
            if let EventKind::Create(_) | EventKind::Modify(_) = event.kind {
                compile::compile_batch(event.event.paths.clone().into_iter(), &config)?;

                if event.event.paths.len() == 1 {
                    log::info!("recompiled path: {:?}", event.event.paths[0]);
                } else {
                    log::info!("recompiled paths: {:?}", event.event.paths);
                }
            }
        }
    }

    Ok(())
}
