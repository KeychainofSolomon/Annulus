use std::collections::BTreeSet;

use super::lexis::Pos;
use super::registra::{Register, Spirit, INK_CLASSES, RITES};
use super::syntaxis::*;
use super::Error;
use crate::nucleus::{Mark, Perm};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Ty {
    Num,
    Bool,
    Text,
    Nil,
    Outcome,
    Entry,
    Inv,
    Prop,
    SealEx,
    SealOp,
    InkRaw,
    InkHoly,
    Name,
    Error,
}

impl Ty {
    pub fn rite(self) -> bool {
        matches!(
            self,
            Ty::Inv
                | Ty::Prop
                | Ty::SealEx
                | Ty::SealOp
                | Ty::InkRaw
                | Ty::InkHoly
                | Ty::Entry
                | Ty::Name
        )
    }

    fn shown(self) -> &'static str {
        match self {
            Ty::Num => "NUMERUS",
            Ty::Bool => "VERITAS",
            Ty::Text => "VERBUM",
            Ty::Nil => "NIHIL",
            Ty::Outcome => "an outcome",
            Ty::Entry => "an entry",
            Ty::Inv => "an invocation",
            Ty::Prop => "a proposal",
            Ty::SealEx => "an exemplar seal",
            Ty::SealOp => "an operative seal",
            Ty::InkRaw => "raw ink",
            Ty::InkHoly => "consecrated ink",
            Ty::Name => "a divine name",
            Ty::Error => "an erroneous value",
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Class {
    Telling,
    Working,
}

#[derive(Clone, Copy, Debug)]
pub struct Signature {
    pub class: Class,
    pub modes: &'static [&'static str],
    pub quaestio: bool,
    pub perm: Perm,
    pub min_places: usize,
}

pub fn signature(kind: &str) -> Option<Signature> {
    let telling = Signature {
        class: Class::Telling,
        modes: &[],
        quaestio: true,
        perm: Perm::Legere,
        min_places: 0,
    };
    let working = |perm, modes| Signature {
        class: Class::Working,
        modes,
        quaestio: false,
        perm,
        min_places: 1,
    };
    Some(match kind {
        "PRAESENTIA" | "FUTURA" | "OCCULTA" | "THESAURI" | "AMISSA" | "RESPONSA" => telling,
        "AEDIFICATIO" => working(Perm::Creare, &[]),
        "APPELLATIO" | "TUTELA" | "TEMPESTAS" => working(Perm::Scribere, &[]),
        "MORBUS" => working(Perm::Scribere, &["corrumpe", "sana"]),
        _ => return None,
    })
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Granted {
    pub kind: String,
    pub target: String,
    pub perms: BTreeSet<Perm>,
    pub amount: Option<i64>,
    pub reserved: bool,
}

#[derive(Clone, Debug)]
pub struct Checked {
    pub grants: Vec<Granted>,
}

type VarId = usize;

#[derive(Clone, Debug)]
struct Var {
    name: String,
    ty: Ty,
    mark: Mark,
    mutable: bool,
    constant: Option<String>,
    spirit: Option<u32>,
    ink: Option<String>,
    office: Option<String>,
    root: VarId,
    pos: Pos,
}

#[derive(Clone, Debug)]
struct Val {
    ty: Ty,
    mark: Mark,
    constant: Option<String>,
    spirit: Option<u32>,
    ink: Option<String>,
    office: Option<String>,
    var: Option<VarId>,
}

impl Val {
    fn plain(ty: Ty) -> Val {
        Val {
            ty,
            mark: Mark::Clean,
            constant: None,
            spirit: None,
            ink: None,
            office: None,
            var: None,
        }
    }

    fn error() -> Val {
        Val::plain(Ty::Error)
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Flow {
    released: BTreeSet<VarId>,
    spent: BTreeSet<VarId>,
    inscribed: Vec<BTreeSet<u32>>,
}

struct Checker<'r> {
    register: &'r Register,
    grants: Vec<Granted>,
    vars: Vec<Var>,
    scopes: Vec<Vec<VarId>>,
    flow: Flow,
    declared: BTreeSet<String>,
    errors: Vec<Error>,
}

pub fn check(program: &Program, register: &Register) -> Result<Checked, Vec<Error>> {
    let mut checker = Checker {
        register,
        grants: Vec::new(),
        vars: Vec::new(),
        scopes: vec![Vec::new()],
        flow: Flow::default(),
        declared: BTreeSet::new(),
        errors: Vec::new(),
    };
    for item in &program.items {
        if let Item::Statement(s) = item {
            collect(s, &mut checker.declared);
        }
    }
    if let Some(grants) = &program.faculty {
        checker.faculty(grants);
    }
    for item in &program.items {
        match item {
            Item::Statement(s) => checker.stmt(s),
            Item::Function(f) => checker.subset(f.pos, "OPUS (functions)"),
            Item::Form(f) => checker.subset(f.pos, "FORMA (forms)"),
            Item::Module(_, _, pos) => checker.subset(*pos, "LIBER and ADHIBE (modules)"),
        }
    }
    checker.close_scope();
    let mut errors = checker.errors;
    let mut seen = BTreeSet::new();
    errors.retain(|e| seen.insert((e.code, e.pos, e.message.clone())));
    errors.sort_by_key(|e| (e.phase(), e.pos));
    if errors.is_empty() {
        Ok(Checked {
            grants: checker.grants,
        })
    } else {
        Err(errors)
    }
}

fn collect(stmt: &Stmt, names: &mut BTreeSet<String>) {
    let mut all = |body: &[Stmt]| body.iter().for_each(|s| collect(s, names));
    match stmt {
        Stmt::Sit { name, .. } => {
            names.insert(name.clone());
        }
        Stmt::Si {
            then, otherwise, ..
        } => {
            all(then);
            if let Some(o) = otherwise {
                all(o);
            }
        }
        Stmt::Dum { body, .. } | Stmt::Circulus { body, .. } => all(body),
        _ => {}
    }
}

fn perm_of(word: &str) -> Option<Perm> {
    Some(match word {
        "legere" => Perm::Legere,
        "scribere" => Perm::Scribere,
        "creare" => Perm::Creare,
        "delere" => Perm::Delere,
        _ => return None,
    })
}

impl<'r> Checker<'r> {
    fn error(&mut self, code: &'static str, pos: Pos, message: String) {
        self.errors.push(Error::new(code, pos, message));
    }

    fn subset(&mut self, pos: Pos, what: &str) {
        self.error("X1", pos, format!("{what} is not yet in the step-2 subset"));
    }

    fn in_circle(&self) -> bool {
        !self.flow.inscribed.is_empty()
    }

    fn rite_only(&mut self, pos: Pos, what: &str) -> bool {
        if self.in_circle() {
            true
        } else {
            self.error("A1", pos, format!("{what} outside a Circle"));
            false
        }
    }

    fn faculty(&mut self, grants: &[Grant]) {
        let mut seen = BTreeSet::new();
        for g in grants {
            if !seen.insert((g.kind.clone(), g.target.clone())) {
                self.error(
                    "A8",
                    g.pos,
                    format!("the grant {} {:?} appears twice", g.kind, g.target),
                );
                continue;
            }
            let reserved = g.words.last().is_some_and(|w| w == "angelis");
            let words = &g.words[..g.words.len() - usize::from(reserved)];
            match g.kind.as_str() {
                "liber" => {
                    let mut perms = BTreeSet::new();
                    for w in words {
                        match perm_of(w) {
                            Some(p) if perms.insert(p) => {}
                            Some(_) => self.error(
                                "A8",
                                g.pos,
                                format!("the permission {w} is given twice"),
                            ),
                            None => self.error("A8", g.pos, format!("unknown permission {w}")),
                        }
                    }
                    if perms.is_empty() {
                        self.error("A8", g.pos, "a liber grant without permission".into());
                    }
                    if g.amount.is_some() {
                        self.error("A8", g.pos, "a liber grant takes no number".into());
                    }
                    if g.target.is_empty() {
                        self.error("A8", g.pos, "a liber grant without a file".into());
                    }
                    self.grants.push(Granted {
                        kind: g.kind.clone(),
                        target: g.target.clone(),
                        perms,
                        amount: None,
                        reserved,
                    });
                }
                "atramentum" => {
                    if !INK_CLASSES.contains(&g.target.as_str()) {
                        self.error("A8", g.pos, format!("unknown ink class {:?}", g.target));
                    }
                    if !words.is_empty() {
                        self.error(
                            "A8",
                            g.pos,
                            "an atramentum grant takes no permission".into(),
                        );
                    }
                    match g.amount {
                        Some(n) if n > 0 => {}
                        _ => self.error(
                            "A8",
                            g.pos,
                            "an atramentum grant needs a number of units".into(),
                        ),
                    }
                    self.grants.push(Granted {
                        kind: g.kind.clone(),
                        target: g.target.clone(),
                        perms: BTreeSet::new(),
                        amount: g.amount,
                        reserved,
                    });
                }
                "locus" | "historia" | "familiaris" => {
                    self.subset(g.pos, &format!("the {} grant", g.kind))
                }
                other => self.error("A8", g.pos, format!("unknown grant kind {other}")),
            }
        }
    }

    fn visible(&self, name: &str) -> Option<VarId> {
        self.scopes
            .iter()
            .rev()
            .flat_map(|s| s.iter().rev())
            .copied()
            .find(|v| self.vars[*v].name == name)
    }

    fn resolve(&mut self, name: &str, pos: Pos) -> Option<VarId> {
        let found = self.visible(name);
        if found.is_none() {
            if self.declared.contains(name) {
                self.error("N2", pos, format!("{name} is used before it is declared"));
            } else {
                self.error("N1", pos, format!("unknown name {name}"));
            }
        }
        found
    }

    fn open_scope(&mut self) {
        self.scopes.push(Vec::new());
    }

    fn close_scope(&mut self) {
        let scope = self.scopes.pop().unwrap_or_default();
        for v in scope {
            let var = &self.vars[v];
            if var.ty == Ty::Inv && var.root == v && !self.flow.released.contains(&v) {
                let (pos, name) = (var.pos, var.name.clone());
                self.error(
                    "A9",
                    pos,
                    format!("the invocation {name} is not released on every path"),
                );
            }
        }
    }

    fn block(&mut self, body: &[Stmt]) {
        self.open_scope();
        for s in body {
            self.stmt(s);
        }
        self.close_scope();
    }

    fn label_fits(&mut self, label: &TypeExpr, ty: Ty) {
        let fits = match label {
            TypeExpr::Named(name, _) => match *name {
                "NUMERUS" => ty == Ty::Num,
                "VERITAS" => ty == Ty::Bool,
                "VERBUM" => ty == Ty::Text,
                "NIHIL" => ty == Ty::Nil,
                "ATRAMENTUM" => matches!(ty, Ty::InkRaw | Ty::InkHoly),
                "SIGILLUM" => matches!(ty, Ty::SealEx | Ty::SealOp),
                _ => ty == Ty::Entry,
            },
            _ => {
                self.subset(label.pos(), "SERIES and FORMA types");
                return;
            }
        };
        if !fits && ty != Ty::Error {
            self.error(
                "T1",
                label.pos(),
                format!("the value is {}, not of the declared type", ty.shown()),
            );
        }
    }

    fn stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Sit {
                mutable,
                name,
                label,
                value,
                pos,
            } => {
                let val = match value {
                    Expr::Invoca(entity, office, p) => self.invoca(entity, office, *p, true),
                    _ => self.expr(value),
                };
                if let Some(label) = label {
                    self.label_fits(label, val.ty);
                }
                if *mutable && val.ty.rite() {
                    self.subset(*pos, "a mutable variable holding a rite value");
                }
                if self.visible(name).is_some() {
                    self.error("N3", *pos, format!("{name} is already declared"));
                }
                let id = self.vars.len();
                let root = match (val.var, val.ty.rite()) {
                    (Some(source), true) => self.vars[source].root,
                    _ => id,
                };
                self.vars.push(Var {
                    name: name.clone(),
                    ty: val.ty,
                    mark: val.mark,
                    mutable: *mutable,
                    constant: match (mutable, value) {
                        (false, Expr::Text(s, _)) => Some(s.clone()),
                        _ => None,
                    },
                    spirit: val.spirit,
                    ink: val.ink,
                    office: val.office,
                    root,
                    pos: *pos,
                });
                if let Some(scope) = self.scopes.last_mut() {
                    scope.push(id);
                }
            }
            Stmt::Manifesta(e, pos) => {
                let val = self.expr(e);
                let shown = matches!(
                    val.ty,
                    Ty::Num
                        | Ty::Bool
                        | Ty::Text
                        | Ty::Nil
                        | Ty::Outcome
                        | Ty::Entry
                        | Ty::SealEx
                        | Ty::Error
                );
                if !shown {
                    self.error(
                        "T7",
                        *pos,
                        format!("{} cannot be shown by MANIFESTA", val.ty.shown()),
                    );
                }
            }
            Stmt::Si {
                cond,
                then,
                otherwise,
                ..
            } => {
                self.condition(cond, "SI");
                let before = self.flow.clone();
                let existing = self.vars.len();
                self.block(then);
                let after_then = std::mem::replace(&mut self.flow, before);
                if let Some(o) = otherwise {
                    self.block(o);
                }
                let after_else = self.flow.clone();
                self.merge(after_then, after_else, existing, cond.pos());
            }
            Stmt::Dum { cond, body, pos } => {
                let before = self.flow.clone();
                let existing = self.vars.len();
                for _ in 0..2 {
                    self.flow = before.clone();
                    self.condition(cond, "DUM");
                    self.block(body);
                }
                let after = std::mem::replace(&mut self.flow, before.clone());
                for v in after.released.difference(&before.released) {
                    if *v < existing {
                        let name = self.vars[*v].name.clone();
                        self.error(
                            "A9",
                            *pos,
                            format!("{name} is released inside a loop, so it may be released more than once"),
                        );
                    }
                }
                for v in after.spent.difference(&before.spent) {
                    if *v < existing {
                        let name = self.vars[*v].name.clone();
                        self.error(
                            "A7",
                            *pos,
                            format!("the ink {name} is spent inside a loop, so it may be spent more than once"),
                        );
                    }
                }
                self.flow.spent.extend(after.spent);
                self.flow.released.extend(after.released);
            }
            Stmt::Circulus {
                rite,
                intent,
                body,
                pos,
            } => {
                if !RITES.contains(&rite.as_str()) {
                    self.error("A2", *pos, format!("unknown rite {rite}"));
                } else if rite != self.register.rite {
                    self.subset(*pos, &format!("the rite {rite}"));
                }
                if intent.is_empty() {
                    self.error("A2", *pos, "the intention is empty".into());
                }
                self.flow.inscribed.push(BTreeSet::new());
                self.block(body);
                self.flow.inscribed.pop();
            }
            Stmt::Release(_, e, pos) => {
                if !self.rite_only(*pos, "a release") {
                    return;
                }
                let Expr::Name(name, npos) = e else {
                    self.expr(e);
                    self.error(
                        "A9",
                        *pos,
                        "a release names the variable that holds the invocation".into(),
                    );
                    return;
                };
                let Some(v) = self.resolve(name, *npos) else {
                    return;
                };
                if self.vars[v].ty != Ty::Inv {
                    let ty = self.vars[v].ty;
                    if ty != Ty::Error {
                        self.error(
                            "T1",
                            *npos,
                            format!("{name} is {}, not an invocation", ty.shown()),
                        );
                    }
                    return;
                }
                let root = self.vars[v].root;
                if !self.flow.released.insert(root) {
                    self.error("A9", *pos, format!("{name} is already released"));
                }
            }
            Stmt::Expr(e, _) => {
                self.expr(e);
            }
            Stmt::Assign(place, value, pos) => {
                let val = self.expr(value);
                let Expr::Name(name, npos) = place else {
                    self.subset(*pos, "assignment to a field or an element");
                    return;
                };
                let Some(v) = self.resolve(name, *npos) else {
                    return;
                };
                if !self.vars[v].mutable {
                    self.error("N4", *pos, format!("{name} is not MUTABILIS"));
                    return;
                }
                let ty = self.vars[v].ty;
                if ty != val.ty && val.ty != Ty::Error && ty != Ty::Error {
                    self.error(
                        "T1",
                        *pos,
                        format!(
                            "{name} is {}, and cannot take {}",
                            ty.shown(),
                            val.ty.shown()
                        ),
                    );
                }
                let var = &mut self.vars[v];
                var.mark = var.mark.join(val.mark);
            }
            Stmt::Per { pos, .. } => self.subset(*pos, "PER"),
            Stmt::Desine(pos) => self.subset(*pos, "DESINE"),
            Stmt::Perge(pos) => self.subset(*pos, "PERGE"),
            Stmt::Redde(_, pos) => self.subset(*pos, "REDDE"),
            Stmt::Tenta { pos, .. } => self.subset(*pos, "TENTA"),
            Stmt::Gere(_, pos) => {
                self.rite_only(*pos, "GERE");
                self.subset(*pos, "the apparatus (GERE)");
            }
            Stmt::Command(_, _, pos) | Stmt::Constringe(_, _, pos) => {
                self.rite_only(*pos, "the apparatus");
                self.subset(*pos, "the apparatus (OFFER, APPELLA, VETA, CONSTRINGE)");
            }
        }
    }

    fn merge(&mut self, a: Flow, b: Flow, existing: usize, pos: Pos) {
        for v in a.released.symmetric_difference(&b.released) {
            if *v < existing {
                let name = self.vars[*v].name.clone();
                self.error(
                    "A9",
                    pos,
                    format!("{name} is released on one path of this SI but not the other"),
                );
            }
        }
        let inscribed = a
            .inscribed
            .iter()
            .zip(&b.inscribed)
            .map(|(x, y)| x.intersection(y).copied().collect())
            .collect();
        self.flow = Flow {
            released: a.released.union(&b.released).copied().collect(),
            spent: a.spent.union(&b.spent).copied().collect(),
            inscribed,
        };
    }

    fn condition(&mut self, cond: &Expr, word: &str) {
        let val = self.expr(cond);
        if val.ty != Ty::Bool && val.ty != Ty::Error {
            self.error(
                "T3",
                cond.pos(),
                format!("the condition of {word} is {}, not VERITAS", val.ty.shown()),
            );
        }
        if val.mark == Mark::Quarantined {
            self.error(
                "A14",
                cond.pos(),
                format!("a spirit's words cannot steer {word} until consecrated"),
            );
        }
    }

    fn entity(&mut self, entity: &Entity) -> Option<&'r Spirit> {
        let register = self.register;
        let found = match entity {
            Entity::Label(label, _) => register.by_label(label),
            Entity::Spiritus(n, _) => register.by_number(*n),
            Entity::Angelus(..) => None,
        };
        if found.is_none() {
            let what = match entity {
                Entity::Label(l, _) => format!("{l} is not in the {} register", register.rite),
                Entity::Spiritus(n, _) => {
                    format!("SPIRITUS {n} is not in the {} register", register.rite)
                }
                Entity::Angelus(n, _) => {
                    format!("ANGELUS {n} is not defined in the rite {}", register.rite)
                }
            };
            self.error("A3", entity.pos(), what);
        }
        found
    }

    fn invoca(&mut self, entity: &Entity, office: &str, pos: Pos, held: bool) -> Val {
        if !self.rite_only(pos, "INVOCA") {
            return Val::error();
        }
        if !held {
            self.error(
                "A9",
                pos,
                "an invocation must be held by a SIT variable, so that it can be released".into(),
            );
        }
        let Some(spirit) = self.entity(entity) else {
            return Val::error();
        };
        let kind = office.to_uppercase();
        if kind == "NULLUM" || !spirit.holds(&kind) {
            self.error(
                "A5",
                pos,
                format!("{} does not hold the office {office}", spirit.name),
            );
        } else if signature(&kind).is_none() {
            self.subset(pos, &format!("the office {office}"));
        }
        let inscribed = self
            .flow
            .inscribed
            .last()
            .is_some_and(|s| s.contains(&spirit.n));
        if !inscribed {
            self.error(
                "A10",
                pos,
                format!(
                    "{}'s operative seal is not inscribed earlier in this Circle on every path",
                    spirit.name
                ),
            );
        }
        let instruments = spirit.instruments();
        if !instruments.is_empty() {
            self.error(
                "A11",
                pos,
                format!(
                    "{} requires {} taken up before INVOCA",
                    spirit.name,
                    instruments.join(" and ")
                ),
            );
        }
        if spirit.only_president() {
            self.error(
                "A13",
                pos,
                format!(
                    "{} is a President and needs INVOCA REGEM of his king first",
                    spirit.name
                ),
            );
        }
        Val {
            spirit: Some(spirit.n),
            office: Some(kind),
            ..Val::plain(Ty::Inv)
        }
    }

    fn pete(&mut self, name: &str, args: &[Expr], pos: Pos) -> Val {
        let vals: Vec<Val> = args.iter().map(|a| self.expr(a)).collect();
        if !self.rite_only(pos, "PETE") {
            return Val::error();
        }
        let Some(v) = self.resolve(name, pos) else {
            return Val::error();
        };
        let var = self.vars[v].clone();
        if var.ty != Ty::Inv {
            if var.ty != Ty::Error {
                self.error(
                    "T1",
                    pos,
                    format!("PETE needs an invocation; {name} is {}", var.ty.shown()),
                );
            }
            return Val::error();
        }
        if self.flow.released.contains(&var.root) {
            self.error("A9", pos, format!("{name} is used after its release"));
        }
        let Some(kind) = var.office.clone() else {
            return Val::error();
        };
        let Some(sig) = signature(&kind) else {
            return Val::error();
        };
        let mut rest = args.iter().zip(&vals).peekable();
        if !sig.modes.is_empty() {
            match rest.next() {
                Some((Expr::Text(mode, _), _)) if sig.modes.contains(&mode.as_str()) => {}
                Some((e, val)) => {
                    let code = if val.mark == Mark::Quarantined {
                        "A14"
                    } else {
                        "A5"
                    };
                    self.error(
                        code,
                        e.pos(),
                        format!(
                            "the mode of {kind} must be the literal {}",
                            sig.modes.join(" or ")
                        ),
                    );
                }
                None => self.error("A5", pos, format!("{kind} needs a mode")),
            }
        }
        if sig.quaestio {
            match rest.next() {
                Some((e, val)) => {
                    if val.ty != Ty::Text && val.ty != Ty::Error {
                        self.error(
                            "A5",
                            e.pos(),
                            format!(
                                "the question to {kind} must be VERBUM, not {}",
                                val.ty.shown()
                            ),
                        );
                    }
                    if val.mark == Mark::Quarantined {
                        self.error(
                            "A15",
                            e.pos(),
                            "a spirit's words cannot be passed to a spirit until consecrated"
                                .into(),
                        );
                    }
                }
                None => self.error("A5", pos, format!("{kind} needs a question")),
            }
        }
        let mut places = 0;
        for (e, val) in rest {
            places += 1;
            if val.mark == Mark::Quarantined {
                self.error(
                    "A14",
                    e.pos(),
                    "a spirit's words cannot choose a place".into(),
                );
                continue;
            }
            let Some(place) = val.constant.clone() else {
                if val.ty != Ty::Error {
                    self.error(
                        "A5",
                        e.pos(),
                        "a place must be a string literal or an immutable SIT bound to one".into(),
                    );
                }
                continue;
            };
            let grant = self
                .grants
                .iter()
                .find(|g| g.kind == "liber" && g.target == place && g.perms.contains(&sig.perm))
                .cloned();
            match grant {
                None => self.error(
                    "A8",
                    e.pos(),
                    format!(
                        "{kind} needs liber {place:?} {} in the FACULTAS",
                        perm_word(sig.perm)
                    ),
                ),
                Some(g) if g.reserved => self.error(
                    "A5",
                    e.pos(),
                    format!("liber {place:?} is reserved for angels"),
                ),
                Some(_) => {}
            }
        }
        if places < sig.min_places {
            self.error(
                "A5",
                pos,
                format!("{kind} needs at least {} place", sig.min_places),
            );
        }
        Val {
            office: Some(kind),
            ..Val::plain(Ty::Prop)
        }
    }

    fn expr(&mut self, e: &Expr) -> Val {
        match e {
            Expr::Integer(..) => Val::plain(Ty::Num),
            Expr::Text(s, _) => Val {
                constant: Some(s.clone()),
                ..Val::plain(Ty::Text)
            },
            Expr::Truth(..) => Val::plain(Ty::Bool),
            Expr::Nihil(_) => Val::plain(Ty::Nil),
            Expr::Outcome(..) => Val::plain(Ty::Outcome),
            Expr::Name(name, pos) => {
                let Some(v) = self.resolve(name, *pos) else {
                    return Val::error();
                };
                let var = self.vars[v].clone();
                if var.ty == Ty::Inv && self.flow.released.contains(&var.root) {
                    self.error("A9", *pos, format!("{name} is used after its release"));
                }
                Val {
                    ty: var.ty,
                    mark: var.mark,
                    constant: if var.mutable { None } else { var.constant },
                    spirit: var.spirit,
                    ink: var.ink,
                    office: var.office,
                    var: Some(v),
                }
            }
            Expr::Sequence(_, pos) => {
                self.subset(*pos, "sequences");
                Val::error()
            }
            Expr::Call(_, _, pos) => {
                self.subset(*pos, "calls");
                Val::error()
            }
            Expr::Field(_, _, pos) => {
                self.subset(*pos, "fields");
                Val::error()
            }
            Expr::Index(_, _, pos) => {
                self.subset(*pos, "indexing");
                Val::error()
            }
            Expr::Binary(op, l, r, pos) => self.binary(*op, l, r, *pos),
            Expr::Unary(op, operand, pos) => self.unary(*op, operand, *pos),
            Expr::Inscribe(seal, ink, pos) => self.inscribe(seal, ink, *pos),
            Expr::Invoca(entity, office, pos) => self.invoca(entity, office, *pos, false),
            Expr::InvocaRegem(_, pos) => {
                if self.rite_only(*pos, "INVOCA REGEM") {
                    self.subset(*pos, "INVOCA REGEM");
                }
                Val::error()
            }
            Expr::Pete(name, args, pos) => self.pete(name, args, *pos),
            Expr::Sigillum(entity, pos) => {
                if !self.rite_only(*pos, "SIGILLUM") {
                    return Val::error();
                }
                match self.entity(entity) {
                    Some(s) => Val {
                        spirit: Some(s.n),
                        ..Val::plain(Ty::SealEx)
                    },
                    None => Val::error(),
                }
            }
            Expr::Atramentum(class, pos) => {
                if !self.rite_only(*pos, "ATRAMENTUM") {
                    return Val::error();
                }
                let grant = self
                    .grants
                    .iter()
                    .find(|g| g.kind == "atramentum" && g.target == *class)
                    .cloned();
                match grant {
                    None => self.error(
                        "A8",
                        *pos,
                        format!("no atramentum {class:?} grant in the FACULTAS"),
                    ),
                    Some(g) if g.reserved => self.error(
                        "A8",
                        *pos,
                        format!("atramentum {class:?} is reserved for angels"),
                    ),
                    Some(_) => {}
                }
                Val {
                    ink: Some(class.clone()),
                    ..Val::plain(Ty::InkRaw)
                }
            }
            Expr::Nomen(_, pos) => {
                if self.rite_only(*pos, "NOMEN") {
                    self.subset(*pos, "NOMEN");
                }
                Val::error()
            }
            Expr::Entry(entity) => {
                let pos = entity.pos();
                if self.rite_only(pos, "an entry") {
                    self.subset(pos, "entries as values");
                }
                Val::error()
            }
        }
    }

    fn binary(&mut self, op: Binary, l: &Expr, r: &Expr, pos: Pos) -> Val {
        let a = self.expr(l);
        let b = self.expr(r);
        let mark = a.mark.join(b.mark);
        let ty = if a.ty == Ty::Error || b.ty == Ty::Error {
            Ty::Error
        } else {
            let result = match op {
                Binary::Vel | Binary::Et => {
                    (a.ty == Ty::Bool && b.ty == Ty::Bool).then_some(Ty::Bool)
                }
                Binary::Add if a.ty == Ty::Text && b.ty == Ty::Text => Some(Ty::Text),
                Binary::Add | Binary::Sub | Binary::Mul | Binary::Div | Binary::Rem => {
                    (a.ty == Ty::Num && b.ty == Ty::Num).then_some(Ty::Num)
                }
                Binary::Lt | Binary::Le | Binary::Gt | Binary::Ge => {
                    (a.ty == b.ty && matches!(a.ty, Ty::Num | Ty::Text)).then_some(Ty::Bool)
                }
                Binary::Eq | Binary::Ne => {
                    if a.ty != b.ty {
                        self.error(
                            "T2",
                            pos,
                            format!("comparing {} with {}", a.ty.shown(), b.ty.shown()),
                        );
                        return Val {
                            mark,
                            ..Val::plain(Ty::Bool)
                        };
                    }
                    (!a.ty.rite() || a.ty == Ty::Entry).then_some(Ty::Bool)
                }
            };
            match result {
                Some(t) => t,
                None => {
                    self.error(
                        "T1",
                        pos,
                        format!(
                            "this operator does not take {} and {}",
                            a.ty.shown(),
                            b.ty.shown()
                        ),
                    );
                    Ty::Error
                }
            }
        };
        Val {
            mark,
            ..Val::plain(ty)
        }
    }

    fn unary(&mut self, op: Unary, operand: &Expr, pos: Pos) -> Val {
        let words = match op {
            Unary::Consecra => Some("CONSECRA"),
            Unary::Examina => Some("EXAMINA"),
            Unary::Responsum => Some("RESPONSUM"),
            _ => None,
        };
        let v = self.expr(operand);
        if let Some(word) = words {
            if !self.rite_only(pos, word) {
                return Val::error();
            }
        }
        if v.ty == Ty::Error {
            return Val::error();
        }
        let wrong = |me: &mut Self, what: &str| {
            me.error("T1", pos, format!("{what} does not take {}", v.ty.shown()));
            Val::error()
        };
        match op {
            Unary::Neg if v.ty == Ty::Num => Val {
                mark: v.mark,
                ..Val::plain(Ty::Num)
            },
            Unary::Neg => wrong(self, "-"),
            Unary::Non if v.ty == Ty::Bool => Val {
                mark: v.mark,
                ..Val::plain(Ty::Bool)
            },
            Unary::Non => wrong(self, "NON"),
            Unary::Consecra => match v.ty {
                Ty::InkRaw => {
                    self.spend(&v, pos, "raw ink is consecrated once");
                    Val {
                        ink: v.ink.clone(),
                        ..Val::plain(Ty::InkHoly)
                    }
                }
                Ty::InkHoly => {
                    self.error("A7", pos, "this ink is already consecrated".into());
                    Val::error()
                }
                Ty::Text => Val {
                    mark: if v.mark == Mark::Quarantined {
                        Mark::Consecrated
                    } else {
                        v.mark
                    },
                    ..Val::plain(Ty::Text)
                },
                _ => wrong(self, "CONSECRA"),
            },
            Unary::Examina if v.ty == Ty::Prop => Val::plain(Ty::Outcome),
            Unary::Examina => wrong(self, "EXAMINA"),
            Unary::Responsum if v.ty == Ty::Prop => {
                let telling = v
                    .office
                    .as_deref()
                    .and_then(signature)
                    .is_some_and(|s| s.class == Class::Telling);
                if !telling {
                    self.error(
                        "A5",
                        pos,
                        "RESPONSUM needs a proposal of an office that gives an answer".into(),
                    );
                }
                Val {
                    mark: Mark::Quarantined,
                    ..Val::plain(Ty::Text)
                }
            }
            Unary::Responsum => wrong(self, "RESPONSUM"),
        }
    }

    fn spend(&mut self, v: &Val, pos: Pos, rule: &str) {
        if let Some(var) = v.var {
            let root = self.vars[var].root;
            if !self.flow.spent.insert(root) {
                let name = self.vars[var].name.clone();
                self.error("A7", pos, format!("{name} is already spent: {rule}"));
            }
        }
    }

    fn inscribe(&mut self, seal: &Expr, ink: &Expr, pos: Pos) -> Val {
        let s = self.expr(seal);
        let i = self.expr(ink);
        if !self.rite_only(pos, "INSCRIBE") {
            return Val::error();
        }
        if s.ty != Ty::SealEx && s.ty != Ty::Error {
            self.error(
                "A7",
                seal.pos(),
                format!("INSCRIBE needs an exemplar seal, not {}", s.ty.shown()),
            );
        }
        match i.ty {
            Ty::InkHoly => self.spend(&i, pos, "consecrated ink serves one inscription"),
            Ty::InkRaw => self.error(
                "A7",
                ink.pos(),
                "raw ink must be consecrated before it is used".into(),
            ),
            Ty::Error => {}
            other => self.error(
                "A7",
                ink.pos(),
                format!("ATRAMENTO needs consecrated ink, not {}", other.shown()),
            ),
        }
        let Some(n) = s.spirit.filter(|_| s.ty == Ty::SealEx) else {
            return Val::error();
        };
        let spirit = self.register.by_number(i64::from(n));
        if let (Some(spirit), Some(class)) = (spirit, i.ink.as_deref()) {
            if let Some(needed) = spirit.ink_class() {
                if needed != class {
                    self.error(
                        "A7",
                        pos,
                        format!(
                            "{}'s seal takes ATRAMENTUM {needed:?}, not {class:?}",
                            spirit.name
                        ),
                    );
                }
            }
        }
        if let Some(set) = self.flow.inscribed.last_mut() {
            set.insert(n);
        }
        Val {
            spirit: Some(n),
            ..Val::plain(Ty::SealOp)
        }
    }
}

fn perm_word(perm: Perm) -> &'static str {
    match perm {
        Perm::Legere => "legere",
        Perm::Scribere => "scribere",
        Perm::Creare => "creare",
        Perm::Delere => "delere",
    }
}
