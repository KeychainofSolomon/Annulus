//! The four kings (`INVOCA REGEM`, `static.md` §4.5b, `dynamic.md` §4): invoked only
//! in their hours, and only then may a President under that king come.

use std::collections::BTreeMap;

use annulus::horae::{local_day, sol, window, Caelum};
use annulus::lingua::check_source;
use annulus::lingua::exsequi::{Exitus, Optiones, Responsum};
use annulus::lingua::registra::{goetia, reges, rex};
use annulus::lingua::run_source;
use annulus::nucleus::*;
use annulus::ritus::test_wand;

const LOCUS: (f64, f64) = (41.9, 12.5);

/// The sky at `t`, in Rome, clear.
fn caelum(t: i64) -> Caelum {
    Caelum {
        hora: Some(t),
        locus: Some(LOCUS),
        serenum: Some(true),
        tentus: None,
    }
}

/// The sun's day of the proving hour (2024-04-13, the Moon five... an allowed age).
fn day() -> annulus::horae::Sol {
    let t = Caelum::probatio().hora.unwrap() as f64;
    sol(local_day(t, LOCUS.1), LOCUS.0, LOCUS.1).unwrap()
}

/// A moment within the kings' second span: the tenth hour of the day.
fn kings_hour() -> i64 {
    let s = day();
    (s.ortus + 9.5 * s.hora()) as i64
}

fn run_at(source: &str, t: i64) -> Exitus {
    let options = Optiones {
        semen: 0,
        sors: Some(Outcome::Obsequium),
        responsa: vec![Responsum {
            spirit: 5,
            office: "RESPONSA".to_string(),
            quaestio: "quid latet".to_string(),
            verum: "ita".to_string(),
            falsum: Some("non".to_string()),
        }],
        caelum: caelum(t),
        ..Default::default()
    };
    match run_source(
        source,
        test_wand(),
        Stores::install(&test_wand()),
        BTreeMap::new(),
        &options,
    ) {
        Ok(e) => e,
        Err(errors) => panic!("refused before running: {errors:?}"),
    }
}

fn codes(source: &str) -> Vec<String> {
    match check_source(source) {
        Ok(_) => Vec::new(),
        Err(errors) => errors.iter().map(|e| e.code.to_string()).collect(),
    }
}

fn only_king(king: &str) -> String {
    format!(
        "CIRCULUS RITU goetia {{\n    INTENTIO \"of the king\";\n    INVOCA REGEM {king};\n}}\n"
    )
}

/// Marbas, a President of the East, asked a question; `before` stands ahead of
/// his invocation.
fn marbas(before: &str) -> String {
    format!(
        "FACULTAS {{\n    atramentum \"praeses\" 1;\n}}\nCIRCULUS RITU goetia {{\n    INTENTIO \"of hidden things\";\n    SIT s = INSCRIBE SIGILLUM marbas ATRAMENTO CONSECRA ATRAMENTUM \"praeses\";\n{before}    SIT v = INVOCA marbas AD OFFICIUM responsa;\n    SIT p = PETE v(\"quid latet\");\n    MANIFESTA EXAMINA p;\n    VADE IN PACE v;\n}}\n"
    )
}

#[test]
fn the_register_holds_the_four_kings_by_every_spelling() {
    let names: Vec<&str> = reges().iter().map(|r| r.name.as_str()).collect();
    assert_eq!(names, ["Amaymon", "Corson", "Ziminiar", "Goap"]);
    assert_eq!(rex("gorson").unwrap().label, "corson");
    assert_eq!(rex("Zymymar").unwrap().quarter, "North");
    // the kings of another rite are not kings of this one
    assert!(rex("oriens").is_none());
    assert!(rex("paimon").is_none());
}

#[test]
fn each_president_comes_under_the_king_of_his_quarter() {
    let king = |n: i64| {
        goetia()
            .by_number(n)
            .unwrap()
            .rex()
            .map(|r| r.name.as_str())
    };
    assert_eq!(king(5), Some("Amaymon")); // Marbas
    assert_eq!(king(33), Some("Amaymon")); // Gaap
    assert_eq!(king(31), Some("Corson")); // Foras
    assert_eq!(king(39), Some("Corson")); // Malphas
    assert_eq!(king(57), Some("Corson")); // Ose
    assert_eq!(king(61), Some("Goap")); // Zagan
                                        // no witness states Buer's quarter
    assert_eq!(king(10), None);
}

#[test]
fn each_king_is_invoked_in_the_kings_hours_and_refused_outside_them() {
    let s = day();
    let open = [
        (s.ortus + 2.5 * s.hora()) as i64, // the third hour
        kings_hour(),
    ];
    let shut = [
        (s.ortus + 1.0 * s.hora()) as i64, // before the third hour
        (s.ortus + 7.0 * s.hora()) as i64, // between noon and the ninth hour
        (s.occasus + 60.0) as i64,         // after evening
    ];
    for king in ["amaymon", "corson", "gorson", "ziminiar", "goap"] {
        for t in open {
            let e = run_at(&only_king(king), t);
            assert_eq!(e.class, 0, "{king} at {t}: {:?}", e.message);
            let recorded = e.state.audit().entries().iter().any(|l| {
                matches!(l, Label::Act(RiteAct::Rex(k), _, true, _) if *k == rex(king).unwrap().label)
            });
            assert!(recorded, "{king} is not on the record");
        }
        for t in shut {
            let e = run_at(&only_king(king), t);
            assert_eq!(e.class, 2, "{king} at {t} should be refused");
            // the refusal is on the record too, and nothing was admitted
            assert!(e
                .state
                .audit()
                .entries()
                .iter()
                .any(|l| matches!(l, Label::Act(RiteAct::Rex(_), _, false, _))));
        }
    }
}

#[test]
fn a_president_comes_after_his_king() {
    let e = run_at(&marbas("    INVOCA REGEM amaymon;\n"), kings_hour());
    assert_eq!(e.class, 0, "{:?}", e.message);
    assert_eq!(e.output.first().map(String::as_str), Some("RATUM"));
}

#[test]
fn a_president_without_his_king_is_refused_by_the_checker() {
    assert!(codes(&marbas("")).contains(&"A13".to_string()));
    // another king is not his king
    assert!(codes(&marbas("    INVOCA REGEM corson;\n")).contains(&"A13".to_string()));
    // his king on one path only is not his king on every path
    let one_path = marbas("    SI 1 == 1 {\n        INVOCA REGEM amaymon;\n    }\n");
    assert!(
        codes(&one_path).contains(&"A13".to_string()),
        "{:?}",
        codes(&one_path)
    );
    // with his king, lawful
    assert_eq!(
        codes(&marbas("    INVOCA REGEM amaymon;\n")),
        Vec::<String>::new()
    );
}

#[test]
fn a_president_of_no_stated_quarter_cannot_be_invoked() {
    let buer = "FACULTAS {\n    atramentum \"praeses\" 1;\n}\nCIRCULUS RITU goetia {\n    INTENTIO \"x\";\n    INVOCA REGEM amaymon;\n    SIT s = INSCRIBE SIGILLUM buer ATRAMENTO CONSECRA ATRAMENTUM \"praeses\";\n    SIT v = INVOCA buer AD OFFICIUM doctrina;\n    VADE IN PACE v;\n}\n";
    assert!(codes(buer).contains(&"A13".to_string()));
}

#[test]
fn a_king_of_another_rite_or_outside_a_circle_is_refused() {
    assert!(codes(&only_king("oriens")).contains(&"A3".to_string()));
    assert!(codes("INVOCA REGEM amaymon;\n").contains(&"A1".to_string()));
}

#[test]
fn the_kings_invocation_does_not_carry_into_a_second_circle() {
    let second = marbas("");
    let (facultas, circle) = second.split_at(second.find("CIRCULUS").unwrap());
    let two = format!("{facultas}{}{circle}", only_king("amaymon"));
    assert_eq!(codes(&two), ["A13"], "{two}");
}

#[test]
fn a_president_is_never_bound_at_nightfall() {
    let s = day();
    let at = |t: f64| window("president", t, &caelum(t as i64));
    assert!(at(s.ortus + 3.0 * s.hora()).is_ok());
    assert!(at(s.occasus - 60.0).is_ok());
    assert!(at(s.occasus + 60.0).is_err());
    assert!(at(s.crepusculum - 60.0).is_err());
    assert!(at(s.crepusculum + 60.0).is_ok());
    // and with his king named, the run at nightfall still ends in a denial
    let e = run_at(
        &marbas("    INVOCA REGEM amaymon;\n"),
        (s.occasus + 60.0) as i64,
    );
    assert_eq!(e.class, 2);
}
