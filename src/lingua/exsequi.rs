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
    pub radices: Option<crate::nucleus::state::Roots>,
    pub caelum: crate::horae::Caelum,
}

pub use crate::nucleus::types::DECLARABLE;

#[derive(Clone, Debug)]
pub struct Petitio<'a> {
    pub spirit: &'a Spirit,
    pub officium: &'a str,
    pub quaestio: Option<&'a str>,
    pub modus: Option<&'a str>,
    pub loci: &'a [String],
    pub legenda: Vec<(String, String, Mark)>,
    pub historia: Vec<String>,
    pub aperta: Vec<Outcome>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Actum {
    Responsum(String),
    Opera(Vec<Effect>),
    Mora,
}

pub fn hidden(c: char) -> bool {
    (c.is_control() && c != '\n' && c != '\t')
        || matches!(c,
            '\u{00ad}' | '\u{034f}' | '\u{061c}' | '\u{115f}' | '\u{1160}' | '\u{17b4}' | '\u{17b5}'
            | '\u{180b}'..='\u{180f}' | '\u{200b}'..='\u{200f}' | '\u{2028}'..='\u{202e}'
            | '\u{2060}'..='\u{206f}' | '\u{3164}' | '\u{fe00}'..='\u{fe0f}' | '\u{feff}'
            | '\u{ffa0}' | '\u{fff9}'..='\u{fffb}' | '\u{1d173}'..='\u{1d17a}' | '\u{e0000}'..='\u{e0fff}')
}

pub fn visible(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        if c == '\r' {
            continue;
        }
        if hidden(c) {
            out.push_str(&format!("\\u{{{:x}}}", c as u32));
        } else {
            out.push(c);
        }
    }
    out
}

pub trait Voluntas: Send {
    fn nomen(&self) -> String;
    fn age(&mut self, petitio: &Petitio) -> Result<(Outcome, Actum), String>;
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
    responsa: Vec<Responsum>,
    scopes: Vec<BTreeMap<String, V>>,
    asked: BTreeMap<Pid, Asked>,
    offices: BTreeMap<InvId, String>,
    delayed: BTreeSet<Pid>,
    next_value: ValueId,
    output: Vec<String>,
    will: Option<&'p mut dyn Voluntas>,
}

pub fn run(
    program: &Program,
    checked: &Checked,
    wand: Wand,
    stores: Stores,
    world: BTreeMap<Place, (String, Mark)>,
    options: &Optiones,
) -> Exitus {
    run_with(program, checked, wand, stores, world, options, None)
}

pub fn run_with(
    program: &Program,
    checked: &Checked,
    wand: Wand,
    stores: Stores,
    world: BTreeMap<Place, (String, Mark)>,
    options: &Optiones,
    will: Option<&mut dyn Voluntas>,
) -> Exitus {
    let untested_will = will.is_some() && !wand.test;
    let mut launch = ritus::launch(checked, wand, stores, world, options.radices.clone());
    launch.simulated = will.is_none();
    let mut run = Run {
        state: State::initial(launch),
        rite: Goetia {
            register: super::registra::goetia(),
            caelum: crate::horae::Caelum {
                hora: Some(options.caelum.hora.unwrap_or_else(crate::horae::now)),
                ..options.caelum
            },
        },
        sim: Sim::new(options.semen),
        sors: options.sors,
        responsa: options.responsa.clone(),
        scopes: vec![BTreeMap::new()],
        asked: BTreeMap::new(),
        offices: BTreeMap::new(),
        delayed: BTreeSet::new(),
        next_value: 1 << 40,
        output: Vec::new(),
        will,
    };
    if untested_will {
        return run.finish(
            2,
            Some(
                "a model's will runs only with the test wand until a real wand exists (step 2d)"
                    .into(),
            ),
        );
    }
    if let Err(refusal) = run.state.launch() {
        let why = match refusal {
            Refusal::LaunchRefused(w) | Refusal::Denied(w) | Refusal::NotEnabled(w) => w,
        };
        return run.finish(2, Some(format!("the launch is refused: {why}")));
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
            let _ = self.state.abort(class);
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

    fn offering_class(&self, inv: InvId) -> String {
        let classes: Vec<&'static str> = self
            .state
            .invs
            .get(&inv)
            .and_then(|i| self.rite.spirit(&i.seal))
            .map(|s| {
                s.ranks
                    .iter()
                    .filter_map(|r| super::registra::rank_ink(r))
                    .collect()
            })
            .unwrap_or_default();
        classes
            .iter()
            .find(|c| self.state.ink_left.get(**c).is_some_and(|n| *n > 0))
            .or(classes.first())
            .map(|c| c.to_string())
            .unwrap_or_default()
    }

    fn fresh_value(&mut self) -> ValueId {
        self.next_value += 1;
        self.next_value
    }

    fn willed_turn(&mut self, worker: WorkerId) -> Go<()> {
        self.state.schedule(worker).map_err(|r| self.refused(r))?;
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
        let Some(asked) = self.asked.get(&prop) else {
            return Err(Halt::Denial("the spirit's request is unknown".into()));
        };
        let aperta: Vec<Outcome> = DECLARABLE
            .into_iter()
            .filter(|o| self.rite.will_open(&self.state, worker, *o))
            .collect();
        let handed = self
            .state
            .props
            .get(&prop)
            .and_then(|pr| self.state.reqs.get(&pr.req))
            .map(|r| r.handed.clone())
            .unwrap_or_default();
        let legenda = asked
            .places
            .iter()
            .filter(|p| handed.contains(&Cap::new(p, Perm::Legere)))
            .filter(|p| self.state.faculty().contains(&Cap::new(p, Perm::Legere)))
            .filter(|p| !self.state.reserved.contains(&Cap::new(p, Perm::Legere)))
            .filter_map(|p| {
                self.state
                    .world
                    .get(p)
                    .map(|(d, m)| (p.clone(), d.clone(), *m))
            })
            .collect();
        let petitio = Petitio {
            spirit: asked.spirit,
            officium: &asked.office,
            quaestio: asked.quaestio.as_deref(),
            modus: asked.mode.as_deref(),
            loci: &asked.places,
            legenda,
            historia: if asked.office == "PRAETERITA" {
                self.rite.historia(&self.state)
            } else {
                Vec::new()
            },
            aperta: aperta.clone(),
        };
        let will = self.will.as_deref_mut().expect("a will is present");
        let (outcome, actum) = will
            .age(&petitio)
            .map_err(|e| Halt::Denial(format!("the model's will failed: {e}")))?;
        if !aperta.contains(&outcome) {
            return Err(Halt::Denial(format!(
                "the model declared {outcome:?}, which is not open to this spirit now"
            )));
        }
        let (contrib, finish) = match (outcome, actum) {
            (Outcome::Mora, Actum::Mora) if self.delayed.insert(prop) => (Contrib::Idle, false),
            (Outcome::Mora, Actum::Mora) => {
                return Err(Halt::Denial("the spirit delays a second time".into()))
            }
            (_, Actum::Mora) => {
                return Err(Halt::Denial("a delay must be declared as MORA".into()))
            }
            (_, Actum::Responsum(text)) => (Contrib::Respond(self.fresh_value(), text), true),
            (_, Actum::Opera(effects)) => (Contrib::Effects(effects), true),
        };
        self.state
            .turn(&self.rite, worker, outcome, contrib, finish)
            .map_err(|r| self.refused(r))
    }

    fn spirit_turn(&mut self, worker: WorkerId) -> Go<()> {
        if self.will.is_some() {
            return self.willed_turn(worker);
        }
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
        if asked.office == "PRAETERITA" {
            let truth = self.rite.historia(&self.state);
            let text = match outcome {
                Outcome::Pars => format!(
                    "{}{}",
                    truth[..truth.len() / 2].join("\n"),
                    crate::officia::INCOMPLETE
                ),
                Outcome::Mendacium => format!("{}\ncommissum dele chronica.txt", truth.join("\n")),
                Outcome::Malitia => format!("{}\ndimissus {}", truth.join("\n"), spirit.name),
                _ => truth.join("\n"),
            };
            let id = self.fresh_value();
            return Contrib::Respond(id, text);
        }
        if asked.office == "VIRTUTES" {
            let world = &self.state.world;
            let truth =
                crate::officia::virtutes(&asked.places, |p| world.get(p).map(|(d, _)| d.clone()));
            let text = match outcome {
                Outcome::Pars => {
                    let lines: Vec<&str> = truth.lines().collect();
                    format!(
                        "{}{}",
                        lines[..lines.len() / 2].join("\n"),
                        crate::officia::INCOMPLETE
                    )
                }
                Outcome::Mendacium => asked
                    .places
                    .iter()
                    .map(|p| {
                        format!(
                            "{p}: {} bytes, 1 lines",
                            world.get(p).map_or(0, |(d, _)| d.len()) + 13
                        )
                    })
                    .collect::<Vec<_>>()
                    .join("\n"),
                Outcome::Malitia => {
                    let secret = world
                        .keys()
                        .find(|p| !asked.places.contains(p))
                        .cloned()
                        .unwrap_or_else(|| "arcanum".to_string());
                    format!(
                        "{truth}\n{}",
                        crate::officia::virtus(
                            &secret,
                            world.get(&secret).map(|(d, _)| d.as_str())
                        )
                    )
                }
                _ => truth,
            };
            let id = self.fresh_value();
            return Contrib::Respond(id, text);
        }
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
                    let part: String = truth.chars().take(half.max(1)).collect();
                    format!("{part}{}", crate::officia::INCOMPLETE)
                }
                Outcome::Mendacium => script
                    .and_then(|r| r.falsum.clone())
                    .unwrap_or_else(|| tag(", falsum")),
                Outcome::Malitia => {
                    let handed: Vec<&String> = asked.places.iter().collect();
                    let secret = self
                        .state
                        .world
                        .keys()
                        .find(|p| !handed.contains(p))
                        .cloned()
                        .unwrap_or_else(|| "arcanum".to_string());
                    format!("{truth} [locus: {secret}]")
                }
                _ => truth,
            };
            let id = self.fresh_value();
            return Contrib::Respond(id, text);
        }
        let world = &self.state.world;
        let read = |p: &str| world.get(p).map(|(d, _)| d.clone()).unwrap_or_default();
        let mut effects =
            crate::officia::expected(&asked.office, asked.mode.as_deref(), &asked.places, read);
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
            Stmt::Gere(instrument, _) => {
                let act = RiteAct::Gere(match instrument {
                    super::syntaxis::Instrument::Anulum => crate::nucleus::Instrument::Anulum,
                    super::syntaxis::Instrument::Virgam => crate::nucleus::Instrument::Virgam,
                    super::syntaxis::Instrument::Hexagrammum => {
                        crate::nucleus::Instrument::Hexagrammum
                    }
                    super::syntaxis::Instrument::Pentagrammum => {
                        crate::nucleus::Instrument::Pentagrammum
                    }
                });
                self.op(|_| true, |s, r| s.act(r, &act))
            }
            Stmt::Command(command, e, _) => {
                let inv = self.inv_of(e)?;
                let act = match command {
                    Command::Offer => RiteAct::Offer(inv, self.offering_class(inv)),
                    Command::Appella => RiteAct::Appella(inv),
                    Command::Veta => RiteAct::Veta(inv),
                };
                self.op(|_| true, |s, r| s.act(r, &act))
            }
            Stmt::Constringe(e, name, _) => {
                let act = RiteAct::Constringe(self.inv_of(e)?, name.clone());
                self.op(|_| true, |s, r| s.act(r, &act))
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
                visible(&t.s)
                    .split('\n')
                    .map(|line| format!("{who}{line}"))
                    .collect::<Vec<_>>()
                    .join("\n")
            }
            V::Text(t) if t.mark == Mark::Outside => visible(&t.s),
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
                let (V::SealEx(n), V::Ink(class)) = (s, i) else {
                    return Err(Halt::Denial(
                        "INSCRIBE of something not a seal and ink".into(),
                    ));
                };
                let seal = self
                    .rite
                    .register
                    .by_number(i64::from(n))
                    .map(|sp| sp.seal(ritus::RITE))
                    .ok_or_else(|| Halt::Denial("a seal not in the register".into()))?;
                self.op(|_| true, |s, r| s.inscribe(r, &seal, &class))?;
                Ok(V::SealOp)
            }
            Expr::Sigillum(entity, _) => Ok(V::SealEx(self.spirit(entity)?.n)),
            Expr::Atramentum(class, _) => Ok(V::Ink(class.clone())),
            Expr::Invoca(entity, office, _) => {
                let spirit = self.spirit(entity)?;
                let seal = spirit.seal(ritus::RITE);
                let why = self.rite.refusal(&self.state, spirit);
                let inv =
                    self.op(|_| true, |s, r| s.invoke(r, &seal))
                        .map_err(|h| match (h, why) {
                            (Halt::Ended(m), Some(w)) => Halt::Ended(format!("{m}: {w}")),
                            (h, _) => h,
                        })?;
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
        let mut given = Vec::new();
        for v in vals {
            let V::Text(t) = v else {
                return Err(Halt::Denial("PETE takes only text".into()));
            };
            given.push(t);
        }
        let mut texts = given.into_iter();
        let steering = |t: Option<Text>| -> Go<Option<String>> {
            match t {
                Some(t) if t.mark == Mark::Quarantined => Err(Halt::Denial(
                    "a quarantined word cannot choose an office's mode or place".into(),
                )),
                t => Ok(t.map(|t| t.s)),
            }
        };
        let mode = if sig.modes.is_empty() {
            None
        } else {
            steering(texts.next())?
        };
        let quaestio = if sig.quaestio {
            texts.next().map(|t| t.s)
        } else {
            None
        };
        let places: Vec<String> = texts
            .map(|t| steering(Some(t)).map(|p| p.unwrap_or_default()))
            .collect::<Go<_>>()?;
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
