use std::io::{self, Write};

use crate::diagnostic::{Diagnostic, Phase};
use crate::program::{Instruction, Program};
use crate::span::Span;
use crate::value::Value;

#[derive(Debug)]
pub enum RunError {
    Diagnostic(Diagnostic),
    Output(io::Error),
}

impl From<Diagnostic> for RunError {
    fn from(diagnostic: Diagnostic) -> Self {
        RunError::Diagnostic(diagnostic)
    }
}

pub fn execute(program: &Program, output: &mut dyn Write) -> Result<(), RunError> {
    Machine::new(program, output).run()
}

struct Machine<'run> {
    program: &'run Program,
    output: &'run mut dyn Write,
    stack: Vec<Value>,
    slots: Vec<Option<Value>>,
}

impl<'run> Machine<'run> {
    fn new(program: &'run Program, output: &'run mut dyn Write) -> Self {
        Self {
            program,
            output,
            stack: Vec::new(),
            slots: vec![None; program.slot_count],
        }
    }

    fn run(&mut self) -> Result<(), RunError> {
        let mut counter = 0;
        loop {
            let Some(step) = self.program.steps.get(counter) else {
                let span = self
                    .program
                    .steps
                    .last()
                    .map_or(Span::START, |step| step.span);
                return Err(internal("execution ran past the end of the program", span).into());
            };
            counter += 1;
            let span = step.span;
            match &step.instruction {
                Instruction::Push(value) => self.stack.push(value.clone()),
                Instruction::Load(slot) => {
                    let value = self
                        .slots
                        .get(*slot)
                        .and_then(Option::clone)
                        .ok_or_else(|| internal("read of an unbound slot", span))?;
                    self.stack.push(value);
                }
                Instruction::Store(slot) => {
                    let value = self.pop(span)?;
                    let cell = self
                        .slots
                        .get_mut(*slot)
                        .ok_or_else(|| internal("store to a slot outside the program", span))?;
                    if cell.is_some() {
                        return Err(internal("store to an already bound slot", span).into());
                    }
                    *cell = Some(value);
                }
                Instruction::Add => self.arithmetic("+", span, i64::checked_add)?,
                Instruction::Subtract => self.arithmetic("-", span, i64::checked_sub)?,
                Instruction::Multiply => self.arithmetic("*", span, i64::checked_mul)?,
                Instruction::Divide => self.divide(span)?,
                Instruction::Equal => {
                    let right = self.pop(span)?;
                    let left = self.pop(span)?;
                    self.stack.push(Value::Boolean(left == right));
                }
                Instruction::Negate => {
                    let operand = self.pop(span)?;
                    let Value::Integer(value) = operand else {
                        return Err(runtime(
                            format!(
                                "operator `-` requires an integer operand, found {}",
                                operand.type_name()
                            ),
                            span,
                        )
                        .into());
                    };
                    let negated = value
                        .checked_neg()
                        .ok_or_else(|| runtime(format!("integer overflow in -({value})"), span))?;
                    self.stack.push(Value::Integer(negated));
                }
                Instruction::Manifest => {
                    let value = self.pop(span)?;
                    writeln!(self.output, "{value}").map_err(RunError::Output)?;
                }
                Instruction::Discard => {
                    self.pop(span)?;
                }
                Instruction::Halt => return Ok(()),
            }
        }
    }

    fn pop(&mut self, span: Span) -> Result<Value, Diagnostic> {
        self.stack
            .pop()
            .ok_or_else(|| internal("operand stack underflow", span))
    }

    fn integer_operands(&mut self, symbol: &str, span: Span) -> Result<(i64, i64), Diagnostic> {
        let right = self.pop(span)?;
        let left = self.pop(span)?;
        match (&left, &right) {
            (Value::Integer(left), Value::Integer(right)) => Ok((*left, *right)),
            _ => Err(runtime(
                format!(
                    "operator `{symbol}` requires integer operands, found {} and {}",
                    left.type_name(),
                    right.type_name()
                ),
                span,
            )),
        }
    }

    fn arithmetic(
        &mut self,
        symbol: &str,
        span: Span,
        operation: fn(i64, i64) -> Option<i64>,
    ) -> Result<(), Diagnostic> {
        let (left, right) = self.integer_operands(symbol, span)?;
        let result = operation(left, right)
            .ok_or_else(|| runtime(format!("integer overflow in {left} {symbol} {right}"), span))?;
        self.stack.push(Value::Integer(result));
        Ok(())
    }

    fn divide(&mut self, span: Span) -> Result<(), Diagnostic> {
        let (left, right) = self.integer_operands("/", span)?;
        if right == 0 {
            return Err(runtime("division by zero", span));
        }
        let result = left
            .checked_div(right)
            .ok_or_else(|| runtime(format!("integer overflow in {left} / {right}"), span))?;
        self.stack.push(Value::Integer(result));
        Ok(())
    }
}

fn runtime(message: impl Into<String>, span: Span) -> Diagnostic {
    Diagnostic::new(Phase::Runtime, message, span)
}

fn internal(message: &str, span: Span) -> Diagnostic {
    Diagnostic::new(Phase::Internal, message, span)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::program::Step;

    fn run_program(program: &Program) -> (Result<(), RunError>, String) {
        let mut output = Vec::new();
        let result = execute(program, &mut output);
        (
            result,
            String::from_utf8(output).expect("output should be UTF-8"),
        )
    }

    fn program(instructions: Vec<Instruction>, slot_count: usize) -> Program {
        Program {
            steps: instructions
                .into_iter()
                .map(|instruction| Step {
                    instruction,
                    span: Span::START,
                })
                .collect(),
            slot_count,
        }
    }

    fn diagnostic(result: Result<(), RunError>) -> Diagnostic {
        match result {
            Err(RunError::Diagnostic(diagnostic)) => diagnostic,
            other => panic!("expected a diagnostic, got {other:?}"),
        }
    }

    #[test]
    fn manifests_values_in_order() {
        let (result, output) = run_program(&program(
            vec![
                Instruction::Push(Value::Integer(7)),
                Instruction::Manifest,
                Instruction::Push(Value::Boolean(false)),
                Instruction::Manifest,
                Instruction::Push(Value::Text("light".into())),
                Instruction::Manifest,
                Instruction::Halt,
            ],
            0,
        ));
        assert!(result.is_ok());
        assert_eq!(output, "7\nfalse\nlight\n");
    }

    #[test]
    fn stack_underflow_is_an_internal_error() {
        let (result, _) = run_program(&program(vec![Instruction::Manifest], 0));
        assert_eq!(diagnostic(result).phase, Phase::Internal);
    }

    #[test]
    fn missing_halt_is_an_internal_error() {
        let (result, _) = run_program(&program(vec![], 0));
        assert_eq!(
            diagnostic(result).message,
            "execution ran past the end of the program"
        );
    }

    #[test]
    fn rebinding_a_slot_is_an_internal_error() {
        let (result, _) = run_program(&program(
            vec![
                Instruction::Push(Value::Integer(1)),
                Instruction::Store(0),
                Instruction::Push(Value::Integer(2)),
                Instruction::Store(0),
                Instruction::Halt,
            ],
            1,
        ));
        assert_eq!(diagnostic(result).message, "store to an already bound slot");
    }

    #[test]
    fn reading_an_unbound_slot_is_an_internal_error() {
        let (result, _) = run_program(&program(vec![Instruction::Load(0), Instruction::Halt], 1));
        assert_eq!(diagnostic(result).message, "read of an unbound slot");
    }

    #[test]
    fn output_failure_is_reported_separately() {
        struct Closed;
        impl Write for Closed {
            fn write(&mut self, _: &[u8]) -> io::Result<usize> {
                Err(io::Error::from(io::ErrorKind::BrokenPipe))
            }
            fn flush(&mut self) -> io::Result<()> {
                Ok(())
            }
        }
        let program = program(
            vec![
                Instruction::Push(Value::Integer(1)),
                Instruction::Manifest,
                Instruction::Halt,
            ],
            0,
        );
        match execute(&program, &mut Closed) {
            Err(RunError::Output(error)) => assert_eq!(error.kind(), io::ErrorKind::BrokenPipe),
            other => panic!("expected an output error, got {other:?}"),
        }
    }
}
