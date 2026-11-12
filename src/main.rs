use std::env;
use std::fs;
use std::io::{self, BufWriter, Read, Write};
use std::process::ExitCode;
use std::thread;

use annulus::mandata::{self, Usage, EXIT_USAGE, USAGE};
use annulus::{run_source, RunError, EXECUTION_STACK_BYTES};

const EXIT_PROGRAM_FAILURE: u8 = 1;

type Command = fn(&[String], &mut dyn Write, &mut dyn Write) -> Result<u8, Usage>;

fn command(run: Command, args: &[String]) -> ExitCode {
    let mut out = io::stdout().lock();
    let mut err = io::stderr().lock();
    match run(args, &mut out, &mut err) {
        Ok(code) => ExitCode::from(code),
        Err(Usage(why)) => {
            let _ = writeln!(err, "error: {why}\n{USAGE}");
            ExitCode::from(EXIT_USAGE)
        }
    }
}

fn main() -> ExitCode {
    let arguments: Vec<String> = env::args().skip(1).collect();
    match arguments.as_slice() {
        [flag] if flag == "--version" => {
            print_line(&format!("annulus {}", env!("CARGO_PKG_VERSION")))
        }
        [flag] if flag == "--help" || flag == "-h" => print_line(USAGE),
        [name, rest @ ..] if name == "ritus" => command(mandata::ritus, rest),
        [name, rest @ ..] if name == "confirma" => command(mandata::confirma, rest),
        [name, rest @ ..] if name == "petitiones" => command(mandata::petitiones, rest),
        [name, rest @ ..] if name == "adopta" => command(mandata::adopta, rest),
        [path] if !path.starts_with('-') => run_file(path),
        _ => {
            report(USAGE);
            ExitCode::from(EXIT_USAGE)
        }
    }
}

fn read_bounded(path: &str) -> io::Result<Option<String>> {
    let mut source = String::new();
    fs::File::open(path)?
        .take(annulus::lingua::lexis::MAX_SOURCE_BYTES as u64 + 1)
        .read_to_string(&mut source)?;
    Ok((source.len() <= annulus::lingua::lexis::MAX_SOURCE_BYTES).then_some(source))
}

fn run_file(path: &str) -> ExitCode {
    let source = match read_bounded(path) {
        Ok(Some(source)) => source,
        Ok(None) => {
            report(&format!(
                "error: {path} is larger than {} bytes",
                annulus::lingua::lexis::MAX_SOURCE_BYTES
            ));
            return ExitCode::from(EXIT_PROGRAM_FAILURE);
        }
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
