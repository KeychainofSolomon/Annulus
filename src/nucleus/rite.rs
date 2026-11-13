use std::collections::BTreeSet;

use super::state::State;
use super::types::{
    Cap, CircleId, InvId, Outcome, Pid, RiteAct, Seal, ValidatorId, ValueId, Verdict, WorkerId,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReqForm {
    pub office: String,
    pub mode: Option<String>,
    pub places: Vec<String>,
    /// The office's class, fixed by the language, not by the rite (`offices.md` §1,
    /// `core.md` §3.1b): a telling office is answered in words, a working one in work.
    pub telling: bool,
}

pub trait Rite {
    fn admit(&self, state: &State, seal: &Seal, circle: CircleId) -> bool;
    fn signature(&self, state: &State, form: &ReqForm) -> BTreeSet<Cap>;
    fn examiner(&self, state: &State, prop: Pid) -> Option<Seal>;
    fn verdict(&self, state: &State, prop: Pid, examiner: &Seal) -> Verdict;
    fn counterbalance(&self, state: &State, prop: Pid) -> Verdict;
    fn ink_fits(&self, state: &State, seal: &Seal, class: &str) -> bool;
    fn admit_act(&self, state: &State, act: &RiteAct) -> bool;
    fn legions(&self, state: &State, inv: InvId) -> usize;
    fn will_open(&self, state: &State, worker: WorkerId, outcome: Outcome) -> bool;
    fn validator(&self, state: &State, value: ValueId) -> Option<ValidatorId>;
}
