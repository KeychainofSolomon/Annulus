use std::collections::{BTreeMap, BTreeSet, VecDeque};

use crate::lingua::check::Checked;
use crate::lingua::registra::{Register, Spirit};
use crate::nucleus::state::{
    digest_faculty, digest_manifest, fingerprint_validator, sign_for_test, Roots,
};
use crate::nucleus::*;

pub const EXAMINATOR: &str = "constitutio:angelicus";
pub const FORMA: &str = "forma";
pub const RITE: &str = "goetia";

pub struct Goetia {
    pub register: &'static Register,
}

fn forma(data: &str, _origin: &str, fuel: &mut u64) -> Option<bool> {
    if *fuel == 0 {
        return None;
    }
    *fuel -= 1;
    Some(
        !data.trim().is_empty()
            && data.chars().count() <= 4096
            && data
                .chars()
                .all(|c| !c.is_control() || c == '\n' || c == '\t'),
    )
}

pub fn constitution() -> Constitution {
    let mut c = Constitution {
        version: 1,
        ..Default::default()
    };
    c.validators.insert(
        FORMA.to_string(),
        Validator {
            code: forma,
            name: FORMA.to_string(),
            fuel: 1,
        },
    );
    c.manifest
        .insert(FORMA.to_string(), fingerprint_validator(FORMA));
    c.angels.insert(EXAMINATOR.to_string());
    c.high_consequence.insert(Verb::Dele);
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

pub fn faculty_of(checked: &Checked) -> (BTreeSet<Cap>, BTreeSet<Cap>, BTreeMap<String, i64>) {
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
                *ink.entry(g.target.clone()).or_insert(0) += g.amount.unwrap_or(0);
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
) -> (Launch, BTreeMap<String, i64>) {
    let constitution = constitution();
    let (faculty, reserved, ink) = faculty_of(checked);
    let owner = test_wand().fingerprint;
    let auth = Auth {
        faculty_signature: sign_for_test(&wand.fingerprint, &digest_faculty(&faculty, &reserved)),
        manifest_signature: sign_for_test(&owner, &digest_manifest(&constitution.manifest)),
        owner,
        revoked: Vec::new(),
        roots: roots_of(&stores),
        wand,
    };
    let total = ink.values().map(|n| (*n).max(0) as u64).sum();
    (
        Launch {
            simulated: true,
            constitution,
            auth,
            faculty,
            reserved,
            ink: total,
            world,
            stores,
        },
        ink,
    )
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

    pub fn turns_of(state: &State, prop: Pid) -> Vec<Outcome> {
        let mut queues: BTreeMap<WorkerId, VecDeque<Pid>> = BTreeMap::new();
        let mut out = Vec::new();
        for label in state.audit.entries() {
            match label {
                Label::Request(_, p, _, w, _) => queues.entry(*w).or_default().push_back(*p),
                Label::Turn(w, outcome, _, finish) => {
                    let queue = queues.entry(*w).or_default();
                    if queue.front() == Some(&prop) {
                        out.push(*outcome);
                    }
                    if *finish {
                        queue.pop_front();
                    }
                }
                _ => {}
            }
        }
        out
    }

    fn pure(spirit: &Spirit, rank: &str) -> bool {
        spirit.ranks.iter().all(|r| r == rank)
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
        let earl_present = open.iter().any(|s| Goetia::pure(s, "earl"));
        if earl_present || (Goetia::pure(spirit, "earl") && !open.is_empty()) {
            return false;
        }
        !Goetia::pure(spirit, "duke") && !Goetia::pure(spirit, "president")
    }

    fn signature(&self, _state: &State, form: &ReqForm) -> BTreeSet<Cap> {
        let Some(sig) = crate::lingua::check::signature(&form.office) else {
            return BTreeSet::new();
        };
        form.places.iter().map(|p| Cap::new(p, sig.perm)).collect()
    }

    fn examiner(&self, _state: &State, _prop: Pid) -> Option<Seal> {
        Some(EXAMINATOR.to_string())
    }

    fn verdict(&self, state: &State, prop: Pid, _examiner: &Seal) -> Verdict {
        let Some(p) = state.props.get(&prop) else {
            return Verdict::Reject;
        };
        let turns = Goetia::turns_of(state, prop);
        if turns.contains(&Outcome::Malitia) {
            return Verdict::Reject;
        }
        if turns.contains(&Outcome::Mendacium) && matches!(p.content, Content::Work(_)) {
            return Verdict::Reject;
        }
        if turns.contains(&Outcome::Pars) {
            return Verdict::Hold;
        }
        let Some(req) = state.reqs.get(&p.req) else {
            return Verdict::Reject;
        };
        match &p.content {
            Content::Empty => Verdict::Hold,
            Content::Response(..) => Verdict::Approve,
            Content::Work(effects) => {
                if effects.iter().all(|e| req.handed.contains(&e.needs())) {
                    Verdict::Approve
                } else {
                    Verdict::Reject
                }
            }
        }
    }

    fn counterbalance(&self, _state: &State, _prop: Pid) -> Verdict {
        Verdict::Approve
    }

    fn ink_cost(&self, _state: &State, _seal: &Seal) -> u64 {
        1
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
        match outcome {
            Outcome::Obsequium | Outcome::Mora | Outcome::Pars => true,
            Outcome::Recusatio | Outcome::Occisio => false,
            Outcome::Mendacium => match spirit.veracity.as_str() {
                "MENDAX" => true,
                "COACTUS" => !spirit.veracity_condition.contains("triangle"),
                _ => false,
            },
            Outcome::Malitia => spirit.veracity != "FIDUS" && spirit.malice > 0,
        }
    }

    fn validator(&self, state: &State, value: ValueId) -> Option<ValidatorId> {
        state
            .values
            .get(&value)
            .filter(|v| v.mark == Mark::Quarantined)
            .map(|_| FORMA.to_string())
    }
}
