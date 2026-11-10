use std::env;
use std::fs;
use std::io::{self, BufWriter, Write};
use std::process::ExitCode;
use std::thread;

use annulus::{run_source, RunError, EXECUTION_STACK_BYTES};

const USAGE: &str = "usage: annulus <source.ann>\n       annulus --version";
const EXIT_PROGRAM_FAILURE: u8 = 1;
const EXIT_USAGE: u8 = 2;

fn main() -> ExitCode {
    let arguments: Vec<String> = env::args().skip(1).collect();
    match arguments.as_slice() {
        [flag] if flag == "--version" => {
            print_line(&format!("annulus {}", env!("CARGO_PKG_VERSION")))
        }
        [flag] if flag == "--help" || flag == "-h" => print_line(USAGE),
        [path] if !path.starts_with('-') => run_file(path),
        _ => {
            report(USAGE);
            ExitCode::from(EXIT_USAGE)
        }
    }
}

fn run_file(path: &str) -> ExitCode {
    let source = match fs::read_to_string(path) {
        Ok(source) => source,
        Err(error) => {
            report(&format!("error: cannot read {path}: {error}"));
            return ExitCode::from(EXIT_PROGRAM_FAILURE);
        }
    };
    let execution = thread::Builder::new()
        .name("annulus".into())
        .stack_size(EXECUTION_STACK_BYTES)
        .spawn(move || {
            let stdout = io::stdout();
            let mut output = BufWriter::new(stdout.lock());
            let result = run_source(&source, &mut output);
            let flushed = output.flush();
            (result, flushed)
        });
    let outcome = match execution.map(thread::JoinHandle::join) {
        Ok(Ok(outcome)) => outcome,
        Ok(Err(_)) | Err(_) => {
            report("error: the interpreter could not complete");
            return ExitCode::from(EXIT_PROGRAM_FAILURE);
        }
    };
    match outcome {
        (Ok(()), Ok(())) => ExitCode::SUCCESS,
        (Err(RunError::Output(error)), _) | (Ok(()), Err(error)) => output_failure(&error),
        (Err(RunError::Diagnostic(diagnostic)), _) => {
            report(&diagnostic.render(path));
            ExitCode::from(EXIT_PROGRAM_FAILURE)
        }
    }
}

fn output_failure(error: &io::Error) -> ExitCode {
    if error.kind() == io::ErrorKind::BrokenPipe {
        return ExitCode::SUCCESS;
    }
    report(&format!("error: cannot write output: {error}"));
    ExitCode::from(EXIT_PROGRAM_FAILURE)
}

fn print_line(text: &str) -> ExitCode {
    match writeln!(io::stdout(), "{text}") {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => output_failure(&error),
    }
}

fn report(message: &str) {
    let _ = writeln!(io::stderr(), "{message}");
}
