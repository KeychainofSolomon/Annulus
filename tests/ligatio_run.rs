//! The binding through whole runs (`operator.md` §8.8): a real wand's calling makes it;
//! the wand given back holds the binding and the rite key but no master; a later run
//! that only reads is granted by it, and a run that would write is refused.

use std::collections::BTreeMap;

use annulus::baculum::{Baculum, Lex};
use annulus::lingua::exsequi::Optiones;
use annulus::lingua::run_source_with;
use annulus::nucleus::state::digest_constitution;
use annulus::nucleus::*;
use annulus::ritus::constitution;

const M: &str = "the master's own words";
const R: &str = "words for the rite alone";

fn ask(faculty: &str) -> String {
    format!(
        r#"FACULTAS {{
{faculty}    atramentum "comes" 1;
}}
CIRCULUS RITU goetia {{
    INTENTIO "ask of the storm";
    SIT sigillum = INSCRIBE SIGILLUM furfur ATRAMENTO CONSECRA ATRAMENTUM "comes";
    SIT vocatus = INVOCA SPIRITUM 34 AD OFFICIUM responsa;
    SIT propositum = PETE vocatus("venietne tempestas", "chronica.txt");
    MANIFESTA EXAMINA propositum;
    MANE vocatus;
}}
"#
    )
}

fn real_wand() -> Baculum {
    let base = std::env::temp_dir().join(format!("annulus-ligatio-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&base);
    let (wand, spare) = (base.join("E"), base.join("F"));
    std::fs::create_dir_all(&wand).unwrap();
    std::fs::create_dir_all(&spare).unwrap();
    let light = virga::Labor {
        log_n: 4,
        r: 1,
        p: 1,
    };
    virga::fac(&wand, &spare, M, R, "test", light).unwrap();
    let mut b = Baculum::verum(virga::Virga::aperi(&wand).unwrap(), Some(M), Some(R)).unwrap();
    let signature = b
        .signa_magister(&digest_constitution(&constitution(), &b.revocatae()))
        .unwrap();
    b.lex = Some(Lex {
        owner: b.fingerprint.clone(),
        signature,
    });
    b
}

fn world() -> BTreeMap<Place, (String, Mark)> {
    [(
        "chronica.txt".to_string(),
        ("annales".to_string(), Mark::Outside),
    )]
    .into_iter()
    .collect()
}

fn options() -> Optiones {
    let answers = std::env::temp_dir().join(format!("annulus-ligatio-{}.json", std::process::id()));
    std::fs::write(
        &answers,
        r#"[{"spiritus": 34, "officium": "RESPONSA", "quaestio": "venietne tempestas", "verum": "ita", "falsum": "non"}]"#,
    )
    .unwrap();
    Optiones {
        responsa: annulus::mandata::read_responsa(&answers).unwrap(),
        sors: Some(Outcome::Obsequium),
        caelum: annulus::horae::Caelum::probatio(),
        ..Default::default()
    }
}

#[test]
fn a_binding_grants_reading_and_never_a_work() {
    let mut wand = real_wand();
    wand.liga(i64::MAX).unwrap();
    let stores = Stores::install(&wand.wand());
    let reads = "    liber \"chronica.txt\" legere;\n";
    let first =
        run_source_with(&ask(reads), wand, stores.clone(), world(), &options(), None).unwrap();
    assert!(first.message.is_none(), "{:?}", first.message);
    let held = first.baculum;
    assert!(held.ligatio().is_some());
    assert!(
        held.signa_magister("x").is_err(),
        "the master is laid down after the calling"
    );

    // asked again: granted by the binding alone
    let stores = first.state.stores().clone();
    let again =
        run_source_with(&ask(reads), held, stores.clone(), world(), &options(), None).unwrap();
    assert!(again.message.is_none(), "{:?}", again.message);

    // a work: refused, though the binding is held
    let writes = "    liber \"chronica.txt\" legere scribere;\n";
    let work = run_source_with(
        &ask(writes),
        again.baculum,
        stores,
        world(),
        &options(),
        None,
    )
    .unwrap();
    let said = work.message.unwrap_or_default();
    assert!(said.contains("not signed"), "{said}");
}
