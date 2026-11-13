//! A hold is believed only when the house keeps it (audit 2, #2): `MANE` in a sealed run
//! records it, licence to depart clears it, and `--tentus` is refused without it.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

const BINARY: &str = env!("CARGO_BIN_EXE_annulus");

fn furfur(end: &str) -> String {
    format!(
        r#"FACULTAS {{
    liber "chronica.txt" legere;
    atramentum "comes" 1;
}}
CIRCULUS RITU goetia {{
    INTENTIO "ask of the storm";
    SIT sigillum = INSCRIBE SIGILLUM furfur ATRAMENTO CONSECRA ATRAMENTUM "comes";
    SIT vocatus = INVOCA SPIRITUM 34 AD OFFICIUM responsa;
    SIT propositum = PETE vocatus("venietne tempestas", "chronica.txt");
    MANIFESTA EXAMINA propositum;
    {end} vocatus;
}}
"#
    )
}

const ANSWERS: &str = r#"[{"spiritus": 34, "officium": "RESPONSA", "quaestio": "venietne tempestas", "verum": "ita", "falsum": "non"}]"#;
const CALLED: &str = "2024-04-13T14:30:00Z";

fn folder(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("annulus-tentus-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    fs::write(dir.join("chronica.txt"), "annales").unwrap();
    fs::write(dir.join("responsa.json"), ANSWERS).unwrap();
    fs::write(dir.join("mane.annulus"), furfur("MANE")).unwrap();
    fs::write(dir.join("vade.annulus"), furfur("VADE IN PACE")).unwrap();
    dir
}

fn rite(dir: &Path, program: &str, hora: &str, tentus: Option<&str>) -> Output {
    let mut args = vec![
        "ritus",
        program,
        "--virga-probationis",
        "--responsa",
        "responsa.json",
        "--sors",
        "obsequium",
        "--hora",
        hora,
        "--locus",
        "41.9,12.5",
    ];
    if let Some(t) = tentus {
        args.extend(["--tentus", t]);
    }
    Command::new(BINARY)
        .current_dir(dir)
        .args(&args)
        .output()
        .expect("the binary starts")
}

fn said(o: &Output) -> String {
    String::from_utf8_lossy(&o.stderr).into_owned()
}

#[test]
fn a_hold_never_recorded_is_refused() {
    let dir = folder("never");
    let o = rite(
        &dir,
        "mane.annulus",
        "2024-04-15T14:30:00Z",
        Some("34@2024-04-13T14:30:00Z"),
    );
    assert_ne!(o.status.code(), Some(0));
    assert!(
        said(&o).contains("keeps no hold of spirit 34"),
        "{}",
        said(&o)
    );
}

#[test]
fn mane_records_the_hold_and_licence_clears_it() {
    let dir = folder("kept");
    let o = rite(&dir, "mane.annulus", CALLED, None);
    assert_eq!(o.status.code(), Some(0), "{}", said(&o));
    let marks = fs::read_to_string(dir.join("annulus.domus/annulus.signa")).unwrap();
    assert!(marks.contains("tentus:34:1713018600"), "{marks}");

    // held: asked again later under the hold, and kept from the same hour
    let held = format!("34@{CALLED}");
    let o = rite(&dir, "mane.annulus", "2024-04-13T18:00:00Z", Some(&held));
    assert_eq!(o.status.code(), Some(0), "{}", said(&o));
    let marks = fs::read_to_string(dir.join("annulus.domus/annulus.signa")).unwrap();
    assert_eq!(marks.matches("tentus:34:").count(), 1, "{marks}");
    assert!(marks.contains("tentus:34:1713018600"), "{marks}");

    // a hold of another hour is not this one
    let o = rite(
        &dir,
        "mane.annulus",
        "2024-04-13T18:00:00Z",
        Some("34@2024-04-12T14:30:00Z"),
    );
    assert!(said(&o).contains("keeps no hold"), "{}", said(&o));

    // licence to depart: the hold is gone
    let o = rite(&dir, "vade.annulus", "2024-04-13T18:30:00Z", Some(&held));
    assert_eq!(o.status.code(), Some(0), "{}", said(&o));
    let marks = fs::read_to_string(dir.join("annulus.domus/annulus.signa")).unwrap();
    assert!(!marks.contains("tentus:"), "{marks}");
    let o = rite(&dir, "mane.annulus", "2024-04-13T19:00:00Z", Some(&held));
    assert!(said(&o).contains("keeps no hold"), "{}", said(&o));
}
