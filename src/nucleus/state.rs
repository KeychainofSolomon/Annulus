use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;

use super::sha256::hash_hex;
use super::types::{
    Addressee, Cap, CircleId, Constitution, Ending, Inst, InvId, InvStatus, IxEntry, KeyEntry,
    Label, Mark, Perm, Phase, Pid, Place, Proposal, Reach, ReqId, Request, Seal, Sealed, Standing,
    Value, ValueId, Wand, Worker, WorkerId, OPERATOR,
};

pub const GENESIS: &str = "annulus";

pub const SEALING: &str = "The sealing: Solomon shuts the 72 spirits with their legions in a vessel of brass, seals it with his secret seal, and casts it into a deep lake or hole in Babilon (Sloane 3825 ff. 110v-111r, f113r-v).";

pub const BREAKING: &str = "The breaking, afterwards: the Babylonians break the vessel open, suspecting to find a great treasure; the spirits return to their former places, save Belial, who enters into a certain image and gives answers (Sloane 3825 ff. 110v-111r; Weyer, Pseudomonarchia daemonum 23).";

#[derive(Clone, Debug)]
pub struct Chain<T> {
    entries: Vec<T>,
    hashes: Vec<String>,
}

impl<T: Debug> Chain<T> {
    pub fn new() -> Self {
        Chain {
            entries: Vec::new(),
            hashes: Vec::new(),
        }
    }

    pub fn push(&mut self, entry: T) {
        let link = format!("{}|{:?}", self.root(), entry);
        self.hashes.push(hash_hex(link.as_bytes()));
        self.entries.push(entry);
    }

    pub fn root(&self) -> String {
        self.hashes
            .last()
            .cloned()
            .unwrap_or_else(|| hash_hex(GENESIS.as_bytes()))
    }

    pub fn entries(&self) -> &[T] {
        &self.entries
    }

    pub fn len(&self) -> usize {
        self.entries.len()
    }

    pub fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    pub fn intact(&self) -> bool {
        if self.entries.len() != self.hashes.len() {
            return false;
        }
        let mut prev = hash_hex(GENESIS.as_bytes());
        for (entry, hash) in self.entries.iter().zip(&self.hashes) {
            let expected = hash_hex(format!("{prev}|{entry:?}").as_bytes());
            if &expected != hash {
                return false;
            }
            prev = expected;
        }
        true
    }

    #[cfg(feature = "probatio")]
    pub fn tamper_for_test(&mut self, index: usize, entry: T) {
        self.entries[index] = entry;
    }
}

impl<T: Debug> Default for Chain<T> {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Debug)]
pub struct Stores {
    pub index: Chain<IxEntry>,
    pub vessel: Chain<Sealed>,
    pub keys: Chain<KeyEntry>,
}

#[derive(Clone, Debug)]
pub struct Roots {
    pub index: String,
    pub vessel: String,
    pub keys: String,
}

impl Stores {
    pub fn install(wand: &Wand) -> Stores {
        let mut vessel = Chain::new();
        vessel.push(Sealed::History(SEALING.to_string()));
        vessel.push(Sealed::History(BREAKING.to_string()));
        vessel.push(Sealed::History(format!(
            "The making of this vessel, after the pattern of Solomon's, closed with the Operator's seal: wand {}{}",
            wand.fingerprint,
            if wand.test { " (test wand)" } else { "" }
        )));
        Stores {
            index: Chain::new(),
            vessel,
            keys: Chain::new(),
        }
    }

    pub fn roots(&self) -> Roots {
        Roots {
            index: self.index.root(),
            vessel: self.vessel.root(),
            keys: self.keys.root(),
        }
    }

    pub fn confirm_petition(&mut self, fingerprint: &str, wand: &Wand) -> bool {
        let sealed = self
            .keys
            .entries()
            .iter()
            .any(|e| *e == KeyEntry::Petition(fingerprint.to_string()));
        if !wand.present || !sealed {
            return false;
        }
        self.keys.push(KeyEntry::Confirmed(fingerprint.to_string()));
        true
    }
}

#[derive(Clone, Debug)]
pub struct Auth {
    pub wand: Wand,
    pub revoked: Vec<String>,
    pub faculty_signature: String,
    pub owner: String,
    pub manifest_signature: String,
    pub roots: Roots,
}

pub fn sign_for_test(key_fingerprint: &str, digest: &str) -> String {
    hash_hex(format!("test-signature|{key_fingerprint}|{digest}").as_bytes())
}

pub fn verify(key_fingerprint: &str, digest: &str, signature: &str) -> bool {
    sign_for_test(key_fingerprint, digest) == signature
}

pub fn digest_faculty(faculty: &BTreeSet<Cap>, reserved: &BTreeSet<Cap>, roots: &Roots) -> String {
    hash_hex(
        format!(
            "faculty|{faculty:?}|{reserved:?}|{}|{}|{}",
            roots.index, roots.vessel, roots.keys
        )
        .as_bytes(),
    )
}

pub fn digest_constitution(c: &super::types::Constitution, revoked: &[String]) -> String {
    hash_hex(
        format!(
            "constitution|{}|{:?}|{:?}|{:?}|{revoked:?}",
            c.version, c.manifest, c.angels, c.high_consequence
        )
        .as_bytes(),
    )
}

pub fn fingerprint_validator(name: &str) -> String {
    hash_hex(format!("validator|{name}").as_bytes())
}

pub fn fingerprint_effects(effects: &[super::types::Effect]) -> String {
    hash_hex(format!("{effects:?}").as_bytes())
}

#[derive(Clone, Debug)]
pub struct State {
    pub(crate) phase: Phase,
    pub(crate) simulated: bool,
    pub(crate) constitution: Constitution,
    pub(crate) auth: Auth,
    pub(crate) faculty: BTreeSet<Cap>,
    pub(crate) reserved: BTreeSet<Cap>,
    pub(crate) ink_left: BTreeMap<String, u64>,
    pub(crate) inscribed: BTreeSet<(Seal, CircleId)>,
    pub(crate) circles: Vec<CircleId>,
    pub(crate) workers: BTreeMap<WorkerId, Worker>,
    pub(crate) invs: BTreeMap<InvId, Inst>,
    pub(crate) reqs: BTreeMap<ReqId, Request>,
    pub(crate) props: BTreeMap<Pid, Proposal>,
    pub(crate) world: BTreeMap<Place, (String, Mark)>,
    pub(crate) values: BTreeMap<ValueId, Value>,
    pub(crate) audit: Chain<Label>,
    pub(crate) stores: Stores,
    pub(crate) current: Option<WorkerId>,
    pub(crate) order: Vec<WorkerId>,
    pub(crate) entropy: u64,
    pub(crate) next_id: u64,
}

impl State {
    pub fn phase(&self) -> &Phase {
        &self.phase
    }

    pub fn simulated(&self) -> bool {
        self.simulated
    }

    pub fn constitution(&self) -> &Constitution {
        &self.constitution
    }

    pub fn auth(&self) -> &Auth {
        &self.auth
    }

    pub fn faculty(&self) -> &BTreeSet<Cap> {
        &self.faculty
    }

    pub fn reserved(&self) -> &BTreeSet<Cap> {
        &self.reserved
    }

    pub fn ink_left(&self) -> &BTreeMap<String, u64> {
        &self.ink_left
    }

    pub fn inscribed(&self) -> &BTreeSet<(Seal, CircleId)> {
        &self.inscribed
    }

    pub fn circles(&self) -> &Vec<CircleId> {
        &self.circles
    }

    pub fn workers(&self) -> &BTreeMap<WorkerId, Worker> {
        &self.workers
    }

    pub fn invs(&self) -> &BTreeMap<InvId, Inst> {
        &self.invs
    }

    pub fn reqs(&self) -> &BTreeMap<ReqId, Request> {
        &self.reqs
    }

    pub fn props(&self) -> &BTreeMap<Pid, Proposal> {
        &self.props
    }

    pub fn world(&self) -> &BTreeMap<Place, (String, Mark)> {
        &self.world
    }

    pub fn values(&self) -> &BTreeMap<ValueId, Value> {
        &self.values
    }

    pub fn audit(&self) -> &Chain<Label> {
        &self.audit
    }

    pub fn stores(&self) -> &Stores {
        &self.stores
    }

    pub fn current(&self) -> Option<WorkerId> {
        self.current
    }

    pub fn order(&self) -> &Vec<WorkerId> {
        &self.order
    }

    pub fn entropy(&self) -> u64 {
        self.entropy
    }

    pub fn next_id(&self) -> u64 {
        self.next_id
    }
}

#[cfg(feature = "probatio")]
impl State {
    pub fn auth_mut(&mut self) -> &mut Auth {
        &mut self.auth
    }

    pub fn faculty_mut(&mut self) -> &mut BTreeSet<Cap> {
        &mut self.faculty
    }

    pub fn world_mut(&mut self) -> &mut BTreeMap<Place, (String, Mark)> {
        &mut self.world
    }

    pub fn stores_mut(&mut self) -> &mut Stores {
        &mut self.stores
    }
}

pub struct Launch {
    pub simulated: bool,
    pub constitution: Constitution,
    pub auth: Auth,
    pub faculty: BTreeSet<Cap>,
    pub reserved: BTreeSet<Cap>,
    pub ink: BTreeMap<String, u64>,
    pub world: BTreeMap<Place, (String, Mark)>,
    pub stores: Stores,
}

impl State {
    pub fn initial(launch: Launch) -> State {
        let mut workers = BTreeMap::new();
        workers.insert(
            OPERATOR,
            Worker {
                owner: None,
                parent: None,
                handed: BTreeSet::new(),
                queue: Default::default(),
                alive: true,
                helper: false,
            },
        );
        State {
            phase: Phase::PreLaunch,
            simulated: launch.simulated,
            constitution: launch.constitution,
            auth: launch.auth,
            faculty: launch.faculty,
            reserved: launch.reserved,
            ink_left: launch.ink,
            inscribed: BTreeSet::new(),
            circles: Vec::new(),
            workers,
            invs: BTreeMap::new(),
            reqs: BTreeMap::new(),
            props: BTreeMap::new(),
            world: launch.world,
            values: BTreeMap::new(),
            audit: Chain::new(),
            stores: launch.stores,
            current: None,
            order: Vec::new(),
            entropy: 0,
            next_id: 1,
        }
    }

    pub fn fresh(&mut self) -> u64 {
        let id = self.next_id;
        self.next_id += 1;
        id
    }

    pub fn running(&self) -> bool {
        self.phase == Phase::Running
    }

    pub fn record(&mut self, label: Label) -> usize {
        self.audit.push(label);
        self.audit.len() - 1
    }

    pub fn angelic(&self, seal: &Seal) -> bool {
        self.constitution.angels.contains(seal)
    }

    pub fn standing(&self, seal: &Seal) -> Standing {
        if self.angelic(seal) {
            return Standing::Unknown;
        }
        standing(self.stores.index.entries(), seal)
    }

    pub fn barred(&self, seal: &Seal) -> bool {
        matches!(
            self.standing(seal),
            Standing::Proscribed | Standing::Damnatus
        )
    }

    pub fn doubtful(&self, seal: &Seal) -> bool {
        !self.angelic(seal) && self.standing(seal) != Standing::Unknown
    }

    pub fn marks(&self) -> BTreeSet<Reach> {
        self.stores
            .index
            .entries()
            .iter()
            .filter_map(|e| match e {
                IxEntry::Proscribe(s, reach) if self.barred(s) => Some(reach.clone()),
                _ => None,
            })
            .flatten()
            .collect()
    }

    pub fn overreach(&self, prop: Pid) -> Vec<Reach> {
        let Some(p) = self.props.get(&prop) else {
            return Vec::new();
        };
        let handed = self
            .reqs
            .get(&p.req)
            .map(|r| r.handed.clone())
            .unwrap_or_default();
        p.content
            .effects()
            .iter()
            .filter(|e| !handed.contains(&e.needs()))
            .map(|e| Reach {
                verb: e.verb,
                place: e.place.clone(),
            })
            .collect()
    }

    pub fn newly_suspect(&self, seal: &Seal, prop: Pid) -> bool {
        if self.angelic(seal) || self.standing(seal) != Standing::Unknown {
            return false;
        }
        let marks = self.marks();
        self.overreach(prop).iter().any(|r| marks.contains(r))
    }

    pub fn live(&self, inv: InvId) -> bool {
        self.invs
            .get(&inv)
            .is_some_and(|i| i.status == InvStatus::Live)
    }

    pub fn invited(&self, seal: &Seal, circle: CircleId) -> bool {
        self.invs
            .values()
            .any(|i| &i.seal == seal && i.circle == circle && i.status == InvStatus::Live)
    }

    pub fn readable(&self, place: &str) -> bool {
        let cap = Cap::new(place, Perm::Legere);
        self.faculty.contains(&cap) && !self.reserved.contains(&cap)
    }

    pub fn contained(&self, to: &Addressee, cap: &Cap) -> bool {
        match to {
            Addressee::Spirit(_) => self.faculty.contains(cap) && !self.reserved.contains(cap),
            Addressee::Angel(_) => self.faculty.contains(cap),
        }
    }

    pub fn inv_of_prop(&self, prop: Pid) -> Option<InvId> {
        let p = self.props.get(&prop)?;
        match &self.reqs.get(&p.req)?.to {
            Addressee::Spirit(i) => Some(*i),
            Addressee::Angel(_) => None,
        }
    }

    pub fn terminate<F: Fn(InvId, &Inst) -> bool>(&mut self, pick: F, status: InvStatus) {
        let chosen: BTreeSet<InvId> = self
            .invs
            .iter()
            .filter(|(id, inst)| inst.status == InvStatus::Live && pick(**id, inst))
            .map(|(id, _)| *id)
            .collect();
        for id in &chosen {
            if let Some(inst) = self.invs.get_mut(id) {
                inst.status = status;
            }
        }
        for worker in self.workers.values_mut() {
            if worker.parent.is_some_and(|p| chosen.contains(&p)) {
                worker.alive = false;
            }
        }
        let abandoned: Vec<Pid> = self
            .props
            .iter()
            .filter(|(id, p)| {
                p.status.lingers() && self.inv_of_prop(**id).is_some_and(|i| chosen.contains(&i))
            })
            .map(|(id, _)| *id)
            .collect();
        self.abandon(&abandoned);
    }

    pub fn abandon(&mut self, props: &[Pid]) {
        for id in props {
            if let Some(p) = self.props.get_mut(id) {
                p.status = super::types::PStatus::Abandoned;
                let content = p.content.clone();
                self.stores.vessel.push(Sealed::Content(*id, content));
            }
        }
    }

    pub fn end_run(&mut self, ending: Ending) {
        self.terminate(|_, _| true, InvStatus::Released);
        let angels: Vec<Pid> = self
            .props
            .iter()
            .filter(|(_, p)| {
                p.status.lingers()
                    && matches!(
                        self.reqs.get(&p.req).map(|r| &r.to),
                        Some(Addressee::Angel(_))
                    )
            })
            .map(|(id, _)| *id)
            .collect();
        self.abandon(&angels);
        self.phase = Phase::Ended(ending);
        self.current = None;
    }
}

fn standing_step(seal: &Seal, st: Standing, entry: &IxEntry) -> Standing {
    match entry {
        IxEntry::Damn(s) if s == seal => Standing::Damnatus,
        IxEntry::Proscribe(s, _) if s == seal && st != Standing::Damnatus => Standing::Proscribed,
        IxEntry::Lift(s) if s == seal && st == Standing::Proscribed => Standing::Unknown,
        IxEntry::Suspect(s) if s == seal && st == Standing::Unknown => Standing::Suspected,
        IxEntry::Clear(s) if s == seal && st == Standing::Suspected => Standing::Unknown,
        _ => st,
    }
}

pub fn standing(index: &[IxEntry], seal: &Seal) -> Standing {
    index
        .iter()
        .fold(Standing::Unknown, |st, e| standing_step(seal, st, e))
}
