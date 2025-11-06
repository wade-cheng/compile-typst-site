//! The function to call to kick off the binary.

use notify_debouncer_full;
use notify_debouncer_full::DebounceEventResult;
use notify_debouncer_full::notify::{EventKind, RecursiveMode};
use std::{sync::mpsc, time::Duration};

use crate::compile;
use crate::config::CONFIG;
use crate::error::Error;
use crate::logging;

pub fn run() -> Result<(), Error> {
    logging::init();

    log::trace!("loaded configuration: {:#?}", &*CONFIG);

    if CONFIG.ignore_initial {
        log::info!("ignoring initial compile from scratch");
    } else {
        compile::compile_from_scratch()?;
    }

    if CONFIG.watch {
        let (tx, rx) = mpsc::channel::<DebounceEventResult>();

        let mut debouncer =
            notify_debouncer_full::new_debouncer(Duration::from_millis(200), None, tx)?;

        debouncer.watch(
            &*CONFIG.project_root.join(&CONFIG.content_root),
            RecursiveMode::Recursive,
        )?;
        debouncer.watch(
            &*CONFIG.project_root.join(&CONFIG.template_root),
            RecursiveMode::Recursive,
        )?;
        for res in rx {
            match res {
                Ok(events) => {
                    for event in events {
                        if let EventKind::Create(_) | EventKind::Modify(_) = event.kind {
                            compile::compile_batch(event.event.paths.into_iter())?;
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
