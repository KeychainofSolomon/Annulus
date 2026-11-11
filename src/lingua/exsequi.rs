use std::collections::{BTreeMap, BTreeSet};

use super::check::Checked;
use super::registra::Spirit;
use super::syntaxis::*;
use crate::nucleus::*;
use crate::ritus::{self, Goetia};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Responsum {
    pub spirit: u32,
    pub office: String,
    pub quaestio: String,
    pub verum: String,
    pub falsum: Option<String>,
}

#[derive(Clone, Debug, Default)]
pub struct Optiones {
    pub semen: u64,
    pub sors: Option<Outcome>,
    pub responsa: Vec<Responsum>,
}

#[derive(Debug)]
pub struct Exitus {
    pub output: Vec<String>,
    pub class: u8,
    pub message: Option<String>,
    pub state: State,
}

#[derive(Clone, Debug)]
struct Text {
    s: String,
    mark: Mark,
    id: Option<ValueId>,
    origin: Option<u32>,
}

#[derive(Clone, Debug)]
enum V {
    Num(i64),
    Bool(bool, Mark),
    Text(Text),
    Nil,
    Outcome(WorkOutcome),
    Inv(InvId),
    Prop(Pid),
    SealEx(u32),
    SealOp,
    Ink(String),
}

enum Halt {
    Computation(String),
    Denial(String),
    Ended(String),
}

type Go<T> = Result<T, Halt>;

struct Asked {
    spirit: &'static Spirit,
    office: String,
    quaestio: Option<String>,
    mode: Option<String>,
    places: Vec<String>,
}

struct Run<'p> {
    state: State,
    rite: Goetia,
    sim: Sim,
    sors: Option<Outcome>,
    responsa: &'p [Responsum],
    ink: BTreeMap<String, i64>,
    scopes: Vec<BTreeMap<String, V>>,
    asked: BTreeMap<Pid, Asked>,
    offices: BTreeMap<InvId, String>,
    delayed: BTreeSet<Pid>,
    next_value: ValueId,
    output: Vec<String>,
}

pub fn run(
    program: &Program,
    checked: &Checked,
    wand: Wand,
    stores: Stores,
    world: BTreeMap<Place, (String, Mark)>,
    options: &Optiones,
) -> Exitus {
    let (launch, ink) = ritus::launch(checked, wand, stores, world);
    let mut run = Run {
        state: State::initial(launch),
        rite: Goetia::new(super::registra::goetia()),
        sim: Sim::new(options.semen),
        sors: options.sors,
        responsa: &options.responsa,
        ink,
        scopes: vec![BTreeMap::new()],
        asked: BTreeMap::new(),
        offices: BTreeMap::new(),
        delayed: BTreeSet::new(),
        next_value: 1 << 40,
        output: Vec::new(),
    };
    if let Err(refusal) = run.state.launch() {
        let why = match refusal {
            Refusal::LaunchRefused(w) | Refusal::Denied(w) | Refusal::NotEnabled(w) => w,
        };
        return run.finish(2, Some(format!("the launch is refused: {why}")));
    }
    if !run.state.auth.wand.present {
        run.ink.clear();
    }
    let result = program.items.iter().try_for_each(|item| match item {
        Item::Statement(s) => run.stmt(s),
        _ => Err(Halt::Denial(
            "this item is outside the step-2 subset".to_string(),
        )),
    });
    match result {
        Ok(()) => match run.op(|_| true, |s, _| s.halt()) {
            Ok(()) => run.finish(0, None),
            Err(h) => run.halt(h),
        },
        Err(h) => run.halt(h),
    }
}

impl<'p> Run<'p> {
    fn finish(self, class: u8, message: Option<String>) -> Exitus {
        Exitus {
            output: self.output,
            class,
            message,
            state: self.state,
        }
    }

    fn halt(mut self, h: Halt) -> Exitus {
        let (class, message) = match h {
            Halt::Computation(m) => (1, m),
            Halt::Denial(m) => (2, m),
            Halt::Ended(m) => {
                let class = match &self.state.phase {
                    Phase::Ended(Ending::Failure(c, _)) => *c,
                    _ => 2,
                };
                return self.finish(class, Some(m));
            }
        };
        if self.state.running() {
            self.state.current = None;
            if self.state.schedule(OPERATOR).is_ok() {
                let _ = self.state.fail(class);
            }
        }
        self.finish(class, Some(message))
    }

    fn refused(&self, r: Refusal) -> Halt {
        match r {
            Refusal::Denied(w) | Refusal::LaunchRefused(w) => Halt::Ended(w.to_string()),
            Refusal::NotEnabled(w) => Halt::Denial(format!("the step is not enabled: {w}")),
        }
    }

    fn op<T>(
        &mut self,
        ready: impl Fn(&State) -> bool,
        step: impl FnOnce(&mut State, &Goetia) -> Step<T>,
    ) -> Go<T> {
        loop {
            if !self.state.running() {
                return Err(Halt::Ended("the run has ended".to_string()));
            }
            let can = ready(&self.state);
            match self.sim.pick(&self.state, can) {
                None => {
                    return Err(Halt::Denial(
                        "the run is stalled: no spirit can act and the Operator cannot go on"
                            .to_string(),
                    ))
                }
                Some(OPERATOR) => {
                    self.state.schedule(OPERATOR).map_err(|r| self.refused(r))?;
                    return step(&mut self.state, &self.rite).map_err(|r| self.refused(r));
                }
                Some(worker) => self.spirit_turn(worker)?,
            }
        }
    }

    fn fresh_value(&mut self) -> ValueId {
        self.next_value += 1;
        self.next_value
    }

    fn spirit_turn(&mut self, worker: WorkerId) -> Go<()> {
        self.state.schedule(worker).map_err(|r| self.refused(r))?;
        let forced = self
            .sors
            .filter(|o| *o != Outcome::Occisio && self.rite.will_open(&self.state, worker, *o));
        let outcome = match forced {
            Some(o) => o,
            None => self
                .sim
                .draw(&self.state, &self.rite, worker)
                .ok_or_else(|| Halt::Denial("the spirit's will has no open outcome".into()))?,
        };
        let Some(&prop) = self
            .state
            .workers
            .get(&worker)
            .and_then(|w| w.queue.front())
        else {
            return Err(Halt::Denial(
                "a spirit was scheduled with no request".into(),
            ));
        };
        let (contrib, finish) = if outcome == Outcome::Mora && self.delayed.insert(prop) {
            (Contrib::Idle, false)
        } else {
            (self.work(prop, outcome), true)
        };
        self.state
            .turn(&self.rite, worker, outcome, contrib, finish)
            .map_err(|r| self.refused(r))
    }

    fn work(&mut self, prop: Pid, outcome: Outcome) -> Contrib {
        let Some(asked) = self.asked.get(&prop) else {
            return Contrib::Idle;
        };
        let spirit = asked.spirit;
        let class = crate::officia::signature(&asked.office).map(|s| s.class);
        if class == Some(crate::officia::Class::Telling) {
            let q = asked.quaestio.clone().unwrap_or_default();
            let script = self.responsa.iter().find(|r| {
                r.spirit == spirit.n
                    && r.office.eq_ignore_ascii_case(&asked.office)
                    && r.quaestio == q
            });
            let tag = |kind: &str| {
                format!(
                    "[simulatum{kind}: {}, {}, {:?}]",
                    spirit.name, asked.office, q
                )
            };
            let truth = script.map(|r| r.verum.clone()).unwrap_or_else(|| tag(""));
            let text = match outcome {
                Outcome::Pars => {
                    let half = truth.chars().count() / 2;
                    truth.chars().take(half.max(1)).collect()
                }
                Outcome::Mendacium => script
                    .and_then(|r| r.falsum.clone())
                    .unwrap_or_else(|| tag(", falsum")),
                Outcome::Malitia => tag(", malitia"),
                _ => truth,
            };
            let id = self.fresh_value();
            return Contrib::Respond(id, text);
        }
        let read = |p: &String| {
            self.state
                .world
                .get(p)
                .map(|(d, _)| d.clone())
                .unwrap_or_default()
        };
        let effect = |verb, place: &String, data: String| Effect {
            verb,
            place: place.clone(),
            data,
        };
        let places = &asked.places;
        let mut effects: Vec<Effect> = match (asked.office.as_str(), asked.mode.as_deref()) {
            ("AEDIFICATIO", _) => places
                .iter()
                .map(|p| effect(Verb::Crea, p, String::new()))
                .collect(),
            ("EVERSIO", _) => places
                .iter()
                .map(|p| effect(Verb::Dele, p, String::new()))
                .collect(),
            ("VECTIO", Some("transfer")) => vec![
                effect(Verb::Transfer, &places[1], read(&places[0])),
                effect(Verb::Dele, &places[0], String::new()),
            ],
            ("VECTIO", _) => vec![effect(Verb::Exscribe, &places[1], read(&places[0]))],
            ("FURTUM", _) => vec![
                effect(Verb::Exscribe, &places[1], read(&places[0])),
                effect(Verb::Aufer, &places[0], String::new()),
            ],
            ("CONCORDIA", _) => {
                let (sources, target) = places.split_at(places.len() - 1);
                let texts: Vec<String> = sources.iter().map(read).collect();
                vec![effect(Verb::Coniunge, &target[0], merge(&texts))]
            }
            ("DISCORDIA", _) => {
                let source = read(&places[0]);
                places[1..]
                    .iter()
                    .map(|p| effect(Verb::Scinde, p, source.clone()))
                    .collect()
            }
            _ => Vec::new(),
        };
        match outcome {
            Outcome::Pars => effects.truncate(effects.len() / 2),
            Outcome::Mendacium => effects
                .iter_mut()
                .for_each(|e| e.data = format!("[falsum: {}]", spirit.name)),
            Outcome::Malitia => {
                if let Some(first) = asked.places.first() {
                    effects.push(Effect {
                        verb: Verb::Dele,
                        place: first.clone(),
                        data: String::new(),
                    });
                }
            }
            _ => {}
        }
        Contrib::Effects(effects)
    }

    fn lookup(&self, name: &str) -> Go<V> {
        self.scopes
            .iter()
            .rev()
            .find_map(|s| s.get(name))
            .cloned()
            .ok_or_else(|| Halt::Denial(format!("unknown name {name} at run time")))
    }

    fn assign(&mut self, name: &str, value: V) -> Go<()> {
        for scope in self.scopes.iter_mut().rev() {
            if let Some(slot) = scope.get_mut(name) {
                *slot = value;
                return Ok(());
            }
        }
        Err(Halt::Denial(format!("unknown name {name} at run time")))
    }

    fn block(&mut self, body: &[Stmt]) -> Go<()> {
        self.scopes.push(BTreeMap::new());
        let result = body.iter().try_for_each(|s| self.stmt(s));
        self.scopes.pop();
        result
    }

    fn truth(&mut self, e: &Expr) -> Go<(bool, Mark)> {
        match self.expr(e)? {
            V::Bool(b, m) => Ok((b, m)),
            _ => Err(Halt::Denial("a condition is not VERITAS".into())),
        }
    }

    fn condition(&mut self, e: &Expr) -> Go<bool> {
        let (b, mark) = self.truth(e)?;
        if mark == Mark::Quarantined {
            return Err(Halt::Denial(
                "a spirit's words would steer the program before consecration".into(),
            ));
        }
        Ok(b)
    }

    fn inv_of(&mut self, e: &Expr) -> Go<InvId> {
        match self.expr(e)? {
            V::Inv(i) => Ok(i),
            _ => Err(Halt::Denial("not an invocation".into())),
        }
    }

    fn stmt(&mut self, stmt: &Stmt) -> Go<()> {
        match stmt {
            Stmt::Sit { name, value, .. } => {
                let v = self.expr(value)?;
                if let Some(scope) = self.scopes.last_mut() {
                    scope.insert(name.clone(), v);
                }
                Ok(())
            }
            Stmt::Manifesta(e, _) => {
                let v = self.expr(e)?;
                let line = self.show(&v);
                self.output.push(line);
                Ok(())
            }
            Stmt::Si {
                cond,
                then,
                otherwise,
                ..
            } => {
                if self.condition(cond)? {
                    self.block(then)
                } else if let Some(o) = otherwise {
                    self.block(o)
                } else {
                    Ok(())
                }
            }
            Stmt::Dum { cond, body, .. } => {
                while self.condition(cond)? {
                    self.block(body)?;
                }
                Ok(())
            }
            Stmt::Circulus { body, .. } => {
                self.op(|_| true, |s, _| s.open_circle())?;
                self.block(body)?;
                self.op(|_| true, |s, _| s.close_circle())
            }
            Stmt::Release(kind, e, _) => {
                let inv = self.inv_of(e)?;
                match kind {
                    Release::Relega => self.op(|_| true, |s, _| s.relegate(inv)),
                    _ => self.op(|_| true, |s, _| s.release(inv)),
                }
            }
            Stmt::Expr(e, _) => self.expr(e).map(|_| ()),
            Stmt::Assign(Expr::Name(name, _), value, _) => {
                let v = self.expr(value)?;
                self.assign(name, v)
            }
            _ => Err(Halt::Denial(
                "this statement is outside the step-2 subset".into(),
            )),
        }
    }

    fn show(&self, v: &V) -> String {
        match v {
            V::Num(n) => n.to_string(),
            V::Bool(true, _) => "VERUM".into(),
            V::Bool(false, _) => "FALSUM".into(),
            V::Nil => "NIHIL".into(),
            V::Outcome(WorkOutcome::Ratum) => "RATUM".into(),
            V::Outcome(WorkOutcome::Suspensum) => "SUSPENSUM".into(),
            V::Outcome(WorkOutcome::Irritum) => "IRRITUM".into(),
            V::Text(t) if t.mark == Mark::Quarantined => {
                let who = t
                    .origin
                    .and_then(|n| self.rite.register.by_number(i64::from(n)))
                    .map(|s| format!("[{}, {}] ", s.name, s.veracity))
                    .unwrap_or_else(|| "[spiritus] ".into());
                format!("{who}{}", t.s)
            }
            V::Text(t) => t.s.clone(),
            V::SealEx(n) => self
                .rite
                .register
                .by_number(i64::from(*n))
                .map(|s| format!("SIGILLUM {} ({})", s.n, s.name))
                .unwrap_or_default(),
            _ => String::new(),
        }
    }

    fn spirit(&self, entity: &Entity) -> Go<&'static Spirit> {
        let register = self.rite.register;
        match entity {
            Entity::Label(l, _) => register.by_label(l),
            Entity::Spiritus(n, _) => register.by_number(*n),
            Entity::Angelus(..) => None,
        }
        .ok_or_else(|| Halt::Denial("an entity not in the register".into()))
    }

    fn text_of(&mut self, s: String, ins: &[&Text]) -> Go<V> {
        let tracked: Vec<ValueId> = ins.iter().filter_map(|t| t.id).collect();
        let mark = Mark::join_all(ins.iter().map(|t| t.mark));
        let origin = ins.iter().find_map(|t| t.origin);
        if tracked.is_empty() {
            return Ok(V::Text(Text {
                s,
                mark,
                id: None,
                origin,
            }));
        }
        let id = self.fresh_value();
        let made = Made {
            id,
            data: s.clone(),
            ins: tracked,
            places: Vec::new(),
            rite: false,
        };
        self.op(|_| true, |st, _| st.compute(&[], &[made]))?;
        let mark = self.state.values.get(&id).map(|v| v.mark).unwrap_or(mark);
        Ok(V::Text(Text {
            s,
            mark,
            id: Some(id),
            origin,
        }))
    }

    fn expr(&mut self, e: &Expr) -> Go<V> {
        match e {
            Expr::Integer(n, _) => Ok(V::Num(*n)),
            Expr::Text(s, _) => Ok(V::Text(Text {
                s: s.clone(),
                mark: Mark::Clean,
                id: None,
                origin: None,
            })),
            Expr::Truth(b, _) => Ok(V::Bool(*b, Mark::Clean)),
            Expr::Nihil(_) => Ok(V::Nil),
            Expr::Outcome(j, _) => Ok(V::Outcome(match j {
                Judged::Ratum => WorkOutcome::Ratum,
                Judged::Suspensum => WorkOutcome::Suspensum,
                Judged::Irritum => WorkOutcome::Irritum,
            })),
            Expr::Name(n, _) => self.lookup(n),
            Expr::Binary(op, l, r, _) => self.binary(*op, l, r),
            Expr::Unary(op, operand, _) => self.unary(*op, operand),
            Expr::Inscribe(seal, ink, _) => {
                let s = self.expr(seal)?;
                let i = self.expr(ink)?;
                match (s, i) {
                    (V::SealEx(_), V::Ink(_)) => Ok(V::SealOp),
                    _ => Err(Halt::Denial(
                        "INSCRIBE of something not a seal and ink".into(),
                    )),
                }
            }
            Expr::Sigillum(entity, _) => Ok(V::SealEx(self.spirit(entity)?.n)),
            Expr::Atramentum(class, _) => {
                let left = self.ink.entry(class.clone()).or_insert(0);
                if *left <= 0 {
                    return Err(Halt::Denial(format!(
                        "no ink {class:?} is left in the faculty"
                    )));
                }
                *left -= 1;
                Ok(V::Ink(class.clone()))
            }
            Expr::Invoca(entity, office, _) => {
                let spirit = self.spirit(entity)?;
                let seal = spirit.seal(ritus::RITE);
                let inv = self.op(|_| true, |s, r| s.invoke(r, &seal))?;
                self.offices.insert(inv, office.to_uppercase());
                Ok(V::Inv(inv))
            }
            Expr::Pete(name, args, _) => self.pete(name, args),
            _ => Err(Halt::Denial(
                "this expression is outside the step-2 subset".into(),
            )),
        }
    }

    fn pete(&mut self, name: &str, args: &[Expr]) -> Go<V> {
        let V::Inv(inv) = self.lookup(name)? else {
            return Err(Halt::Denial("PETE of something not an invocation".into()));
        };
        let mut vals = Vec::new();
        for a in args {
            vals.push(self.expr(a)?);
        }
        let seal = self
            .state
            .invs
            .get(&inv)
            .map(|i| i.seal.clone())
            .unwrap_or_default();
        let spirit = self
            .rite
            .spirit(&seal)
            .ok_or_else(|| Halt::Denial("PETE to an unknown spirit".into()))?;
        let office = self
            .offices
            .get(&inv)
            .cloned()
            .ok_or_else(|| Halt::Denial("an invocation without an office".into()))?;
        let sig = crate::officia::signature(&office)
            .ok_or_else(|| Halt::Denial("an office outside the step-2 subset".into()))?;
        let mut texts = vals.into_iter().map(|v| match v {
            V::Text(t) => t.s,
            _ => String::new(),
        });
        let mode = if sig.modes.is_empty() {
            None
        } else {
            texts.next()
        };
        let quaestio = if sig.quaestio { texts.next() } else { None };
        let places: Vec<String> = texts.collect();
        let form = ReqForm {
            office: office.clone(),
            mode: mode.clone(),
            places: places.clone(),
        };
        let prop = self.op(|_| true, |s, r| s.request(r, Addressee::Spirit(inv), &form))?;
        self.asked.insert(
            prop,
            Asked {
                spirit,
                office,
                quaestio,
                mode,
                places,
            },
        );
        Ok(V::Prop(prop))
    }

    fn binary(&mut self, op: Binary, l: &Expr, r: &Expr) -> Go<V> {
        if matches!(op, Binary::Et | Binary::Vel) {
            let (a, ma) = self.truth(l)?;
            if (op == Binary::Et && !a) || (op == Binary::Vel && a) {
                return Ok(V::Bool(a, ma));
            }
            let (b, mb) = self.truth(r)?;
            return Ok(V::Bool(b, ma.join(mb)));
        }
        let a = self.expr(l)?;
        let b = self.expr(r)?;
        let overflow = || Halt::Computation("NUMERUS overflow".into());
        match (op, &a, &b) {
            (Binary::Add, V::Num(x), V::Num(y)) => {
                x.checked_add(*y).map(V::Num).ok_or_else(overflow)
            }
            (Binary::Sub, V::Num(x), V::Num(y)) => {
                x.checked_sub(*y).map(V::Num).ok_or_else(overflow)
            }
            (Binary::Mul, V::Num(x), V::Num(y)) => {
                x.checked_mul(*y).map(V::Num).ok_or_else(overflow)
            }
            (Binary::Div | Binary::Rem, V::Num(_), V::Num(0)) => {
                Err(Halt::Computation("division by zero".into()))
            }
            (Binary::Div, V::Num(x), V::Num(y)) => {
                x.checked_div(*y).map(V::Num).ok_or_else(overflow)
            }
            (Binary::Rem, V::Num(x), V::Num(y)) => {
                x.checked_rem(*y).map(V::Num).ok_or_else(overflow)
            }
            (Binary::Add, V::Text(x), V::Text(y)) => {
                let s = format!("{}{}", x.s, y.s);
                let (x, y) = (x.clone(), y.clone());
                self.text_of(s, &[&x, &y])
            }
            (Binary::Lt | Binary::Le | Binary::Gt | Binary::Ge | Binary::Eq | Binary::Ne, _, _) => {
                let ord = match (&a, &b) {
                    (V::Num(x), V::Num(y)) => Some(x.cmp(y)),
                    (V::Text(x), V::Text(y)) => Some(x.s.cmp(&y.s)),
                    _ => None,
                };
                let equal = match (&a, &b) {
                    (V::Bool(x, _), V::Bool(y, _)) => x == y,
                    (V::Nil, V::Nil) => true,
                    (V::Outcome(x), V::Outcome(y)) => x == y,
                    _ => ord == Some(std::cmp::Ordering::Equal),
                };
                let mark = mark_of(&a).join(mark_of(&b));
                Ok(V::Bool(
                    match (op, ord) {
                        (Binary::Eq, _) => equal,
                        (Binary::Ne, _) => !equal,
                        (Binary::Lt, Some(o)) => o.is_lt(),
                        (Binary::Le, Some(o)) => o.is_le(),
                        (Binary::Gt, Some(o)) => o.is_gt(),
                        (Binary::Ge, Some(o)) => o.is_ge(),
                        _ => return Err(Halt::Denial("an operator on the wrong values".into())),
                    },
                    mark,
                ))
            }
            _ => Err(Halt::Denial("an operator on the wrong values".into())),
        }
    }

    fn unary(&mut self, op: Unary, operand: &Expr) -> Go<V> {
        let v = self.expr(operand)?;
        match (op, v) {
            (Unary::Neg, V::Num(n)) => n
                .checked_neg()
                .map(V::Num)
                .ok_or_else(|| Halt::Computation("NUMERUS overflow".into())),
            (Unary::Non, V::Bool(b, m)) => Ok(V::Bool(!b, m)),
            (Unary::Consecra, V::Ink(class)) => Ok(V::Ink(class)),
            (Unary::Consecra, V::Text(t)) => {
                let Some(id) = t.id.filter(|_| t.mark == Mark::Quarantined) else {
                    return Ok(V::Text(t));
                };
                let accepted = self.op(|_| true, |s, r| s.consecrate(r, id))?;
                if !accepted {
                    return Err(Halt::Denial(
                        "CONSECRA: the validator does not accept the spirit's words".into(),
                    ));
                }
                Ok(V::Text(Text {
                    mark: Mark::Consecrated,
                    ..t
                }))
            }
            (Unary::Examina, V::Prop(p)) => {
                let outcome = self.op(
                    move |s| {
                        s.props
                            .get(&p)
                            .is_some_and(|q| q.status != PStatus::Pending)
                    },
                    |s, r| s.examine(r, p),
                )?;
                Ok(V::Outcome(outcome))
            }
            (Unary::Responsum, V::Prop(p)) => {
                let prop = self.state.props.get(&p).cloned();
                match prop {
                    Some(Proposal {
                        status: PStatus::Judged(WorkOutcome::Ratum),
                        content: Content::Response(v, _),
                        ..
                    }) => {
                        let s = self
                            .state
                            .values
                            .get(&v)
                            .and_then(|x| x.data.clone())
                            .unwrap_or_default();
                        let origin = self.asked.get(&p).map(|a| a.spirit.n);
                        Ok(V::Text(Text {
                            s,
                            mark: Mark::Quarantined,
                            id: Some(v),
                            origin,
                        }))
                    }
                    _ => Err(Halt::Denial(
                        "RESPONSUM of a proposal that is not in RATUM".into(),
                    )),
                }
            }
            _ => Err(Halt::Denial("an operator on the wrong values".into())),
        }
    }
}

fn mark_of(v: &V) -> Mark {
    match v {
        V::Text(t) => t.mark,
        V::Bool(_, m) => *m,
        _ => Mark::Clean,
    }
}

fn merge(sources: &[String]) -> String {
    let split: Vec<Vec<&str>> = sources.iter().map(|s| s.lines().collect()).collect();
    let longest = split.iter().map(Vec::len).max().unwrap_or(0);
    (0..longest)
        .filter_map(|i| split.iter().find_map(|lines| lines.get(i).copied()))
        .collect::<Vec<&str>>()
        .join("\n")
}
