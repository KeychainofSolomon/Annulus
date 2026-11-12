use std::collections::BTreeMap;

use annulus::horae::{parse_hora, Caelum};
use annulus::lingua::exsequi::{Exitus, Optiones};
use annulus::lingua::run_source;
use annulus::nucleus::*;
use annulus::ritus::test_wand;

fn rite(spirit: &str, ink: &str, office: &str) -> String {
    format!(
        "FACULTAS {{\n    liber \"chronica.txt\" legere;\n    atramentum \"{ink}\" 1;\n}}\nCIRCULUS RITU goetia {{\n    INTENTIO \"interroga\";\n    SIT s = INSCRIBE SIGILLUM {spirit} ATRAMENTO CONSECRA ATRAMENTUM \"{ink}\";\n    SIT v = INVOCA {spirit} AD OFFICIUM {office};\n    SIT p = PETE v(\"quid agitur\", \"chronica.txt\");\n    MANIFESTA EXAMINA p;\n    VADE IN PACE v;\n}}\n"
    )
}

fn at(source: &str, hora: &str, serenum: Option<bool>) -> Exitus {
    let world: BTreeMap<String, (String, Mark)> = [(
        "chronica.txt".to_string(),
        ("annales".to_string(), Mark::Outside),
    )]
    .into();
    let options = Optiones {
        sors: Some(Outcome::Obsequium),
        caelum: Caelum {
            hora: parse_hora(hora),
            locus: Some((41.9, 12.5)),
            serenum,
        },
        ..Default::default()
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

fn refused(e: &Exitus, why: &str) {
    assert_eq!(e.class, 2, "{:?} {:?}", e.output, e.message);
    let m = e.message.clone().unwrap_or_default();
    assert!(
        m.contains("the invocation is refused") && m.contains(why),
        "{m}"
    );
}

#[test]
fn a_duke_comes_in_his_hours_under_a_clear_sky() {
    let gusion = rite("gusion", "dux", "praesentia");
    let e = at(&gusion, "2024-04-13T09:00:00+02:00", Some(true));
    assert_eq!(
        (e.class, e.output.as_slice()),
        (0, &["RATUM".to_string()][..]),
        "{:?}",
        e.message
    );
    refused(
        &at(&gusion, "2024-04-13T09:00:00+02:00", Some(false)),
        "not clear",
    );
    refused(
        &at(&gusion, "2024-04-13T09:00:00+02:00", None),
        "undeclared",
    );
    refused(
        &at(&gusion, "2024-04-13T15:00:00+02:00", Some(true)),
        "not the hour of a duke",
    );
}

#[test]
fn nothing_comes_on_a_wrong_day_of_the_moon() {
    let gusion = rite("gusion", "dux", "praesentia");
    refused(
        &at(&gusion, "2024-04-12T09:00:00+02:00", Some(true)),
        "Moon is 3 days old",
    );
    let furfur = rite("furfur", "comes", "responsa");
    refused(
        &at(&furfur, "2024-04-20T12:00:00+02:00", None),
        "Moon is 11 days old",
    );
    assert_eq!(at(&furfur, "2024-04-13T03:00:00+02:00", None).class, 0);
}

#[test]
fn a_king_and_a_marquis_keep_their_hours() {
    let paimon = "FACULTAS {\n    liber \"chronica.txt\" legere;\n    atramentum \"rex\" 2;\n}\nCIRCULUS RITU goetia {\n    INTENTIO \"interroga\";\n    SIT s = INSCRIBE SIGILLUM paimon ATRAMENTO CONSECRA ATRAMENTUM \"rex\";\n    SIT v = INVOCA paimon AD OFFICIUM occulta;\n    OFFER v;\n    CONSTRINGE v PER NOMEN \"Adonai\";\n    SIT p = PETE v(\"quid latet\", \"chronica.txt\");\n    MANIFESTA EXAMINA p;\n    VADE IN PACE v;\n}\n".to_string();
    assert_eq!(at(&paimon, "2024-04-13T10:30:00+02:00", None).class, 0);
    refused(
        &at(&paimon, "2024-04-13T13:30:00+02:00", None),
        "not the hour of a king",
    );
    let amon = rite("amon", "marchio", "futura");
    assert_eq!(at(&amon, "2024-04-13T04:00:00+02:00", None).class, 0);
    refused(
        &at(&amon, "2024-04-13T10:00:00+02:00", None),
        "not the hour of a marquis",
    );
}

#[test]
fn without_a_place_only_an_earl_comes() {
    let world: BTreeMap<String, (String, Mark)> = [(
        "chronica.txt".to_string(),
        ("annales".to_string(), Mark::Outside),
    )]
    .into();
    for (src, ok) in [
        (rite("furfur", "comes", "responsa"), true),
        (rite("amon", "marchio", "futura"), false),
    ] {
        let options = Optiones {
            sors: Some(Outcome::Obsequium),
            caelum: Caelum {
                hora: parse_hora("2024-04-13T16:30:00+02:00"),
                locus: None,
                serenum: None,
            },
            ..Default::default()
        };
        let e = run_source(
            &src,
            test_wand(),
            Stores::install(&test_wand()),
            world.clone(),
            &options,
        )
        .unwrap();
        assert_eq!(e.class == 0, ok, "{:?}", e.message);
        if !ok {
            assert!(e.message.unwrap().contains("place is not declared"));
        }
    }
}
