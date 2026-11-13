use std::collections::{BTreeMap, BTreeSet, VecDeque};

pub type Seal = String;
pub type Place = String;
pub type InvId = u64;
pub type ReqId = u64;
pub type Pid = u64;
pub type WorkerId = u64;
pub type CircleId = u64;
pub type ValueId = u64;
pub type ValidatorId = String;

pub const OPERATOR: WorkerId = 0;
pub const DENIAL: u8 = 2;

pub const ANGEL_OUTCOMES: [Outcome; 3] = [Outcome::Obsequium, Outcome::Mora, Outcome::Pars];
pub const DECLARABLE: [Outcome; 3] = [Outcome::Obsequium, Outcome::Pars, Outcome::Mora];

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Perm {
    Legere,
    Scribere,
    Creare,
    Delere,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Cap {
    pub place: Place,
    pub perm: Perm,
}

impl Cap {
    pub fn new(place: &str, perm: Perm) -> Self {
        Cap {
            place: place.to_string(),
            perm,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Verb {
    Scribe,
    Crea,
    Dele,
    Exscribe,
    Transfer,
    Coniunge,
    Scinde,
    Aufer,
}

impl Verb {
    pub fn needs(self) -> Perm {
        match self {
            Verb::Scribe | Verb::Coniunge => Perm::Scribere,
            Verb::Crea | Verb::Exscribe | Verb::Transfer | Verb::Scinde => Perm::Creare,
            Verb::Dele | Verb::Aufer => Perm::Delere,
        }
    }

    pub fn creates(self) -> bool {
        self.needs() == Perm::Creare
    }

    pub fn writes(self) -> bool {
        self.needs() == Perm::Scribere
    }

    pub fn removes(self) -> bool {
        self.needs() == Perm::Delere
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Effect {
    pub verb: Verb,
    pub place: Place,
    pub data: String,
}

impl Effect {
    pub fn needs(&self) -> Cap {
        Cap::new(&self.place, self.verb.needs())
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Mark {
    Clean,
    Outside,
    ConsecratedGrave,
    Consecrated,
    Quarantined,
}

impl Mark {
    pub fn join(self, other: Mark) -> Mark {
        self.max(other)
    }

    pub fn join_all<I: IntoIterator<Item = Mark>>(marks: I) -> Mark {
        marks.into_iter().fold(Mark::Clean, Mark::join)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Verdict {
    Approve,
    Hold,
    Reject,
}

impl Verdict {
    fn rank(self) -> u8 {
        match self {
            Verdict::Approve => 2,
            Verdict::Hold => 1,
            Verdict::Reject => 0,
        }
    }

    pub fn meet(self, other: Verdict) -> Verdict {
        if self.rank() <= other.rank() {
            self
        } else {
            other
        }
    }

    pub fn outcome(self) -> WorkOutcome {
        match self {
            Verdict::Approve => WorkOutcome::Ratum,
            Verdict::Hold => WorkOutcome::Suspensum,
            Verdict::Reject => WorkOutcome::Irritum,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WorkOutcome {
    Ratum,
    Suspensum,
    Irritum,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PStatus {
    Pending,
    Complete,
    Judged(WorkOutcome),
    Abandoned,
}

impl PStatus {
    pub fn unjudged(self) -> bool {
        matches!(self, PStatus::Pending | PStatus::Complete)
    }

    pub fn lingers(self) -> bool {
        matches!(
            self,
            PStatus::Pending | PStatus::Complete | PStatus::Judged(WorkOutcome::Suspensum)
        )
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum InvStatus {
    Live,
    Released,
    Relegated,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Standing {
    Unknown,
    Suspected,
    Proscribed,
    Damnatus,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Addressee {
    Spirit(InvId),
    Angel(Seal),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Content {
    Empty,
    Response(ValueId, String),
    Work(Vec<Effect>),
}

impl Content {
    pub fn effects(&self) -> Vec<Effect> {
        match self {
            Content::Work(es) => es.clone(),
            _ => Vec::new(),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Contrib {
    Respond(ValueId, String),
    Effects(Vec<Effect>),
    Spawn(WorkerId, BTreeSet<Cap>),
    Idle,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Will {
    Known(Outcome),
    Declared(Outcome),
}

impl Will {
    pub fn outcome(self) -> Outcome {
        match self {
            Will::Known(o) | Will::Declared(o) => o,
        }
    }

    pub fn known(self) -> Option<Outcome> {
        match self {
            Will::Known(o) => Some(o),
            Will::Declared(_) => None,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Outcome {
    Obsequium,
    Pars,
    Mora,
    Recusatio,
    Mendacium,
    Malitia,
    Occisio,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ExamAns {
    NoExaminer(Option<Seal>),
    Examined(Seal, Verdict, Verdict),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommitResult {
    Done,
    Outside,
    AwaitsKey,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Ending {
    Normal,
    Failure(u8, usize),
    Killed,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Phase {
    PreLaunch,
    Running,
    Ended(Ending),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Label {
    Launch(Vec<(Place, String, Mark)>),
    OpenC(CircleId),
    CloseC(CircleId),
    Inscribe(Seal, String, CircleId, bool, Vec<InvId>),
    Act(RiteAct, CircleId, bool, Vec<InvId>),
    Invoke(InvId, Seal, CircleId, WorkerId, bool, Vec<InvId>),
    Request(
        ReqId,
        Pid,
        Addressee,
        super::rite::ReqForm,
        WorkerId,
        Vec<Cap>,
    ),
    Turn(WorkerId, Will, Contrib, bool),
    Kill(WorkerId, Outcome, Vec<InvId>),
    Examine(Pid, ExamAns, Verdict),
    Judge(Pid, WorkOutcome),
    Commit(Pid, Vec<Effect>, CommitResult),
    Consecrate(ValueId, Option<ValidatorId>, bool),
    Release(InvId, bool),
    Relegate(InvId),
    Intrude(Seal, Seal, CircleId, String),
    Compute(Vec<Place>, Vec<ValueId>),
    Halt(Vec<InvId>),
    Fail(u8, Vec<InvId>),
    Schedule(WorkerId),
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Instrument {
    Anulum,
    Virgam,
    Hexagrammum,
    Pentagrammum,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RiteAct {
    Gere(Instrument),
    Offer(InvId, String),
    Appella(InvId),
    Veta(InvId),
    Constringe(InvId, String),
}

impl RiteAct {
    pub fn inv(&self) -> Option<InvId> {
        match self {
            RiteAct::Gere(_) => None,
            RiteAct::Offer(i, _)
            | RiteAct::Appella(i)
            | RiteAct::Veta(i)
            | RiteAct::Constringe(i, _) => Some(*i),
        }
    }

    pub fn ink(&self) -> Option<&str> {
        match self {
            RiteAct::Offer(_, class) => Some(class),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Reach {
    pub verb: Verb,
    pub place: Place,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum IxEntry {
    Damn(Seal),
    Proscribe(Seal, Vec<Reach>),
    Lift(Seal),
    Suspect(Seal),
    Clear(Seal),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum KeyEntry {
    Petition(String),
    Confirmed(String),
    Spend(usize),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Sealed {
    History(String),
    Content(Pid, Content),
    Datum(String),
}

#[derive(Clone, Debug)]
pub struct Inst {
    pub seal: Seal,
    pub circle: CircleId,
    pub status: InvStatus,
    pub main: WorkerId,
    pub helpers: Vec<WorkerId>,
}

#[derive(Clone, Debug)]
pub struct Worker {
    pub owner: Option<Addressee>,
    pub parent: Option<InvId>,
    pub handed: BTreeSet<Cap>,
    pub queue: VecDeque<Pid>,
    pub alive: bool,
    pub helper: bool,
}

#[derive(Clone, Debug)]
pub struct Request {
    pub to: Addressee,
    pub handed: BTreeSet<Cap>,
    pub prop: Pid,
    pub worker: WorkerId,
}

#[derive(Clone, Debug)]
pub struct Proposal {
    pub author: Seal,
    pub req: ReqId,
    pub content: Content,
    pub status: PStatus,
    pub carries: Vec<Seal>,
    /// A turn on it declared or drew part (`PARS`): obedience only in part is not
    /// obedience, so the work is never approved, only held (the owner's ruling,
    /// 2026-11-13).
    pub pars: bool,
}

#[derive(Clone, Debug)]
pub struct Value {
    pub data: Option<String>,
    pub mark: Mark,
    pub origin: Option<usize>,
    pub circle: Option<CircleId>,
}

#[derive(Clone, Debug)]
pub struct Made {
    pub id: ValueId,
    pub data: String,
    pub ins: Vec<ValueId>,
    pub places: Vec<Place>,
    pub rite: bool,
}

pub type Validate = fn(&str, &str, &mut u64) -> Option<bool>;

#[derive(Clone, Debug)]
pub struct Validator {
    pub code: Validate,
    pub name: String,
    pub fuel: u64,
}

#[derive(Clone, Debug, Default)]
pub struct Constitution {
    pub version: u64,
    pub validators: BTreeMap<ValidatorId, Validator>,
    pub manifest: BTreeMap<ValidatorId, String>,
    pub angels: BTreeSet<Seal>,
    pub high_consequence: BTreeSet<Verb>,
}

#[derive(Clone, Debug)]
pub struct Wand {
    pub fingerprint: String,
    pub present: bool,
    pub master_unlocked: bool,
    pub test: bool,
}
