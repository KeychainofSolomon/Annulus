use super::lexis::Pos;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Program {
    pub faculty: Option<Vec<Grant>>,
    pub items: Vec<Item>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Grant {
    pub kind: String,
    pub target: String,
    pub words: Vec<String>,
    pub amount: Option<i64>,
    pub pos: Pos,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Item {
    Function(Function),
    Form(Form),
    Module(bool, Vec<String>, Pos),
    Statement(Stmt),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Function {
    pub name: String,
    pub params: Vec<(String, TypeExpr)>,
    pub result: Option<TypeExpr>,
    pub body: Vec<Stmt>,
    pub pos: Pos,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Form {
    pub name: String,
    pub fields: Vec<(String, TypeExpr)>,
    pub pos: Pos,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TypeExpr {
    Named(&'static str, Pos),
    Series(Box<TypeExpr>, Pos),
    Form(String, Pos),
}

impl TypeExpr {
    pub fn pos(&self) -> Pos {
        match self {
            TypeExpr::Named(_, p) | TypeExpr::Series(_, p) | TypeExpr::Form(_, p) => *p,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Release {
    VadeInPace,
    Dimitte,
    Relega,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instrument {
    Anulum,
    Virgam,
    Hexagrammum,
    Pentagrammum,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Command {
    Offer,
    Appella,
    Veta,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Stmt {
    Sit {
        mutable: bool,
        name: String,
        label: Option<TypeExpr>,
        value: Expr,
        pos: Pos,
    },
    Manifesta(Expr, Pos),
    Si {
        cond: Expr,
        then: Vec<Stmt>,
        otherwise: Option<Vec<Stmt>>,
        pos: Pos,
    },
    Dum {
        cond: Expr,
        body: Vec<Stmt>,
        pos: Pos,
    },
    Per {
        name: String,
        over: Expr,
        body: Vec<Stmt>,
        pos: Pos,
    },
    Desine(Pos),
    Perge(Pos),
    Redde(Option<Expr>, Pos),
    Tenta {
        body: Vec<Stmt>,
        name: String,
        handler: Vec<Stmt>,
        pos: Pos,
    },
    Circulus {
        rite: String,
        intent: String,
        body: Vec<Stmt>,
        pos: Pos,
    },
    Release(Release, Expr, Pos),
    Gere(Instrument, Pos),
    Command(Command, Expr, Pos),
    Constringe(Expr, String, Pos),
    Expr(Expr, Pos),
    Assign(Expr, Expr, Pos),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Binary {
    Vel,
    Et,
    Eq,
    Ne,
    Lt,
    Le,
    Gt,
    Ge,
    Add,
    Sub,
    Mul,
    Div,
    Rem,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Unary {
    Neg,
    Non,
    Consecra,
    Examina,
    Responsum,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Judged {
    Ratum,
    Suspensum,
    Irritum,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Entity {
    Label(String, Pos),
    Spiritus(i64, Pos),
    Angelus(i64, Pos),
}

impl Entity {
    pub fn pos(&self) -> Pos {
        match self {
            Entity::Label(_, p) | Entity::Spiritus(_, p) | Entity::Angelus(_, p) => *p,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Expr {
    Integer(i64, Pos),
    Text(String, Pos),
    Truth(bool, Pos),
    Nihil(Pos),
    Outcome(Judged, Pos),
    Name(String, Pos),
    Sequence(Vec<Expr>, Pos),
    Binary(Binary, Box<Expr>, Box<Expr>, Pos),
    Unary(Unary, Box<Expr>, Pos),
    Inscribe(Box<Expr>, Box<Expr>, Pos),
    Call(Box<Expr>, Vec<Expr>, Pos),
    Field(Box<Expr>, String, Pos),
    Index(Box<Expr>, Box<Expr>, Pos),
    Invoca(Entity, String, Pos),
    InvocaRegem(String, Pos),
    Pete(String, Vec<Expr>, Pos),
    Sigillum(Entity, Pos),
    Atramentum(String, Pos),
    Nomen(String, Pos),
    Entry(Entity),
}

impl Expr {
    pub fn pos(&self) -> Pos {
        match self {
            Expr::Integer(_, p)
            | Expr::Text(_, p)
            | Expr::Truth(_, p)
            | Expr::Nihil(p)
            | Expr::Outcome(_, p)
            | Expr::Name(_, p)
            | Expr::Sequence(_, p)
            | Expr::Binary(_, _, _, p)
            | Expr::Unary(_, _, p)
            | Expr::Inscribe(_, _, p)
            | Expr::Call(_, _, p)
            | Expr::Field(_, _, p)
            | Expr::Index(_, _, p)
            | Expr::Invoca(_, _, p)
            | Expr::InvocaRegem(_, p)
            | Expr::Pete(_, _, p)
            | Expr::Sigillum(_, p)
            | Expr::Atramentum(_, p)
            | Expr::Nomen(_, p) => *p,
            Expr::Entry(e) => e.pos(),
        }
    }

    pub fn is_place(&self) -> bool {
        match self {
            Expr::Name(..) => true,
            Expr::Field(base, _, _) | Expr::Index(base, _, _) => base.is_place(),
            _ => false,
        }
    }
}
