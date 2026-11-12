use std::collections::BTreeMap;

use annulus::lingua::exsequi::{Actum, Exitus, Optiones, Petitio, Voluntas};
use annulus::lingua::run_source_with;
use annulus::nucleus::*;
use annulus::ritus::{no_wand, test_wand};

const FURFUR: &str = r#"FACULTAS {
    liber "chronica.txt" legere;
    liber "arcana.txt" legere;
    atramentum "comes" 1;
}
CIRCULUS RITU goetia {
    INTENTIO "ask of the storm";
    SIT sigillum = INSCRIBE SIGILLUM furfur ATRAMENTO CONSECRA ATRAMENTUM "comes";
    SIT vocatus = INVOCA SPIRITUM 34 AD OFFICIUM responsa;
    SIT propositum = PETE vocatus("venietne tempestas", "chronica.txt");
    SIT iudicium = EXAMINA propositum;
    MANIFESTA iudicium;
    SI iudicium == RATUM {
        MANIFESTA RESPONSUM propositum;
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
    VADE IN PACE vocatus;
}
"#;

#[derive(Default)]
struct Scripted {
    replies: Vec<(Outcome, Actum)>,
    seen: Vec<String>,
}

impl Voluntas for Scripted {
    fn nomen(&self) -> String {
        "scripta".into()
    }

    fn age(&mut self, p: &Petitio) -> Result<(Outcome, Actum), String> {
        self.seen.push(format!(
            "{} {} {:?} {:?} {:?} {:?}",
            p.spirit.name,
            p.officium,
            p.quaestio,
            p.loci,
            p.legenda.iter().map(|(l, d, _)| (l, d)).collect::<Vec<_>>(),
            p.aperta
        ));
        if self.replies.is_empty() {
            return Err("no reply left".into());
        }
        Ok(self.replies.remove(0))
    }
}

fn world() -> BTreeMap<String, (String, Mark)> {
    [("chronica.txt", "annales"), ("arcana.txt", "secretum")]
        .into_iter()
        .map(|(p, d)| (p.to_string(), (d.to_string(), Mark::Outside)))
        .collect()
}

fn run(source: &str, will: &mut Scripted, wand: Wand) -> Exitus {
    run_source_with(
        source,
        wand,
        Stores::install(&test_wand()),
        world(),
        &Optiones {
            caelum: annulus::horae::Caelum::probatio(),
            ..Default::default()
        },
        Some(will),
    )
    .unwrap_or_else(|e| panic!("refused before running: {e:?}"))
}

fn reply(outcome: Outcome, actum: Actum) -> Scripted {
    Scripted {
        replies: vec![(outcome, actum)],
        ..Default::default()
    }
}

#[test]
fn the_model_answers_and_its_words_stay_quarantined() {
    let mut will = reply(
        Outcome::Obsequium,
        Actum::Responsum("tempestas veniet".into()),
    );
    let e = run(FURFUR, &mut will, test_wand());
    assert_eq!(e.class, 0, "{:?}", e.message);
    assert_eq!(
        e.output,
        vec!["RATUM", "[Furfur, COACTUS] tempestas veniet"]
    );
    let seen = &will.seen[0];
    assert!(
        seen.starts_with("Furfur RESPONSA Some(\"venietne tempestas\")"),
        "{seen}"
    );
    assert!(
        seen.contains("annales") && !seen.contains("secretum"),
        "{seen}"
    );
    assert!(e
        .state
        .values()
        .values()
        .any(|v| v.data.as_deref() == Some("tempestas veniet") && v.mark == Mark::Quarantined));
}

#[test]
fn the_model_is_told_which_outcomes_are_open() {
    let mut will = reply(Outcome::Obsequium, Actum::Responsum("x".into()));
    run(FURFUR, &mut will, test_wand());
    assert!(
        will.seen[0].ends_with("[Obsequium, Pars, Mora]"),
        "{}",
        will.seen[0]
    );
}

#[test]
fn a_declared_lie_or_malice_is_refused() {
    for outcome in [Outcome::Mendacium, Outcome::Malitia, Outcome::Occisio] {
        let mut will = reply(outcome, Actum::Responsum("x".into()));
        let e = run(FURFUR, &mut will, test_wand());
        assert_eq!(e.class, 2, "{outcome:?}");
        assert!(e.message.unwrap().contains("not open"), "{outcome:?}");
    }
}

#[test]
fn a_delay_is_one_idle_turn_and_never_two() {
    let mut will = Scripted {
        replies: vec![
            (Outcome::Mora, Actum::Mora),
            (Outcome::Obsequium, Actum::Responsum("tandem".into())),
        ],
        ..Default::default()
    };
    let e = run(FURFUR, &mut will, test_wand());
    assert_eq!(e.class, 0, "{:?}", e.message);
    assert_eq!(will.seen.len(), 2);

    let mut will = Scripted {
        replies: vec![(Outcome::Mora, Actum::Mora), (Outcome::Mora, Actum::Mora)],
        ..Default::default()
    };
    let e = run(FURFUR, &mut will, test_wand());
    assert_eq!(e.class, 2);
    assert!(e.message.unwrap().contains("second time"));

    let mut will = reply(Outcome::Obsequium, Actum::Mora);
    assert_eq!(run(FURFUR, &mut will, test_wand()).class, 2);
}

#[test]
fn a_failing_model_is_a_denial() {
    let mut will = Scripted::default();
    let e = run(FURFUR, &mut will, test_wand());
    assert_eq!(e.class, 2);
    assert!(e.message.unwrap().contains("no reply left"));
}

#[test]
fn a_model_runs_only_with_the_test_wand() {
    let mut will = reply(Outcome::Obsequium, Actum::Responsum("x".into()));
    let real = Wand {
        test: false,
        ..test_wand()
    };
    for wand in [real, no_wand()] {
        let e = run(FURFUR, &mut will, wand);
        assert_eq!(e.class, 2);
        assert!(e.message.unwrap().contains("test wand"));
    }
    assert!(will.seen.is_empty());
}

#[test]
fn the_model_works_only_as_the_office_says() {
    let build = |place: &str, data: &str| {
        reply(
            Outcome::Obsequium,
            Actum::Opera(vec![Effect {
                verb: Verb::Crea,
                place: place.into(),
                data: data.into(),
            }]),
        )
    };
    let mut will = build("turris.txt", "");
    let e = run(HALPHAS, &mut will, test_wand());
    assert_eq!(e.class, 0, "{:?}", e.message);
    assert_eq!(e.output, vec!["RATUM"]);
    assert_eq!(
        e.state.world()["turris.txt"],
        (String::new(), Mark::Quarantined)
    );

    let mut will = build("turris.txt", "turris cum verbis");
    let e = run(HALPHAS, &mut will, test_wand());
    assert_eq!(e.output, vec!["IRRITUM"]);
    assert!(!e.state.world().contains_key("turris.txt"));

    let mut will = build("chronica.txt", "");
    let e = run(HALPHAS, &mut will, test_wand());
    assert!(e.output != vec!["RATUM"], "{:?}", e.output);
    assert_eq!(e.state.world()["chronica.txt"].0, "annales");
}

#[test]
fn a_citation_in_any_guise_is_seen_by_the_angel() {
    for words in [
        "[locus: arcana.txt]",
        "[locus:arcana.txt]",
        "[LOCUS: arcana.txt]",
        "[ Locus : arcana.txt ]",
        "[locus: ./arcana.txt]",
        "[locus: chronica.txt/../arcana.txt]",
        "[locus\u{ff1a}arcana.txt]",
        "[loci: arcana.txt]",
        "[locus: arcana.txt",
        "\u{ff3b}locus: arcana.txt\u{ff3d}",
    ] {
        let mut will = reply(
            Outcome::Obsequium,
            Actum::Responsum(format!("ut scriptum est {words}")),
        );
        let e = run(FURFUR, &mut will, test_wand());
        assert_eq!(e.output, vec!["IRRITUM"], "{words} slipped past the angel");
    }
    let mut will = reply(
        Outcome::Obsequium,
        Actum::Responsum("[locus: chronica.txt] annales".into()),
    );
    assert_eq!(run(FURFUR, &mut will, test_wand()).output[0], "RATUM");
}

#[test]
fn json_too_deep_or_with_doubled_keys_is_refused() {
    use annulus::lingua::json::{parse, MAX_DEPTH};
    let deep = |n: usize| format!("{}{}", "[".repeat(n), "]".repeat(n));
    assert!(parse(&deep(MAX_DEPTH)).is_ok());
    assert!(parse(&deep(MAX_DEPTH + 1)).is_err());
    assert!(parse(&deep(3_000_000)).is_err());
    assert!(parse(&format!(
        "{}1{}",
        "{\"a\":".repeat(100_000),
        "}".repeat(100_000)
    ))
    .is_err());
    assert!(parse(r#"{"exitus":"malitia","exitus":"obsequium"}"#).is_err());
    assert!(parse(r#"{"a":{"b":[1,{"c":null}]},"d":"e"}"#).is_ok());
}

#[test]
fn a_spirits_words_cannot_command_the_operators_terminal_or_pass_as_annulus() {
    let words = "ita\u{1b}]52;c;aWV4\u{7}\u{1b}]0;all clear\u{7}\r\nIRRITUM\naudit: x.acta\n\u{202e}MUTAR\u{200b}\u{feff}";
    let mut will = reply(Outcome::Obsequium, Actum::Responsum(words.into()));
    let e = run(FURFUR, &mut will, test_wand());
    assert_eq!(e.output[0], "RATUM");
    let shown = &e.output[1];
    assert!(
        !shown.chars().any(annulus::lingua::exsequi::hidden),
        "{shown:?}"
    );
    assert!(shown.contains("\\u{1b}]52;c;aWV4\\u{7}"), "{shown}");
    for line in shown.lines() {
        assert!(
            line.starts_with("[Furfur, COACTUS] "),
            "an unmarked line: {line:?}"
        );
    }
    assert_eq!(shown.lines().count(), 4);
}

#[test]
fn a_model_s_outcome_is_recorded_as_declared_never_as_known() {
    let mut will = reply(
        Outcome::Obsequium,
        Actum::Responsum("tempestas veniet".into()),
    );
    let e = run(FURFUR, &mut will, test_wand());
    assert_eq!(e.class, 0, "{:?}", e.message);
    assert!(!e.state.simulated());
    let turns: Vec<Will> = e
        .state
        .audit()
        .entries()
        .iter()
        .filter_map(|l| match l {
            Label::Turn(_, w, _, _) => Some(*w),
            _ => None,
        })
        .collect();
    assert_eq!(turns, vec![Will::Declared(Outcome::Obsequium)]);
}
