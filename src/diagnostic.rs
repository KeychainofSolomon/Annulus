use std::fmt;

use crate::span::Span;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Phase {
    Syntax,
    Name,
    Runtime,
    Internal,
}

impl fmt::Display for Phase {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Phase::Syntax => "syntax error",
            Phase::Name => "name error",
            Phase::Runtime => "runtime error",
            Phase::Internal => "internal error",
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Diagnostic {
    pub phase: Phase,
    pub message: String,
    pub span: Span,
    pub hint: Option<String>,
}

impl Diagnostic {
    pub fn new(phase: Phase, message: impl Into<String>, span: Span) -> Self {
        Self {
            phase,
            message: message.into(),
            span,
            hint: None,
        }
    }

    pub fn with_hint(mut self, hint: impl Into<String>) -> Self {
        self.hint = Some(hint.into());
        self
    }

    pub fn render(&self, origin: &str) -> String {
        let mut rendered = format!("{origin}:{}: {}: {}", self.span, self.phase, self.message);
        if let Some(hint) = &self.hint {
            rendered.push_str("\n  hint: ");
            rendered.push_str(hint);
        }
        rendered
    }
}
