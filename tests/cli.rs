use std::process::Command;
use std::thread;

use annulus::parser::MAX_EXPRESSION_DEPTH;
use annulus::{run_source, Phase, RunError, EXECUTION_STACK_BYTES};

const BINARY: &str = env!("CARGO_BIN_EXE_annulus");

fn run(arguments: &[&str]) -> std::process::Output {
    Command::new(BINARY)
        .args(arguments)
        .output()
        .expect("binary should start")
}

#[test]
fn version_flag_prints_the_package_version() {
    let output = run(&["--version"]);
    assert!(output.status.success());
    assert_eq!(
        String::from_utf8_lossy(&output.stdout),
        format!("annulus {}\n", env!("CARGO_PKG_VERSION"))
    );
}

#[test]
fn missing_arguments_print_usage_with_status_two() {
    let output = run(&[]);
    assert_eq!(output.status.code(), Some(2));
    assert!(String::from_utf8_lossy(&output.stderr).starts_with("usage: annulus"));
}

#[test]
fn unknown_flags_print_usage_with_status_two() {
    let output = run(&["--frobnicate"]);
    assert_eq!(output.status.code(), Some(2));
}

#[test]
fn unreadable_file_is_reported_with_status_one() {
    let output = run(&["definitely-missing.annulus"]);
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr)
        .starts_with("error: cannot read definitely-missing.annulus"));
}

#[cfg(unix)]
#[test]
fn closed_output_ends_quietly() {
    let directory = std::env::temp_dir().join(format!("annulus-pipe-{}", std::process::id()));
    std::fs::create_dir_all(&directory).expect("temporary directory");
    let source = directory.join("many.annulus");
    std::fs::write(&source, "PRINT 1;\n".repeat(200_000)).expect("write program");
    let mut child = Command::new(BINARY)
        .arg(&source)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .expect("binary should start");
    drop(child.stdout.take());
    let output = child.wait_with_output().expect("binary should finish");
    std::fs::remove_dir_all(&directory).ok();
    assert!(output.status.success(), "status was {:?}", output.status);
    assert!(
        output.stderr.is_empty(),
        "stderr was {:?}",
        String::from_utf8_lossy(&output.stderr)
    );
}

fn run_on_execution_stack(source: String) -> Result<String, RunError> {
    thread::Builder::new()
        .stack_size(EXECUTION_STACK_BYTES)
        .spawn(move || {
            let mut output = Vec::new();
            run_source(&source, &mut output).map(|()| String::from_utf8_lossy(&output).into_owned())
        })
        .expect("thread should start")
        .join()
        .expect("thread should not overflow its stack")
}

#[test]
fn hostile_nesting_is_rejected_within_the_execution_stack() {
    let depth = 200_000;
    let source = format!("PRINT {}1{};", "(".repeat(depth), ")".repeat(depth));
    match run_on_execution_stack(source) {
        Err(RunError::Diagnostic(diagnostic)) => assert_eq!(diagnostic.phase, Phase::Syntax),
        other => panic!("expected a syntax diagnostic, got {other:?}"),
    }
}

#[test]
fn maximum_nesting_compiles_and_runs_within_the_execution_stack() {
    let depth = MAX_EXPRESSION_DEPTH - 1;
    let source = format!("PRINT {}1{};", "(1 + ".repeat(depth), ")".repeat(depth));
    let output = run_on_execution_stack(source).expect("program should run");
    assert_eq!(output, format!("{}\n", depth + 1));
}

#[test]
fn hostile_operator_chain_is_rejected_within_the_execution_stack() {
    let source = format!("PRINT 1{};", " + 1".repeat(200_000));
    assert!(matches!(
        run_on_execution_stack(source),
        Err(RunError::Diagnostic(_))
    ));
}

#[test]
fn binary_rejects_hostile_nesting_without_crashing() {
    let directory = std::env::temp_dir().join(format!("annulus-deep-{}", std::process::id()));
    std::fs::create_dir_all(&directory).expect("temporary directory");
    let source = directory.join("deep.annulus");
    let depth = 200_000;
    std::fs::write(
        &source,
        format!("PRINT {}1{};", "(".repeat(depth), ")".repeat(depth)),
    )
    .expect("write program");
    let output = run(&[source.to_str().expect("UTF-8 path")]);
    std::fs::remove_dir_all(&directory).ok();
    assert_eq!(output.status.code(), Some(1));
    assert!(String::from_utf8_lossy(&output.stderr).contains("expression is nested too deeply"));
}
