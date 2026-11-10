use crate::span::Span;
use crate::value::Value;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Instruction {
    Push(Value),
    Load(usize),
    Store(usize),
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
    Negate,
    Manifest,
    Discard,
    Halt,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Step {
    pub instruction: Instruction,
    pub span: Span,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Program {
    pub steps: Vec<Step>,
    pub slot_count: usize,
}

impl Program {
    pub fn emit(&mut self, instruction: Instruction, span: Span) {
        self.steps.push(Step { instruction, span });
    }

    pub fn instructions(&self) -> impl Iterator<Item = &Instruction> {
        self.steps.iter().map(|step| &step.instruction)
    }
}
