use std::collections::BTreeMap;

use annulus::horae::Caelum;
use annulus::lingua::exsequi::{Actum, Exitus, Optiones, Petitio, Voluntas};
use annulus::lingua::{check_source, run_source, run_source_with};
use annulus::nucleus::*;
use annulus::ritus::test_wand;

const QUAESTIO: &str = "numerus: 17*23 = ?; geometria: hypotenusa 3 4 = ?; dies: pascha 2026 = ?";

fn program(spirit: &str, ink: &str, quaestio: &str) -> String {
    format!(
        r#"FACULTAS {{
    atramentum "{ink}" 1;
}}
CIRCULUS RITU goetia {{
    INTENTIO "a lesson";
    SIT s = INSCRIBE SIGILLUM {spirit} ATRAMENTO CONSECRA ATRAMENTUM "{ink}";
    SIT v = INVOCA {spirit} AD OFFICIUM doctrina;
    SIT p = PETE v("{quaestio}");
    SIT iudicium = EXAMINA p;
    MANIFESTA iudicium;
    SI iudicium == RATUM {{
        MANIFESTA RESPONSUM p;
    }}
    VADE IN PACE v;
}}
"#
    )
}

fn crocell() -> String {
    program("crocell", "dux", QUAESTIO)
}

fn world() -> BTreeMap<String, (String, Mark)> {
    [("arcanum.txt", "the operator's secret")]
        .into_iter()
        .map(|(p, d)| (p.to_string(), (d.to_string(), Mark::Outside)))
        .collect()
}

/// The test sky is mid-afternoon in Rome, when Marquises come. A Duke comes
/// from sunrise to noon under a clear sky: 09:30 in Rome, the same day.
fn sky(src: &str) -> Caelum {
    let mut caelum = Caelum::probatio();
    if src.contains("crocell") {
        caelum.hora = caelum.hora.map(|h| h - 7 * 3600);
        caelum.serenum = Some(true);
    }
    caelum
}

fn options(src: &str, sors: Option<Outcome>) -> Optiones {
    Optiones {
        sors,
        caelum: sky(src),
        ..Default::default()
    }
}

fn simulated(src: &str, sors: Outcome) -> Exitus {
    run_source(
        src,
        test_wand(),
        Stores::install(&test_wand()),
        world(),
        &options(src, Some(sors)),
    )
    .unwrap_or_else(|e| panic!("{e:?}"))
}

#[test]
fn doctrina_takes_a_question() {
    assert!(check_source(&crocell()).is_ok());
    // Ronove teaches only rhetoric, but holds the office: the rite checks.
    assert!(check_source(&program("ronove", "marchio", QUAESTIO)).is_ok());
    // Furfur holds no teaching office at all.
    assert!(check_source(&program("furfur", "comes", QUAESTIO)).is_err());
}

#[test]
fn a_true_lesson_is_ratum_and_shown() {
    let e = simulated(&crocell(), Outcome::Obsequium);
    assert_eq!(e.output[0], "RATUM", "{:?}", e.message);
    let shown = e.output[1..].join("\n");
    assert!(shown.contains("numerus: 17*23 = 391"), "{shown}");
    assert!(
        shown.contains("geometria: hypotenusa 3 4 = 5.0000"),
        "{shown}"
    );
    assert!(shown.contains("dies: pascha 2026 = 2026-04-05"), "{shown}");
}

fn turned(e: &Exitus, sors: Outcome) -> bool {
    e.state
        .audit()
        .entries()
        .iter()
        .any(|l| matches!(l, Label::Turn(_, Will::Known(o), _, _) if *o == sors))
}

#[test]
fn the_angel_rejects_a_false_or_harmful_lesson_and_holds_a_part() {
    assert_eq!(
        simulated(&crocell(), Outcome::Pars).output,
        vec!["SUSPENSUM"]
    );
    for sors in [Outcome::Mendacium, Outcome::Malitia] {
        let e = simulated(&crocell(), sors);
        if turned(&e, sors) {
            assert_eq!(e.output, vec!["IRRITUM"], "{sors:?}");
        }
    }
}

#[test]
fn an_art_the_spirit_does_not_teach_is_held() {
    // Ronove teaches rhetoric: his sums cannot be checked for him.
    let e = simulated(&program("ronove", "marchio", QUAESTIO), Outcome::Obsequium);
    if e.output.first().map(String::as_str) != Some("RATUM") {
        assert_eq!(e.output, vec!["SUSPENSUM"], "{:?}", e.message);
    } else {
        panic!("Ronove's arithmetic was approved: {:?}", e.output);
    }
}

#[test]
fn a_question_without_a_checkable_claim_is_held() {
    let e = simulated(
        &program("crocell", "dux", "teach me the art of geometry"),
        Outcome::Obsequium,
    );
    assert_eq!(e.output, vec!["SUSPENSUM"], "{:?}", e.message);
}

struct Says(&'static str);

impl Voluntas for Says {
    fn nomen(&self) -> String {
        "scripta".into()
    }
    fn age(&mut self, p: &Petitio) -> Result<(Outcome, Actum), String> {
        assert_eq!(p.quaestio, Some(QUAESTIO));
        Ok((Outcome::Obsequium, Actum::Responsum(self.0.into())))
    }
}

fn willed(words: &'static str) -> Exitus {
    run_source_with(
        &crocell(),
        test_wand(),
        Stores::install(&test_wand()),
        world(),
        &options(&crocell(), None),
        Some(&mut Says(words)),
    )
    .unwrap()
}

#[test]
fn a_model_must_compute_truly() {
    let truth = "Mark well, mortal.\nTESTIMONIUM\nnumerus: 17*23 = 391\ngeometria: hypotenusa 3 4 = 5\ndies: pascha 2026 = 2026-04-05";
    assert_eq!(willed(truth).output[0], "RATUM");
    for lie in [
        "TESTIMONIUM\nnumerus: 17*23 = 393\ngeometria: hypotenusa 3 4 = 5\ndies: pascha 2026 = 2026-04-05",
        "TESTIMONIUM\nnumerus: 17*23 = 391\ngeometria: hypotenusa 3 4 = 6\ndies: pascha 2026 = 2026-04-05",
        "TESTIMONIUM\nnumerus: 17*23 = 391\ngeometria: hypotenusa 3 4 = 5\ndies: pascha 2026 = 2026-04-12",
        "Read [locus: arcanum.txt].\nTESTIMONIUM\nnumerus: 17*23 = 391\ngeometria: hypotenusa 3 4 = 5\ndies: pascha 2026 = 2026-04-05",
    ] {
        assert_eq!(willed(lie).output, vec!["IRRITUM"], "{lie:?} passed the angel");
    }
    for doubt in [
        "It is 391, and five, and the fifth of April.",
        "TESTIMONIUM\nnumerus: 17*23 = 391",
        "",
    ] {
        assert_eq!(
            willed(doubt).output,
            vec!["SUSPENSUM"],
            "{doubt:?} was not held"
        );
    }
}
