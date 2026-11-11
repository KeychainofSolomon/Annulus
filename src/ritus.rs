use std::collections::{BTreeMap, BTreeSet};

use crate::lingua::check::Checked;
use crate::lingua::registra::{Register, Spirit};
use crate::nucleus::state::{
    digest_constitution, digest_faculty, fingerprint_validator, sign_for_test, Roots,
};
use crate::nucleus::*;

pub const EXAMINATOR: &str = "constitutio:angelicus";
pub const ITA_NON: &str = "ita_non";
pub const RITE: &str = "goetia";

pub fn instrument(word: &str) -> Instrument {
    match word {
        "ANULUM" => Instrument::Anulum,
        "VIRGAM" => Instrument::Virgam,
        "HEXAGRAMMUM" => Instrument::Hexagrammum,
        _ => Instrument::Pentagrammum,
    }
}

pub struct Goetia {
    pub register: &'static Register,
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

pub fn test_wand() -> Wand {
    Wand {
        fingerprint: "virga-probationis".to_string(),
        present: true,
        master_unlocked: false,
        test: true,
    }
}

pub fn no_wand() -> Wand {
    Wand {
        fingerprint: String::new(),
        present: false,
        master_unlocked: false,
        test: false,
    }
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

pub fn launch(
    checked: &Checked,
    wand: Wand,
    stores: Stores,
    world: BTreeMap<Place, (String, Mark)>,
) -> Launch {
    let constitution = constitution();
    let (faculty, reserved, ink) = faculty_of(checked);
    let owner = test_wand().fingerprint;
    let auth = Auth {
        faculty_signature: sign_for_test(&wand.fingerprint, &digest_faculty(&faculty, &reserved)),
        manifest_signature: sign_for_test(&owner, &digest_constitution(&constitution)),
        owner,
        revoked: Vec::new(),
        roots: roots_of(&stores),
        wand,
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
        Goetia { register }
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
        let Some(spirit) = self.spirit(seal) else {
            return false;
        };
        let open: Vec<&'static Spirit> = state
            .invs
            .values()
            .filter(|i| i.status == InvStatus::Live)
            .filter_map(|i| self.spirit(&i.seal))
            .collect();
        if open.iter().any(|s| Goetia::bound_as_earl(s)) {
            return false;
        }
        let held = Goetia::held(state);
        if !spirit
            .instruments()
            .iter()
            .all(|i| held.contains(&instrument(i)))
        {
            return false;
        }
        spirit.ranks.iter().any(|rank| match rank.as_str() {
            "earl" => open.is_empty(),
            "duke" | "president" => false,
            _ => true,
        })
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
        match &p.content {
            Content::Empty => Verdict::Hold,
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
            RiteAct::Appella(inv) | RiteAct::Veta(inv) => spirit_of(inv).is_some(),
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
