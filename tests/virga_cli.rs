//! The real wand through the command line: ratifying, running, sealing, the spare, a
//! revocation, and attacks on each. Wands are made with light work so the tests are quick;
//! the wand's own cryptography is tested in `virga`.

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};

use annulus::mandata::HOUSE;
use annulus::tabularium::{LEX, LINEAGE, ROOTS, SIGNUM};

const BINARY: &str = env!("CARGO_BIN_EXE_annulus");
const M: &str = "the master's own words";
const R: &str = "words for the rite alone";
const LEVIS: virga::Labor = virga::Labor {
    log_n: 4,
    r: 1,
    p: 1,
};

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
    VADE IN PACE vocatus;
}
"#;

const ANSWERS: &str = r#"[{"spiritus": 34, "officium": "RESPONSA", "quaestio": "venietne tempestas", "verum": "ita", "falsum": "non"}]"#;

struct Scene {
    dir: PathBuf,
    wand: PathBuf,
    spare: PathBuf,
}

fn scene(name: &str) -> Scene {
    let base = std::env::temp_dir().join(format!("annulus-virga-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&base);
    let (dir, wand, spare) = (base.join("opus"), base.join("E"), base.join("F"));
    for d in [&dir, &wand, &spare] {
        fs::create_dir_all(d).unwrap();
    }
    fs::write(dir.join("furfur.annulus"), FURFUR).unwrap();
    fs::write(dir.join("chronica.txt"), "annales").unwrap();
    fs::write(dir.join("responsa.json"), ANSWERS).unwrap();
    virga::fac(&wand, &spare, M, R, "test", LEVIS).unwrap();
    Scene { dir, wand, spare }
}

fn annulus(dir: &Path, args: &[&str], words: &[&str]) -> Output {
    annulus_env(dir, args, words, None)
}

fn annulus_env(dir: &Path, args: &[&str], words: &[&str], cade: Option<u32>) -> Output {
    let mut args: Vec<&str> = args.to_vec();
    if args.first() == Some(&"ritus") {
        args.extend(["--hora", "2024-04-13T14:30:00Z", "--locus", "41.9,12.5"]);
    }
    let mut child = Command::new(BINARY)
        .current_dir(dir)
        .args(&args)
        .env_remove("ANNULUS_VIRGA")
        .env(
            "ANNULUS_PROBATIO_CADE",
            cade.map(|n| n.to_string()).unwrap_or_default(),
        )
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the binary starts");
    let mut input = child.stdin.take().unwrap();
    for w in words {
        let _ = writeln!(input, "{w}");
    }
    drop(input);
    child.wait_with_output().unwrap()
}

fn err(o: &Output) -> String {
    String::from_utf8_lossy(&o.stderr).to_string()
}

fn sanci(s: &Scene, stick: &Path, words: &[&str]) -> Output {
    annulus(
        &s.dir,
        &["sanci", "--virga", stick.to_str().unwrap()],
        words,
    )
}

fn ritus(s: &Scene, stick: &Path, words: &[&str]) -> Output {
    annulus(
        &s.dir,
        &[
            "ritus",
            "furfur.annulus",
            "--virga",
            stick.to_str().unwrap(),
            "--responsa",
            "responsa.json",
            "--sors",
            "obsequium",
        ],
        words,
    )
}

#[test]
fn a_real_wand_ratifies_then_runs_and_seals() {
    let s = scene("run");
    let o = ritus(&s, &s.wand, &[M, R]);
    assert_eq!(o.status.code(), Some(2), "{}", err(&o));
    assert!(err(&o).contains("annulus sanci"), "{}", err(&o));

    let o = sanci(&s, &s.wand, &[M]);
    assert_eq!(o.status.code(), Some(0), "{}", err(&o));
    assert!(s.dir.join(HOUSE).join(LEX).exists());

    let o = ritus(&s, &s.wand, &[M, R]);
    assert_eq!(o.status.code(), Some(0), "{}", err(&o));
    assert_eq!(String::from_utf8_lossy(&o.stdout), "RATUM\n");
    let acta = fs::read_to_string(s.dir.join("furfur.acta")).unwrap();
    assert!(!acta.starts_with("# made with the test wand"));
    // the roots are signed by the rite key, which the master vouches for
    let v = virga::Virga::aperi(&s.wand).unwrap();
    let roots = fs::read_to_string(s.dir.join(HOUSE).join(ROOTS)).unwrap();
    let f: Vec<&str> = roots.lines().last().unwrap().split('\t').collect();
    assert_eq!(f[4], v.magister);
    assert_eq!(f[5], v.ritus);
    assert_eq!(f[6], v.vinculum);
    // the audit is signed the same way (core audit 1, #5)
    let signum: Vec<&str> = acta.lines().last().unwrap().split('\t').collect();
    assert_eq!(signum[0], SIGNUM);
    assert_eq!(signum[3], v.magister);
    assert_eq!(signum[4], v.ritus);
    assert_eq!(signum[5], v.vinculum);
    // the house keeps the lineage; the wand keeps the anchor
    assert!(s.dir.join(HOUSE).join(LINEAGE).exists());
    assert_eq!(fs::read_dir(v.ancorae()).unwrap().count(), 1);

    let o = ritus(&s, &s.wand, &[M, R]);
    assert_eq!(
        o.status.code(),
        Some(0),
        "a second run opens what the first sealed: {}",
        err(&o)
    );
}

#[test]
fn the_wrong_words_or_none_refuse() {
    let s = scene("words");
    assert_eq!(sanci(&s, &s.wand, &[R]).status.code(), Some(2));
    assert_eq!(sanci(&s, &s.wand, &[M]).status.code(), Some(0));
    let o = ritus(&s, &s.wand, &[R, M]);
    assert_eq!(o.status.code(), Some(2));
    assert!(err(&o).contains("do not open"), "{}", err(&o));
    let o = ritus(&s, &s.wand, &[M, "wrong rite words"]);
    assert_eq!(o.status.code(), Some(2));
    let o = ritus(&s, &s.wand, &[]);
    assert_eq!(o.status.code(), Some(2));
    assert!(err(&o).contains("no words"), "{}", err(&o));
    assert!(
        !s.dir.join(HOUSE).join(ROOTS).exists(),
        "nothing was sealed"
    );
}

#[test]
fn the_test_wand_is_refused_by_a_real_wands_house() {
    let s = scene("probatio");
    sanci(&s, &s.wand, &[M]);
    assert_eq!(ritus(&s, &s.wand, &[M, R]).status.code(), Some(0));
    let o = annulus(
        &s.dir,
        &[
            "ritus",
            "furfur.annulus",
            "--virga-probationis",
            "--responsa",
            "responsa.json",
        ],
        &[],
    );
    assert_eq!(o.status.code(), Some(2));
    assert!(err(&o).contains("test wand is refused"), "{}", err(&o));
}

#[test]
fn a_house_of_the_test_wand_is_not_taken_over_by_a_real_wand() {
    let s = scene("takeover");
    let o = annulus(
        &s.dir,
        &[
            "ritus",
            "furfur.annulus",
            "--virga-probationis",
            "--responsa",
            "responsa.json",
            "--sors",
            "obsequium",
        ],
        &[],
    );
    assert_eq!(o.status.code(), Some(0), "{}", err(&o));
    let o = ritus(&s, &s.wand, &[M, R]);
    assert_eq!(o.status.code(), Some(2));
    assert!(err(&o).contains("make a new house"), "{}", err(&o));
}

#[test]
fn a_wand_of_another_operator_is_refused() {
    let s = scene("stranger");
    sanci(&s, &s.wand, &[M]);
    assert_eq!(ritus(&s, &s.wand, &[M, R]).status.code(), Some(0));
    let other = s.dir.parent().unwrap().join("G");
    let other_spare = s.dir.parent().unwrap().join("H");
    fs::create_dir_all(&other).unwrap();
    fs::create_dir_all(&other_spare).unwrap();
    virga::fac(&other, &other_spare, M, R, "", LEVIS).unwrap();
    let o = ritus(&s, &other, &[M, R]);
    assert_eq!(o.status.code(), Some(2));
    assert!(err(&o).contains("another Operator"), "{}", err(&o));
}

#[test]
fn the_spare_works_the_same_house_and_a_rollback_is_caught() {
    let s = scene("spare");
    sanci(&s, &s.wand, &[M]);
    assert_eq!(ritus(&s, &s.wand, &[M, R]).status.code(), Some(0));
    let before = s.dir.parent().unwrap().join("old-house");
    copy_dir(&s.dir.join(HOUSE), &before);
    // the spare has never seen this house: it is refused until the Operator adopts it
    let o = ritus(&s, &s.spare, &[M, R]);
    assert_eq!(o.status.code(), Some(2));
    assert!(err(&o).contains("annulus adopta"), "{}", err(&o));
    let o = annulus(
        &s.dir,
        &["adopta", "--virga", s.spare.to_str().unwrap()],
        &[M],
    );
    assert_eq!(o.status.code(), Some(0), "{}", err(&o));
    let o = ritus(&s, &s.spare, &[M, R]);
    assert_eq!(
        o.status.code(),
        Some(0),
        "the spare is of the lineage: {}",
        err(&o)
    );
    assert_eq!(ritus(&s, &s.wand, &[M, R]).status.code(), Some(0));
    // put back the house as it was two seals ago
    fs::remove_dir_all(s.dir.join(HOUSE)).unwrap();
    copy_dir(&before, &s.dir.join(HOUSE));
    let o = ritus(&s, &s.wand, &[M, R]);
    assert_eq!(o.status.code(), Some(2));
    assert!(err(&o).contains("rolled back or replaced"), "{}", err(&o));
    // the house deleted outright
    fs::remove_dir_all(s.dir.join(HOUSE)).unwrap();
    let o = ritus(&s, &s.wand, &[M, R]);
    assert_eq!(o.status.code(), Some(2));
}

#[test]
fn a_revoked_wand_opens_nothing_and_the_spare_ratifies_again() {
    let s = scene("revoke");
    sanci(&s, &s.wand, &[M]);
    assert_eq!(ritus(&s, &s.wand, &[M, R]).status.code(), Some(0));
    let lost = virga::Virga::aperi(&s.wand).unwrap();
    let mut spare = virga::Virga::aperi(&s.spare).unwrap();
    spare.revoca(lost.signum(), M).unwrap();
    // what the lost wand sealed is no longer trusted until the Operator adopts it
    let o = ritus(&s, &s.spare, &[M, R]);
    assert_eq!(o.status.code(), Some(2));
    assert!(err(&o).contains("annulus adopta"), "{}", err(&o));
    assert!(fs::read_to_string(s.dir.join(HOUSE).join(LINEAGE))
        .unwrap()
        .contains("REVOCAT"));
    let o = annulus(
        &s.dir,
        &["adopta", "--virga", s.spare.to_str().unwrap()],
        &[M],
    );
    assert_eq!(o.status.code(), Some(0), "{}", err(&o));
    // a revocation changes the Constitution's digest: the old ratification no longer holds
    let o = ritus(&s, &s.spare, &[M, R]);
    assert_eq!(o.status.code(), Some(2));
    assert!(err(&o).contains("annulus sanci"), "{}", err(&o));
    assert_eq!(sanci(&s, &s.spare, &[M]).status.code(), Some(0));
    assert_eq!(ritus(&s, &s.spare, &[M, R]).status.code(), Some(0));
    // the lost wand learns of its revocation from the house, and is refused
    let o = ritus(&s, &s.wand, &[M, R]);
    assert_eq!(o.status.code(), Some(2));
    assert!(err(&o).contains("revoked"), "{}", err(&o));
    assert_eq!(
        virga::Virga::aperi(&s.wand).unwrap().status(),
        virga::Status::Revocata
    );
    let o = sanci(&s, &s.wand, &[M]);
    assert_eq!(o.status.code(), Some(2));
}

#[test]
fn a_forged_roots_record_is_refused() {
    let s = scene("forged");
    sanci(&s, &s.wand, &[M]);
    assert_eq!(ritus(&s, &s.wand, &[M, R]).status.code(), Some(0));
    let path = s.dir.join(HOUSE).join(ROOTS);
    let line = fs::read_to_string(&path).unwrap();
    let mut f: Vec<String> = line.trim_end().split('\t').map(String::from).collect();
    // a stranger's key, with a bond it made itself
    let stranger = virga::Clavis::ex_semine([5; 32]);
    f[5] = stranger.publica();
    f[6] = virga::vincula(&stranger, &f[5]);
    fs::write(&path, f.join("\t") + "\n").unwrap();
    let o = ritus(&s, &s.wand, &[M, R]);
    assert_eq!(o.status.code(), Some(2), "{}", err(&o));
}

#[test]
fn the_second_key_needs_the_master_words() {
    let s = scene("confirma");
    sanci(&s, &s.wand, &[M]);
    assert_eq!(ritus(&s, &s.wand, &[M, R]).status.code(), Some(0));
    let o = annulus(
        &s.dir,
        &["confirma", "00", "--virga", s.wand.to_str().unwrap()],
        &[R],
    );
    assert_eq!(o.status.code(), Some(2));
    assert!(err(&o).contains("do not open"), "{}", err(&o));
}

fn copy_dir(from: &Path, to: &Path) {
    fs::create_dir_all(to).unwrap();
    for e in fs::read_dir(from).unwrap().flatten() {
        let p = e.path();
        if p.is_file() {
            fs::copy(&p, to.join(e.file_name())).unwrap();
        }
    }
}

#[test]
fn an_old_signed_record_replayed_at_the_end_is_refused() {
    let s = scene("replay");
    sanci(&s, &s.wand, &[M]);
    assert_eq!(ritus(&s, &s.wand, &[M, R]).status.code(), Some(0));
    let first = s.dir.parent().unwrap().join("first-house");
    copy_dir(&s.dir.join(HOUSE), &first);
    assert_eq!(ritus(&s, &s.wand, &[M, R]).status.code(), Some(0));
    // the stores of the first run, with the first record appended after the second
    let roots = fs::read_to_string(s.dir.join(HOUSE).join(ROOTS)).unwrap();
    let line1 = roots.lines().next().unwrap().to_string();
    fs::remove_dir_all(s.dir.join(HOUSE)).unwrap();
    copy_dir(&first, &s.dir.join(HOUSE));
    fs::write(s.dir.join(HOUSE).join(ROOTS), format!("{roots}{line1}\n")).unwrap();
    let o = ritus(&s, &s.wand, &[M, R]);
    assert_eq!(o.status.code(), Some(2), "{}", err(&o));
}

#[test]
fn a_ratification_by_a_stranger_is_refused() {
    let s = scene("lex");
    sanci(&s, &s.wand, &[M]);
    assert_eq!(ritus(&s, &s.wand, &[M, R]).status.code(), Some(0));
    let stranger = virga::Clavis::ex_semine([6; 32]);
    let digest = annulus::nucleus::state::digest_constitution(&annulus::ritus::constitution(), &[]);
    let sig = annulus::baculum::signa(&stranger, &digest);
    fs::write(
        s.dir.join(HOUSE).join(LEX),
        format!("{}\t{sig}\n", stranger.publica()),
    )
    .unwrap();
    let o = ritus(&s, &s.wand, &[M, R]);
    assert_eq!(o.status.code(), Some(2));
    assert!(err(&o).contains("not ratified"), "{}", err(&o));
}

#[test]
fn a_removed_anchor_and_an_older_house_are_refused() {
    // audit V01: the wand saw a later state; its anchor is deleted and an older, validly
    // signed house put back
    let s = scene("noanchor");
    sanci(&s, &s.wand, &[M]);
    assert_eq!(ritus(&s, &s.wand, &[M, R]).status.code(), Some(0));
    let old = s.dir.parent().unwrap().join("old-house");
    copy_dir(&s.dir.join(HOUSE), &old);
    assert_eq!(ritus(&s, &s.wand, &[M, R]).status.code(), Some(0));
    let v = virga::Virga::aperi(&s.wand).unwrap();
    for e in fs::read_dir(v.ancorae()).unwrap().flatten() {
        fs::remove_file(e.path()).unwrap();
    }
    fs::remove_dir_all(s.dir.join(HOUSE)).unwrap();
    copy_dir(&old, &s.dir.join(HOUSE));
    let o = ritus(&s, &s.wand, &[M, R]);
    assert_eq!(o.status.code(), Some(2));
    assert!(err(&o).contains("no anchor"), "{}", err(&o));
}

#[test]
fn a_lineage_rolled_back_behind_the_anchor_is_refused() {
    let s = scene("stirps");
    sanci(&s, &s.wand, &[M]);
    assert_eq!(ritus(&s, &s.wand, &[M, R]).status.code(), Some(0));
    let before_house = fs::read_to_string(s.dir.join(HOUSE).join(LINEAGE)).unwrap();
    let before_stick = fs::read_to_string(s.wand.join("VIRGA").join("stirps.txt")).unwrap();
    // a new spare is made, and the wand sees the longer lineage
    let mut w = virga::Virga::aperi(&s.wand).unwrap();
    let g = s.dir.parent().unwrap().join("G");
    fs::create_dir_all(&g).unwrap();
    virga::adde(&mut w, &g, M, R, LEVIS).unwrap();
    assert_eq!(ritus(&s, &s.wand, &[M, R]).status.code(), Some(0));
    // both copies put back to the shorter lineage, which is a true, signed prefix
    fs::write(s.dir.join(HOUSE).join(LINEAGE), before_house).unwrap();
    fs::write(s.wand.join("VIRGA").join("stirps.txt"), before_stick).unwrap();
    let o = ritus(&s, &s.wand, &[M, R]);
    assert_eq!(o.status.code(), Some(2));
    assert!(err(&o).contains("lineage"), "{}", err(&o));
}

#[test]
fn an_interrupted_seal_is_undone_at_the_next_opening() {
    // audit V06: a seal stopped after the stores grew but before the roots were signed
    use annulus::tabularium::{INDEX, JOURNAL, KEYS, MARKS, VESSEL};
    let s = scene("journal");
    sanci(&s, &s.wand, &[M]);
    assert_eq!(ritus(&s, &s.wand, &[M, R]).status.code(), Some(0));
    let house = s.dir.join(HOUSE);
    let len = |n: &str| fs::metadata(house.join(n)).map(|m| m.len()).unwrap_or(0);
    let marks = fs::read_to_string(house.join(MARKS)).unwrap_or_default();
    let journal = format!(
        "{}\t{}\t{}\t{}\n{}\n",
        len(INDEX),
        len(VESSEL),
        len(KEYS),
        len(ROOTS),
        marks.replace('\n', "\\n")
    );
    fs::write(house.join(JOURNAL), journal).unwrap();
    let mut f = fs::OpenOptions::new()
        .append(true)
        .open(house.join(INDEX))
        .unwrap();
    writeln!(f, "half-written").unwrap();
    drop(f);
    let o = ritus(&s, &s.wand, &[M, R]);
    assert_eq!(o.status.code(), Some(0), "{}", err(&o));
    assert!(!house.join(JOURNAL).exists());
}

#[test]
fn a_journal_cannot_roll_the_house_back_past_the_anchor() {
    let s = scene("journal-attack");
    sanci(&s, &s.wand, &[M]);
    assert_eq!(ritus(&s, &s.wand, &[M, R]).status.code(), Some(0));
    assert_eq!(ritus(&s, &s.wand, &[M, R]).status.code(), Some(0));
    let house = s.dir.join(HOUSE);
    // a planted journal cutting every store back to nothing
    fs::write(house.join(annulus::tabularium::JOURNAL), "0\t0\t0\t0\n-\n").unwrap();
    let o = ritus(&s, &s.wand, &[M, R]);
    assert_eq!(o.status.code(), Some(2), "{}", err(&o));
}

#[test]
fn a_seal_cut_off_after_any_step_leaves_a_house_that_opens() {
    // kill after every write of a seal (outside reviews: crash recovery); the next run
    // must open the house and seal it whole
    for step in 1..=9 {
        let s = scene(&format!("cade-{step}"));
        sanci(&s, &s.wand, &[M]);
        assert_eq!(ritus(&s, &s.wand, &[M, R]).status.code(), Some(0));
        let args = [
            "ritus",
            "furfur.annulus",
            "--virga",
            s.wand.to_str().unwrap(),
            "--responsa",
            "responsa.json",
            "--sors",
            "obsequium",
        ];
        let o = annulus_env(&s.dir, &args, &[M, R], Some(step));
        assert_eq!(o.status.code(), Some(99), "step {step}: {}", err(&o));
        let o = ritus(&s, &s.wand, &[M, R]);
        assert_eq!(
            o.status.code(),
            Some(0),
            "after a cut at step {step}: {}",
            err(&o)
        );
        let o = ritus(&s, &s.wand, &[M, R]);
        assert_eq!(
            o.status.code(),
            Some(0),
            "and again, step {step}: {}",
            err(&o)
        );
    }
}
