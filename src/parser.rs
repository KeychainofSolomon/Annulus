use crate::ast::{BinaryOperator, Expr, Literal, Stmt, StmtKind, UnaryOperator};
use crate::diagnostic::{Diagnostic, Phase};
use crate::span::Span;
use crate::token::{keyword, Token, TokenKind};

pub const MAX_EXPRESSION_DEPTH: usize = 256;

pub fn parse(tokens: Vec<Token>) -> Result<Vec<Stmt>, Diagnostic> {
    let mut parser = Parser::new(tokens);
    let mut statements = Vec::new();
    while parser.peek().kind != TokenKind::EndOfInput {
        statements.push(parser.statement()?);
    }
    Ok(statements)
}

struct Parser {
    tokens: Vec<Token>,
    position: usize,
    nesting: usize,
}

impl Parser {
    fn new(mut tokens: Vec<Token>) -> Self {
        if tokens
            .last()
            .is_none_or(|token| token.kind != TokenKind::EndOfInput)
        {
            let span = tokens.last().map_or(Span::START, |token| token.span);
            tokens.push(Token {
                kind: TokenKind::EndOfInput,
                span,
            });
        }
        Self {
            tokens,
            position: 0,
            nesting: 0,
        }
    }

    fn peek(&self) -> &Token {
        &self.tokens[self.position]
    }

    fn advance(&mut self) -> Token {
        let token = self.peek().clone();
        if token.kind != TokenKind::EndOfInput {
            self.position += 1;
        }
        token
    }

    fn expect(&mut self, kind: &TokenKind, context: &str) -> Result<Token, Diagnostic> {
        if &self.peek().kind == kind {
            Ok(self.advance())
        } else {
            Err(self.unexpected(&format!("expected {kind} {context}")))
        }
    }

    fn unexpected(&self, expectation: &str) -> Diagnostic {
        let token = self.peek();
        Diagnostic::new(
            Phase::Syntax,
            format!("{expectation}, found {}", token.kind),
            token.span,
        )
    }

    fn statement(&mut self) -> Result<Stmt, Diagnostic> {
        match self.peek().kind {
            TokenKind::Declare => self.declaration(),
            TokenKind::Manifest => self.manifestation(),
            _ => self.evaluation(),
        }
    }

    fn declaration(&mut self) -> Result<Stmt, Diagnostic> {
        let keyword_token = self.advance();
        let name_token = self.advance();
        let TokenKind::Identifier(name) = name_token.kind else {
            return Err(Diagnostic::new(
                Phase::Syntax,
                format!(
                    "expected a name after the declaration keyword, found {}",
                    name_token.kind
                ),
                name_token.span,
            ));
        };
        self.expect(&TokenKind::Assign, "after the declared name")?;
        let value = self.expression()?;
        self.expect(&TokenKind::Semicolon, "after the declaration")?;
        Ok(Stmt {
            kind: StmtKind::Declare {
                name,
                name_span: name_token.span,
                value,
            },
            span: keyword_token.span,
        })
    }

    fn manifestation(&mut self) -> Result<Stmt, Diagnostic> {
        let keyword_token = self.advance();
        let value = self.expression()?;
        self.expect(&TokenKind::Semicolon, "after the output expression")?;
        Ok(Stmt {
            kind: StmtKind::Manifest(value),
            span: keyword_token.span,
        })
    }

    fn evaluation(&mut self) -> Result<Stmt, Diagnostic> {
        let leading = self.peek().clone();
        let value = self.expression()?;
        self.expect(&TokenKind::Semicolon, "after the expression")
            .map_err(|diagnostic| match uppercase_keyword_hint(&leading.kind) {
                Some(hint) => diagnostic.with_hint(hint),
                None => diagnostic,
            })?;
        Ok(Stmt {
            kind: StmtKind::Evaluate(value),
            span: leading.span,
        })
    }

    fn expression(&mut self) -> Result<Expr, Diagnostic> {
        self.binary(0)
    }

    fn binary(&mut self, minimum_precedence: u8) -> Result<Expr, Diagnostic> {
        let mut left = self.unary()?;
        while let Some((operator, precedence)) = binary_operator(&self.peek().kind) {
            if precedence < minimum_precedence {
                break;
            }
            let operator_token = self.advance();
            let right = self.binary(precedence + 1)?;
            left = bounded(Expr::binary(operator, left, right, operator_token.span))?;
        }
        Ok(left)
    }

    fn unary(&mut self) -> Result<Expr, Diagnostic> {
        if self.peek().kind != TokenKind::Minus {
            return self.primary();
        }
        let minus = self.advance();
        if let TokenKind::Integer(magnitude) = self.peek().kind {
            let literal = self.advance();
            return negative_literal(magnitude, minus.span, literal.span);
        }
        self.enter(minus.span)?;
        let operand = self.unary();
        self.leave();
        bounded(Expr::unary(UnaryOperator::Negate, operand?, minus.span))
    }

    fn primary(&mut self) -> Result<Expr, Diagnostic> {
        let token = self.advance();
        match token.kind {
            TokenKind::Integer(magnitude) => i64::try_from(magnitude)
                .map(|value| Expr::literal(Literal::Integer(value), token.span))
                .map_err(|_| out_of_range(magnitude, token.span)),
            TokenKind::Text(text) => Ok(Expr::literal(Literal::Text(text), token.span)),
            TokenKind::True => Ok(Expr::literal(Literal::Boolean(true), token.span)),
            TokenKind::False => Ok(Expr::literal(Literal::Boolean(false), token.span)),
            TokenKind::Identifier(name) => Ok(Expr::name(name, token.span)),
            TokenKind::LeftParen => {
                self.enter(token.span)?;
                let inner = self.expression();
                self.leave();
                let inner = inner?;
                self.expect(&TokenKind::RightParen, "to close the parenthesis")?;
                Ok(inner)
            }
            other => Err(Diagnostic::new(
                Phase::Syntax,
                format!("expected an expression, found {other}"),
                token.span,
            )),
        }
    }

    fn enter(&mut self, span: Span) -> Result<(), Diagnostic> {
        if self.nesting >= MAX_EXPRESSION_DEPTH {
            return Err(too_deep(span));
        }
        self.nesting += 1;
        Ok(())
    }

    fn leave(&mut self) {
        self.nesting -= 1;
    }
}

fn binary_operator(kind: &TokenKind) -> Option<(BinaryOperator, u8)> {
    match kind {
        TokenKind::EqualEqual => Some((BinaryOperator::Equal, 1)),
        TokenKind::Plus => Some((BinaryOperator::Add, 2)),
        TokenKind::Minus => Some((BinaryOperator::Subtract, 2)),
        TokenKind::Star => Some((BinaryOperator::Multiply, 3)),
        TokenKind::Slash => Some((BinaryOperator::Divide, 3)),
        _ => None,
    }
}

fn bounded(expression: Expr) -> Result<Expr, Diagnostic> {
    if expression.depth > MAX_EXPRESSION_DEPTH {
        Err(too_deep(expression.span))
    } else {
        Ok(expression)
    }
}

fn too_deep(span: Span) -> Diagnostic {
    Diagnostic::new(
        Phase::Syntax,
        format!("expression is nested too deeply (limit {MAX_EXPRESSION_DEPTH})"),
        span,
    )
}

fn negative_literal(magnitude: u64, minus: Span, literal: Span) -> Result<Expr, Diagnostic> {
    let value = if magnitude == i64::MIN.unsigned_abs() {
        i64::MIN
    } else {
        i64::try_from(magnitude)
            .map(|positive| -positive)
            .map_err(|_| out_of_range(magnitude, literal))?
    };
    Ok(Expr::literal(Literal::Integer(value), minus))
}

fn out_of_range(magnitude: u64, span: Span) -> Diagnostic {
    Diagnostic::new(
        Phase::Syntax,
        format!("integer literal `{magnitude}` is out of range"),
        span,
    )
}

fn uppercase_keyword_hint(kind: &TokenKind) -> Option<String> {
    let TokenKind::Identifier(word) = kind else {
        return None;
    };
    let uppercase = word.to_ascii_uppercase();
    if uppercase == *word || keyword(&uppercase).is_none() {
        return None;
    }
    Some(format!("keywords are written in uppercase: `{uppercase}`"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::ExprKind;
    use crate::lexer::tokenize;

    fn parse_source(source: &str) -> Result<Vec<Stmt>, Diagnostic> {
        parse(tokenize(source).expect("source should tokenize"))
    }

    fn single_expression(source: &str) -> Expr {
        let mut statements = parse_source(source).expect("source should parse");
        assert_eq!(statements.len(), 1);
        match statements.remove(0).kind {
            StmtKind::Manifest(expression) | StmtKind::Evaluate(expression) => expression,
            StmtKind::Declare { value, .. } => value,
        }
    }

    fn render(expression: &Expr) -> String {
        match &expression.kind {
            ExprKind::Literal(Literal::Integer(value)) => value.to_string(),
            ExprKind::Literal(Literal::Boolean(value)) => value.to_string(),
            ExprKind::Literal(Literal::Text(text)) => format!("{text:?}"),
            ExprKind::Name(name) => name.clone(),
            ExprKind::Unary { operand, .. } => format!("(neg {})", render(operand)),
            ExprKind::Binary {
                operator,
                left,
                right,
            } => {
                let symbol = match operator {
                    BinaryOperator::Add => "+",
                    BinaryOperator::Subtract => "-",
                    BinaryOperator::Multiply => "*",
                    BinaryOperator::Divide => "/",
                    BinaryOperator::Equal => "==",
                };
                format!("({symbol} {} {})", render(left), render(right))
            }
        }
    }

    #[test]
    fn multiplication_binds_tighter_than_addition() {
        assert_eq!(render(&single_expression("2 + 3 * 4;")), "(+ 2 (* 3 4))");
    }

    #[test]
    fn binary_operators_are_left_associative() {
        assert_eq!(render(&single_expression("10 - 3 - 2;")), "(- (- 10 3) 2)");
        assert_eq!(
            render(&single_expression("1 == 1 == TRUE;")),
            "(== (== 1 1) true)"
        );
    }

    #[test]
    fn equality_binds_loosest() {
        assert_eq!(render(&single_expression("1 + 2 == 3;")), "(== (+ 1 2) 3)");
    }

    #[test]
    fn parentheses_override_precedence() {
        assert_eq!(render(&single_expression("(2 + 3) * 4;")), "(* (+ 2 3) 4)");
    }

    #[test]
    fn negation_applies_to_the_nearest_operand() {
        assert_eq!(render(&single_expression("-x * 3;")), "(* (neg x) 3)");
        assert_eq!(render(&single_expression("- -x;")), "(neg (neg x))");
        assert_eq!(render(&single_expression("-(1 + 2);")), "(neg (+ 1 2))");
    }

    #[test]
    fn negative_integer_literals_fold() {
        assert_eq!(render(&single_expression("-2 * 3;")), "(* -2 3)");
        assert_eq!(
            render(&single_expression("-9223372036854775808;")),
            "-9223372036854775808"
        );
    }

    #[test]
    fn positive_literal_beyond_signed_range_is_rejected() {
        let diagnostic = parse_source("PRINT 9223372036854775808;").expect_err("should fail");
        assert_eq!(
            diagnostic.message,
            "integer literal `9223372036854775808` is out of range"
        );
        assert_eq!(diagnostic.span, Span::new(1, 7));
    }

    #[test]
    fn declaration_records_name_and_spans() {
        let statements = parse_source("  DECLARE answer = 42;").expect("should parse");
        assert_eq!(statements[0].span, Span::new(1, 3));
        let StmtKind::Declare {
            name, name_span, ..
        } = &statements[0].kind
        else {
            panic!("expected a declaration");
        };
        assert_eq!(name, "answer");
        assert_eq!(*name_span, Span::new(1, 11));
    }

    #[test]
    fn missing_semicolon_is_reported_at_the_offending_token() {
        let diagnostic = parse_source("LET a = 1\nLET b = 2;").expect_err("should fail");
        assert_eq!(
            diagnostic.message,
            "expected `;` after the declaration, found declaration keyword"
        );
        assert_eq!(diagnostic.span, Span::new(2, 1));
    }

    #[test]
    fn missing_expression_is_reported() {
        let diagnostic = parse_source("LET b = ;").expect_err("should fail");
        assert_eq!(diagnostic.message, "expected an expression, found `;`");
    }

    #[test]
    fn lowercase_keyword_gets_a_hint() {
        let diagnostic = parse_source("let x = 5;").expect_err("should fail");
        assert_eq!(
            diagnostic.hint.as_deref(),
            Some("keywords are written in uppercase: `LET`")
        );
    }

    #[test]
    fn unclosed_parenthesis_is_reported() {
        let diagnostic = parse_source("PRINT (1 + 2;").expect_err("should fail");
        assert_eq!(
            diagnostic.message,
            "expected `)` to close the parenthesis, found `;`"
        );
    }

    #[test]
    fn deep_parenthesis_nesting_is_rejected() {
        let depth = MAX_EXPRESSION_DEPTH + 1;
        let source = format!("PRINT {}1{};", "(".repeat(depth), ")".repeat(depth));
        let diagnostic = parse_source(&source).expect_err("should fail");
        assert!(diagnostic
            .message
            .starts_with("expression is nested too deeply"));
    }

    #[test]
    fn nesting_at_the_limit_is_accepted() {
        let depth = MAX_EXPRESSION_DEPTH;
        let source = format!("PRINT {}1{};", "(".repeat(depth), ")".repeat(depth));
        assert!(parse_source(&source).is_ok());
    }

    #[test]
    fn long_operator_chains_are_bounded() {
        let source = format!("PRINT 1{};", " + 1".repeat(MAX_EXPRESSION_DEPTH));
        let diagnostic = parse_source(&source).expect_err("should fail");
        assert!(diagnostic
            .message
            .starts_with("expression is nested too deeply"));
    }

    #[test]
    fn deep_negation_is_rejected() {
        let source = format!("PRINT {}x;", "- ".repeat(MAX_EXPRESSION_DEPTH + 1));
        let diagnostic = parse_source(&source).expect_err("should fail");
        assert!(diagnostic
            .message
            .starts_with("expression is nested too deeply"));
    }

    #[test]
    fn empty_program_parses() {
        assert_eq!(parse_source("").expect("should parse"), Vec::new());
    }
}
