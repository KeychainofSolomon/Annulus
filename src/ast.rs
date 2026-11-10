use crate::span::Span;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Equal,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UnaryOperator {
    Negate,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Literal {
    Integer(i64),
    Boolean(bool),
    Text(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExprKind {
    Literal(Literal),
    Name(String),
    Unary {
        operator: UnaryOperator,
        operand: Box<Expr>,
    },
    Binary {
        operator: BinaryOperator,
        left: Box<Expr>,
        right: Box<Expr>,
    },
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Expr {
    pub kind: ExprKind,
    pub span: Span,
    pub depth: usize,
}

impl Expr {
    pub fn literal(literal: Literal, span: Span) -> Self {
        Self {
            kind: ExprKind::Literal(literal),
            span,
            depth: 1,
        }
    }

    pub fn name(name: String, span: Span) -> Self {
        Self {
            kind: ExprKind::Name(name),
            span,
            depth: 1,
        }
    }

    pub fn unary(operator: UnaryOperator, operand: Expr, span: Span) -> Self {
        let depth = operand.depth + 1;
        Self {
            kind: ExprKind::Unary {
                operator,
                operand: Box::new(operand),
            },
            span,
            depth,
        }
    }

    pub fn binary(operator: BinaryOperator, left: Expr, right: Expr, span: Span) -> Self {
        let depth = left.depth.max(right.depth) + 1;
        Self {
            kind: ExprKind::Binary {
                operator,
                left: Box::new(left),
                right: Box::new(right),
            },
            span,
            depth,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StmtKind {
    Declare {
        name: String,
        name_span: Span,
        value: Expr,
    },
    Manifest(Expr),
    Evaluate(Expr),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}
