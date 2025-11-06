//! The function to call to kick off the binary.

use anyhow::Result;
use notify_debouncer_full;
use notify_debouncer_full::DebounceEventResult;
use notify_debouncer_full::notify::{EventKind, RecursiveMode};
use std::{sync::mpsc, time::Duration};

use crate::compile;
use crate::config::Config;
use crate::logging;

pub fn run(config: &Config) -> Result<()> {
    logging::init(&config);

    log::trace!("loaded configuration: {:#?}", &config);

    if config.ignore_initial {
        log::info!("ignoring initial compile from scratch");
    } else {
        compile::compile_from_scratch(&config)?;
    }

    if config.watch {
        let (tx, rx) = mpsc::channel::<DebounceEventResult>();

        let mut debouncer =
            notify_debouncer_full::new_debouncer(Duration::from_millis(200), None, tx)?;

        debouncer.watch(
            &config.project_root.join(&config.content_root),
            RecursiveMode::Recursive,
        )?;
        debouncer.watch(
            &config.project_root.join(&config.template_root),
            RecursiveMode::Recursive,
        )?;
        for res in rx {
            match res {
                Ok(events) => {
                    for event in events {
                        if let EventKind::Create(_) | EventKind::Modify(_) = event.kind {
                            compile::compile_batch(event.event.paths.into_iter(), &config)?;
                            // TODO: figure out howo to debug-level log watched compilations
                        }
                    }
                }
                Err(errors) => errors.iter().for_each(|error| eprintln!("{error:?}")),
            }
        }
    }

    Ok(())
}
