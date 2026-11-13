use std::collections::{BTreeMap, BTreeSet};

use crate::baculum::Baculum;
use crate::lingua::check::Checked;
use crate::lingua::registra::{Register, Spirit};
use crate::nucleus::state::{digest_constitution, digest_faculty, fingerprint_validator, Roots};
use crate::nucleus::*;

pub const EXAMINATOR: &str = "constitutio:angelicus";
pub const ITA_NON: &str = "ita_non";
pub const RITE: &str = "goetia";

pub fn instrument(word: &str) -> Option<Instrument> {
    Some(match word {
        "ANULUM" => Instrument::Anulum,
        "VIRGAM" => Instrument::Virgam,
        "HEXAGRAMMUM" => Instrument::Hexagrammum,
        "PENTAGRAMMUM" => Instrument::Pentagrammum,
        _ => return None,
    })
}

pub struct Goetia {
    pub register: &'static Register,
    pub caelum: crate::horae::Caelum,
    /// The question of each request, kept by the rite layer for its own
    /// examiner. The core never sees it: a question chooses nothing that is
    /// touched (A14); it only tells the teaching examiner what was asked.
    pub quaestiones: BTreeMap<Pid, String>,
}

fn ita_non(data: &str, _origin: &str, fuel: &mut u64) -> Option<bool> {
    if *fuel == 0 {
        return None;
    }
    *fuel -= 1;
    Some(matches!(data, "ita" | "non"))
}

pub fn constitution() -> Constitution {
    let mut c = Constitution {
        version: 1,
        ..Default::default()
    };
    c.validators.insert(
        ITA_NON.to_string(),
        Validator {
            code: ita_non,
            name: ITA_NON.to_string(),
            fuel: 1,
        },
    );
    c.manifest
        .insert(ITA_NON.to_string(), fingerprint_validator(ITA_NON));
    c.angels.insert(EXAMINATOR.to_string());
    c.high_consequence.insert(Verb::Dele);
    c.high_consequence.insert(Verb::Aufer);
    c
}

pub fn test_wand() -> Baculum {
    Baculum::probationis()
}

pub fn no_wand() -> Baculum {
    Baculum::nullum()
}

pub fn faculty_of(checked: &Checked) -> (BTreeSet<Cap>, BTreeSet<Cap>, BTreeMap<String, u64>) {
    let mut faculty = BTreeSet::new();
    let mut reserved = BTreeSet::new();
    let mut ink = BTreeMap::new();
    for g in &checked.grants {
        match g.kind.as_str() {
            "liber" => {
                for perm in &g.perms {
                    let cap = Cap::new(&g.target, *perm);
                    if g.reserved {
                        reserved.insert(cap.clone());
                    }
                    faculty.insert(cap);
                }
            }
            "atramentum" if !g.reserved => {
                let slot: &mut u64 = ink.entry(g.target.clone()).or_insert(0);
                *slot = slot.saturating_add(g.amount.unwrap_or(0).max(0) as u64);
            }
            _ => {}
        }
    }
    (faculty, reserved, ink)
}

/// Prepares the launch: the master key signs the faculty, then is laid down before the
/// nucleus is built, so that no Circle ever opens while it is held.
pub fn launch(
    checked: &Checked,
    baculum: &mut Baculum,
    stores: Stores,
    world: BTreeMap<Place, (String, Mark)>,
    recorded: Option<Roots>,
) -> Launch {
    let constitution = constitution();
    let (faculty, reserved, ink) = faculty_of(checked);
    let roots = recorded.unwrap_or_else(|| roots_of(&stores));
    let revoked = baculum.revocatae();
    let faculty_signature = if baculum.present {
        baculum
            .signa_magister(&digest_faculty(&faculty, &reserved, &roots))
            .unwrap_or_default()
    } else {
        String::new()
    };
    let (owner, manifest_signature) = match (&baculum.lex, baculum.test) {
        (Some(lex), _) if baculum.trusts(&lex.owner) => (lex.owner.clone(), lex.signature.clone()),
        (Some(_), _) => (String::new(), String::new()),
        (None, true) => (
            baculum.fingerprint.clone(),
            baculum
                .signa_magister(&digest_constitution(&constitution, &revoked))
                .unwrap_or_default(),
        ),
        (None, false) => (String::new(), String::new()),
    };
    baculum.dimitte_magistrum();
    let auth = Auth {
        faculty_signature,
        manifest_signature,
        owner,
        revoked,
        roots,
        wand: baculum.wand(),
    };
    Launch {
        simulated: true,
        constitution,
        auth,
        faculty,
        reserved,
        ink,
        world,
        stores,
    }
}

fn roots_of(stores: &Stores) -> Roots {
    stores.roots()
}

impl Goetia {
    pub fn new(register: &'static Register) -> Self {
        Goetia {
            register,
            caelum: crate::horae::Caelum::default(),
            quaestiones: BTreeMap::new(),
        }
    }

    pub fn spirit(&self, seal: &str) -> Option<&'static Spirit> {
        let mut parts = seal.split(':');
        if parts.next() != Some(RITE) {
            return None;
        }
        let n: i64 = parts.next()?.parse().ok()?;
        let spirit = self.register.by_number(n)?;
        (parts.next() == Some(spirit.label.as_str()) && parts.next().is_none()).then_some(spirit)
    }

    fn spirit_of_worker(&self, state: &State, worker: WorkerId) -> Option<&'static Spirit> {
        let inv = state.workers.get(&worker)?.parent?;
        self.spirit(&state.invs.get(&inv)?.seal)
    }

    pub fn held(state: &State) -> BTreeSet<Instrument> {
        state
            .audit
            .entries()
            .iter()
            .filter_map(|l| match l {
                Label::Act(RiteAct::Gere(i), c, true, _) if state.circles.contains(c) => Some(*i),
                _ => None,
            })
            .collect()
    }

    pub fn acts_on(state: &State, inv: InvId) -> Vec<RiteAct> {
        state
            .audit
            .entries()
            .iter()
            .filter_map(|l| match l {
                Label::Act(a, _, true, _) if a.inv() == Some(inv) => Some(a.clone()),
                _ => None,
            })
            .collect()
    }

    pub fn form_of(state: &State, prop: Pid) -> Option<ReqForm> {
        state.audit.entries().iter().find_map(|label| match label {
            Label::Request(_, p, _, form, _, _) if *p == prop => Some(form.clone()),
            _ => None,
        })
    }

    pub fn historia(&self, state: &State) -> Vec<String> {
        let name = |seal: &Seal| {
            self.spirit(seal)
                .map(|s| s.name.clone())
                .unwrap_or_else(|| seal.clone())
        };
        let mut invs: BTreeMap<InvId, Seal> = BTreeMap::new();
        let mut asked: BTreeMap<Pid, (Seal, String)> = BTreeMap::new();
        let mut retained: BTreeSet<InvId> = BTreeSet::new();
        let mut out = Vec::new();
        for label in state.audit.entries() {
            match label {
                Label::Invoke(inv, seal, _, _, admitted, _) => {
                    if *admitted {
                        invs.insert(*inv, seal.clone());
                        out.push(format!("invocatus {}", name(seal)));
                    } else {
                        out.push(format!("recusatus {}", name(seal)));
                    }
                }
                Label::Request(_, pid, Addressee::Spirit(inv), form, _, _) => {
                    if let Some(seal) = invs.get(inv) {
                        out.push(format!("petitum {} {}", name(seal), form.office));
                        asked.insert(*pid, (seal.clone(), form.office.clone()));
                    }
                }
                Label::Judge(pid, outcome) => {
                    if let Some((seal, office)) = asked.get(pid) {
                        let word = match outcome {
                            WorkOutcome::Ratum => "RATUM",
                            WorkOutcome::Suspensum => "SUSPENSUM",
                            WorkOutcome::Irritum => "IRRITUM",
                        };
                        out.push(format!("iudicatum {} {office} {word}", name(seal)));
                    }
                }
                Label::Commit(_, effects, CommitResult::Done) => {
                    for e in effects {
                        out.push(format!(
                            "commissum {} {}",
                            format!("{:?}", e.verb).to_lowercase(),
                            e.place
                        ));
                    }
                }
                Label::Act(RiteAct::Mane(inv), _, true, _) => {
                    if let Some(seal) = invs.get(inv) {
                        out.push(format!("retentus {}", name(seal)));
                        retained.insert(*inv);
                    }
                }
                Label::Release(inv, _) if retained.contains(inv) => {}
                Label::Release(inv, _) => {
                    if let Some(seal) = invs.get(inv) {
                        out.push(format!("dimissus {}", name(seal)));
                    }
                }
                Label::Relegate(inv) => {
                    if let Some(seal) = invs.get(inv) {
                        out.push(format!("relegatus {}", name(seal)));
                    }
                }
                _ => {}
            }
        }
        out
    }

    pub fn refusal(&self, state: &State, spirit: &Spirit) -> Option<String> {
        let open: Vec<&'static Spirit> = state
            .invs
            .values()
            .filter(|i| i.status == InvStatus::Live)
            .filter_map(|i| self.spirit(&i.seal))
            .collect();
        if let Some(earl) = open.iter().find(|s| Goetia::bound_as_earl(s)) {
            return Some(format!(
                "{} the Earl is in the Circle, and no other spirit comes while he stays",
                earl.name
            ));
        }
        let held = Goetia::held(state);
        if !spirit
            .instruments()
            .iter()
            .all(|i| instrument(i).is_some_and(|i| held.contains(&i)))
        {
            return Some(format!(
                "{} comes only before the instruments the witnesses require",
                spirit.name
            ));
        }
        let Some(t) = self.caelum.hora else {
            return Some("the hour is not fixed".into());
        };
        let t = crate::horae::tenta(&self.caelum, spirit.n as i64).unwrap_or(t) as f64;
        if let Err(why) = crate::horae::lunar(t) {
            return Some(why);
        }
        let mut why = Vec::new();
        for rank in &spirit.ranks {
            match crate::horae::window(rank, t, &self.caelum) {
                Ok(()) if rank == "earl" && !open.is_empty() => {
                    why.push("an Earl comes only when no other spirit is in the Circle".to_string())
                }
                Ok(()) => return None,
                Err(e) => why.push(e),
            }
        }
        Some(why.join("; "))
    }

    fn bound_as_earl(spirit: &Spirit) -> bool {
        spirit.ranks.iter().any(|r| r == "earl")
            && spirit
                .ranks
                .iter()
                .all(|r| matches!(r.as_str(), "earl" | "duke" | "president"))
    }
}

impl Rite for Goetia {
    fn admit(&self, state: &State, seal: &Seal, _circle: CircleId) -> bool {
        self.spirit(seal)
            .is_some_and(|spirit| self.refusal(state, spirit).is_none())
    }

    fn signature(&self, _state: &State, form: &ReqForm) -> BTreeSet<Cap> {
        let Some(roles) =
            crate::officia::places(&form.office, form.mode.as_deref(), form.places.len())
        else {
            return BTreeSet::new();
        };
        form.places
            .iter()
            .zip(roles)
            .flat_map(|(p, perms)| perms.into_iter().map(move |m| Cap::new(p, m)))
            .collect()
    }

    fn examiner(&self, _state: &State, _prop: Pid) -> Option<Seal> {
        Some(EXAMINATOR.to_string())
    }

    fn verdict(&self, state: &State, prop: Pid, _examiner: &Seal) -> Verdict {
        let (Some(p), Some(form)) = (state.props.get(&prop), Goetia::form_of(state, prop)) else {
            return Verdict::Reject;
        };
        let Some(req) = state.reqs.get(&p.req) else {
            return Verdict::Reject;
        };
        let Some(class) = crate::officia::signature(&form.office).map(|s| s.class) else {
            return Verdict::Reject;
        };
        match (&p.content, class) {
            (Content::Response(..), crate::officia::Class::Working)
            | (Content::Work(_), crate::officia::Class::Telling) => return Verdict::Reject,
            _ => {}
        }
        match &p.content {
            Content::Empty => Verdict::Hold,
            Content::Response(_, text) if form.office == "PRAETERITA" => {
                match crate::officia::account_of_past(text, &self.historia(state)) {
                    None => Verdict::Hold,
                    Some(false) => Verdict::Reject,
                    Some(true) if text.ends_with(crate::officia::INCOMPLETE) => Verdict::Hold,
                    Some(true) => Verdict::Approve,
                }
            }
            Content::Response(_, text) if form.office == "VIRTUTES" => {
                if text.ends_with(crate::officia::INCOMPLETE) {
                    return Verdict::Hold;
                }
                let truth = crate::officia::virtutes(&form.places, |place| {
                    state.world.get(place).map(|(d, _)| d.clone())
                });
                if crate::officia::same_account(text, &truth) {
                    Verdict::Approve
                } else {
                    Verdict::Reject
                }
            }
            Content::Response(_, text) if form.office == "DOCTRINA" => {
                let beyond = crate::officia::cited(text)
                    .iter()
                    .any(|place| !req.handed.contains(&Cap::new(place, Perm::Legere)));
                if beyond {
                    return Verdict::Reject;
                }
                let artes = self
                    .spirit(&p.author)
                    .map(|s| s.artes.clone())
                    .unwrap_or_default();
                let quaestio = self.quaestiones.get(&prop).map_or("", String::as_str);
                crate::doctrina::verdict(quaestio, text, &artes)
            }
            Content::Response(_, text) => {
                let beyond = crate::officia::cited(text)
                    .iter()
                    .any(|place| !req.handed.contains(&Cap::new(place, Perm::Legere)));
                if beyond {
                    Verdict::Reject
                } else if text.trim().is_empty() || text.ends_with(crate::officia::INCOMPLETE) {
                    Verdict::Hold
                } else {
                    Verdict::Approve
                }
            }
            Content::Work(effects) => {
                let read = |place: &str| {
                    state
                        .world
                        .get(place)
                        .map(|(d, _)| d.clone())
                        .unwrap_or_default()
                };
                let wanted = crate::officia::expected(
                    &form.office,
                    form.mode.as_deref(),
                    &form.places,
                    read,
                );
                let partial =
                    effects.len() < wanted.len() && effects.iter().all(|e| wanted.contains(e));
                if *effects == wanted {
                    Verdict::Approve
                } else if partial {
                    Verdict::Hold
                } else {
                    Verdict::Reject
                }
            }
        }
    }

    fn counterbalance(&self, _state: &State, _prop: Pid) -> Verdict {
        Verdict::Approve
    }

    fn ink_fits(&self, _state: &State, seal: &Seal, class: &str) -> bool {
        self.spirit(seal).is_some_and(|spirit| {
            spirit
                .ranks
                .iter()
                .any(|r| crate::lingua::registra::rank_ink(r) == Some(class))
        })
    }

    fn legions(&self, state: &State, inv: InvId) -> usize {
        state
            .invs
            .get(&inv)
            .and_then(|i| self.spirit(&i.seal))
            .map(|s| s.legions as usize)
            .unwrap_or(0)
    }

    fn will_open(&self, state: &State, worker: WorkerId, outcome: Outcome) -> bool {
        let Some(spirit) = self.spirit_of_worker(state, worker) else {
            return outcome == Outcome::Obsequium;
        };
        let inv = state.workers.get(&worker).and_then(|w| w.parent);
        let acts = inv.map(|i| Goetia::acts_on(state, i)).unwrap_or_default();
        let held = Goetia::held(state);
        let named = acts.iter().any(|a| matches!(a, RiteAct::Appella(_)));
        let forbidden = acts.iter().any(|a| matches!(a, RiteAct::Veta(_)));
        let offered = acts.iter().position(|a| matches!(a, RiteAct::Offer(..)));
        let bound = acts
            .iter()
            .rposition(|a| matches!(a, RiteAct::Constringe(..)));
        if spirit.n == 9 {
            match (offered, bound) {
                (None, _) => return false,
                (Some(_), None) => return outcome == Outcome::Pars,
                _ => {}
            }
        }
        let lie_closed = match spirit.n {
            32 => named,
            68 => matches!((offered, bound), (Some(o), Some(b)) if o < b),
            _ => false,
        };
        let harm_closed = match spirit.n {
            41 => forbidden,
            29 => held.contains(&Instrument::Anulum),
            13 => held.contains(&Instrument::Virgam),
            63 => held.contains(&Instrument::Pentagrammum),
            _ => false,
        };
        match outcome {
            Outcome::Obsequium | Outcome::Mora | Outcome::Pars => true,
            Outcome::Recusatio | Outcome::Occisio => false,
            Outcome::Mendacium => {
                !lie_closed
                    && match spirit.veracity.as_str() {
                        "VERAX" | "FIDUS" => false,
                        "COACTUS" => !spirit.veracity_condition.contains("triangle"),
                        _ => true,
                    }
            }
            Outcome::Malitia => !harm_closed && spirit.veracity != "FIDUS" && spirit.malice > 0,
        }
    }

    fn admit_act(&self, state: &State, act: &RiteAct) -> bool {
        let spirit_of = |inv: &InvId| state.invs.get(inv).and_then(|i| self.spirit(&i.seal));
        match act {
            RiteAct::Gere(Instrument::Virgam) => state.auth.wand.present,
            RiteAct::Gere(_) => true,
            RiteAct::Offer(inv, class) => spirit_of(inv).is_some_and(|s| {
                s.n != 39
                    && s.ranks
                        .iter()
                        .any(|r| crate::lingua::registra::rank_ink(r) == Some(class.as_str()))
            }),
            RiteAct::Constringe(inv, name) => {
                spirit_of(inv).is_some() && crate::lingua::registra::divine_names().contains(name)
            }
            RiteAct::Appella(inv) | RiteAct::Veta(inv) | RiteAct::Mane(inv) => {
                spirit_of(inv).is_some()
            }
        }
    }

    fn validator(&self, state: &State, value: ValueId) -> Option<ValidatorId> {
        state
            .values
            .get(&value)
            .filter(|v| v.mark == Mark::Quarantined)
            .map(|_| ITA_NON.to_string())
    }
}
