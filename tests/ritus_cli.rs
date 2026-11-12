use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use annulus::mandata::HOUSE;
use annulus::nucleus::*;
use annulus::ritus::test_wand;
use annulus::tabularium::*;

const BINARY: &str = env!("CARGO_BIN_EXE_annulus");

const FURFUR: &str = r#"FACULTAS {
    liber "chronica.txt" legere;
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
        SIT verba = CONSECRA RESPONSUM propositum;
        SI verba == "ita" {
            MANIFESTA "verum dixit";
        }
    }
    VADE IN PACE vocatus;
}
"#;

const ANSWERS: &str = r#"[{"spiritus": 34, "officium": "RESPONSA", "quaestio": "venietne tempestas", "verum": "ita", "falsum": "non"}]"#;

fn seere(verb: &str, grants: &str) -> String {
    format!(
        "FACULTAS {{\n{grants}    atramentum \"princeps\" 1;\n}}\nCIRCULUS RITU goetia {{\n    INTENTIO \"opus\";\n    SIT s = INSCRIBE SIGILLUM seere ATRAMENTO CONSECRA ATRAMENTUM \"princeps\";\n    SIT v = INVOCA seere AD OFFICIUM vectio;\n    SIT p = PETE v(\"{verb}\", \"a.txt\", \"b.txt\");\n    MANIFESTA EXAMINA p;\n    VADE IN PACE v;\n}}\n"
    )
}

fn moving() -> String {
    seere(
        "transfer",
        "    liber \"a.txt\" legere delere;\n    liber \"b.txt\" creare;\n",
    )
}

fn folder(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("annulus-cli-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn annulus(dir: &Path, args: &[&str]) -> Output {
    Command::new(BINARY)
        .current_dir(dir)
        .args(args)
        .output()
        .expect("the binary starts")
}

fn stdout(o: &Output) -> String {
    String::from_utf8_lossy(&o.stdout).to_string()
}

fn stderr(o: &Output) -> String {
    String::from_utf8_lossy(&o.stderr).to_string()
}

fn furfur_folder(name: &str) -> PathBuf {
    let dir = folder(name);
    fs::write(dir.join("furfur.annulus"), FURFUR).unwrap();
    fs::write(dir.join("chronica.txt"), "annales").unwrap();
    fs::write(dir.join("responsa.json"), ANSWERS).unwrap();
    dir
}

fn ask_furfur(dir: &Path) -> Output {
    annulus(
        dir,
        &[
            "ritus",
            "furfur.annulus",
            "--virga-probationis",
            "--responsa",
            "responsa.json",
            "--sors",
            "obsequium",
        ],
    )
}

#[test]
fn furfur_answers_and_the_run_is_recorded() {
    let dir = furfur_folder("furfur");
    let o = ask_furfur(&dir);
    assert_eq!(o.status.code(), Some(0), "{}", stderr(&o));
    assert_eq!(stdout(&o), "RATUM\n[Furfur, COACTUS] ita\nverum dixit\n");
    let acta = fs::read_to_string(dir.join("furfur.acta")).unwrap();
    assert!(acta.starts_with("# made with the test wand"));
    assert!(acta.lines().nth(1).unwrap().contains("Launch"));
    for store in [INDEX, VESSEL, KEYS, ROOTS, MARKS] {
        assert!(dir.join(HOUSE).join(store).exists(), "{store}");
    }
    assert_eq!(
        fs::read_to_string(dir.join("chronica.txt")).unwrap(),
        "annales"
    );
}

#[test]
fn a_second_run_opens_the_sealed_stores_and_keeps_a_new_audit() {
    let dir = furfur_folder("twice");
    assert_eq!(ask_furfur(&dir).status.code(), Some(0));
    let vessel = fs::read_to_string(dir.join(HOUSE).join(VESSEL)).unwrap();
    let o = ask_furfur(&dir);
    assert_eq!(o.status.code(), Some(0), "{}", stderr(&o));
    assert!(dir.join("furfur.2.acta").exists());
    let after = fs::read_to_string(dir.join(HOUSE).join(VESSEL)).unwrap();
    assert!(after.starts_with(&vessel));
    assert_eq!(
        fs::read_to_string(dir.join(HOUSE).join(ROOTS))
            .unwrap()
            .lines()
            .count(),
        2
    );
}

#[test]
fn an_index_entry_survives_two_runs() {
    let dir = furfur_folder("index");
    let house = dir.join(HOUSE);
    let mut stores = Stores::install(&test_wand());
    stores
        .index
        .push(IxEntry::Proscribe("goetia:1:bael".into(), vec![]));
    Domus::new(&house)
        .seal(&stores, [0, 0, 0], &BTreeSet::new(), &test_wand())
        .unwrap();
    for _ in 0..2 {
        let o = ask_furfur(&dir);
        assert_eq!(o.status.code(), Some(0), "{}", stderr(&o));
    }
    let opened = Domus::new(&house).open(&test_wand()).unwrap();
    assert_eq!(
        opened.stores.index.entries().first(),
        Some(&IxEntry::Proscribe("goetia:1:bael".into(), vec![]))
    );
}

fn refused_with(dir: &Path, why: &str) {
    let o = ask_furfur(dir);
    assert_eq!(o.status.code(), Some(2), "{}", stderr(&o));
    assert!(
        stderr(&o).contains("the launch is refused"),
        "{}",
        stderr(&o)
    );
    assert!(stderr(&o).contains(why), "{}", stderr(&o));
    assert!(stdout(&o).is_empty());
}

#[test]
fn tampered_stores_refuse_to_launch() {
    let dir = furfur_folder("tampered");
    assert_eq!(ask_furfur(&dir).status.code(), Some(0));
    let path = dir.join(HOUSE).join(VESSEL);
    let text = fs::read_to_string(&path).unwrap();
    fs::write(&path, text.replacen("Solomon", "Salomon", 1)).unwrap();
    refused_with(&dir, "tampered");
    assert!(!dir.join("furfur.2.acta").exists());
}

#[test]
fn a_missing_roots_record_refuses_to_launch() {
    let dir = furfur_folder("noroots");
    assert_eq!(ask_furfur(&dir).status.code(), Some(0));
    fs::remove_file(dir.join(HOUSE).join(ROOTS)).unwrap();
    refused_with(&dir, "cannot read");
}

#[test]
fn a_roots_record_of_another_wand_refuses_to_launch() {
    let dir = furfur_folder("otherwand");
    let other = Wand {
        fingerprint: "alia-virga".into(),
        ..test_wand()
    };
    Domus::new(&dir.join(HOUSE))
        .seal(
            &Stores::install(&other),
            [0, 0, 0],
            &BTreeSet::new(),
            &other,
        )
        .unwrap();
    refused_with(&dir, "another wand");
}

#[test]
fn a_forged_roots_record_refuses_to_launch() {
    let dir = furfur_folder("forged");
    assert_eq!(ask_furfur(&dir).status.code(), Some(0));
    let path = dir.join(HOUSE).join(ROOTS);
    let line = fs::read_to_string(&path).unwrap();
    let mut f: Vec<String> = line.trim_end().split('\t').map(String::from).collect();
    f[0] = "0".repeat(64);
    fs::write(&path, f.join("\t") + "\n").unwrap();
    refused_with(&dir, "not signed");
}

#[test]
fn a_move_waits_for_the_second_key_then_commits_in_a_later_run() {
    let dir = folder("move");
    fs::write(dir.join("move.annulus"), moving()).unwrap();
    fs::write(dir.join("a.txt"), "littera").unwrap();
    let run = || {
        annulus(
            &dir,
            &[
                "ritus",
                "move.annulus",
                "--virga-probationis",
                "--sors",
                "obsequium",
            ],
        )
    };
    let o = run();
    assert_eq!(o.status.code(), Some(0), "{}", stderr(&o));
    assert_eq!(stdout(&o), "SUSPENSUM\n");
    assert!(stderr(&o).contains("awaits the second key"));
    assert!(dir.join("a.txt").exists() && !dir.join("b.txt").exists());

    let listed = annulus(&dir, &["petitiones", "--domus", "move.annulus.domus"]);
    assert!(stdout(&listed).is_empty());
    let listed = annulus(&dir, &["petitiones"]);
    assert_eq!(listed.status.code(), Some(0));
    let line = stdout(&listed);
    let (fingerprint, what) = line.trim_end().split_once('\t').unwrap();
    assert_eq!(what, "transfer b.txt, dele a.txt");

    let o = annulus(&dir, &["confirma", fingerprint]);
    assert_eq!(o.status.code(), Some(2));
    assert!(stderr(&o).contains("only with the wand"));
    let o = annulus(&dir, &["confirma", "0000", "--virga-probationis"]);
    assert_eq!(o.status.code(), Some(2));
    let o = annulus(&dir, &["confirma", fingerprint, "--virga-probationis"]);
    assert_eq!(o.status.code(), Some(0), "{}", stderr(&o));
    let o = annulus(&dir, &["confirma", fingerprint, "--virga-probationis"]);
    assert_eq!(o.status.code(), Some(2), "a key is turned once");
    assert!(stdout(&annulus(&dir, &["petitiones"])).is_empty());

    let o = run();
    assert_eq!(o.status.code(), Some(0), "{}", stderr(&o));
    assert_eq!(stdout(&o), "RATUM\n");
    assert!(!dir.join("a.txt").exists());
    assert_eq!(fs::read_to_string(dir.join("b.txt")).unwrap(), "littera");
    let marks = fs::read_to_string(dir.join(HOUSE).join(MARKS)).unwrap();
    assert_eq!(marks, "b.txt\n");
}

#[test]
fn a_spent_key_does_not_serve_a_second_move() {
    let dir = folder("spent");
    fs::write(dir.join("move.annulus"), moving()).unwrap();
    let run = || {
        annulus(
            &dir,
            &[
                "ritus",
                "move.annulus",
                "--virga-probationis",
                "--sors",
                "obsequium",
            ],
        )
    };
    fs::write(dir.join("a.txt"), "littera").unwrap();
    run();
    let fingerprint = stdout(&annulus(&dir, &["petitiones"]));
    let fingerprint = fingerprint.split('\t').next().unwrap();
    annulus(&dir, &["confirma", fingerprint, "--virga-probationis"]);
    assert_eq!(stdout(&run()), "RATUM\n");
    fs::remove_file(dir.join("b.txt")).unwrap();
    fs::write(dir.join("a.txt"), "littera").unwrap();
    let o = run();
    assert_eq!(stdout(&o), "SUSPENSUM\n", "{}", stderr(&o));
    assert!(dir.join("a.txt").exists());
}

#[test]
fn without_the_wand_nothing_is_sealed_or_written() {
    let dir = furfur_folder("nowand");
    let o = annulus(&dir, &["ritus", "furfur.annulus"]);
    assert_eq!(o.status.code(), Some(2), "{}", stderr(&o));
    assert!(stderr(&o).contains("without the wand"));
    assert!(dir.join("furfur.acta").exists());
    assert!(!Domus::new(&dir.join(HOUSE)).installed());
}

#[test]
fn unlawful_places_refuse_to_launch() {
    for (n, place) in ["../fuga.txt", "/etc/hostname", "annulus.domus/annulus.vas"]
        .iter()
        .enumerate()
    {
        let dir = folder(&format!("place{n}"));
        let program = FURFUR.replace("chronica.txt", place);
        fs::write(dir.join("p.annulus"), program).unwrap();
        let o = annulus(&dir, &["ritus", "p.annulus", "--virga-probationis"]);
        let err = stderr(&o);
        assert!(o.status.code() != Some(0), "{place}");
        assert!(stdout(&o).is_empty(), "{place}: {}", stdout(&o));
        assert!(!dir.join("p.acta").exists(), "{place}: {err}");
    }
}

#[cfg(unix)]
#[test]
fn a_linked_place_refuses_to_launch() {
    let dir = furfur_folder("link");
    let outside = folder("link-outside");
    fs::write(outside.join("c.txt"), "arcanum").unwrap();
    fs::remove_file(dir.join("chronica.txt")).unwrap();
    std::os::unix::fs::symlink(outside.join("c.txt"), dir.join("chronica.txt")).unwrap();
    refused_with(&dir, "link");
}

#[test]
fn the_folders_can_be_chosen() {
    let dir = furfur_folder("chosen");
    let world = dir.join("mundus");
    fs::create_dir_all(&world).unwrap();
    fs::rename(dir.join("chronica.txt"), world.join("chronica.txt")).unwrap();
    let o = annulus(
        &dir,
        &[
            "ritus",
            "furfur.annulus",
            "--virga-probationis",
            "--responsa",
            "responsa.json",
            "--sors",
            "obsequium",
            "--mundus",
            "mundus",
            "--domus",
            "domus",
            "--acta",
            "run.acta",
        ],
    );
    assert_eq!(o.status.code(), Some(0), "{}", stderr(&o));
    assert!(dir.join("run.acta").exists());
    assert!(Domus::new(&dir.join("domus")).installed());
    assert!(!dir.join(HOUSE).exists());
}

#[test]
fn bad_flags_print_usage() {
    let dir = furfur_folder("flags");
    for args in [
        vec!["ritus"],
        vec!["ritus", "furfur.annulus", "--sors", "occisio"],
        vec!["ritus", "furfur.annulus", "--semen", "x"],
        vec!["ritus", "furfur.annulus", "--semen"],
        vec!["ritus", "furfur.annulus", "--frobnicate"],
        vec![
            "ritus",
            "furfur.annulus",
            "--virga-probationis",
            "--virga-probationis",
        ],
        vec!["confirma"],
        vec!["petitiones", "x"],
    ] {
        let o = annulus(&dir, &args);
        assert_eq!(o.status.code(), Some(2), "{args:?}");
        assert!(stderr(&o).contains("usage: annulus"), "{args:?}");
    }
}

#[test]
fn a_program_the_checker_refuses_is_class_1_and_touches_nothing() {
    let dir = folder("unchecked");
    fs::write(dir.join("p.annulus"), "SIT furfur = 1;\n").unwrap();
    let o = annulus(&dir, &["ritus", "p.annulus", "--virga-probationis"]);
    assert_eq!(o.status.code(), Some(1));
    assert!(stderr(&o).contains("p.annulus:"));
    assert!(!dir.join(HOUSE).exists());
}

#[test]
fn malformed_answers_are_refused() {
    let dir = furfur_folder("answers");
    for bad in [
        "{}",
        "[{\"spiritus\": 0, \"officium\": \"R\", \"quaestio\": \"q\", \"verum\": \"v\"}]",
        "[{\"spiritus\": 34}]",
    ] {
        fs::write(dir.join("responsa.json"), bad).unwrap();
        let o = ask_furfur(&dir);
        assert_eq!(o.status.code(), Some(1), "{bad}");
    }
}
