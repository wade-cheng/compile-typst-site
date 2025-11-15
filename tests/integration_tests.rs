mod util;
use std::{env, fs, io};

use compile_typst_site::internals::config::CONFIG_FNAME;

use crate::util::IntegrationTest;

#[test]
fn blank_project_does_nothing() {
    let (project_root, output) = IntegrationTest::new("blank_project").run().unwrap();

    assert!(output.status.success());
    let project_root_files: Vec<_> = fs::read_dir(project_root)
        .unwrap()
        .map(|file| file.unwrap())
        .collect();

    assert_eq!(project_root_files.len(), 1);
    assert_eq!(project_root_files[0].file_name(), CONFIG_FNAME);
}

#[test]
fn empty_folder_causes_error() -> io::Result<()> {
    const EMPTY_DIR: &str = "empty_dir";
    fs::create_dir_all(
        env::current_dir()?
            .join("tests/integration_test_contents")
            .join(EMPTY_DIR),
    )?;
    let (_, output) = IntegrationTest::new(EMPTY_DIR).run()?;

    assert!(!output.status.success());

    Ok(())
}

#[test]
fn junk_folder_causes_error() {
    let (project_root, output) = IntegrationTest::new("junk").run().unwrap();
    let mut project_root_filenames: Vec<_> = fs::read_dir(project_root)
        .unwrap()
        .map(|file| file.unwrap().file_name())
        .collect();
    project_root_filenames.sort();

    assert!(!output.status.success());
    assert_eq!(project_root_filenames, ["1.txt", "2.txt", "3.txt"]);
}

#[test]
fn just_empty_src_does_nothing() {
    fs::create_dir_all(
        env::current_dir()
            .unwrap()
            .join("tests/integration_test_contents/just_empty_src/src"),
    )
    .unwrap();
    let (_, output) = IntegrationTest::new("just_empty_src").run().unwrap();

    assert!(output.status.success());
}

#[test]
fn just_empty_templates_does_nothing() {
    fs::create_dir_all(
        env::current_dir()
            .unwrap()
            .join("tests/integration_test_contents/just_empty_templates/templates"),
    )
    .unwrap();
    let (_, output) = IntegrationTest::new("just_empty_templates").run().unwrap();

    assert!(output.status.success());
}

#[test]
fn just_populated_templates_does_nothing() {
    let (_, output) = IntegrationTest::new("just_populated_templates")
        .run()
        .unwrap();

    assert!(output.status.success());
}

#[test]
fn simple_test_succeeds() {
    let (_, output) = IntegrationTest::new("simple").run().unwrap();

    assert!(output.status.success());
}

/// This one's roughly a mirror of my personal site from 2025-11-15.
///
/// Should be a real stress test.
#[test]
fn wade_mirror_succeeds() {
    let (_, output) = IntegrationTest::new("wade-mirror").run().unwrap();

    println!("stdout: {}", String::from_utf8(output.stdout).unwrap());
    println!("stderr: {}", String::from_utf8(output.stderr).unwrap());

    assert!(output.status.success());
}

/// This one's a mirror of the "hardcoded links example" (1) from 2025-11-15.
///
/// Simple, but has a template.
///
/// (1) https://github.com/wade-cheng/compile-typst-site-hardcoded-links-example
#[test]
fn medium_succeeds() {
    let (_, output) = IntegrationTest::new("hardcoded_links_example")
        .run()
        .unwrap();

    println!("stdout: {}", String::from_utf8(output.stdout).unwrap());
    println!("stderr: {}", String::from_utf8(output.stderr).unwrap());

    assert!(output.status.success());
}
