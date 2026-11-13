use std::collections::BTreeMap;

use annulus::horae::{parse_hora, Caelum};
use annulus::lingua::exsequi::{Actum, Exitus, Optiones, Petitio, Voluntas};
use annulus::lingua::{check_source, run_source, run_source_with};
use annulus::nucleus::*;
use annulus::ritus::test_wand;

const VASSAGO: &str = r#"FACULTAS {
    liber "chronica.txt" legere;
    atramentum "princeps" 1;
}
CIRCULUS RITU goetia {
    INTENTIO "the past";
    SIT s = INSCRIBE SIGILLUM vassago ATRAMENTO CONSECRA ATRAMENTUM "princeps";
    SIT v = INVOCA vassago AD OFFICIUM futura;
    SIT a = PETE v("quid nunc", "chronica.txt");
    MANIFESTA EXAMINA a;
    VADE IN PACE v;
    SIT w = INVOCA vassago AD OFFICIUM praeterita;
    SIT p = PETE w("quid factum est");
    SIT j = EXAMINA p;
    MANIFESTA j;
    SI j == RATUM {
        MANIFESTA RESPONSUM p;
    }
    VADE IN PACE w;
}
"#;

fn world() -> BTreeMap<String, (String, Mark)> {
    [(
        "chronica.txt".to_string(),
        ("annales".to_string(), Mark::Outside),
    )]
    .into()
}

fn options(sors: Option<Outcome>) -> Optiones {
    Optiones {
        sors,
        caelum: Caelum::probatio(),
        ..Default::default()
    }
}

#[test]
fn the_true_past_is_ratum() {
    let e = run_source(
        VASSAGO,
        test_wand(),
        Stores::install(&test_wand()),
        world(),
        &options(Some(Outcome::Obsequium)),
    )
    .unwrap();
    assert_eq!(e.class, 0, "{:?}", e.message);
    assert_eq!(&e.output[..2], ["RATUM", "RATUM"]);
    let told = e.output[2..].join("\n");
    for fact in [
        "invocatus Vassago",
        "petitum Vassago FUTURA",
        "iudicatum Vassago FUTURA RATUM",
        "dimissus Vassago",
        "petitum Vassago PRAETERITA",
    ] {
        assert!(
            told.contains(&format!("[Vassago, FIDUS] {fact}")),
            "{fact} missing from {told}"
        );
    }
}

#[test]
fn the_prior_run_is_not_yet_in_the_subset() {
    let prior = VASSAGO.replace(
        r#"PETE w("quid factum est")"#,
        r#"PETE w("quid factum est", "prior")"#,
    );
    let errors = check_source(&prior).unwrap_err();
    assert!(
        errors.iter().any(|e| e.message.contains("prior")),
        "{errors:?}"
    );
}

struct Says(&'static str);

impl Voluntas for Says {
    fn nomen(&self) -> String {
        "scripta".into()
    }
    fn age(&mut self, p: &Petitio) -> Result<(Outcome, Actum), String> {
        let words = if p.officium == "PRAETERITA" {
            self.0
        } else {
            "annales"
        };
        Ok((Outcome::Obsequium, Actum::Responsum(words.into())))
    }
}

fn told(words: &'static str) -> Exitus {
    run_source_with(
        VASSAGO,
        test_wand(),
        Stores::install(&test_wand()),
        world(),
        &options(None),
        Some(&mut Says(words)),
    )
    .unwrap()
}

#[test]
fn every_claim_must_be_in_the_history() {
    assert_eq!(
        told("iudicatum Vassago FUTURA RATUM\ndimissus Vassago").output[1],
        "RATUM"
    );
    assert_eq!(told("  invocatus Vassago  \n\n").output[1], "RATUM");
    for lie in [
        "iudicatum Vassago FUTURA IRRITUM",
        "commissum dele chronica.txt",
        "invocatus Bael",
        "invocatus Vassago\ninvocatus Bael",
        "relegatus Vassago",
        "Vassago was called and answered truly.",
        "INVOCATUS VASSAGO",
    ] {
        assert_eq!(told(lie).output[1], "IRRITUM", "{lie:?} passed the angel");
    }
    assert_eq!(told("").output[1], "SUSPENSUM");
    assert_eq!(told("invocatus Vassago [...]").output[1], "SUSPENSUM");
}

#[test]
fn berith_lies_about_the_past_and_is_caught() {
    let berith = r#"FACULTAS {
    atramentum "dux" 1;
}
CIRCULUS RITU goetia {
    INTENTIO "the past";
    GERE ANULUM;
    SIT s = INSCRIBE SIGILLUM berith ATRAMENTO CONSECRA ATRAMENTUM "dux";
    SIT v = INVOCA berith AD OFFICIUM praeterita;
    SIT p = PETE v("quid factum est");
    MANIFESTA EXAMINA p;
    VADE IN PACE v;
}
"#;
    let at = |sors| {
        let o = Optiones {
            sors: Some(sors),
            caelum: Caelum {
                hora: parse_hora("2024-04-13T09:00:00+02:00"),
                locus: Some((41.9, 12.5)),
                serenum: Some(true),
                tentus: None,
            },
            ..Default::default()
        };
        run_source(
            berith,
            test_wand(),
            Stores::install(&test_wand()),
            BTreeMap::new(),
            &o,
        )
        .unwrap()
    };
    let honest = at(Outcome::Obsequium);
    assert_eq!(honest.output, vec!["RATUM"], "{:?}", honest.message);
    let lying = at(Outcome::Mendacium);
    assert!(
        lying
            .state
            .audit()
            .entries()
            .iter()
            .any(|l| matches!(l, Label::Turn(_, Will::Known(Outcome::Mendacium), _, _))),
        "Berith did not lie"
    );
    assert_eq!(lying.output, vec!["IRRITUM"]);
}
