use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

const BINARY: &str = env!("CARGO_BIN_EXE_annulus");

struct Expectation {
    stdout: String,
    stderr: String,
    status: i32,
}

fn read_optional(path: &Path) -> Option<String> {
    fs::read_to_string(path).ok()
}

fn expectation_for(source: &Path) -> Expectation {
    let stdout_path = source.with_extension("stdout");
    let stdout = read_optional(&stdout_path)
        .unwrap_or_else(|| panic!("missing expected output {}", stdout_path.display()));
    let stderr = read_optional(&source.with_extension("stderr")).unwrap_or_default();
    let status = read_optional(&source.with_extension("status"))
        .map(|text| {
            text.trim()
                .parse()
                .expect("status file must hold an integer")
        })
        .unwrap_or(0);
    Expectation {
        stdout,
        stderr,
        status,
    }
}

fn sources_in(directory: &str) -> Vec<PathBuf> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join(directory);
    let mut sources: Vec<PathBuf> = fs::read_dir(&root)
        .unwrap_or_else(|error| panic!("cannot list {}: {error}", root.display()))
        .map(|entry| entry.expect("directory entry").path())
        .filter(|path| path.extension().is_some_and(|extension| extension == "annulus"))
        .collect();
    sources.sort();
    sources
}

fn verify_directory(directory: &str) {
    let sources = sources_in(directory);
    assert!(!sources.is_empty(), "no programs found in {directory}");
    let mut failures = Vec::new();
    for source in &sources {
        let expected = expectation_for(source);
        let file_name = source.file_name().expect("file name");
        let output = Command::new(BINARY)
            .arg(file_name)
            .current_dir(source.parent().expect("parent directory"))
            .output()
            .expect("binary should start");
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        let status = output.status.code().unwrap_or(-1);
        if stdout != expected.stdout || stderr != expected.stderr || status != expected.status {
            failures.push(format!(
                "{}\n  status: expected {}, got {status}\n  stdout: expected {:?}, got {stdout:?}\n  stderr: expected {:?}, got {stderr:?}",
                source.display(),
                expected.status,
                expected.stdout,
                expected.stderr,
            ));
        }
    }
    assert!(
        failures.is_empty(),
        "{} of {} programs failed:\n{}",
        failures.len(),
        sources.len(),
        failures.join("\n")
    );
}

#[test]
fn example_programs_produce_their_recorded_output() {
    verify_directory("examples");
}

#[test]
fn test_programs_produce_their_recorded_output_and_diagnostics() {
    verify_directory("tests/programs");
}
