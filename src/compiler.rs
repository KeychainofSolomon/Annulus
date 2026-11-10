use std::collections::HashMap;

use crate::ast::{BinaryOperator, Expr, ExprKind, Stmt, StmtKind, UnaryOperator};
use crate::diagnostic::{Diagnostic, Phase};
use crate::program::{Instruction, Program};
use crate::span::Span;
use crate::value::Value;

pub fn compile(statements: &[Stmt]) -> Result<Program, Diagnostic> {
    let mut compiler = Compiler::default();
    for statement in statements {
        compiler.statement(statement)?;
    }
    let end = compiler
        .program
        .steps
        .last()
        .map_or(Span::START, |step| step.span);
    compiler.program.emit(Instruction::Halt, end);
    compiler.program.slot_count = compiler.declarations.len();
    Ok(compiler.program)
}

#[derive(Clone, Copy, Debug)]
struct Declaration {
    slot: usize,
    declared_at: Span,
}

#[derive(Default)]
struct Compiler {
    program: Program,
    declarations: HashMap<String, Declaration>,
}

impl Compiler {
    fn statement(&mut self, statement: &Stmt) -> Result<(), Diagnostic> {
        match &statement.kind {
            StmtKind::Declare {
                name,
                name_span,
                value,
            } => {
                self.expression(value)?;
                let slot = self.declare(name, *name_span)?;
                self.program.emit(Instruction::Store(slot), statement.span);
            }
            StmtKind::Manifest(value) => {
                self.expression(value)?;
                self.program.emit(Instruction::Manifest, statement.span);
            }
            StmtKind::Evaluate(value) => {
                self.expression(value)?;
                self.program.emit(Instruction::Discard, statement.span);
            }
        }
        Ok(())
    }

    fn declare(&mut self, name: &str, span: Span) -> Result<usize, Diagnostic> {
        if let Some(existing) = self.declarations.get(name) {
            return Err(Diagnostic::new(
                Phase::Name,
                format!("`{name}` is already declared"),
                span,
            )
            .with_hint(format!(
                "declarations are immutable; `{name}` was first declared at {}",
                existing.declared_at
            )));
        }
        let slot = self.declarations.len();
        self.declarations.insert(
            name.to_owned(),
            Declaration {
                slot,
                declared_at: span,
            },
        );
        Ok(slot)
    }

    fn expression(&mut self, expression: &Expr) -> Result<(), Diagnostic> {
        match &expression.kind {
            ExprKind::Literal(literal) => {
                self.program
                    .emit(Instruction::Push(Value::from(literal)), expression.span);
            }
            ExprKind::Name(name) => {
                let declaration = self.declarations.get(name).ok_or_else(|| {
                    Diagnostic::new(
                        Phase::Name,
                        format!("`{name}` is not declared"),
                        expression.span,
                    )
                })?;
                self.program
                    .emit(Instruction::Load(declaration.slot), expression.span);
            }
            ExprKind::Unary { operator, operand } => {
                self.expression(operand)?;
                let instruction = match operator {
                    UnaryOperator::Negate => Instruction::Negate,
                };
                self.program.emit(instruction, expression.span);
            }
            ExprKind::Binary {
                operator,
                left,
                right,
            } => {
                self.expression(left)?;
                self.expression(right)?;
                let instruction = match operator {
                    BinaryOperator::Add => Instruction::Add,
                    BinaryOperator::Subtract => Instruction::Subtract,
                    BinaryOperator::Multiply => Instruction::Multiply,
                    BinaryOperator::Divide => Instruction::Divide,
                    BinaryOperator::Equal => Instruction::Equal,
                };
                self.program.emit(instruction, expression.span);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::lexer::tokenize;
    use crate::parser::parse;

    fn compile_source(source: &str) -> Result<Program, Diagnostic> {
        compile(&parse(tokenize(source).expect("should tokenize")).expect("should parse"))
    }

    #[test]
    fn declarations_receive_sequential_slots() {
        let program = compile_source("LET a = 1; LET b = a + 2; PRINT b;").expect("should compile");
        assert_eq!(program.slot_count, 2);
        assert_eq!(
            program.instructions().cloned().collect::<Vec<_>>(),
            vec![
                Instruction::Push(Value::Integer(1)),
                Instruction::Store(0),
                Instruction::Load(0),
                Instruction::Push(Value::Integer(2)),
                Instruction::Add,
                Instruction::Store(1),
                Instruction::Load(1),
                Instruction::Manifest,
                Instruction::Halt,
            ]
        );
    }

    #[test]
    fn expression_statements_discard_their_value() {
        let program = compile_source("1;").expect("should compile");
        assert_eq!(
            program.instructions().cloned().collect::<Vec<_>>(),
            vec![
                Instruction::Push(Value::Integer(1)),
                Instruction::Discard,
                Instruction::Halt,
            ]
        );
    }

    #[test]
    fn empty_program_halts() {
        let program = compile_source("").expect("should compile");
        assert_eq!(
            program.instructions().cloned().collect::<Vec<_>>(),
            vec![Instruction::Halt]
        );
    }

    #[test]
    fn undeclared_name_is_rejected_before_execution() {
        let diagnostic = compile_source("PRINT \"before\";\nPRINT nope;").expect_err("should fail");
        assert_eq!(diagnostic.phase, Phase::Name);
        assert_eq!(diagnostic.message, "`nope` is not declared");
        assert_eq!(diagnostic.span, Span::new(2, 7));
    }

    #[test]
    fn redeclaration_is_rejected_with_the_original_location() {
        let diagnostic = compile_source("LET a = 1;\nLET a = 2;").expect_err("should fail");
        assert_eq!(diagnostic.message, "`a` is already declared");
        assert_eq!(diagnostic.span, Span::new(2, 5));
        assert_eq!(
            diagnostic.hint.as_deref(),
            Some("declarations are immutable; `a` was first declared at 1:5")
        );
    }

    #[test]
    fn a_declaration_cannot_refer_to_itself() {
        let diagnostic = compile_source("LET a = a;").expect_err("should fail");
        assert_eq!(diagnostic.message, "`a` is not declared");
    }
}
