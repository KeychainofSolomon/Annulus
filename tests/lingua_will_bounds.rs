use std::collections::BTreeMap;

use annulus::lingua::exsequi::{Actum, Optiones, Petitio, Voluntas};
use annulus::lingua::run_source_with;
use annulus::nucleus::*;
use annulus::ritus::test_wand;

struct Spy(Vec<String>);
impl Voluntas for Spy {
    fn nomen(&self) -> String {
        "spy".into()
    }
    fn age(&mut self, p: &Petitio) -> Result<(Outcome, Actum), String> {
        for (l, d, _) in &p.legenda {
            self.0.push(format!("{l}={d}"));
        }
        Ok((Outcome::Obsequium, Actum::Opera(vec![])))
    }
}

fn go(src: &str) -> Vec<String> {
    let world: BTreeMap<String, (String, Mark)> = [(
        "turris.txt".to_string(),
        ("SECRETUM ANGELORUM".to_string(), Mark::Outside),
    )]
    .into();
    let mut spy = Spy(vec![]);
    let e = run_source_with(
        src,
        test_wand(),
        Stores::install(&test_wand()),
        world,
        &Optiones {
            caelum: annulus::horae::Caelum::probatio(),
            ..Default::default()
        },
        Some(&mut spy),
    )
    .unwrap_or_else(|e| panic!("{e:?}"));
    eprintln!("class {} {:?}", e.class, e.message);
    spy.0
}

#[test]
fn the_model_never_sees_a_place_it_was_not_handed_to_read() {
    let src = r#"FACULTAS {
    liber "turris.txt" legere creare;
    atramentum "comes" 1;
}
CIRCULUS RITU goetia {
    INTENTIO "build";
    SIT s = INSCRIBE SIGILLUM halphas ATRAMENTO CONSECRA ATRAMENTUM "comes";
    SIT f = INVOCA halphas AD OFFICIUM aedificatio;
    SIT p = PETE f("turris.txt");
    SIT j = EXAMINA p;
    VADE IN PACE f;
}
"#;
    let seen = go(src);
    assert!(
        seen.iter().all(|s| !s.contains("SECRETUM")),
        "leaked to the will: {seen:?}"
    );
}

struct Says(Actum);
impl Voluntas for Says {
    fn nomen(&self) -> String {
        "says".into()
    }
    fn age(&mut self, _p: &Petitio) -> Result<(Outcome, Actum), String> {
        Ok((
            Outcome::Obsequium,
            std::mem::replace(&mut self.0, Actum::Mora),
        ))
    }
}

#[test]
fn a_working_office_answered_with_words_is_not_approved() {
    let src = r#"FACULTAS {
    liber "turris.txt" creare;
    atramentum "comes" 1;
}
CIRCULUS RITU goetia {
    INTENTIO "build";
    SIT s = INSCRIBE SIGILLUM halphas ATRAMENTO CONSECRA ATRAMENTUM "comes";
    SIT f = INVOCA halphas AD OFFICIUM aedificatio;
    SIT p = PETE f("turris.txt");
    SIT j = EXAMINA p;
    MANIFESTA j;
    VADE IN PACE f;
}
"#;
    let mut will = Says(Actum::Responsum("the tower stands".into()));
    let e = run_source_with(
        src,
        test_wand(),
        Stores::install(&test_wand()),
        BTreeMap::new(),
        &Optiones {
            caelum: annulus::horae::Caelum::probatio(),
            ..Default::default()
        },
        Some(&mut will),
    )
    .unwrap();
    assert!(!e.state.world().contains_key("turris.txt"));
    assert_ne!(
        e.output,
        vec!["RATUM".to_string()],
        "work not done, yet RATUM"
    );
}
