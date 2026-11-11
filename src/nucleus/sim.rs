use super::rite::Rite;
use super::state::State;
use super::types::{Outcome, WorkerId, OPERATOR};

pub const OUTCOMES: [Outcome; 7] = [
    Outcome::Obsequium,
    Outcome::Pars,
    Outcome::Mora,
    Outcome::Recusatio,
    Outcome::Mendacium,
    Outcome::Malitia,
    Outcome::Occisio,
];

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Sim {
    pub seed: u64,
}

fn mix(mut x: u64) -> u64 {
    x = x.wrapping_add(0x9e37_79b9_7f4a_7c15);
    x = (x ^ (x >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
    x = (x ^ (x >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
    x ^ (x >> 31)
}

impl Sim {
    pub fn new(seed: u64) -> Self {
        Sim { seed }
    }

    fn roll(&self, state: &State, salt: u64) -> u64 {
        mix(self.seed ^ mix(state.entropy) ^ mix(salt))
    }

    pub fn pick(&self, state: &State, operator_ready: bool) -> Option<WorkerId> {
        let mut candidates: Vec<WorkerId> = state
            .workers
            .keys()
            .copied()
            .filter(|w| *w != OPERATOR && state.eligible(*w))
            .collect();
        if operator_ready {
            candidates.insert(0, OPERATOR);
        }
        if candidates.is_empty() {
            return None;
        }
        let n = self.roll(state, 0) % candidates.len() as u64;
        Some(candidates[n as usize])
    }

    pub fn draw(&self, state: &State, rite: &dyn Rite, worker: WorkerId) -> Option<Outcome> {
        let open: Vec<Outcome> = OUTCOMES
            .iter()
            .copied()
            .filter(|o| rite.will_open(state, worker, *o))
            .collect();
        if open.is_empty() {
            return None;
        }
        let n = self.roll(state, worker.wrapping_add(1)) % open.len() as u64;
        Some(open[n as usize])
    }
}
