use std::{env, fs, process};

#[derive(Clone, Debug, PartialEq)]
enum Token {
    Ident(String), Number(i64), String(String),
    Let, Print, True, False,
    Plus, Minus, Star, Slash, Equal, EqualEqual,
    LeftParen, RightParen, Semicolon, Eof,
}

struct Lexer {
    chars: Vec<char>,
    pos: usize,
}

impl Lexer {
    fn new(source: &str) -> Self { Self { chars: source.chars().collect(), pos: 0 } }
    fn peek(&self) -> char { self.chars.get(self.pos).copied().unwrap_or('\0') }
    fn advance(&mut self) -> char { let c = self.peek(); if c != '\0' { self.pos += 1; } c }
    fn skip_space(&mut self) { while self.peek().is_whitespace() { self.advance(); } }
    fn scan(&mut self) -> Result<Vec<Token>, String> {
        let mut out = Vec::new();
        loop {
            self.skip_space();
            let c = self.advance();
            let token = match c {
                '\0' => { out.push(Token::Eof); break; }
                '+' => Token::Plus, '-' => Token::Minus, '*' => Token::Star, '/' => Token::Slash,
                '(' => Token::LeftParen, ')' => Token::RightParen, ';' => Token::Semicolon,
                '=' if self.peek() == '=' => { self.advance(); Token::EqualEqual }
                '=' => Token::Equal,
                '"' => {
                    let mut value = String::new();
                    while self.peek() != '"' && self.peek() != '\0' { value.push(self.advance()); }
                    if self.advance() != '"' { return Err("unterminated string".into()); }
                    Token::String(value)
                }
                c if c.is_ascii_digit() => {
                    let mut n = c.to_string();
                    while self.peek().is_ascii_digit() { n.push(self.advance()); }
                    Token::Number(n.parse().map_err(|_| "integer out of range")?)
                }
                c if c.is_ascii_alphabetic() || c == '_' => {
                    let mut s = c.to_string();
                    while self.peek().is_ascii_alphanumeric() || self.peek() == '_' { s.push(self.advance()); }
                    match s.as_str() {
                        "LET" | "DECLARE" => Token::Let,
                        "MANIFEST" | "PRINT" => Token::Print,
                        "TRUE" => Token::True,
                        "FALSE" => Token::False,
                        _ => Token::Ident(s),
                    }
                }
                _ => return Err(format!("unexpected character: {c}")),
            };
            out.push(token);
        }
        Ok(out)
    }
}

#[derive(Clone, Debug)]
enum Value { Integer(i64), Boolean(bool), String(String) }

#[derive(Clone, Debug)]
enum Expr {
    Value(Value), Name(String),
    Unary(Token, Box<Expr>),
    Binary(Box<Expr>, Token, Box<Expr>),
}

#[derive(Clone, Debug)]
enum Stmt { Declare(String, Expr), Print(Expr), Expr(Expr) }

struct Parser { tokens: Vec<Token>, pos: usize }

impl Parser {
    fn new(tokens: Vec<Token>) -> Self { Self { tokens, pos: 0 } }
    fn peek(&self) -> &Token { self.tokens.get(self.pos).unwrap_or(&Token::Eof) }
    fn advance(&mut self) -> Token { let t = self.peek().clone(); if t != Token::Eof { self.pos += 1; } t }
    fn take(&mut self, expected: &Token) -> Result<(), String> {
        if self.peek() == expected { self.advance(); Ok(()) } else { Err(format!("expected {expected:?}, found {:?}", self.peek())) }
    }
    fn parse(&mut self) -> Result<Vec<Stmt>, String> {
        let mut statements = Vec::new();
        while self.peek() != &Token::Eof { statements.push(self.statement()?); }
        Ok(statements)
    }
    fn statement(&mut self) -> Result<Stmt, String> {
        match self.peek() {
            Token::Let => {
                self.advance();
                let name = match self.advance() { Token::Ident(s) => s, t => return Err(format!("expected identifier, found {t:?}")) };
                self.take(&Token::Equal)?;
                let expr = self.expression(0)?;
                self.take(&Token::Semicolon)?;
                Ok(Stmt::Declare(name, expr))
            }
            Token::Print => {
                self.advance();
                let expr = self.expression(0)?;
                self.take(&Token::Semicolon)?;
                Ok(Stmt::Print(expr))
            }
            _ => {
                let expr = self.expression(0)?;
                self.take(&Token::Semicolon)?;
                Ok(Stmt::Expr(expr))
            }
        }
    }
    fn precedence(token: &Token) -> u8 {
        match token { Token::EqualEqual => 1, Token::Plus | Token::Minus => 2, Token::Star | Token::Slash => 3, _ => 0 }
    }
    fn expression(&mut self, min: u8) -> Result<Expr, String> {
        let token = self.advance();
        let mut left = match token {
            Token::Number(n) => Expr::Value(Value::Integer(n)),
            Token::String(s) => Expr::Value(Value::String(s)),
            Token::True => Expr::Value(Value::Boolean(true)),
            Token::False => Expr::Value(Value::Boolean(false)),
            Token::Ident(s) => Expr::Name(s),
            Token::Minus => Expr::Unary(Token::Minus, Box::new(self.expression(4)?)),
            Token::LeftParen => { let e = self.expression(0)?; self.take(&Token::RightParen)?; e }
            t => return Err(format!("expected expression, found {t:?}")),
        };
        loop {
            let p = Self::precedence(self.peek());
            if p == 0 || p < min { break; }
            let op = self.advance();
            let right = self.expression(p + 1)?;
            left = Expr::Binary(Box::new(left), op, Box::new(right));
        }
        Ok(left)
    }
}

#[derive(Clone, Debug)]
enum Instruction {
    Push(Value), Load(String), Store(String),
    Add, Subtract, Multiply, Divide, Equal, Negate, Output, Pop, Halt,
}

fn compile_expr(expr: &Expr, code: &mut Vec<Instruction>) {
    match expr {
        Expr::Value(v) => code.push(Instruction::Push(v.clone())),
        Expr::Name(n) => code.push(Instruction::Load(n.clone())),
        Expr::Unary(_, e) => { compile_expr(e, code); code.push(Instruction::Negate); }
        Expr::Binary(a, op, b) => {
            compile_expr(a, code); compile_expr(b, code);
            code.push(match op {
                Token::Plus => Instruction::Add, Token::Minus => Instruction::Subtract,
                Token::Star => Instruction::Multiply, Token::Slash => Instruction::Divide,
                Token::EqualEqual => Instruction::Equal, _ => unreachable!(),
            });
        }
    }
}

fn compile(statements: &[Stmt]) -> Vec<Instruction> {
    let mut code = Vec::new();
    for stmt in statements {
        match stmt {
            Stmt::Declare(name, expr) => { compile_expr(expr, &mut code); code.push(Instruction::Store(name.clone())); }
            Stmt::Print(expr) => { compile_expr(expr, &mut code); code.push(Instruction::Output); }
            Stmt::Expr(expr) => { compile_expr(expr, &mut code); code.push(Instruction::Pop); }
        }
    }
    code.push(Instruction::Halt);
    code
}

fn integer(value: Value) -> Result<i64, String> {
    match value { Value::Integer(n) => Ok(n), _ => Err("integer operand required".into()) }
}

fn execute(code: &[Instruction]) -> Result<(), String> {
    use std::collections::HashMap;
    let mut stack: Vec<Value> = Vec::new();
    let mut memory: HashMap<String, Value> = HashMap::new();
    let mut pc = 0usize;
    loop {
        let instruction = code.get(pc).ok_or("instruction pointer out of range")?.clone();
        pc += 1;
        match instruction {
            Instruction::Push(v) => stack.push(v),
            Instruction::Load(name) => stack.push(memory.get(&name).cloned().ok_or_else(|| format!("undefined name: {name}"))?),
            Instruction::Store(name) => {
                if memory.contains_key(&name) { return Err(format!("name already declared: {name}")); }
                memory.insert(name, stack.pop().ok_or("stack underflow")?);
            }
            Instruction::Add | Instruction::Subtract | Instruction::Multiply | Instruction::Divide => {
                let b = integer(stack.pop().ok_or("stack underflow")?)?;
                let a = integer(stack.pop().ok_or("stack underflow")?)?;
                let result = match instruction {
                    Instruction::Add => a.checked_add(b),
                    Instruction::Subtract => a.checked_sub(b),
                    Instruction::Multiply => a.checked_mul(b),
                    Instruction::Divide => if b == 0 { return Err("division by zero".into()); } else { a.checked_div(b) },
                    _ => unreachable!(),
                }.ok_or("integer arithmetic overflow")?;
                stack.push(Value::Integer(result));
            }
            Instruction::Equal => {
                let b = stack.pop().ok_or("stack underflow")?;
                let a = stack.pop().ok_or("stack underflow")?;
                let equal = match (a, b) {
                    (Value::Integer(x), Value::Integer(y)) => x == y,
                    (Value::Boolean(x), Value::Boolean(y)) => x == y,
                    (Value::String(x), Value::String(y)) => x == y,
                    _ => false,
                };
                stack.push(Value::Boolean(equal));
            }
            Instruction::Negate => {
                let n = integer(stack.pop().ok_or("stack underflow")?)?;
                stack.push(Value::Integer(n.checked_neg().ok_or("integer arithmetic overflow")?));
            }
            Instruction::Output => {
                let value = stack.pop().ok_or("stack underflow")?;
                match value { Value::Integer(n) => println!("{n}"), Value::Boolean(b) => println!("{b}"), Value::String(s) => println!("{s}") }
            }
            Instruction::Pop => { stack.pop().ok_or("stack underflow")?; }
            Instruction::Halt => return Ok(()),
        }
    }
}

fn run(path: &str) -> Result<(), String> {
    let source = fs::read_to_string(path).map_err(|e| format!("cannot read {path}: {e}"))?;
    let tokens = Lexer::new(&source).scan()?;
    let statements = Parser::new(tokens).parse()?;
    let bytecode = compile(&statements);
    execute(&bytecode)
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        eprintln!("usage: ars-magna <source.ars>");
        process::exit(2);
    }
    if let Err(error) = run(&args[1]) {
        eprintln!("error: {error}");
        process::exit(1);
    }
}
