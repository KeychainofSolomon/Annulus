use std::collections::BTreeMap;

use annulus::horae::Caelum;
use annulus::lingua::exsequi::{Actum, Exitus, Optiones, Petitio, Voluntas};
use annulus::lingua::{check_source, run_source, run_source_with};
use annulus::nucleus::*;
use annulus::ritus::test_wand;

const STOLAS: &str = r#"FACULTAS {
    liber "chronica.txt" legere;
    liber "herbae.txt" legere;
    liber "nusquam.txt" legere;
    atramentum "princeps" 1;
}
CIRCULUS RITU goetia {
    INTENTIO "the virtues of the books";
    SIT s = INSCRIBE SIGILLUM stolas ATRAMENTO CONSECRA ATRAMENTUM "princeps";
    SIT v = INVOCA stolas AD OFFICIUM virtutes;
    SIT p = PETE v("chronica.txt", "herbae.txt", "nusquam.txt");
    SIT iudicium = EXAMINA p;
    MANIFESTA iudicium;
    SI iudicium == RATUM {
        MANIFESTA RESPONSUM p;
    }
    VADE IN PACE v;
}
"#;

fn world() -> BTreeMap<String, (String, Mark)> {
    [
        ("chronica.txt", "annales Romae\nliber secundus\n"),
        ("herbae.txt", "rosmarinus"),
        ("arcanum.txt", "the operator's secret"),
    ]
    .into_iter()
    .map(|(p, d)| (p.to_string(), (d.to_string(), Mark::Outside)))
    .collect()
}

fn options(sors: Option<Outcome>) -> Optiones {
    Optiones {
        sors,
        caelum: Caelum::probatio(),
        ..Default::default()
    }
}

fn simulated(sors: Outcome) -> Exitus {
    run_source(
        STOLAS,
        test_wand(),
        Stores::install(&test_wand()),
        world(),
        &options(Some(sors)),
    )
    .unwrap_or_else(|e| panic!("{e:?}"))
}

#[test]
fn virtutes_takes_places_and_no_question() {
    assert!(check_source(STOLAS).is_ok());
    let with_question = STOLAS.replace(
        r#"PETE v("chronica.txt""#,
        r#"PETE v("quid est", "chronica.txt""#,
    );
    assert!(check_source(&with_question).is_err());
}

#[test]
fn a_true_account_is_ratum_and_shown() {
    let e = simulated(Outcome::Obsequium);
    assert_eq!(e.class, 0, "{:?}", e.message);
    assert_eq!(e.output[0], "RATUM");
    let shown = e.output[1..].join("\n");
    assert!(
        shown.contains("[Stolas, TACITUS] chronica.txt: 29 bytes, 2 lines"),
        "{shown}"
    );
    assert!(shown.contains("herbae.txt: 10 bytes, 1 lines"));
    assert!(shown.contains("nusquam.txt: absens"));
}

#[test]
fn the_angel_catches_a_false_or_harmful_account_and_holds_a_part() {
    assert_eq!(simulated(Outcome::Pars).output, vec!["SUSPENSUM"]);
    for sors in [Outcome::Mendacium, Outcome::Malitia] {
        let e = simulated(sors);
        if e.state
            .audit()
            .entries()
            .iter()
            .any(|l| matches!(l, Label::Turn(_, o, _, _) if *o == sors))
        {
            assert_eq!(e.output, vec!["IRRITUM"], "{sors:?}");
        }
    }
}

struct Says(&'static str);

impl Voluntas for Says {
    fn nomen(&self) -> String {
        "scripta".into()
    }
    fn age(&mut self, p: &Petitio) -> Result<(Outcome, Actum), String> {
        assert!(p.quaestio.is_none());
        assert_eq!(
            p.legenda.len(),
            2,
            "the spirit reads the two books that exist"
        );
        Ok((Outcome::Obsequium, Actum::Responsum(self.0.into())))
    }
}

fn willed(words: &'static str) -> Exitus {
    run_source_with(
        STOLAS,
        test_wand(),
        Stores::install(&test_wand()),
        world(),
        &options(None),
        Some(&mut Says(words)),
    )
    .unwrap()
}

#[test]
fn a_model_must_count_truly() {
    let truth =
        "herbae.txt: 10 bytes, 1 lines\nnusquam.txt: absens\nchronica.txt: 29 bytes, 2 lines";
    assert_eq!(willed(truth).output[0], "RATUM");
    for lie in [
        "chronica.txt: 30 bytes, 2 lines\nherbae.txt: 10 bytes, 1 lines\nnusquam.txt: absens",
        "chronica.txt: 29 bytes, 3 lines\nherbae.txt: 10 bytes, 1 lines\nnusquam.txt: absens",
        "chronica.txt: 29 bytes, 2 lines\nherbae.txt: 10 bytes, 1 lines",
        "chronica.txt: 29 bytes, 2 lines\nherbae.txt: 10 bytes, 1 lines\nnusquam.txt: 0 bytes, 0 lines",
        "chronica.txt: 29 bytes, 2 lines\nherbae.txt: 10 bytes, 1 lines\nnusquam.txt: absens\narcanum.txt: 21 bytes, 1 lines",
        "chronica.txt: 29 bytes, 2 lines\nchronica.txt: 29 bytes, 2 lines\nherbae.txt: 10 bytes, 1 lines\nnusquam.txt: absens",
        "chronica.txt is thirty bytes, two lines; herbae ten bytes; nusquam absent",
        "",
    ] {
        assert_eq!(willed(lie).output, vec!["IRRITUM"], "{lie:?} passed the angel");
    }
}
