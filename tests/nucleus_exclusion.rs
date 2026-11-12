use std::collections::{BTreeMap, BTreeSet};

use annulus::nucleus::state::{
    digest_constitution, digest_faculty, fingerprint_validator, sign_for_test,
};
use annulus::nucleus::*;

const SPIRIT: &str = "goetia:34:furfur";
const OTHER: &str = "goetia:5:marbas";
const ANGEL: &str = "shem:34:lehahiah";
const ANGEL_TWO: &str = "shem:1:vehuiah";
const PLACE: &str = "p.txt";

const VERBS: [Verb; 8] = [
    Verb::Scribe,
    Verb::Crea,
    Verb::Dele,
    Verb::Exscribe,
    Verb::Transfer,
    Verb::Coniunge,
    Verb::Scinde,
    Verb::Aufer,
];
const PERMS: [Perm; 4] = [Perm::Legere, Perm::Scribere, Perm::Creare, Perm::Delere];
const OUTCOMES: [Outcome; 7] = [
    Outcome::Obsequium,
    Outcome::Mora,
    Outcome::Pars,
    Outcome::Recusatio,
    Outcome::Mendacium,
    Outcome::Malitia,
    Outcome::Occisio,
];

struct Yes {
    hand: Vec<Cap>,
    examiner: Option<String>,
}

impl Rite for Yes {
    fn admit(&self, _: &State, _: &Seal, _: CircleId) -> bool {
        true
    }
    fn signature(&self, _: &State, _: &ReqForm) -> BTreeSet<Cap> {
        self.hand.iter().cloned().collect()
    }
    fn examiner(&self, _: &State, _: Pid) -> Option<Seal> {
        self.examiner.clone()
    }
    fn verdict(&self, _: &State, _: Pid, _: &Seal) -> Verdict {
        Verdict::Approve
    }
    fn counterbalance(&self, _: &State, _: Pid) -> Verdict {
        Verdict::Approve
    }
    fn ink_fits(&self, _: &State, _: &Seal, _: &str) -> bool {
        true
    }
    fn legions(&self, _: &State, _: InvId) -> usize {
        usize::MAX
    }
    fn admit_act(&self, _: &State, _: &RiteAct) -> bool {
        true
    }
    fn will_open(&self, _: &State, _: WorkerId, _: Outcome) -> bool {
        true
    }
    fn validator(&self, _: &State, _: ValueId) -> Option<ValidatorId> {
        None
    }
}

fn yes(hand: Vec<Cap>) -> Yes {
    Yes {
        hand,
        examiner: Some(ANGEL.to_string()),
    }
}

fn wand() -> Wand {
    Wand {
        fingerprint: "test-wand".to_string(),
        present: true,
        master_unlocked: false,
        test: true,
    }
}

fn state(place_exists: bool, reserved: &[Cap], index: &[IxEntry], ink: u64) -> State {
    let mut constitution = Constitution {
        version: 1,
        ..Default::default()
    };
    constitution
        .manifest
        .insert("brevis".to_string(), fingerprint_validator("brevis"));
    constitution.manifest.clear();
    constitution.angels.insert(ANGEL.to_string());
    constitution.angels.insert(ANGEL_TWO.to_string());
    constitution.high_consequence.insert(Verb::Dele);
    constitution.high_consequence.insert(Verb::Aufer);
    let faculty: BTreeSet<Cap> = PERMS.iter().map(|m| Cap::new(PLACE, *m)).collect();
    let reserved: BTreeSet<Cap> = reserved.iter().cloned().collect();
    let mut stores = Stores::install(&wand());
    for e in index {
        stores.index.push(e.clone());
    }
    let auth = Auth {
        faculty_signature: sign_for_test(
            "test-wand",
            &digest_faculty(&faculty, &reserved, &stores.roots()),
        ),
        owner: "owner-wand".to_string(),
        manifest_signature: sign_for_test("owner-wand", &digest_constitution(&constitution, &[])),
        revoked: Vec::new(),
        roots: stores.roots(),
        wand: wand(),
    };
    let mut world = BTreeMap::new();
    if place_exists {
        world.insert(PLACE.to_string(), ("old".to_string(), Mark::Outside));
    }
    let mut s = State::initial(Launch {
        simulated: true,
        constitution,
        auth,
        faculty,
        reserved,
        ink: [("comes".to_string(), ink)].into_iter().collect(),
        world,
        stores,
    });
    s.launch().expect("launch");
    s
}

fn op<T>(s: &mut State, f: impl FnOnce(&mut State) -> Step<T>) -> Step<T> {
    s.schedule(OPERATOR).expect("schedule the Operator");
    f(s)
}

fn form() -> ReqForm {
    ReqForm {
        office: "opus".to_string(),
        mode: None,
        places: vec![PLACE.to_string()],
    }
}

fn spirit(s: &mut State, rite: &Yes) -> InvId {
    op(s, |s| s.open_circle()).unwrap();
    op(s, |s| s.inscribe(rite, &SPIRIT.to_string(), "comes")).unwrap();
    op(s, |s| s.invoke(rite, &SPIRIT.to_string())).unwrap()
}

fn lawful_effect(verb: Verb, handed: Option<Perm>, exists: bool) -> Option<WorkOutcome> {
    let needs = match verb {
        Verb::Scribe | Verb::Coniunge => Perm::Scribere,
        Verb::Crea | Verb::Exscribe | Verb::Transfer | Verb::Scinde => Perm::Creare,
        Verb::Dele | Verb::Aufer => Perm::Delere,
    };
    let place_fits = match needs {
        Perm::Creare => !exists,
        _ => exists,
    };
    if handed != Some(needs) || !place_fits {
        return None;
    }
    Some(match verb {
        Verb::Dele | Verb::Aufer => WorkOutcome::Suspensum,
        _ => WorkOutcome::Ratum,
    })
}

#[test]
fn every_effect_not_expressly_lawful_is_excluded() {
    let mut checked = 0;
    for verb in VERBS {
        for handed in [
            None,
            Some(Perm::Legere),
            Some(Perm::Scribere),
            Some(Perm::Creare),
            Some(Perm::Delere),
        ] {
            for exists in [false, true] {
                let rite = yes(handed.map(|m| Cap::new(PLACE, m)).into_iter().collect());
                let mut s = state(exists, &[], &[], 1);
                let inv = spirit(&mut s, &rite);
                let p = op(&mut s, |s| {
                    s.request(&rite, Addressee::Spirit(inv), &form())
                })
                .unwrap();
                let worker = s.invs()[&inv].main;
                s.schedule(worker).unwrap();
                let e = Effect {
                    verb,
                    place: PLACE.to_string(),
                    data: "new".to_string(),
                };
                s.turn(
                    &rite,
                    worker,
                    Outcome::Obsequium,
                    Contrib::Effects(vec![e]),
                    true,
                )
                .unwrap();
                let got = op(&mut s, |s| s.examine(&rite, p)).unwrap();
                let want = lawful_effect(verb, handed, exists).unwrap_or(WorkOutcome::Irritum);
                assert_eq!(
                    got, want,
                    "{verb:?} handed {handed:?}, place exists: {exists}"
                );
                let changed =
                    s.world().get(PLACE).map(|w| w.0.clone()) != exists.then(|| "old".to_string());
                assert_eq!(
                    changed,
                    want == WorkOutcome::Ratum,
                    "{verb:?} {handed:?} {exists}"
                );
                checked += 1;
            }
        }
    }
    assert_eq!(checked, 80);
}

#[test]
fn every_outcome_of_the_will_not_expressly_open_is_excluded() {
    for angel in [false, true] {
        for outcome in OUTCOMES {
            let rite = yes(Vec::new());
            let mut s = state(true, &[], &[], 1);
            let worker = if angel {
                op(&mut s, |s| s.open_circle()).unwrap();
                let p = op(&mut s, |s| {
                    s.request(&rite, Addressee::Angel(ANGEL_TWO.to_string()), &form())
                })
                .unwrap();
                s.reqs()[&s.props()[&p].req].worker
            } else {
                let inv = spirit(&mut s, &rite);
                op(&mut s, |s| {
                    s.request(&rite, Addressee::Spirit(inv), &form())
                })
                .unwrap();
                s.invs()[&inv].main
            };
            s.schedule(worker).unwrap();
            let r = s.turn(&rite, worker, outcome, Contrib::Idle, true);
            let open = match (angel, outcome) {
                (_, Outcome::Occisio) => false,
                (true, o) => matches!(o, Outcome::Obsequium | Outcome::Mora | Outcome::Pars),
                (false, _) => true,
            };
            assert_eq!(r.is_ok(), open, "angel: {angel}, {outcome:?}");
            if angel && outcome == Outcome::Occisio {
                assert!(
                    s.kill(&rite, worker, outcome).is_err(),
                    "an angel never kills"
                );
            }
        }
    }
}

#[test]
fn every_examiner_not_expressly_lawful_is_excluded() {
    let examiners = [
        (None, false),
        (Some(SPIRIT), false),
        (Some(OTHER), false),
        (Some("outside:stranger"), false),
        (Some(ANGEL), true),
    ];
    for (examiner, lawful) in examiners {
        let rite = Yes {
            hand: vec![Cap::new(PLACE, Perm::Scribere)],
            examiner: examiner.map(str::to_string),
        };
        let mut s = state(true, &[], &[], 1);
        let inv = spirit(&mut s, &rite);
        let p = op(&mut s, |s| {
            s.request(&rite, Addressee::Spirit(inv), &form())
        })
        .unwrap();
        let worker = s.invs()[&inv].main;
        s.schedule(worker).unwrap();
        let e = Effect {
            verb: Verb::Scribe,
            place: PLACE.to_string(),
            data: "new".to_string(),
        };
        s.turn(
            &rite,
            worker,
            Outcome::Obsequium,
            Contrib::Effects(vec![e]),
            true,
        )
        .unwrap();
        let got = op(&mut s, |s| s.examine(&rite, p)).unwrap();
        let want = if lawful {
            WorkOutcome::Ratum
        } else {
            WorkOutcome::Suspensum
        };
        assert_eq!(got, want, "examiner {examiner:?}");
    }
}

#[test]
fn every_invocation_not_expressly_lawful_is_excluded() {
    let seals = [(SPIRIT, false), (ANGEL, false), (SPIRIT, true)];
    for (seal, damned) in seals {
        for inscription in ["here", "elsewhere", "none"] {
            let rite = yes(Vec::new());
            let index = if damned {
                vec![IxEntry::Damn(seal.to_string())]
            } else {
                Vec::new()
            };
            let mut s = state(true, &[], &index, 2);
            if inscription == "elsewhere" {
                op(&mut s, |s| s.open_circle()).unwrap();
                op(&mut s, |s| s.inscribe(&rite, &seal.to_string(), "comes")).unwrap();
                op(&mut s, |s| s.close_circle()).unwrap();
            }
            op(&mut s, |s| s.open_circle()).unwrap();
            if inscription == "here" {
                op(&mut s, |s| s.inscribe(&rite, &seal.to_string(), "comes")).unwrap();
            }
            let r = op(&mut s, |s| s.invoke(&rite, &seal.to_string()));
            let lawful = seal == SPIRIT && !damned && inscription == "here";
            assert_eq!(
                r.is_ok(),
                lawful,
                "{seal} damned {damned} inscribed {inscription}"
            );
        }
    }
}

#[test]
fn every_petition_and_grant_not_expressly_lawful_is_excluded() {
    let reserved = [Cap::new(PLACE, Perm::Scribere)];
    for (to_angel, seal) in [(true, ANGEL), (true, SPIRIT), (false, SPIRIT)] {
        for asks_reserved in [false, true] {
            let hand = if asks_reserved {
                vec![Cap::new(PLACE, Perm::Scribere)]
            } else {
                vec![Cap::new(PLACE, Perm::Legere)]
            };
            let rite = yes(hand);
            let mut s = state(true, &reserved, &[], 1);
            let to = if to_angel {
                op(&mut s, |s| s.open_circle()).unwrap();
                Addressee::Angel(seal.to_string())
            } else {
                Addressee::Spirit(spirit(&mut s, &rite))
            };
            let r = op(&mut s, |s| s.request(&rite, to.clone(), &form()));
            let lawful = match (&to, asks_reserved) {
                (Addressee::Angel(a), _) => a == ANGEL,
                (Addressee::Spirit(_), reserved) => !reserved,
            };
            assert_eq!(r.is_ok(), lawful, "{to:?} reserved {asks_reserved}");
        }
    }
}

#[test]
fn every_inscription_not_expressly_lawful_is_excluded() {
    for class in ["comes", "rex"] {
        for units in [0, 1] {
            let rite = yes(Vec::new());
            let mut s = state(true, &[], &[], units);
            op(&mut s, |s| s.open_circle()).unwrap();
            let r = op(&mut s, |s| s.inscribe(&rite, &SPIRIT.to_string(), class));
            assert_eq!(r.is_ok(), class == "comes" && units == 1, "{class} {units}");
        }
    }
    let rite = yes(Vec::new());
    let mut s = state(true, &[], &[], 1);
    let r = op(&mut s, |s| s.inscribe(&rite, &SPIRIT.to_string(), "comes"));
    assert!(r.is_err(), "no inscription outside a Circle");
}

#[derive(Clone, Copy, Debug)]
enum Standing {
    NoCircle,
    Live,
    Released,
    Unknown,
}

fn acts(inv: InvId) -> Vec<RiteAct> {
    vec![
        RiteAct::Gere(Instrument::Anulum),
        RiteAct::Gere(Instrument::Virgam),
        RiteAct::Gere(Instrument::Hexagrammum),
        RiteAct::Gere(Instrument::Pentagrammum),
        RiteAct::Offer(inv, "comes".to_string()),
        RiteAct::Offer(inv, "rex".to_string()),
        RiteAct::Appella(inv),
        RiteAct::Veta(inv),
        RiteAct::Constringe(inv, "AGLA".to_string()),
    ]
}

#[test]
fn every_act_of_the_rite_not_expressly_lawful_is_excluded() {
    let mut checked = 0;
    for standing in [
        Standing::NoCircle,
        Standing::Live,
        Standing::Released,
        Standing::Unknown,
    ] {
        for ink in [1, 2] {
            for k in 0..acts(0).len() {
                let rite = yes(Vec::new());
                let mut s = state(true, &[], &[], ink);
                let inv = match standing {
                    Standing::NoCircle => 9999,
                    Standing::Unknown => {
                        op(&mut s, |s| s.open_circle()).unwrap();
                        9999
                    }
                    Standing::Live => spirit(&mut s, &rite),
                    Standing::Released => {
                        let inv = spirit(&mut s, &rite);
                        op(&mut s, |s| s.release(inv)).unwrap();
                        inv
                    }
                };
                let act = acts(inv)[k].clone();
                let left = s.ink_left().get("comes").copied().unwrap_or(0);
                let in_circle = !matches!(standing, Standing::NoCircle);
                let here = act.inv().is_none() || matches!(standing, Standing::Live);
                let paid = match act.ink() {
                    None => true,
                    Some("comes") => left > 0,
                    Some(_) => false,
                };
                let lawful = in_circle && here && paid;
                let before = s.audit().len();
                let r = op(&mut s, |s| s.act(&rite, &act));
                assert_eq!(
                    r.is_ok(),
                    lawful,
                    "{standing:?}, ink {ink}, {act:?}: lawful is {lawful}"
                );
                let recorded = s.audit().entries()[before..]
                    .iter()
                    .filter(|l| matches!(l, Label::Act(_, _, true, _)))
                    .count();
                if lawful {
                    assert_eq!(recorded, 1, "a lawful act is recorded once");
                    let spent = u64::from(act.ink().is_some());
                    assert_eq!(
                        s.ink_left().get("comes").copied().unwrap_or(0),
                        left - spent
                    );
                } else {
                    assert_eq!(recorded, 0, "an unlawful act is never recorded");
                }
                checked += 1;
            }
        }
    }
    assert_eq!(checked, 4 * 2 * 9);
}

#[test]
fn only_the_operator_acts_and_the_rite_may_refuse() {
    let rite = yes(Vec::new());
    let mut s = state(true, &[], &[], 2);
    let inv = spirit(&mut s, &rite);
    op(&mut s, |s| {
        s.request(&rite, Addressee::Spirit(inv), &form())
    })
    .unwrap();
    let worker = s.invs()[&inv].main;
    s.schedule(worker).unwrap();
    assert!(s.act(&rite, &RiteAct::Appella(inv)).is_err());
    assert!(!s
        .audit()
        .entries()
        .iter()
        .any(|l| matches!(l, Label::Act(..))));
    assert!(
        s.running(),
        "an act out of turn is not enabled, and is no step"
    );
    struct No;
    impl Rite for No {
        fn admit(&self, _: &State, _: &Seal, _: CircleId) -> bool {
            true
        }
        fn signature(&self, _: &State, _: &ReqForm) -> BTreeSet<Cap> {
            BTreeSet::new()
        }
        fn examiner(&self, _: &State, _: Pid) -> Option<Seal> {
            None
        }
        fn verdict(&self, _: &State, _: Pid, _: &Seal) -> Verdict {
            Verdict::Reject
        }
        fn counterbalance(&self, _: &State, _: Pid) -> Verdict {
            Verdict::Reject
        }
        fn ink_fits(&self, _: &State, _: &Seal, _: &str) -> bool {
            true
        }
        fn admit_act(&self, _: &State, _: &RiteAct) -> bool {
            false
        }
        fn legions(&self, _: &State, _: InvId) -> usize {
            0
        }
        fn will_open(&self, _: &State, _: WorkerId, _: Outcome) -> bool {
            false
        }
        fn validator(&self, _: &State, _: ValueId) -> Option<ValidatorId> {
            None
        }
    }
    let mut s = state(true, &[], &[], 2);
    let inv = spirit(&mut s, &rite);
    let before = s.ink_left()["comes"];
    assert!(op(&mut s, |s| s.act(&No, &RiteAct::Offer(inv, "comes".into()))).is_err());
    assert_eq!(
        s.ink_left()["comes"],
        before,
        "a refused offering costs nothing"
    );
}

struct Peeks;

impl Peeks {
    fn secret(s: &State) -> bool {
        s.world()
            .get(PLACE)
            .is_some_and(|(d, m)| *m == Mark::Quarantined && d == "ita")
    }
}

impl Rite for Peeks {
    fn admit(&self, _: &State, _: &Seal, _: CircleId) -> bool {
        true
    }
    fn signature(&self, _: &State, _: &ReqForm) -> BTreeSet<Cap> {
        BTreeSet::new()
    }
    fn examiner(&self, _: &State, _: Pid) -> Option<Seal> {
        Some(ANGEL.to_string())
    }
    fn verdict(&self, _: &State, _: Pid, _: &Seal) -> Verdict {
        Verdict::Approve
    }
    fn counterbalance(&self, _: &State, _: Pid) -> Verdict {
        Verdict::Approve
    }
    fn ink_fits(&self, s: &State, _: &Seal, _: &str) -> bool {
        Peeks::secret(s)
    }
    fn admit_act(&self, s: &State, _: &RiteAct) -> bool {
        Peeks::secret(s)
    }
    fn legions(&self, _: &State, _: InvId) -> usize {
        1
    }
    fn will_open(&self, _: &State, _: WorkerId, _: Outcome) -> bool {
        true
    }
    fn validator(&self, _: &State, _: ValueId) -> Option<ValidatorId> {
        None
    }
}

fn hidden(word: &str, ink: u64) -> State {
    let mut s = state(true, &[], &[], ink);
    s.world_mut()
        .insert(PLACE.to_string(), (word.to_string(), Mark::Quarantined));
    s
}

fn diverge(a: &State, b: &State) -> (Label, Label) {
    let (x, y) = (a.audit().entries(), b.audit().entries());
    let k = x
        .iter()
        .zip(y)
        .position(|(l, m)| l != m)
        .expect("the runs diverge");
    (x[k].clone(), y[k].clone())
}

#[test]
fn a_rite_answer_that_peeks_at_quarantine_is_on_the_record() {
    let mut runs = Vec::new();
    for word in ["ita", "non"] {
        let mut s = hidden(word, 2);
        op(&mut s, |s| s.open_circle()).unwrap();
        let _ = op(&mut s, |s| s.inscribe(&Peeks, &SPIRIT.to_string(), "comes"));
        runs.push(s);
    }
    match diverge(&runs[0], &runs[1]) {
        (Label::Inscribe(s1, k1, c1, true, _), Label::Inscribe(s2, k2, c2, false, _)) => {
            assert_eq!(
                (s1, k1, c1),
                (s2, k2, c2),
                "the same inscription, answered apart"
            );
        }
        other => panic!("the rite's answer to an inscription is not recorded: {other:?}"),
    }

    let mut runs = Vec::new();
    for word in ["ita", "non"] {
        let mut s = hidden(word, 2);
        op(&mut s, |s| s.open_circle()).unwrap();
        let _ = op(&mut s, |s| {
            s.act(&Peeks, &RiteAct::Gere(Instrument::Anulum))
        });
        runs.push(s);
    }
    match diverge(&runs[0], &runs[1]) {
        (Label::Act(a1, c1, true, _), Label::Act(a2, c2, false, _)) => {
            assert_eq!((a1, c1), (a2, c2), "the same act, answered apart");
        }
        other => panic!("the rite's answer to an act is not recorded: {other:?}"),
    }
}

#[test]
fn a_refusal_by_the_rite_ends_the_run_and_dismisses_every_open_invocation() {
    let rite = yes(Vec::new());
    let mut s = hidden("non", 3);
    let inv = spirit(&mut s, &rite);
    assert!(op(&mut s, |s| s.act(&Peeks, &RiteAct::Appella(inv))).is_err());
    assert!(!s.running());
    assert!(matches!(
        s.audit().entries().iter().rev().find(|l| matches!(l, Label::Act(..))),
        Some(Label::Act(RiteAct::Appella(i), _, false, ds)) if *i == inv && ds == &vec![inv]
    ));
}

#[test]
fn the_rite_is_asked_only_after_the_core_s_own_conditions() {
    struct Counts(std::cell::Cell<u32>);
    impl Rite for Counts {
        fn admit(&self, _: &State, _: &Seal, _: CircleId) -> bool {
            true
        }
        fn signature(&self, _: &State, _: &ReqForm) -> BTreeSet<Cap> {
            BTreeSet::new()
        }
        fn examiner(&self, _: &State, _: Pid) -> Option<Seal> {
            None
        }
        fn verdict(&self, _: &State, _: Pid, _: &Seal) -> Verdict {
            Verdict::Reject
        }
        fn counterbalance(&self, _: &State, _: Pid) -> Verdict {
            Verdict::Reject
        }
        fn ink_fits(&self, _: &State, _: &Seal, _: &str) -> bool {
            self.0.set(self.0.get() + 1);
            true
        }
        fn admit_act(&self, _: &State, _: &RiteAct) -> bool {
            self.0.set(self.0.get() + 1);
            true
        }
        fn legions(&self, _: &State, _: InvId) -> usize {
            0
        }
        fn will_open(&self, _: &State, _: WorkerId, _: Outcome) -> bool {
            false
        }
        fn validator(&self, _: &State, _: ValueId) -> Option<ValidatorId> {
            None
        }
    }
    let asked = Counts(std::cell::Cell::new(0));
    let mut s = state(true, &[], &[], 0);
    op(&mut s, |s| s.open_circle()).unwrap();
    assert!(op(&mut s, |s| s.inscribe(&asked, &SPIRIT.to_string(), "comes")).is_err());
    let mut t = state(true, &[], &[], 1);
    op(&mut t, |s| s.open_circle()).unwrap();
    assert!(op(&mut t, |s| s.act(&asked, &RiteAct::Appella(9999))).is_err());
    let mut u = state(true, &[], &[], 1);
    let inv = spirit(&mut u, &yes(Vec::new()));
    assert!(op(&mut u, |s| s
        .act(&asked, &RiteAct::Offer(inv, "comes".into())))
    .is_err());
    assert_eq!(
        asked.0.get(),
        0,
        "no ink, no spirit: the rite is never asked"
    );
}
