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
        faculty_signature: sign_for_test("test-wand", &digest_faculty(&faculty, &reserved)),
        owner: "owner-wand".to_string(),
        manifest_signature: sign_for_test("owner-wand", &digest_constitution(&constitution)),
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
                let worker = s.invs[&inv].main;
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
                    s.world.get(PLACE).map(|w| w.0.clone()) != exists.then(|| "old".to_string());
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
                s.reqs[&s.props[&p].req].worker
            } else {
                let inv = spirit(&mut s, &rite);
                op(&mut s, |s| {
                    s.request(&rite, Addressee::Spirit(inv), &form())
                })
                .unwrap();
                s.invs[&inv].main
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
        let worker = s.invs[&inv].main;
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
