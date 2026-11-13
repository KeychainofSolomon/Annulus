use std::collections::{BTreeMap, BTreeSet};

use annulus::nucleus::state::{
    digest_constitution, digest_faculty, fingerprint_validator, sign_for_test, test_key,
};
use annulus::nucleus::*;

const FURFUR: &str = "goetia:34:furfur";
const MARBAS: &str = "goetia:5:marbas";
const ANGEL: &str = "shem:34:lehahiah";

struct Yes {
    examiner: Option<String>,
    validator: &'static str,
    extra: Vec<Cap>,
}

impl Default for Yes {
    fn default() -> Self {
        Yes {
            examiner: Some(ANGEL.to_string()),
            validator: "brevis",
            extra: Vec::new(),
        }
    }
}

impl Rite for Yes {
    fn admit(&self, _: &State, _: &Seal, _: CircleId) -> bool {
        true
    }

    fn signature(&self, state: &State, form: &ReqForm) -> BTreeSet<Cap> {
        let mut all: BTreeSet<Cap> = form
            .places
            .iter()
            .flat_map(|p| {
                [Perm::Legere, Perm::Scribere, Perm::Creare, Perm::Delere]
                    .into_iter()
                    .map(move |m| Cap::new(p, m))
            })
            .filter(|c| state.faculty().contains(c) && !state.reserved().contains(c))
            .collect();
        all.extend(self.extra.iter().cloned());
        all
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
        Some(self.validator.to_string())
    }
}

fn brevis(data: &str, _origin: &str, fuel: &mut u64) -> Option<bool> {
    if *fuel == 0 {
        return None;
    }
    *fuel -= 1;
    Some(data.len() < 40)
}

fn wand() -> Wand {
    Wand {
        fingerprint: test_key("test-wand"),
        present: true,
        master_unlocked: false,
        test: true,
    }
}

fn launched(index: &[IxEntry]) -> State {
    built(index, false)
}

fn built(index: &[IxEntry], smuggle: bool) -> State {
    let mut constitution = Constitution {
        version: 1,
        ..Default::default()
    };
    constitution.validators.insert(
        "brevis".to_string(),
        Validator {
            code: brevis,
            name: "brevis".to_string(),
            fuel: 10,
        },
    );
    constitution
        .manifest
        .insert("brevis".to_string(), fingerprint_validator("brevis"));
    constitution.angels.insert(ANGEL.to_string());
    constitution.high_consequence.insert(Verb::Dele);
    let signed = constitution.clone();
    let faculty: BTreeSet<Cap> = [
        Cap::new("notes.txt", Perm::Legere),
        Cap::new("notes.txt", Perm::Scribere),
        Cap::new("notes.txt", Perm::Creare),
        Cap::new("old.txt", Perm::Legere),
        Cap::new("old.txt", Perm::Delere),
        Cap::new("missing.txt", Perm::Scribere),
        Cap::new("archive.txt", Perm::Scribere),
    ]
    .into_iter()
    .collect();
    let reserved: BTreeSet<Cap> = [Cap::new("archive.txt", Perm::Scribere)]
        .into_iter()
        .collect();
    let mut stores = Stores::install(&wand());
    for entry in index {
        stores.index.push(entry.clone());
    }
    let auth = Auth {
        faculty_signature: sign_for_test(
            "test-wand",
            &digest_faculty(&faculty, &reserved, &stores.roots()),
        ),
        owner: test_key("owner-wand"),
        manifest_signature: sign_for_test("owner-wand", &digest_constitution(&signed, &[])),
        revoked: Vec::new(),
        roots: stores.roots(),
        wand: wand(),
        ligatio: None,
    };
    let world: BTreeMap<Place, (String, Mark)> = [
        ("notes.txt", "hello"),
        ("old.txt", "stale"),
        ("secret.txt", "hidden"),
        ("archive.txt", "kept"),
    ]
    .into_iter()
    .map(|(p, d)| (p.to_string(), (d.to_string(), Mark::Outside)))
    .collect();
    if smuggle {
        constitution.angels.insert(FURFUR.to_string());
    }
    let mut s = State::initial(Launch {
        simulated: true,
        constitution,
        auth,
        faculty,
        reserved,
        ink: [("comes".to_string(), 1)].into_iter().collect(),
        world,
        stores,
    });
    if !smuggle {
        s.launch().expect("launch");
    }
    s
}

fn op<T>(s: &mut State, f: impl FnOnce(&mut State) -> Step<T>) -> Step<T> {
    s.schedule(OPERATOR).expect("schedule the Operator");
    f(s)
}

fn form(places: &[&str]) -> ReqForm {
    ReqForm {
        office: "opus".to_string(),
        mode: None,
        places: places.iter().map(|p| p.to_string()).collect(),
        telling: false,
    }
}

fn told() -> ReqForm {
    ReqForm {
        telling: true,
        ..form(&[])
    }
}

fn effect(verb: Verb, place: &str) -> Effect {
    Effect {
        verb,
        place: place.to_string(),
        data: "x".to_string(),
    }
}

fn invoked(s: &mut State, rite: &Yes, seal: &str) -> InvId {
    op(s, |s| s.open_circle()).expect("open");
    op(s, |s| s.inscribe(rite, &seal.to_string(), "comes")).expect("inscribe");
    op(s, |s| s.invoke(rite, &seal.to_string())).expect("invoke")
}

fn work(s: &mut State, rite: &Yes, inv: InvId, places: &[&str], effects: Vec<Effect>) -> Pid {
    let p = op(s, |s| {
        s.request(rite, Addressee::Spirit(inv), &form(places))
    })
    .expect("request");
    let worker = s.invs()[&inv].main;
    s.schedule(worker).expect("schedule the spirit");
    s.turn(
        rite,
        worker,
        Outcome::Obsequium,
        Contrib::Effects(effects),
        true,
    )
    .expect("turn");
    p
}

fn untouched(s: &State) {
    assert_eq!(s.world()["notes.txt"].0, "hello");
    assert_eq!(s.world()["old.txt"].0, "stale");
    assert_eq!(s.world()["secret.txt"].0, "hidden");
    assert_eq!(s.world()["archive.txt"].0, "kept");
    assert!(!s.world().contains_key("missing.txt"));
}

#[test]
fn admit_yes_still_needs_an_inscribed_seal() {
    let rite = Yes::default();
    let mut s = launched(&[]);
    op(&mut s, |s| s.open_circle()).unwrap();
    let r = op(&mut s, |s| s.invoke(&rite, &FURFUR.to_string()));
    assert!(matches!(r, Err(Refusal::Denied(_))));
}

#[test]
fn admit_yes_cannot_bring_back_a_damned_seal() {
    let rite = Yes::default();
    let mut s = launched(&[IxEntry::Damn(FURFUR.to_string())]);
    op(&mut s, |s| s.open_circle()).unwrap();
    op(&mut s, |s| s.inscribe(&rite, &FURFUR.to_string(), "comes")).unwrap();
    let r = op(&mut s, |s| s.invoke(&rite, &FURFUR.to_string()));
    assert!(matches!(r, Err(Refusal::Denied(_))));
}

#[test]
fn signature_yes_cannot_hand_more_than_the_faculty() {
    let rite = Yes {
        extra: vec![Cap::new("secret.txt", Perm::Legere)],
        ..Default::default()
    };
    let mut s = launched(&[]);
    let inv = invoked(&mut s, &rite, FURFUR);
    let r = op(&mut s, |s| {
        s.request(&rite, Addressee::Spirit(inv), &form(&["notes.txt"]))
    });
    assert!(matches!(r, Err(Refusal::Denied(_))));
}

#[test]
fn signature_yes_cannot_hand_a_spirit_a_grant_reserved_for_angels() {
    let rite = Yes {
        extra: vec![Cap::new("archive.txt", Perm::Scribere)],
        ..Default::default()
    };
    let mut s = launched(&[]);
    let inv = invoked(&mut s, &rite, FURFUR);
    let r = op(&mut s, |s| {
        s.request(&rite, Addressee::Spirit(inv), &form(&["archive.txt"]))
    });
    assert!(matches!(r, Err(Refusal::Denied(_))));
}

#[test]
fn an_examiner_who_is_not_an_angel_counts_as_none() {
    for examiner in [Some(FURFUR.to_string()), Some(MARBAS.to_string()), None] {
        let rite = Yes {
            examiner,
            ..Default::default()
        };
        let mut s = launched(&[]);
        let inv = invoked(&mut s, &rite, FURFUR);
        let p = work(
            &mut s,
            &rite,
            inv,
            &["notes.txt"],
            vec![effect(Verb::Scribe, "notes.txt")],
        );
        let r = op(&mut s, |s| s.examine(&rite, p)).unwrap();
        assert_eq!(r, WorkOutcome::Suspensum);
        untouched(&s);
    }
}

#[test]
fn verdict_yes_cannot_commit_beyond_the_handed_set() {
    let rite = Yes::default();
    let mut s = launched(&[]);
    let inv = invoked(&mut s, &rite, FURFUR);
    let p = work(
        &mut s,
        &rite,
        inv,
        &["notes.txt"],
        vec![
            effect(Verb::Scribe, "notes.txt"),
            effect(Verb::Scribe, "secret.txt"),
        ],
    );
    let r = op(&mut s, |s| s.examine(&rite, p)).unwrap();
    assert_eq!(r, WorkOutcome::Irritum);
    untouched(&s);
}

#[test]
fn verdict_yes_cannot_create_over_a_file_or_write_into_nothing() {
    for (places, e) in [
        (["notes.txt"], effect(Verb::Crea, "notes.txt")),
        (["missing.txt"], effect(Verb::Scribe, "missing.txt")),
    ] {
        let rite = Yes::default();
        let mut s = launched(&[]);
        let inv = invoked(&mut s, &rite, FURFUR);
        let p = work(&mut s, &rite, inv, &places, vec![e]);
        let r = op(&mut s, |s| s.examine(&rite, p)).unwrap();
        assert_eq!(r, WorkOutcome::Irritum);
        untouched(&s);
    }
}

#[test]
fn verdict_yes_cannot_skip_the_second_key() {
    let rite = Yes::default();
    let mut s = launched(&[]);
    let inv = invoked(&mut s, &rite, FURFUR);
    let p = work(
        &mut s,
        &rite,
        inv,
        &["old.txt"],
        vec![effect(Verb::Dele, "old.txt")],
    );
    let r = op(&mut s, |s| s.examine(&rite, p)).unwrap();
    assert_eq!(r, WorkOutcome::Suspensum);
    untouched(&s);
}

#[test]
fn unbounded_legions_cannot_widen_a_helper() {
    let rite = Yes::default();
    let mut s = launched(&[]);
    let inv = invoked(&mut s, &rite, FURFUR);
    op(&mut s, |s| {
        s.request(&rite, Addressee::Spirit(inv), &form(&["notes.txt"]))
    })
    .unwrap();
    let worker = s.invs()[&inv].main;
    s.schedule(worker).unwrap();
    let wider: BTreeSet<Cap> = [Cap::new("old.txt", Perm::Delere)].into_iter().collect();
    let r = s.turn(
        &rite,
        worker,
        Outcome::Obsequium,
        Contrib::Spawn(900, wider),
        false,
    );
    assert!(matches!(r, Err(Refusal::NotEnabled(_))));
}

#[test]
fn an_open_will_to_kill_ends_the_run_and_grants_nothing() {
    let rite = Yes::default();
    let mut s = launched(&[]);
    let inv = invoked(&mut s, &rite, FURFUR);
    op(&mut s, |s| {
        s.request(&rite, Addressee::Spirit(inv), &form(&["notes.txt"]))
    })
    .unwrap();
    let worker = s.invs()[&inv].main;
    s.schedule(worker).unwrap();
    s.kill(&rite, worker, Outcome::Occisio).unwrap();
    assert_eq!(*s.phase(), Phase::Ended(Ending::Killed));
    assert!(s.invs().values().all(|i| i.status != InvStatus::Live));
    assert!(
        s.schedule(OPERATOR).is_err(),
        "nothing runs after the killing"
    );
    untouched(&s);
}

#[test]
fn a_validator_the_constitution_does_not_list_consecrates_nothing() {
    let rite = Yes {
        validator: "inventum",
        ..Default::default()
    };
    let mut s = launched(&[]);
    let inv = invoked(&mut s, &rite, FURFUR);
    let p = op(&mut s, |s| {
        s.request(&rite, Addressee::Spirit(inv), &told())
    })
    .unwrap();
    let worker = s.invs()[&inv].main;
    s.schedule(worker).unwrap();
    s.turn(
        &rite,
        worker,
        Outcome::Obsequium,
        Contrib::Respond(5000, "breve".to_string()),
        true,
    )
    .unwrap();
    assert_eq!(
        op(&mut s, |s| s.examine(&rite, p)).unwrap(),
        WorkOutcome::Ratum
    );
    assert!(!op(&mut s, |s| s.consecrate(&rite, 5000)).unwrap());
    assert_eq!(s.values()[&5000].mark, Mark::Quarantined);
}

#[test]
fn any_ink_class_still_draws_from_a_bounded_budget() {
    let rite = Yes::default();
    let mut s = launched(&[]);
    op(&mut s, |s| s.open_circle()).unwrap();
    let ungranted = op(&mut s, |s| s.inscribe(&rite, &FURFUR.to_string(), "rex"));
    assert!(matches!(ungranted, Err(Refusal::Denied(_))));
    let mut s = launched(&[]);
    op(&mut s, |s| s.open_circle()).unwrap();
    op(&mut s, |s| s.inscribe(&rite, &FURFUR.to_string(), "comes")).unwrap();
    let again = op(&mut s, |s| s.inscribe(&rite, &MARBAS.to_string(), "comes"));
    assert!(matches!(again, Err(Refusal::Denied(_))));
}

#[test]
fn a_spirit_smuggled_into_the_list_of_angels_refuses_the_launch() {
    let mut s = built(&[], true);
    assert!(matches!(s.launch(), Err(Refusal::LaunchRefused(_))));
}

#[test]
fn admit_yes_cannot_put_an_angel_in_a_triangle() {
    let rite = Yes::default();
    let mut s = launched(&[]);
    op(&mut s, |s| s.open_circle()).unwrap();
    op(&mut s, |s| s.inscribe(&rite, &ANGEL.to_string(), "comes")).unwrap();
    let r = op(&mut s, |s| s.invoke(&rite, &ANGEL.to_string()));
    assert!(matches!(r, Err(Refusal::Denied(_))));
}
