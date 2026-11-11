pub mod check;
pub mod json;
pub mod lexis;
pub mod parse;
pub mod registra;
pub mod syntaxis;

use lexis::Pos;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Phase {
    Lexis,
    Syntaxis,
    Nomina,
    Typi,
    Auctoritas,
    Subset,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Error {
    pub code: &'static str,
    pub pos: Pos,
    pub message: String,
}

impl Error {
    pub fn new(code: &'static str, pos: Pos, message: String) -> Self {
        Error { code, pos, message }
    }

    pub fn phase(&self) -> Phase {
        match self.code.as_bytes().first() {
            Some(b'L') => Phase::Lexis,
            Some(b'P') => Phase::Syntaxis,
            Some(b'N') => Phase::Nomina,
            Some(b'T') => Phase::Typi,
            Some(b'A') => Phase::Auctoritas,
            _ => Phase::Subset,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}: {}", self.pos, self.code, self.message)
    }
}

fn on_stack<T: Send>(work: impl FnOnce() -> T + Send) -> T {
    std::thread::scope(|scope| {
        std::thread::Builder::new()
            .stack_size(crate::EXECUTION_STACK_BYTES)
            .spawn_scoped(scope, work)
            .expect("a thread for the checker")
            .join()
            .expect("the checker does not panic")
    })
}

pub fn parse_source(source: &str) -> Result<syntaxis::Program, Error> {
    on_stack(|| parse::parse(lexis::tokenize(source)?))
}

pub fn check_source(source: &str) -> Result<check::Checked, Vec<Error>> {
    on_stack(|| {
        let program =
            parse::parse(lexis::tokenize(source).map_err(|e| vec![e])?).map_err(|e| vec![e])?;
        let checked = check::check(&program, registra::goetia());
        drop(program);
        checked
    })
}
