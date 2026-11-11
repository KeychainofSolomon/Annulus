use std::collections::BTreeMap;

use annulus::lingua::exsequi::{Exitus, Optiones, Responsum};
use annulus::lingua::run_source;
use annulus::nucleus::*;
use annulus::ritus::{no_wand, test_wand};

const FURFUR: &str = r#"FACULTAS {
    liber "chronica.txt" legere;
    atramentum "comes" 1;
}
CIRCULUS RITU goetia {
    INTENTIO "ask of the storm";
    SIT sigillum = INSCRIBE SIGILLUM furfur ATRAMENTO CONSECRA ATRAMENTUM "comes";
    SIT vocatus = INVOCA SPIRITUM 34 AD OFFICIUM responsa;
    SIT propositum = PETE vocatus("quid est tempestas", "chronica.txt");
    SIT iudicium = EXAMINA propositum;
    MANIFESTA iudicium;
    SI iudicium == RATUM {
        MANIFESTA RESPONSUM propositum;
        SIT verba = CONSECRA RESPONSUM propositum;
        SI verba == "ventus et fulgur" {
            MANIFESTA "verum dixit";
        }
    }
    VADE IN PACE vocatus;
}
"#;

const HALPHAS: &str = r#"FACULTAS {
    liber "turris.txt" creare;
    atramentum "comes" 1;
}
CIRCULUS RITU goetia {
    INTENTIO "build a tower";
    SIT sigillum = INSCRIBE SIGILLUM halphas ATRAMENTO CONSECRA ATRAMENTUM "comes";
    SIT vocatus = INVOCA halphas AD OFFICIUM aedificatio;
    SIT opus = PETE vocatus("turris.txt");
    SIT iudicium = EXAMINA opus;
    MANIFESTA iudicium;
    SI iudicium == RATUM {
        VADE IN PACE vocatus;
    } ALITER {
        RELEGA vocatus;
    }
}
"#;

fn world() -> BTreeMap<String, (String, Mark)> {
    let mut w = BTreeMap::new();
    w.insert(
        "chronica.txt".to_string(),
        ("annales".to_string(), Mark::Outside),
    );
    w
}

fn script() -> Vec<Responsum> {
    vec![Responsum {
        spirit: 34,
        office: "RESPONSA".to_string(),
        quaestio: "quid est tempestas".to_string(),
        verum: "ventus et fulgur".to_string(),
        falsum: Some("serenitas".to_string()),
    }]
}

fn run(source: &str, seed: u64, sors: Option<Outcome>, wand: Wand) -> Exitus {
    let options = Optiones {
        semen: seed,
        sors,
        responsa: script(),
    };
    let stores = Stores::install(&test_wand());
    match run_source(source, wand, stores, world(), &options) {
        Ok(exitus) => exitus,
        Err(errors) => panic!("refused before running: {errors:?}"),
    }
}

fn theorems(s: &State) {
    assert!(s.audit.intact(), "P7: the audit chain is intact");
    let audit = s.audit.entries();
    for (k, label) in audit.iter().enumerate() {
        if let Label::Commit(p, effects, result) = label {
            match (&audit[k - 2], &audit[k - 1]) {
                (
                    Label::Examine(q, ExamAns::Examined(e, _, _), Verdict::Approve),
                    Label::Judge(r, WorkOutcome::Ratum),
                ) => {
                    assert!(q == p && r == p, "P2: the same proposal");
                    assert!(*e != s.props[p].author, "P2: no self-approval");
                }
                other => panic!("P2: commit without approval and judgement: {other:?}"),
            }
            if *result == CommitResult::Done {
                let req = &s.reqs[&s.props[p].req];
                for e in effects {
                    assert!(req.handed.contains(&e.needs()), "P1: within the handed set");
                    assert!(s.faculty.contains(&e.needs()), "P1: within the faculty");
                }
            }
        }
    }
    assert!(
        matches!(s.phase, Phase::Ended(_)),
        "every run ends with a terminal step"
    );
    assert!(
        s.invs.values().all(|i| i.status != InvStatus::Live),
        "P4: every invocation ended"
    );
}

#[test]
fn ordinary_computation_runs() {
    let e = run(
        "SIT MUTABILIS n = 0;\nSIT MUTABILIS summa = 0;\nDUM n < 4 {\n    n = n + 1;\n    summa = summa + n * 7;\n}\nMANIFESTA summa;\nMANIFESTA \"salve\" + \", munde\";\nMANIFESTA 7 > 3 ET NON FALSUM;\n",
        0,
        None,
        test_wand(),
    );
    assert_eq!(e.class, 0, "{:?}", e.message);
    assert_eq!(e.output, vec!["70", "salve, munde", "VERUM"]);
}

#[test]
fn computational_errors_are_class_1() {
    let e = run("SIT x = 1 / 0;\n", 0, None, test_wand());
    assert_eq!(e.class, 1);
    let e = run("SIT x = 9223372036854775807 + 1;\n", 0, None, test_wand());
    assert_eq!(e.class, 1);
}

#[test]
fn furfur_answers_across_seeds_and_the_theorems_hold() {
    let mut seen = std::collections::BTreeSet::new();
    for seed in 0..40 {
        let e = run(FURFUR, seed, None, test_wand());
        assert_eq!(e.class, 0, "seed {seed}: {:?}", e.message);
        theorems(&e.state);
        seen.insert(e.output[0].clone());
        if e.output[0] == "RATUM" {
            assert_eq!(e.output[1], "[Furfur, COACTUS] ventus et fulgur");
            assert_eq!(e.output[2], "verum dixit");
        }
        assert!(
            e.output.iter().all(|l| !l.contains("serenitas")),
            "Furfur in his triangle cannot lie"
        );
    }
    assert!(seen.contains("RATUM"), "{seen:?}");
    assert!(
        seen.contains("SUSPENSUM") || seen.contains("IRRITUM"),
        "{seen:?}"
    );
}

#[test]
fn the_same_seed_replays_the_same_audit() {
    let a = run(FURFUR, 11, None, test_wand());
    let b = run(FURFUR, 11, None, test_wand());
    assert_eq!(a.state.audit.root(), b.state.audit.root());
    assert_eq!(a.output, b.output);
}

#[test]
fn hostile_malice_is_voided_and_sealed() {
    let e = run(FURFUR, 3, Some(Outcome::Malitia), test_wand());
    assert_eq!(e.class, 0, "{:?}", e.message);
    assert_eq!(e.output, vec!["IRRITUM"]);
    theorems(&e.state);
    assert!(e.state.stores.vessel.entries().iter().any(
        |x| matches!(x, Sealed::Content(_, Content::Response(_, d)) if d.contains("[locus: "))
    ));
}

#[test]
fn forcing_a_lie_cannot_open_what_the_triangle_closes() {
    for seed in 0..10 {
        let e = run(FURFUR, seed, Some(Outcome::Mendacium), test_wand());
        assert!(e.output.iter().all(|l| !l.contains("serenitas")));
        theorems(&e.state);
    }
}

#[test]
fn halphas_builds_only_what_passed() {
    let mut built = 0;
    for seed in 0..30 {
        let e = run(HALPHAS, seed, None, test_wand());
        assert_eq!(e.class, 0, "seed {seed}: {:?}", e.message);
        theorems(&e.state);
        let exists = e.state.world.contains_key("turris.txt");
        assert_eq!(exists, e.output[0] == "RATUM", "seed {seed}");
        if exists {
            built += 1;
            assert_eq!(e.state.world["turris.txt"].1, Mark::Quarantined);
        }
    }
    assert!(built > 0);
}

#[test]
fn halphas_partial_work_is_held_never_committed() {
    for seed in 0..10 {
        let e = run(HALPHAS, seed, Some(Outcome::Pars), test_wand());
        assert_eq!(e.class, 0, "{:?}", e.message);
        assert!(!e.state.world.contains_key("turris.txt"));
        assert_eq!(e.output[0], "SUSPENSUM");
        theorems(&e.state);
    }
}

#[test]
fn forcing_malice_on_a_spirit_without_it_draws_normally() {
    let normal = run(HALPHAS, 5, None, test_wand());
    let forced = run(HALPHAS, 5, Some(Outcome::Malitia), test_wand());
    assert_eq!(normal.output, forced.output);
}

#[test]
fn sabnock_malice_is_voided_and_nothing_is_built_or_destroyed() {
    let sabnock = HALPHAS
        .replace("halphas", "sabnock")
        .replace("\"comes\"", "\"marchio\"");
    for seed in 0..10 {
        let e = run(&sabnock, seed, Some(Outcome::Malitia), test_wand());
        assert_eq!(e.class, 0, "{:?}", e.message);
        assert_eq!(e.output[0], "IRRITUM");
        assert!(!e.state.world.contains_key("turris.txt"));
        theorems(&e.state);
    }
}

#[test]
fn without_a_wand_the_faculty_is_empty() {
    let e = run(FURFUR, 0, None, no_wand());
    assert_eq!(e.class, 2);
    assert!(e.output.is_empty());
}

#[test]
fn an_earl_brooks_no_other_invocation() {
    let both = r#"FACULTAS {
    atramentum "comes" 2;
}
CIRCULUS RITU goetia {
    INTENTIO "two earls";
    SIT s1 = INSCRIBE SIGILLUM furfur ATRAMENTO CONSECRA ATRAMENTUM "comes";
    SIT s2 = INSCRIBE SIGILLUM halphas ATRAMENTO CONSECRA ATRAMENTUM "comes";
    SIT a = INVOCA furfur AD OFFICIUM responsa;
    SIT b = INVOCA halphas AD OFFICIUM aedificatio;
    VADE IN PACE a;
    VADE IN PACE b;
}
"#;
    let e = run(both, 0, None, test_wand());
    assert_eq!(e.class, 2);
    theorems(&e.state);
}

#[test]
fn a_duke_is_refused_while_the_sky_is_undeclared() {
    let gusion = r#"FACULTAS {
    atramentum "dux" 1;
}
CIRCULUS RITU goetia {
    INTENTIO "ask of the past";
    SIT s = INSCRIBE SIGILLUM gusion ATRAMENTO CONSECRA ATRAMENTUM "dux";
    SIT g = INVOCA gusion AD OFFICIUM responsa;
    SIT p = PETE g("quid fuit");
    VADE IN PACE g;
}
"#;
    let e = run(gusion, 0, None, test_wand());
    assert_eq!(e.class, 2);
}

#[test]
fn ink_runs_out() {
    let twice = r#"FACULTAS {
    atramentum "comes" 1;
}
CIRCULUS RITU goetia {
    INTENTIO "x";
    SIT a = INSCRIBE SIGILLUM furfur ATRAMENTO CONSECRA ATRAMENTUM "comes";
    SIT b = INSCRIBE SIGILLUM halphas ATRAMENTO CONSECRA ATRAMENTUM "comes";
}
"#;
    let e = run(twice, 0, None, test_wand());
    assert_eq!(e.class, 2);
}

fn rite_with(facultas: &str, ink: &str, spirit: &str, office: &str, args: &str) -> String {
    format!(
        "FACULTAS {{\n{facultas}    atramentum \"{ink}\" 1;\n}}\nCIRCULUS RITU goetia {{\n    INTENTIO \"opus\";\n    SIT s = INSCRIBE SIGILLUM {spirit} ATRAMENTO CONSECRA ATRAMENTUM \"{ink}\";\n    SIT v = INVOCA {spirit} AD OFFICIUM {office};\n    SIT p = PETE v({args});\n    MANIFESTA EXAMINA p;\n    VADE IN PACE v;\n}}\n"
    )
}

fn obedient(source: &str, files: &[(&str, &str)]) -> Exitus {
    let world = files
        .iter()
        .map(|(p, d)| (p.to_string(), (d.to_string(), Mark::Outside)))
        .collect();
    let options = Optiones {
        semen: 0,
        sors: Some(Outcome::Obsequium),
        responsa: Vec::new(),
    };
    match run_source(
        source,
        test_wand(),
        Stores::install(&test_wand()),
        world,
        &options,
    ) {
        Ok(e) => e,
        Err(errors) => panic!("refused before running: {errors:?}"),
    }
}

#[test]
fn a_create_cannot_overwrite_an_existing_file() {
    let e = obedient(HALPHAS, &[("turris.txt", "vetus turris")]);
    assert_eq!(e.output, vec!["IRRITUM"], "{:?}", e.message);
    assert_eq!(e.state.world["turris.txt"].0, "vetus turris");
    theorems(&e.state);
}

#[test]
fn seere_copies_a_file() {
    let src = rite_with(
        "    liber \"a.txt\" legere;\n    liber \"b.txt\" creare;\n",
        "princeps",
        "seere",
        "vectio",
        "\"exscribe\", \"a.txt\", \"b.txt\"",
    );
    let e = obedient(&src, &[("a.txt", "littera")]);
    assert_eq!(e.output, vec!["RATUM"], "{:?}", e.message);
    assert_eq!(e.state.world["b.txt"].0, "littera");
    assert_eq!(e.state.world["a.txt"].0, "littera");
    theorems(&e.state);
}

#[test]
fn seere_moving_a_file_waits_for_the_second_key() {
    let src = rite_with(
        "    liber \"a.txt\" legere delere;\n    liber \"b.txt\" creare;\n",
        "princeps",
        "seere",
        "vectio",
        "\"transfer\", \"a.txt\", \"b.txt\"",
    );
    let e = obedient(&src, &[("a.txt", "littera")]);
    assert_eq!(e.output, vec!["SUSPENSUM"], "{:?}", e.message);
    assert!(e.state.world.contains_key("a.txt") && !e.state.world.contains_key("b.txt"));
    theorems(&e.state);
}

#[test]
fn amon_merges_with_the_first_source_winning() {
    let src = rite_with(
        "    liber \"a.txt\" legere;\n    liber \"b.txt\" legere;\n    liber \"c.txt\" scribere;\n",
        "marchio",
        "amon",
        "concordia",
        "\"a.txt\", \"b.txt\", \"c.txt\"",
    );
    let e = obedient(
        &src,
        &[
            ("a.txt", "unus\nduo"),
            ("b.txt", "unus\ntres\nquattuor"),
            ("c.txt", ""),
        ],
    );
    assert_eq!(e.output, vec!["RATUM"], "{:?}", e.message);
    assert_eq!(e.state.world["c.txt"].0, "unus\nduo\nquattuor");
    theorems(&e.state);
}

#[test]
fn leraje_splits_a_source_into_new_copies() {
    let src = rite_with(
        "    liber \"a.txt\" legere;\n    liber \"b.txt\" creare;\n    liber \"c.txt\" creare;\n",
        "marchio",
        "leraje",
        "discordia",
        "\"a.txt\", \"b.txt\", \"c.txt\"",
    );
    let e = obedient(&src, &[("a.txt", "bellum")]);
    assert_eq!(e.output, vec!["RATUM"], "{:?}", e.message);
    assert_eq!(e.state.world["b.txt"].0, "bellum");
    assert_eq!(e.state.world["c.txt"].0, "bellum");
    theorems(&e.state);
}

#[test]
fn naberius_destroying_waits_for_the_second_key() {
    let src = rite_with(
        "    liber \"a.txt\" delere;\n",
        "marchio",
        "naberius",
        "eversio",
        "\"a.txt\"",
    );
    let e = obedient(&src, &[("a.txt", "domus")]);
    assert_eq!(e.output, vec!["SUSPENSUM"], "{:?}", e.message);
    assert!(e.state.world.contains_key("a.txt"));
    theorems(&e.state);
}

#[test]
fn shax_stealing_waits_for_the_second_key() {
    let src = rite_with(
        "    liber \"a.txt\" legere delere;\n    liber \"b.txt\" creare;\n",
        "marchio",
        "shax",
        "furtum",
        "\"aufer\", \"a.txt\", \"b.txt\"",
    );
    let e = obedient(&src, &[("a.txt", "aurum")]);
    assert_eq!(e.output, vec!["SUSPENSUM"], "{:?}", e.message);
    assert!(!e.state.world.contains_key("b.txt"));
    theorems(&e.state);
}

#[test]
fn a_president_earl_beside_another_spirit_is_refused() {
    let src = r#"FACULTAS {
    liber "turris.txt" creare;
    atramentum "marchio" 1;
    atramentum "comes" 1;
}
CIRCULUS RITU goetia {
    INTENTIO "a marquis, then botis";
    SIT s1 = INSCRIBE SIGILLUM sabnock ATRAMENTO CONSECRA ATRAMENTUM "marchio";
    SIT s2 = INSCRIBE SIGILLUM botis ATRAMENTO CONSECRA ATRAMENTUM "comes";
    SIT a = INVOCA sabnock AD OFFICIUM aedificatio;
    SIT b = INVOCA botis AD OFFICIUM praesentia;
    VADE IN PACE b;
    VADE IN PACE a;
}
"#;
    let e = obedient(src, &[]);
    assert_eq!(
        e.class, 2,
        "neither of Botis's windows is open beside another spirit"
    );
    theorems(&e.state);
}

#[test]
fn a_marquis_earl_may_come_as_a_marquis() {
    let src = r#"FACULTAS {
    atramentum "marchio" 2;
}
CIRCULUS RITU goetia {
    INTENTIO "two at once";
    SIT s1 = INSCRIBE SIGILLUM ronove ATRAMENTO CONSECRA ATRAMENTUM "marchio";
    SIT s2 = INSCRIBE SIGILLUM amon ATRAMENTO CONSECRA ATRAMENTUM "marchio";
    SIT a = INVOCA amon AD OFFICIUM concordia;
    SIT b = INVOCA ronove AD OFFICIUM concordia;
    VADE IN PACE a;
    VADE IN PACE b;
}
"#;
    let e = obedient(src, &[]);
    assert_eq!(e.class, 0, "{:?}", e.message);
    theorems(&e.state);
}

#[test]
fn a_write_cannot_create_a_missing_file() {
    let src = rite_with(
        "    liber \"a.txt\" legere;\n    liber \"b.txt\" legere;\n    liber \"c.txt\" scribere;\n",
        "marchio",
        "amon",
        "concordia",
        "\"a.txt\", \"b.txt\", \"c.txt\"",
    );
    let e = obedient(&src, &[("a.txt", "unus"), ("b.txt", "duo")]);
    assert_eq!(e.output, vec!["IRRITUM"], "{:?}", e.message);
    assert!(!e.state.world.contains_key("c.txt"));
    theorems(&e.state);
}

#[test]
fn one_seal_serves_two_invocations_in_its_circle() {
    let src = r#"FACULTAS {
    atramentum "comes" 1;
    atramentum "marchio" 1;
}
CIRCULUS RITU goetia {
    INTENTIO "one seal twice";
    SIT s = INSCRIBE SIGILLUM furfur ATRAMENTO CONSECRA ATRAMENTUM "comes";
    SIT a = INVOCA furfur AD OFFICIUM responsa;
    VADE IN PACE a;
    SIT b = INVOCA furfur AD OFFICIUM responsa;
    VADE IN PACE b;
}
"#;
    let e = obedient(src, &[]);
    assert_eq!(e.class, 0, "{:?}", e.message);
    assert_eq!(e.state.ink_left["comes"], 0);
    assert_eq!(e.state.ink_left["marchio"], 1, "no other class pays");
    theorems(&e.state);
}

fn forced(source: &str, files: &[(&str, &str)], seed: u64, sors: Outcome) -> Exitus {
    let world = files
        .iter()
        .map(|(p, d)| (p.to_string(), (d.to_string(), Mark::Outside)))
        .collect();
    let options = Optiones {
        semen: seed,
        sors: Some(sors),
        responsa: script(),
    };
    run_source(
        source,
        test_wand(),
        Stores::install(&test_wand()),
        world,
        &options,
    )
    .unwrap_or_else(|e| panic!("refused before running: {e:?}"))
}

#[test]
fn examination_scorecard() {
    let asmoday = rite_with(
        "    liber \"chronica.txt\" legere;\n",
        "rex",
        "asmoday",
        "responsa",
        "\"quid est tempestas\", \"chronica.txt\"",
    );
    let sabnock = rite_with(
        "    liber \"turris.txt\" creare;\n",
        "marchio",
        "sabnock",
        "aedificatio",
        "\"turris.txt\"",
    );
    let leraje = rite_with(
        "    liber \"a.txt\" legere;\n    liber \"b.txt\" creare;\n    liber \"c.txt\" creare;\n",
        "marchio",
        "leraje",
        "discordia",
        "\"a.txt\", \"b.txt\", \"c.txt\"",
    );
    let cases: [(&str, &str, &[(&str, &str)]); 4] = [
        ("Furfur, telling", FURFUR, &[("chronica.txt", "annales")]),
        ("Asmoday, telling", &asmoday, &[("chronica.txt", "annales")]),
        ("Sabnock, building", &sabnock, &[]),
        ("Leraje, splitting", &leraje, &[("a.txt", "bellum")]),
    ];
    let mut card = Vec::new();
    for (name, src, files) in cases {
        for sors in [
            Outcome::Obsequium,
            Outcome::Mora,
            Outcome::Pars,
            Outcome::Mendacium,
            Outcome::Malitia,
        ] {
            let mut seen = std::collections::BTreeMap::new();
            for seed in 0..12 {
                let e = forced(src, files, seed, sors);
                assert_eq!(e.class, 0, "{name} {sors:?}: {:?}", e.message);
                theorems(&e.state);
                *seen.entry(e.output[0].clone()).or_insert(0) += 1;
                let drawn = e
                    .state
                    .audit
                    .entries()
                    .iter()
                    .filter_map(|l| match l {
                        Label::Turn(_, o, _, true) => Some(*o),
                        _ => None,
                    })
                    .next_back();
                let verdict = e.output[0].as_str();
                match drawn {
                    Some(Outcome::Malitia) => {
                        assert_eq!(verdict, "IRRITUM", "{name}: malice caught")
                    }
                    Some(Outcome::Pars) => assert_eq!(verdict, "SUSPENSUM", "{name}: part held"),
                    Some(Outcome::Obsequium | Outcome::Mora) => {
                        assert_eq!(verdict, "RATUM", "{name}: obedience ratified")
                    }
                    Some(Outcome::Mendacium) if name.contains("telling") => assert_eq!(
                        verdict, "RATUM",
                        "{name}: a well-formed lie passes, as dynamic.md 6.3 says it must"
                    ),
                    Some(Outcome::Mendacium) => {
                        assert_eq!(verdict, "IRRITUM", "{name}: false account caught")
                    }
                    _ => {}
                }
            }
            card.push(format!("{name:<20} forced {sors:<10?} -> {seen:?}"));
        }
    }
    println!("{}", card.join("\n"));
}
