use error_iter::ErrorIter as _;
use globset::{Glob, GlobSet, GlobSetBuilder};
use log::LevelFilter;
use notify_debouncer_full;
use notify_debouncer_full::DebounceEventResult;
use notify_debouncer_full::notify::{EventKind, RecursiveMode};
use onlyargs::CliError;
use onlyargs_derive::OnlyArgs;
use onlyerror::Error;
use serde::Deserialize;
use simple_logger::SimpleLogger;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio, exit};
use std::sync::LazyLock;
use std::{fs, io};
use std::{sync::mpsc, time::Duration};
use walkdir::WalkDir;

/// Build a site with typst.
#[derive(Clone, Debug, Eq, PartialEq, OnlyArgs)]
struct Args {
    /// Build and then watch for changes.
    watch: bool,
    /// Ignore initial full-site compilation step.
    ignore_initial: bool,
    /// Enable verbose logging.
    verbose: bool,
    /// Enable very verbose logging.
    trace: bool,
}

#[derive(Deserialize)]
struct Config {
    passthrough_copy: Option<Vec<String>>,
    init: Option<Vec<String>>,
    post_processing_typ: Option<Vec<String>>,
}

#[derive(Debug, Error)]
enum Error {
    /// Argument parsing error.
    Cli(#[from] CliError),
    /// I/O error.
    Io(#[from] std::io::Error),
    /// notify crate error.
    Notify(#[from] notify_debouncer_full::notify::Error),
    /// toml crate error.
    Toml(#[from] toml::de::Error),
    /// globset crate error.
    Globset(#[from] globset::Error),
}

impl Error {
    fn print_msg(&self) {
        eprintln!("Error: {self}");
        for source in self.sources().skip(1) {
            eprintln!("  Caused by: {source}");
        }

        if matches!(self, Error::Cli(_)) {
            eprintln!("Try {} --help", std::env::args().next().unwrap());
        }
    }
}

const CONFIG_FNAME: &str = "sfn-typst-site.toml";

static PROJECT_ROOT: LazyLock<PathBuf> = LazyLock::new(|| match get_project_root() {
    Ok(path) => path,
    Err(err) => {
        err.print_msg();
        exit(1)
    }
});
static CONFIG: LazyLock<Config> = LazyLock::new(|| match get_config() {
    Ok(config) => config,
    Err(err) => {
        err.print_msg();
        exit(1)
    }
});
static PASSTHROUGH_COPY_GLOBS: LazyLock<GlobSet> = LazyLock::new(|| match compile_globs() {
    Ok(config) => config,
    Err(err) => {
        err.print_msg();
        exit(1)
    }
});

fn compile_globs() -> Result<GlobSet, Error> {
    let mut builder = GlobSetBuilder::new();

    for glob in CONFIG.passthrough_copy.as_ref().unwrap_or(&vec![]) {
        let glob = PROJECT_ROOT
            .join(CONTENT_ROOT)
            .join(glob)
            .to_str()
            .unwrap()
            .to_string();
        log::trace!("passthroughcopy glob is: {glob}");
        builder.add(Glob::new(&glob)?);
    }
    Ok(builder.build()?)
}

fn get_project_root() -> Result<PathBuf, Error> {
    let mut root = std::env::current_dir()?;

    loop {
        let candidate = root.join(CONFIG_FNAME);

        if candidate.exists() {
            return Ok(root);
        }

        if !root.pop() {
            return Err(Error::Io(io::Error::new(
                io::ErrorKind::NotFound,
                format!(
                    "Couldn't find a configuration file (looking for {CONFIG_FNAME}) in the current directory or any parent directories."
                ),
            )));
        }
    }
}

fn get_config() -> Result<Config, Error> {
    let file = PROJECT_ROOT.join(CONFIG_FNAME);
    let contents = fs::read_to_string(file)?;
    let config = toml::from_str(&contents)?;
    Ok(config)
}

fn compile_from_scratch() -> Result<(), Error> {
    log::info!("running init command");
    if CONFIG.init.as_ref().unwrap_or(&vec![]).len() > 0 {
        Command::new(&CONFIG.init.as_ref().unwrap()[0])
            .args(&CONFIG.init.as_ref().unwrap()[1..])
            .spawn()
            .unwrap()
            .wait()
            .unwrap();
        log::trace!("finished init");
    }

    log::info!("starting compilation");
    for entry in WalkDir::new(&*PROJECT_ROOT.join(CONTENT_ROOT))
        .into_iter()
        .filter_map(|e| e.ok())
    {
        if entry.metadata().unwrap().is_file() {
            compile(entry.path())?
        }
    }

    log::info!("compiled project from scratch");

    Ok(())
}

fn compile(path: &Path) -> Result<(), Error> {
    log::trace!("here1 compiling {}", path.to_str().unwrap());
    if PASSTHROUGH_COPY_GLOBS.is_match(path) {
        log::trace!("here2");
        let path_in_src = path.strip_prefix(PROJECT_ROOT.join(CONTENT_ROOT)).unwrap();
        let rel_path = path_in_src.parent().unwrap();
        let parent_dir_in_dst = PROJECT_ROOT.join(OUTPUT_ROOT).join(rel_path);
        let file_in_dst = parent_dir_in_dst.join(path.file_name().unwrap());

        fs::create_dir_all(PROJECT_ROOT.join(OUTPUT_ROOT).join(rel_path))?;
        fs::copy(path, &file_in_dst)?;

        log::trace!(
            "passthroughcopied {} to {}",
            path.to_str().unwrap(),
            file_in_dst.to_str().unwrap()
        );
    } else if path.extension().is_some() && path.extension().unwrap() == "typ" {
        log::trace!("here3");

        if let Ok(_) = path.strip_prefix(PROJECT_ROOT.join(TEMPLATE_ROOT)) {
            compile_from_scratch()?;
            // need to be careful of infinite recursion, compile_everything calls us (compile)
            // should be fine because this code path should only trigger when compiling
            // on the template root.
            //
            // ... what if someone puts their template code in their src folder?
        } else if let Ok(path_in_src) = path.strip_prefix(PROJECT_ROOT.join(CONTENT_ROOT)) {
            let rel_path = path_in_src.parent().unwrap();
            let parent_dir_in_dst = PROJECT_ROOT.join(OUTPUT_ROOT).join(rel_path);
            let mut file_in_dst = parent_dir_in_dst.join(path.file_name().unwrap());
            file_in_dst.set_extension("html");

            fs::create_dir_all(PROJECT_ROOT.join(OUTPUT_ROOT).join(rel_path)).unwrap();
            let mut child = Command::new("typst")
                .arg("c")
                .arg(path)
                .arg("-") // to stdout
                .arg("--features")
                .arg("html")
                .arg("--format")
                .arg("html")
                .arg("--root")
                .arg(&*PROJECT_ROOT)
                .stdout(Stdio::piped())
                .spawn()
                .unwrap();

            if CONFIG.post_processing_typ.as_ref().unwrap_or(&vec![]).len() > 0 {
                child = Command::new(&CONFIG.post_processing_typ.as_ref().unwrap()[0])
                    .args(&CONFIG.post_processing_typ.as_ref().unwrap()[1..])
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

const CONTENT_ROOT: &str = "src";
const OUTPUT_ROOT: &str = "_site";
const TEMPLATE_ROOT: &str = "templates";

fn run() -> Result<(), Error> {
    let args: Args = onlyargs::parse()?;

    SimpleLogger::new()
        .without_timestamps()
        .with_level(LevelFilter::Off)
        .with_module_level(
            "sfn_typst_site",
            if args.trace {
                LevelFilter::Trace
            } else if args.verbose {
                LevelFilter::Debug
            } else {
                LevelFilter::Info
            },
        )
        .init()
        .unwrap();

    log::debug!("project root is {:?}", &*PROJECT_ROOT);

    if args.ignore_initial {
        log::info!("ignoring initial compile from scratch");
    } else {
        compile_from_scratch()?;
    }

    if args.watch {
        let (tx, rx) = mpsc::channel::<DebounceEventResult>();

        let mut debouncer =
            notify_debouncer_full::new_debouncer(Duration::from_millis(200), None, tx)?;

        debouncer.watch(&*PROJECT_ROOT.join(CONTENT_ROOT), RecursiveMode::Recursive)?;
        debouncer.watch(&*PROJECT_ROOT.join(TEMPLATE_ROOT), RecursiveMode::Recursive)?;
        for res in rx {
            match res {
                Ok(events) => {
                    for event in events {
                        if let EventKind::Create(_) | EventKind::Modify(_) = event.kind {
                            for path in &event.event.paths {
                                compile(path)?;
                                log::info!("compiled {}", path.to_str().unwrap());
                            }
                        }
                    }
                }
                Err(errors) => errors.iter().for_each(|error| eprintln!("{error:?}")),
            }
        }
    }

    Ok(())
}

fn main() {
    run().unwrap_or_else(|err| err.print_msg());
}
