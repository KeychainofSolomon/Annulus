use std::collections::BTreeSet;
use std::fs;
use std::path::PathBuf;

use annulus::nucleus::*;
use annulus::ritus::test_wand;
use annulus::tabularium::*;

fn house(name: &str) -> PathBuf {
    let d = std::env::temp_dir().join(format!("annulus-audit-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&d);
    fs::create_dir_all(&d).unwrap();
    d
}

fn drop_last_line(p: &PathBuf) {
    let t = fs::read_to_string(p).unwrap();
    let mut l: Vec<&str> = t.lines().collect();
    l.pop();
    fs::write(p, l.iter().map(|x| format!("{x}\n")).collect::<String>()).unwrap();
}

fn unspent_confirmed(s: &Stores, fp: &str) -> bool {
    let e = s.keys.entries();
    e.iter()
        .enumerate()
        .any(|(n, k)| *k == KeyEntry::Confirmed(fp.into()) && !e.contains(&KeyEntry::Spend(n)))
}

#[test]
fn a_rollback_cannot_revive_a_spent_second_key() {
    let w = test_wand();
    let dir = house("rollback");
    let d = Domus::new(&dir);
    let fp = "f00d".to_string();
    let none = BTreeSet::new();
    let mut o = d.open(&w).unwrap();
    o.stores.keys.push(KeyEntry::Petition(fp.clone()));
    d.seal(&o.stores, o.counts, &none, &w).unwrap();
    let mut o = d.open(&w).unwrap();
    assert!(o.stores.confirm_petition(&fp, &w));
    d.seal(&o.stores, o.counts, &none, &w).unwrap();
    let mut o = d.open(&w).unwrap();
    let n = o
        .stores
        .keys
        .entries()
        .iter()
        .position(|k| *k == KeyEntry::Confirmed(fp.clone()))
        .unwrap();
    o.stores.keys.push(KeyEntry::Spend(n));
    d.seal(&o.stores, o.counts, &none, &w).unwrap();
    assert!(!unspent_confirmed(&d.open(&w).unwrap().stores, &fp));
    drop_last_line(&dir.join(KEYS));
    drop_last_line(&dir.join(ROOTS));
    let e = d.open(&w).err().expect("a rollback is refused");
    assert!(e.contains("rolled back or replaced"), "{e}");
}

#[test]
fn a_deleted_house_is_refused_not_started_afresh() {
    let w = test_wand();
    let dir = house("wipe");
    let d = Domus::new(&dir);
    let mut o = d.open(&w).unwrap();
    o.stores.index.push(IxEntry::Damn("goetia:1:bael".into()));
    let marks: BTreeSet<String> = ["spirit.txt".to_string()].into();
    d.seal(&o.stores, o.counts, &marks, &w).unwrap();
    for f in [INDEX, VESSEL, KEYS, ROOTS, MARKS] {
        fs::remove_file(dir.join(f)).unwrap();
    }
    let e = d.open(&w).err().expect("a deleted house is refused");
    assert!(e.contains("refusing to start afresh"), "{e}");
}

#[test]
fn a_rolled_back_index_cannot_lift_a_damnation() {
    let w = test_wand();
    let dir = house("lift");
    let d = Domus::new(&dir);
    let o = d.open(&w).unwrap();
    d.seal(&o.stores, o.counts, &BTreeSet::new(), &w).unwrap();
    let mut o = d.open(&w).unwrap();
    o.stores.index.push(IxEntry::Damn("goetia:1:bael".into()));
    d.seal(&o.stores, o.counts, &BTreeSet::new(), &w).unwrap();
    drop_last_line(&dir.join(INDEX));
    drop_last_line(&dir.join(ROOTS));
    let e = d.open(&w).err().expect("refused");
    assert!(e.contains("rolled back or replaced"), "{e}");
}

#[test]
fn an_unchanged_house_opens_and_seals_across_runs() {
    let w = test_wand();
    let dir = house("steady");
    let d = Domus::new(&dir);
    for _ in 0..3 {
        let o = d.open(&w).unwrap();
        d.seal(&o.stores, o.counts, &BTreeSet::new(), &w).unwrap();
    }
    assert!(d.open(&w).unwrap().roots.is_some());
}

#[test]
fn a_petition_is_confirmed_once() {
    let w = test_wand();
    let mut s = Stores::install(&w);
    assert!(!s.confirm_petition("f00d", &w));
    s.keys.push(KeyEntry::Petition("f00d".into()));
    assert!(s.confirm_petition("f00d", &w));
    assert!(!s.confirm_petition("f00d", &w));
    s.keys.push(KeyEntry::Spend(1));
    assert!(!s.confirm_petition("f00d", &w));
    s.keys.push(KeyEntry::Petition("f00d".into()));
    assert!(s.confirm_petition("f00d", &w));
}

#[cfg(unix)]
#[test]
fn a_link_to_the_house_finds_the_same_anchor() {
    let w = test_wand();
    let dir = house("alias");
    let d = Domus::new(&dir);
    let o = d.open(&w).unwrap();
    d.seal(&o.stores, o.counts, &BTreeSet::new(), &w).unwrap();
    let mut o = d.open(&w).unwrap();
    o.stores.index.push(IxEntry::Damn("goetia:1:bael".into()));
    d.seal(&o.stores, o.counts, &BTreeSet::new(), &w).unwrap();
    drop_last_line(&dir.join(INDEX));
    drop_last_line(&dir.join(ROOTS));
    let alias = dir.with_extension("alias");
    let _ = fs::remove_file(&alias);
    std::os::unix::fs::symlink(&dir, &alias).unwrap();
    let e = Domus::new(&alias)
        .open(&w)
        .err()
        .expect("a rollback through a link is refused");
    assert!(e.contains("rolled back or replaced"), "{e}");
}

#[test]
fn a_house_without_its_anchor_is_refused_until_adopted() {
    let w = test_wand();
    let dir = house("adopt");
    let d = Domus::new(&dir);
    let o = d.open(&w).unwrap();
    d.seal(&o.stores, o.counts, &BTreeSet::new(), &w).unwrap();
    let elsewhere = std::env::temp_dir().join(format!("annulus-elsewhere-{}", std::process::id()));
    let _ = fs::remove_dir_all(&elsewhere);
    let moved = Domus::anchored(&dir, &elsewhere);
    let e = moved.open(&w).err().expect("refused");
    assert!(e.contains("no anchor"), "{e}");
    moved.adopt(&w).unwrap();
    assert!(moved.open(&w).is_ok());
    assert!(moved.adopt(&w).is_err());
}

#[test]
fn only_one_run_holds_the_house() {
    let dir = house("lock");
    let d = Domus::new(&dir);
    let first = d.lock().unwrap();
    let e = d.lock().expect_err("a second holder is refused");
    assert!(e.contains("another run holds this house"), "{e}");
    drop(first);
    assert!(d.lock().is_ok());
}
