pub mod ast;
pub mod baculum;
pub mod compiler;
pub mod diagnostic;
pub mod horae;
pub mod lexer;
pub mod lingua;
pub mod mandata;
pub mod nucleus;
pub mod officia;
pub mod parser;
pub mod program;
pub mod ritus;
pub mod span;
pub mod tabularium;
pub mod token;
pub mod value;
pub mod vm;

use std::io::Write;

pub use diagnostic::{Diagnostic, Phase};
pub use program::Program;
pub use vm::RunError;

pub const EXECUTION_STACK_BYTES: usize = 16 * 1024 * 1024;

pub fn compile_source(source: &str) -> Result<Program, Diagnostic> {
    let tokens = lexer::tokenize(source)?;
    let statements = parser::parse(tokens)?;
    compiler::compile(&statements)
}

pub fn run_source(source: &str, output: &mut dyn Write) -> Result<(), RunError> {
    let program = compile_source(source)?;
    vm::execute(&program, output)
}
