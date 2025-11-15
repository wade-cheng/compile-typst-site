//! Testing utilities.

use std::{
    env, fs, io,
    path::PathBuf,
    process::{Command, Output},
    sync::mpsc::{self, RecvTimeoutError},
    thread,
    time::Duration,
};

pub struct IntegrationTest {
    /// The full path to the `compile-typst-site` project root for this test.
    project_root: PathBuf,
    timeout: Duration,
    args: Vec<String>,
}

impl IntegrationTest {
    pub fn new(testname: &str) -> Self {
        let project_root = env::current_dir()
            .unwrap()
            .join("tests/integration_test_contents")
            .join(testname);

        Self {
            project_root,
            timeout: Duration::from_secs(5),
            args: Vec::new(),
        }
    }

    pub fn args(mut self, args: Vec<String>) -> Self {
        self.args = args;
        self
    }

    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Runs the test, returning the project root and the output of the test on success.
    pub fn run(&self) -> io::Result<(PathBuf, Output)> {
        thread::scope(|s| {
            let _dont_validate_removal = fs::remove_dir_all(self.project_root.join("_site"));

            let (tx, rx) = mpsc::channel();

            // we don't join this, just let it run out.
            // if it's borked, it's borked, if it's not, we get output sent.
            s.spawn(move || {
                let command = env::current_dir()
                    .unwrap()
                    .join("target/debug/compile-typst-site");
                let output = Command::new(&command)
                    .arg("--path")
                    .arg(&self.project_root)
                    .args(&self.args)
                    .output()
                    .expect("args are all developer-hardcoded, should be correct");
                tx.send(output).unwrap();
            });

            use std::io::ErrorKind as E;
            match rx.recv_timeout(self.timeout) {
                Ok(output) => Ok((self.project_root.clone(), output)),
                Err(RecvTimeoutError::Timeout) => Err(E::TimedOut.into()),
                Err(RecvTimeoutError::Disconnected) => panic!("runner disconnected"),
            }
        })
    }
}
