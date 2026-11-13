use super::lexis::{Kind, Pos, Token};
use super::syntaxis::*;
use super::Error;

pub fn parse(tokens: Vec<Token>) -> Result<Program, Error> {
    let mut parser = Parser {
        tokens,
        at: 0,
        depth: 0,
    };
    parser.program()
}

struct Parser {
    tokens: Vec<Token>,
    at: usize,
    depth: usize,
}

pub const MAX_DEPTH: usize = 256;

type Parsed<T> = Result<T, Error>;

const BASE_TYPES: &[&str] = &[
    "NUMERUS",
    "VERITAS",
    "VERBUM",
    "NIHIL",
    "ATRAMENTUM",
    "SIGILLUM",
    "SPIRITUS",
    "ANGELUS",
];

impl Parser {
    fn peek(&self) -> &Token {
        &self.tokens[self.at.min(self.tokens.len() - 1)]
    }

    fn pos(&self) -> Pos {
        self.peek().pos
    }

    fn bump(&mut self) -> Token {
        let token = self.peek().clone();
        if self.at < self.tokens.len() - 1 {
            self.at += 1;
        }
        token
    }

    fn is(&self, word: &str) -> bool {
        match &self.peek().kind {
            Kind::Keyword(k) => *k == word,
            Kind::Operator(o) => *o == word,
            Kind::Punctuation(c) => word.len() == 1 && word.starts_with(*c),
            _ => false,
        }
    }

    fn accept(&mut self, word: &str) -> bool {
        if self.is(word) {
            self.bump();
            true
        } else {
            false
        }
    }

    fn fail<T>(&self, message: &str) -> Parsed<T> {
        let token = self.peek();
        Err(Error::new(
            "P1",
            token.pos,
            format!("{message}, found {}", token.kind.shown()),
        ))
    }

    fn expect(&mut self, word: &str) -> Parsed<()> {
        if self.accept(word) {
            Ok(())
        } else {
            self.fail(&format!("expected {word}"))
        }
    }

    fn identifier(&mut self) -> Parsed<(String, Pos)> {
        match &self.peek().kind {
            Kind::Identifier(name) => {
                let name = name.clone();
                let pos = self.bump().pos;
                Ok((name, pos))
            }
            _ => self.fail("expected a name"),
        }
    }

    fn text(&mut self) -> Parsed<String> {
        match &self.peek().kind {
            Kind::Text(s) => {
                let s = s.clone();
                self.bump();
                Ok(s)
            }
            _ => self.fail("expected a string"),
        }
    }

    fn integer(&mut self) -> Parsed<i64> {
        match self.peek().kind {
            Kind::Integer(n) => {
                self.bump();
                Ok(n)
            }
            _ => self.fail("expected an integer"),
        }
    }

    fn at_end(&self) -> bool {
        self.peek().kind == Kind::End
    }

    fn program(&mut self) -> Parsed<Program> {
        let faculty = if self.is("FACULTAS") {
            Some(self.faculty()?)
        } else {
            None
        };
        let mut items = Vec::new();
        while !self.at_end() {
            items.push(self.item()?);
        }
        Ok(Program { faculty, items })
    }

    fn faculty(&mut self) -> Parsed<Vec<Grant>> {
        self.expect("FACULTAS")?;
        self.expect("{")?;
        let mut grants = Vec::new();
        while !self.is("}") {
            let (kind, pos) = self.identifier()?;
            let target = self.text()?;
            let mut words = Vec::new();
            while let Kind::Identifier(w) = &self.peek().kind {
                words.push(w.clone());
                self.bump();
            }
            let amount = if let Kind::Integer(n) = self.peek().kind {
                self.bump();
                Some(n)
            } else {
                None
            };
            self.expect(";")?;
            grants.push(Grant {
                kind,
                target,
                words,
                amount,
                pos,
            });
        }
        self.expect("}")?;
        Ok(grants)
    }

    fn item(&mut self) -> Parsed<Item> {
        let pos = self.pos();
        if self.is("OPUS") {
            return Ok(Item::Function(self.function()?));
        }
        if self.is("FORMA") {
            return Ok(Item::Form(self.form()?));
        }
        if self.is("LIBER") || self.is("ADHIBE") {
            let import = self.is("ADHIBE");
            self.bump();
            let mut path = vec![self.identifier()?.0];
            while self.accept(".") {
                path.push(self.identifier()?.0);
            }
            self.expect(";")?;
            return Ok(Item::Module(import, path, pos));
        }
        Ok(Item::Statement(self.statement()?))
    }

    fn typed_names(&mut self, closer: &str) -> Parsed<Vec<(String, TypeExpr)>> {
        let mut out = Vec::new();
        if !self.is(closer) {
            loop {
                let (name, _) = self.identifier()?;
                self.expect(":")?;
                out.push((name, self.type_expr()?));
                if !self.accept(",") {
                    break;
                }
            }
        }
        self.expect(closer)?;
        Ok(out)
    }

    fn form(&mut self) -> Parsed<Form> {
        let pos = self.pos();
        self.expect("FORMA")?;
        let (name, _) = self.identifier()?;
        self.expect("{")?;
        let fields = self.typed_names("}")?;
        Ok(Form { name, fields, pos })
    }

    fn function(&mut self) -> Parsed<Function> {
        let pos = self.pos();
        self.expect("OPUS")?;
        let (name, _) = self.identifier()?;
        self.expect("(")?;
        let params = self.typed_names(")")?;
        let result = if self.accept("->") {
            Some(self.type_expr()?)
        } else {
            None
        };
        let body = self.block()?;
        Ok(Function {
            name,
            params,
            result,
            body,
            pos,
        })
    }

    fn type_expr(&mut self) -> Parsed<TypeExpr> {
        let pos = self.pos();
        match self.peek().kind.clone() {
            Kind::Keyword(k) if BASE_TYPES.contains(&k) => {
                self.bump();
                Ok(TypeExpr::Named(k, pos))
            }
            Kind::Keyword("SERIES") => {
                self.bump();
                self.expect("[")?;
                let inner = self.type_expr()?;
                self.expect("]")?;
                Ok(TypeExpr::Series(Box::new(inner), pos))
            }
            Kind::Identifier(name) => {
                self.bump();
                Ok(TypeExpr::Form(name, pos))
            }
            _ => self.fail("expected a type"),
        }
    }

    fn block(&mut self) -> Parsed<Vec<Stmt>> {
        self.expect("{")?;
        let mut body = Vec::new();
        while !self.is("}") {
            if self.at_end() {
                return self.fail("unclosed block");
            }
            body.push(self.statement()?);
        }
        self.expect("}")?;
        Ok(body)
    }

    fn statement(&mut self) -> Parsed<Stmt> {
        self.depth += 1;
        let result = if self.depth > MAX_DEPTH {
            self.fail("the program is nested too deeply")
        } else {
            self.statement_inner()
        };
        self.depth -= 1;
        result
    }

    fn statement_inner(&mut self) -> Parsed<Stmt> {
        let pos = self.pos();
        if self.accept("SIT") {
            let mutable = self.accept("MUTABILIS");
            let (name, _) = self.identifier()?;
            let label = if self.accept(":") {
                Some(self.type_expr()?)
            } else {
                None
            };
            self.expect("=")?;
            let value = self.expression()?;
            self.expect(";")?;
            return Ok(Stmt::Sit {
                mutable,
                name,
                label,
                value,
                pos,
            });
        }
        if self.accept("MANIFESTA") {
            let e = self.expression()?;
            self.expect(";")?;
            return Ok(Stmt::Manifesta(e, pos));
        }
        if self.is("SI") {
            return self.conditional();
        }
        if self.accept("DUM") {
            let cond = self.expression()?;
            let body = self.block()?;
            return Ok(Stmt::Dum { cond, body, pos });
        }
        if self.accept("PER") {
            let (name, _) = self.identifier()?;
            self.expect("IN")?;
            let over = self.expression()?;
            let body = self.block()?;
            return Ok(Stmt::Per {
                name,
                over,
                body,
                pos,
            });
        }
        if self.accept("DESINE") {
            self.expect(";")?;
            return Ok(Stmt::Desine(pos));
        }
        if self.accept("PERGE") {
            self.expect(";")?;
            return Ok(Stmt::Perge(pos));
        }
        if self.accept("REDDE") {
            let value = if self.is(";") {
                None
            } else {
                Some(self.expression()?)
            };
            self.expect(";")?;
            return Ok(Stmt::Redde(value, pos));
        }
        if self.accept("TENTA") {
            let body = self.block()?;
            self.expect("CAPE")?;
            let (name, _) = self.identifier()?;
            let handler = self.block()?;
            return Ok(Stmt::Tenta {
                body,
                name,
                handler,
                pos,
            });
        }
        if self.is("CIRCULUS") {
            return self.circle();
        }
        if self.accept("VADE") {
            self.expect("IN")?;
            self.expect("PACE")?;
            return self.release(Release::VadeInPace, pos);
        }
        if self.accept("DIMITTE") {
            return self.release(Release::Dimitte, pos);
        }
        if self.accept("RELEGA") {
            return self.release(Release::Relega, pos);
        }
        if self.accept("MANE") {
            return self.release(Release::Mane, pos);
        }
        if self.accept("GERE") {
            let instrument = if self.accept("ANULUM") {
                Instrument::Anulum
            } else if self.accept("VIRGAM") {
                Instrument::Virgam
            } else if self.accept("HEXAGRAMMUM") {
                Instrument::Hexagrammum
            } else if self.accept("PENTAGRAMMUM") {
                Instrument::Pentagrammum
            } else {
                return self.fail("expected an instrument");
            };
            self.expect(";")?;
            return Ok(Stmt::Gere(instrument, pos));
        }
        for (word, command) in [
            ("OFFER", Command::Offer),
            ("APPELLA", Command::Appella),
            ("VETA", Command::Veta),
        ] {
            if self.accept(word) {
                let e = self.expression()?;
                self.expect(";")?;
                return Ok(Stmt::Command(command, e, pos));
            }
        }
        if self.accept("CONSTRINGE") {
            let e = self.expression()?;
            self.expect("PER")?;
            self.expect("NOMEN")?;
            let name = self.text()?;
            self.expect(";")?;
            return Ok(Stmt::Constringe(e, name, pos));
        }
        let bare = matches!(self.peek().kind, Kind::Identifier(_));
        let left = self.expression()?;
        if self.is("=") {
            self.bump();
            if !bare || !left.is_place() {
                return self.fail("left side of = is not a place");
            }
            let right = self.expression()?;
            self.expect(";")?;
            return Ok(Stmt::Assign(left, right, pos));
        }
        self.expect(";")?;
        Ok(Stmt::Expr(left, pos))
    }

    fn release(&mut self, kind: Release, pos: Pos) -> Parsed<Stmt> {
        let e = self.expression()?;
        self.expect(";")?;
        Ok(Stmt::Release(kind, e, pos))
    }

    fn conditional(&mut self) -> Parsed<Stmt> {
        let pos = self.pos();
        self.expect("SI")?;
        let cond = self.expression()?;
        let then = self.block()?;
        let otherwise = if self.accept("ALITER") {
            if self.is("SI") {
                Some(vec![self.conditional()?])
            } else {
                Some(self.block()?)
            }
        } else {
            None
        };
        Ok(Stmt::Si {
            cond,
            then,
            otherwise,
            pos,
        })
    }

    fn circle(&mut self) -> Parsed<Stmt> {
        let pos = self.pos();
        self.expect("CIRCULUS")?;
        self.expect("RITU")?;
        let (rite, _) = self.identifier()?;
        self.expect("{")?;
        self.expect("INTENTIO")?;
        let intent = self.text()?;
        self.expect(";")?;
        let mut body = Vec::new();
        while !self.is("}") {
            if self.at_end() {
                return self.fail("unclosed circle");
            }
            body.push(self.statement()?);
        }
        self.expect("}")?;
        Ok(Stmt::Circulus {
            rite,
            intent,
            body,
            pos,
        })
    }

    fn expression(&mut self) -> Parsed<Expr> {
        self.depth += 1;
        let result = if self.depth > MAX_DEPTH {
            self.fail("the program is nested too deeply")
        } else {
            self.expression_inner()
        };
        self.depth -= 1;
        result
    }

    fn expression_inner(&mut self) -> Parsed<Expr> {
        let mut left = self.conjunction()?;
        while self.is("VEL") {
            let pos = self.bump().pos;
            let right = self.conjunction()?;
            left = Expr::Binary(Binary::Vel, Box::new(left), Box::new(right), pos);
        }
        Ok(left)
    }

    fn conjunction(&mut self) -> Parsed<Expr> {
        let mut left = self.comparison()?;
        while self.is("ET") {
            let pos = self.bump().pos;
            let right = self.comparison()?;
            left = Expr::Binary(Binary::Et, Box::new(left), Box::new(right), pos);
        }
        Ok(left)
    }

    fn comparison_op(&self) -> Option<Binary> {
        match self.peek().kind {
            Kind::Operator("==") => Some(Binary::Eq),
            Kind::Operator("!=") => Some(Binary::Ne),
            Kind::Operator("<") => Some(Binary::Lt),
            Kind::Operator("<=") => Some(Binary::Le),
            Kind::Operator(">") => Some(Binary::Gt),
            Kind::Operator(">=") => Some(Binary::Ge),
            _ => None,
        }
    }

    fn comparison(&mut self) -> Parsed<Expr> {
        let left = self.sum()?;
        let Some(op) = self.comparison_op() else {
            return Ok(left);
        };
        let pos = self.bump().pos;
        let right = self.sum()?;
        if self.comparison_op().is_some() {
            return self.fail("comparisons do not chain");
        }
        Ok(Expr::Binary(op, Box::new(left), Box::new(right), pos))
    }

    fn sum(&mut self) -> Parsed<Expr> {
        let mut left = self.product()?;
        loop {
            let op = match self.peek().kind {
                Kind::Operator("+") => Binary::Add,
                Kind::Operator("-") => Binary::Sub,
                _ => return Ok(left),
            };
            let pos = self.bump().pos;
            let right = self.product()?;
            left = Expr::Binary(op, Box::new(left), Box::new(right), pos);
        }
    }

    fn product(&mut self) -> Parsed<Expr> {
        let mut left = self.unary()?;
        loop {
            let op = match self.peek().kind {
                Kind::Operator("*") => Binary::Mul,
                Kind::Operator("/") => Binary::Div,
                Kind::Operator("%") => Binary::Rem,
                _ => return Ok(left),
            };
            let pos = self.bump().pos;
            let right = self.unary()?;
            left = Expr::Binary(op, Box::new(left), Box::new(right), pos);
        }
    }

    fn unary(&mut self) -> Parsed<Expr> {
        self.depth += 1;
        let result = if self.depth > MAX_DEPTH {
            self.fail("the program is nested too deeply")
        } else {
            self.unary_inner()
        };
        self.depth -= 1;
        result
    }

    fn unary_inner(&mut self) -> Parsed<Expr> {
        let op = match self.peek().kind {
            Kind::Operator("-") => Some(Unary::Neg),
            Kind::Keyword("NON") => Some(Unary::Non),
            Kind::Keyword("CONSECRA") => Some(Unary::Consecra),
            Kind::Keyword("EXAMINA") => Some(Unary::Examina),
            Kind::Keyword("RESPONSUM") => Some(Unary::Responsum),
            _ => None,
        };
        if let Some(op) = op {
            let pos = self.bump().pos;
            let operand = self.unary()?;
            return Ok(Expr::Unary(op, Box::new(operand), pos));
        }
        if self.is("INSCRIBE") {
            let pos = self.bump().pos;
            let seal = self.unary()?;
            self.expect("ATRAMENTO")?;
            let ink = self.unary()?;
            return Ok(Expr::Inscribe(Box::new(seal), Box::new(ink), pos));
        }
        self.postfix()
    }

    fn postfix(&mut self) -> Parsed<Expr> {
        let mut e = self.primary()?;
        loop {
            let pos = self.pos();
            if self.accept("(") {
                let args = self.arguments(")")?;
                e = Expr::Call(Box::new(e), args, pos);
            } else if self.accept(".") {
                let (name, _) = self.identifier()?;
                e = Expr::Field(Box::new(e), name, pos);
            } else if self.accept("[") {
                let index = self.expression()?;
                self.expect("]")?;
                e = Expr::Index(Box::new(e), Box::new(index), pos);
            } else {
                return Ok(e);
            }
        }
    }

    fn arguments(&mut self, closer: &str) -> Parsed<Vec<Expr>> {
        let mut args = Vec::new();
        if !self.is(closer) {
            args.push(self.expression()?);
            while self.accept(",") {
                args.push(self.expression()?);
            }
        }
        self.expect(closer)?;
        Ok(args)
    }

    fn entity(&mut self, spirit: &str, angel: &str) -> Parsed<Entity> {
        let pos = self.pos();
        if self.accept(spirit) {
            return Ok(Entity::Spiritus(self.integer()?, pos));
        }
        if self.accept(angel) {
            return Ok(Entity::Angelus(self.integer()?, pos));
        }
        if matches!(self.peek().kind, Kind::Keyword(_)) {
            return self.fail(&format!("expected a name, {spirit} or {angel}"));
        }
        let (name, pos) = self.identifier()?;
        Ok(Entity::Label(name, pos))
    }

    fn primary(&mut self) -> Parsed<Expr> {
        let pos = self.pos();
        match self.peek().kind.clone() {
            Kind::Integer(n) => {
                self.bump();
                return Ok(Expr::Integer(n, pos));
            }
            Kind::Text(s) => {
                self.bump();
                return Ok(Expr::Text(s, pos));
            }
            Kind::Identifier(name) => {
                self.bump();
                return Ok(Expr::Name(name, pos));
            }
            _ => {}
        }
        let literal = match self.peek().kind {
            Kind::Keyword("VERUM") => Some(Expr::Truth(true, pos)),
            Kind::Keyword("FALSUM") => Some(Expr::Truth(false, pos)),
            Kind::Keyword("NIHIL") => Some(Expr::Nihil(pos)),
            Kind::Keyword("RATUM") => Some(Expr::Outcome(Judged::Ratum, pos)),
            Kind::Keyword("SUSPENSUM") => Some(Expr::Outcome(Judged::Suspensum, pos)),
            Kind::Keyword("IRRITUM") => Some(Expr::Outcome(Judged::Irritum, pos)),
            _ => None,
        };
        if let Some(e) = literal {
            self.bump();
            return Ok(e);
        }
        if self.accept("(") {
            let e = self.expression()?;
            self.expect(")")?;
            return Ok(e);
        }
        if self.accept("[") {
            return Ok(Expr::Sequence(self.arguments("]")?, pos));
        }
        if self.accept("INVOCA") {
            if self.accept("REGEM") {
                return Ok(Expr::InvocaRegem(self.identifier()?.0, pos));
            }
            let entity = self.entity("SPIRITUM", "ANGELUM")?;
            self.expect("AD")?;
            self.expect("OFFICIUM")?;
            let (office, _) = self.identifier()?;
            return Ok(Expr::Invoca(entity, office, pos));
        }
        if self.accept("PETE") {
            let (name, _) = self.identifier()?;
            self.expect("(")?;
            let args = self.arguments(")")?;
            return Ok(Expr::Pete(name, args, pos));
        }
        if self.accept("SIGILLUM") {
            return Ok(Expr::Sigillum(self.entity("SPIRITUS", "ANGELI")?, pos));
        }
        if self.accept("ATRAMENTUM") {
            return Ok(Expr::Atramentum(self.text()?, pos));
        }
        if self.accept("NOMEN") {
            return Ok(Expr::Nomen(self.text()?, pos));
        }
        if self.is("SPIRITUS") || self.is("ANGELUS") {
            return Ok(Expr::Entry(self.entity("SPIRITUS", "ANGELUS")?));
        }
        self.fail("expected an expression")
    }
}
