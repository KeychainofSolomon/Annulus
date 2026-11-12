use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};

use annulus::nucleus::*;
use annulus::ritus::{no_wand, test_wand};
use annulus::tabularium::*;

fn house(name: &str) -> PathBuf {
    let dir = std::env::temp_dir().join(format!("annulus-tab-{}-{name}", std::process::id()));
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir).unwrap();
    dir
}

fn awkward(s: &str) -> String {
    format!("{s}\t\\n\n\r\\\\ ünï\u{1b}]52;c;x\u{7}\u{202e}\\u{{41}}")
}

fn every_kind() -> Stores {
    let mut stores = Stores::install(&test_wand());
    let reach = |verb, place: &str| Reach {
        verb,
        place: place.to_string(),
    };
    stores.index.push(IxEntry::Damn(awkward("goetia:1:bael")));
    stores
        .index
        .push(IxEntry::Proscribe(awkward("goetia:34:furfur"), vec![]));
    stores.index.push(IxEntry::Proscribe(
        "goetia:43:sabnock".into(),
        vec![
            reach(Verb::Dele, "a.txt"),
            reach(Verb::Aufer, &awkward("b")),
        ],
    ));
    stores.index.push(IxEntry::Lift("goetia:34:furfur".into()));
    stores
        .index
        .push(IxEntry::Suspect("goetia:70:seere".into()));
    stores.index.push(IxEntry::Clear("goetia:70:seere".into()));
    stores.vessel.push(Sealed::Datum(awkward("datum")));
    stores.vessel.push(Sealed::Content(7, Content::Empty));
    stores
        .vessel
        .push(Sealed::Content(8, Content::Response(9, awkward("verba"))));
    stores.vessel.push(Sealed::Content(
        10,
        Content::Work(
            [
                Verb::Scribe,
                Verb::Crea,
                Verb::Dele,
                Verb::Exscribe,
                Verb::Transfer,
                Verb::Coniunge,
                Verb::Scinde,
                Verb::Aufer,
            ]
            .into_iter()
            .map(|verb| Effect {
                verb,
                place: "x.txt".into(),
                data: awkward("data"),
            })
            .collect(),
        ),
    ));
    stores
        .vessel
        .push(Sealed::Content(11, Content::Work(vec![])));
    stores.keys.push(KeyEntry::Petition("f1".into()));
    stores.keys.push(KeyEntry::Confirmed("f1".into()));
    stores.keys.push(KeyEntry::Spend(1));
    stores
}

fn seal_fresh(dir: &Path, stores: &Stores, marks: &BTreeSet<Place>) {
    Domus::new(dir)
        .seal(stores, [0, 0, 0], marks, &test_wand())
        .unwrap();
}

#[test]
fn every_entry_kind_survives_the_round_trip() {
    let dir = house("round");
    let stores = every_kind();
    let marks: BTreeSet<Place> = [awkward("q.txt")].into();
    seal_fresh(&dir, &stores, &marks);
    let opened = Domus::new(&dir).open(&test_wand()).unwrap();
    assert_eq!(opened.stores.index.entries(), stores.index.entries());
    assert_eq!(opened.stores.vessel.entries(), stores.vessel.entries());
    assert_eq!(opened.stores.keys.entries(), stores.keys.entries());
    assert_eq!(opened.stores.roots().index, stores.roots().index);
    assert_eq!(opened.marks, marks);
    assert_eq!(opened.counts, [6, 8, 3]);
}

#[test]
fn appending_keeps_earlier_entries_and_their_chain() {
    let dir = house("append");
    let mut stores = every_kind();
    seal_fresh(&dir, &stores, &BTreeSet::new());
    let domus = Domus::new(&dir);
    let opened = domus.open(&test_wand()).unwrap();
    stores
        .index
        .push(IxEntry::Suspect("goetia:2:agares".into()));
    domus
        .seal(&stores, opened.counts, &BTreeSet::new(), &test_wand())
        .unwrap();
    let again = domus.open(&test_wand()).unwrap();
    assert_eq!(again.stores.index.entries(), stores.index.entries());
    assert_eq!(
        fs::read_to_string(dir.join(INDEX)).unwrap().lines().count(),
        7
    );
}

fn refused(dir: &Path) -> String {
    match Domus::new(dir).open(&test_wand()) {
        Ok(_) => panic!("tampered stores were opened"),
        Err(e) => e,
    }
}

fn edit(path: PathBuf, from: &str, to: &str) {
    let text = fs::read_to_string(&path).unwrap();
    assert!(text.contains(from), "{from:?} not in {}", path.display());
    fs::write(&path, text.replacen(from, to, 1)).unwrap();
}

#[test]
fn a_tampered_line_in_any_store_refuses() {
    for (name, from, to) in [
        (INDEX, "sabnock", "sabnocK"),
        (VESSEL, "verba", "Verba"),
        (KEYS, "spend\t1", "spend\t0"),
    ] {
        let dir = house(&format!("tamper-{name}"));
        seal_fresh(&dir, &every_kind(), &BTreeSet::new());
        edit(dir.join(name), from, to);
        assert!(refused(&dir).contains("tampered"), "{name}");
    }
}

#[test]
fn a_removed_or_appended_line_refuses() {
    let dir = house("removed");
    seal_fresh(&dir, &every_kind(), &BTreeSet::new());
    let path = dir.join(INDEX);
    let text = fs::read_to_string(&path).unwrap();
    let shorter: Vec<&str> = text.lines().skip(1).collect();
    fs::write(&path, shorter.join("\n") + "\n").unwrap();
    refused(&dir);

    let dir = house("appended");
    seal_fresh(&dir, &every_kind(), &BTreeSet::new());
    let path = dir.join(INDEX);
    let text = fs::read_to_string(&path).unwrap();
    let last = text.lines().last().unwrap().to_string();
    fs::write(&path, format!("{text}{last}\n")).unwrap();
    refused(&dir);
}

#[test]
fn a_truncated_store_matching_its_own_chain_still_refuses() {
    let dir = house("truncated");
    seal_fresh(&dir, &every_kind(), &BTreeSet::new());
    let path = dir.join(INDEX);
    let text = fs::read_to_string(&path).unwrap();
    let kept: Vec<&str> = text.lines().take(2).collect();
    fs::write(&path, kept.join("\n") + "\n").unwrap();
    assert!(refused(&dir).contains("last signed record"));
}

#[test]
fn a_missing_roots_record_refuses() {
    let dir = house("noroots");
    seal_fresh(&dir, &every_kind(), &BTreeSet::new());
    fs::remove_file(dir.join(ROOTS)).unwrap();
    refused(&dir);
    fs::write(dir.join(ROOTS), "").unwrap();
    refused(&dir);
}

#[test]
fn a_missing_store_beside_the_others_refuses() {
    let dir = house("nostore");
    seal_fresh(&dir, &every_kind(), &BTreeSet::new());
    fs::remove_file(dir.join(KEYS)).unwrap();
    refused(&dir);
}

#[test]
fn a_forged_or_unsigned_roots_record_refuses() {
    let dir = house("forged");
    seal_fresh(&dir, &every_kind(), &BTreeSet::new());
    let path = dir.join(ROOTS);
    let line = fs::read_to_string(&path).unwrap();
    let mut f: Vec<String> = line.trim_end().split('\t').map(String::from).collect();
    f[5] = "0".repeat(64);
    fs::write(&path, f.join("\t") + "\n").unwrap();
    assert!(refused(&dir).contains("rolled back or replaced"));
    let unanchored =
        std::env::temp_dir().join(format!("annulus-unanchored-{}", std::process::id()));
    let e = Domus::anchored(&dir, &unanchored)
        .adopt(&test_wand())
        .expect_err("an unsigned record cannot be adopted");
    assert!(e.contains("not signed"), "{e}");
}

#[test]
fn stores_sealed_by_another_wand_refuse() {
    let dir = house("otherwand");
    let other = Wand {
        fingerprint: "alia-virga".into(),
        ..test_wand()
    };
    Domus::new(&dir)
        .seal(&every_kind(), [0, 0, 0], &BTreeSet::new(), &other)
        .unwrap();
    assert!(refused(&dir).contains("another wand"));
    assert!(Domus::new(&dir).open(&other).is_ok());
}

#[test]
fn the_quarantine_record_cannot_be_edited() {
    let dir = house("marks");
    let marks: BTreeSet<Place> = ["b.txt".to_string()].into();
    seal_fresh(&dir, &every_kind(), &marks);
    fs::write(dir.join(MARKS), "").unwrap();
    assert!(refused(&dir).contains("quarantined"));
    fs::remove_file(dir.join(MARKS)).unwrap();
    assert!(refused(&dir).contains("quarantined"));
}

#[test]
fn without_the_wand_nothing_is_sealed() {
    let dir = house("nowand");
    assert!(Domus::new(&dir)
        .seal(&every_kind(), [0, 0, 0], &BTreeSet::new(), &no_wand())
        .is_err());
    assert!(!Domus::new(&dir).installed());
}

#[test]
fn a_fresh_house_installs_the_vessel() {
    let dir = house("fresh");
    let opened = Domus::new(&dir).open(&test_wand()).unwrap();
    assert!(opened.roots.is_none());
    assert_eq!(opened.stores.vessel.len(), 3);
}

#[test]
fn pending_petitions_are_those_not_yet_confirmed() {
    let mut stores = Stores::install(&test_wand());
    stores.keys.push(KeyEntry::Petition("a".into()));
    stores.keys.push(KeyEntry::Petition("b".into()));
    stores.keys.push(KeyEntry::Confirmed("a".into()));
    stores.keys.push(KeyEntry::Spend(2));
    stores.keys.push(KeyEntry::Petition("a".into()));
    stores.keys.push(KeyEntry::Petition("b".into()));
    let dir = house("pending");
    let listed: Vec<String> = Domus::new(&dir)
        .petitions(&stores)
        .into_iter()
        .map(|(f, _)| f)
        .collect();
    assert_eq!(listed, vec!["a", "b"]);
    stores.keys.push(KeyEntry::Confirmed("b".into()));
    let listed: Vec<String> = Domus::new(&dir)
        .petitions(&stores)
        .into_iter()
        .map(|(f, _)| f)
        .collect();
    assert_eq!(listed, vec!["a"]);
}

fn places(list: &[&str]) -> BTreeSet<String> {
    list.iter().map(|s| s.to_string()).collect()
}

#[test]
fn unlawful_places_are_refused() {
    for place in [
        "",
        "../a.txt",
        "a/../../b.txt",
        "./a.txt",
        "/etc/passwd",
        "C:\\x.txt",
        "c:x.txt",
        "a\\b.txt",
        "a.txt.annulus-novum",
        "annulus.domus./annulus.claves",
        "a.txt ",
        " a.txt",
        "ANNULU~1/annulus.claves",
        "con",
        "NUL.txt",
        "com1.log",
        "a?.txt",
        "a|b",
        "a\u{7}b",
    ] {
        assert!(!lawful_place(place), "{place:?}");
    }
    for place in [
        "a.txt",
        "libri/a.txt",
        "..a.txt",
        "a..txt",
        "console.txt",
        "comma.txt",
    ] {
        assert!(lawful_place(place), "{place:?}");
    }
    let dir = house("unlawful");
    assert!(read_world(&dir, &places(&["../x"]), &BTreeSet::new()).is_err());
    let inner = dir.join("annulus.domus");
    fs::create_dir_all(&inner).unwrap();
    for place in [
        "annulus.domus/annulus.claves",
        "ANNULUS.DOMUS/annulus.claves",
        "Annulus.Domus/x",
    ] {
        assert!(
            guard_places(&dir, &inner, &places(&[place])).is_err(),
            "{place:?}"
        );
    }
    assert!(guard_places(&dir, &inner, &places(&["annulus.domus2/x"])).is_ok());
}

#[cfg(unix)]
#[test]
fn links_anywhere_on_the_path_are_refused() {
    let dir = house("links");
    let outside = house("links-outside");
    fs::write(outside.join("secret.txt"), "arcanum").unwrap();
    std::os::unix::fs::symlink(&outside, dir.join("libri")).unwrap();
    std::os::unix::fs::symlink(outside.join("secret.txt"), dir.join("s.txt")).unwrap();
    for place in ["libri/secret.txt", "s.txt"] {
        let e = read_world(&dir, &places(&[place]), &BTreeSet::new()).unwrap_err();
        assert!(e.contains("link"), "{place}: {e}");
    }
    let after: BTreeMap<Place, (String, Mark)> = [(
        "libri/new.txt".to_string(),
        ("x".to_string(), Mark::Quarantined),
    )]
    .into();
    assert!(write_world(&dir, &BTreeMap::new(), &after, &places(&["libri/new.txt"])).is_err());
    assert!(!outside.join("new.txt").exists());
}

#[test]
fn places_in_the_house_of_the_stores_are_refused() {
    let dir = house("inhouse");
    let domus = dir.join("annulus.domus");
    fs::create_dir_all(&domus).unwrap();
    let e = guard_places(&dir, &domus, &places(&["annulus.domus/annulus.vas"])).unwrap_err();
    assert!(e.contains("house"));
    assert!(guard_places(&dir, &dir, &places(&["a.txt"])).is_err());
    assert!(guard_places(&dir, &domus, &places(&["a.txt"])).is_ok());
}

#[test]
fn quarantine_is_read_back_from_the_record() {
    let dir = house("readmarks");
    fs::write(dir.join("a.txt"), "x").unwrap();
    fs::write(dir.join("b.txt"), "y").unwrap();
    let marks: BTreeSet<Place> = ["b.txt".to_string()].into();
    let world = read_world(&dir, &places(&["a.txt", "b.txt", "c.txt"]), &marks).unwrap();
    assert_eq!(world["a.txt"].1, Mark::Outside);
    assert_eq!(world["b.txt"].1, Mark::Quarantined);
    assert!(!world.contains_key("c.txt"));
    let after: BTreeMap<Place, (String, Mark)> = [
        ("a.txt".to_string(), ("x".to_string(), Mark::Outside)),
        ("c.txt".to_string(), ("z".to_string(), Mark::Quarantined)),
    ]
    .into();
    let next = marks_after(&marks, &after);
    assert_eq!(next, places(&["b.txt", "c.txt"]));
}

#[test]
fn a_place_changed_during_the_run_is_left_alone() {
    let dir = house("changed");
    let before = read_world(&dir, &places(&["a.txt"]), &BTreeSet::new()).unwrap();
    fs::write(dir.join("a.txt"), "the owner's").unwrap();
    let after: BTreeMap<Place, (String, Mark)> = [(
        "a.txt".to_string(),
        ("the spirit's".to_string(), Mark::Quarantined),
    )]
    .into();
    let e = write_world(&dir, &before, &after, &places(&["a.txt"])).unwrap_err();
    assert!(e.contains("changed on disk"));
    assert_eq!(
        fs::read_to_string(dir.join("a.txt")).unwrap(),
        "the owner's"
    );
}

#[test]
fn writes_are_staged_and_leave_no_stray_file() {
    let dir = house("staged");
    fs::write(dir.join("a.md"), "unrelated").unwrap();
    let after: BTreeMap<Place, (String, Mark)> = [(
        "libri/a.txt".to_string(),
        ("novum".to_string(), Mark::Quarantined),
    )]
    .into();
    let changed = write_world(&dir, &BTreeMap::new(), &after, &places(&["libri/a.txt"])).unwrap();
    assert_eq!(changed, vec!["written: libri/a.txt"]);
    assert_eq!(
        fs::read_to_string(dir.join("libri/a.txt")).unwrap(),
        "novum"
    );
    assert_eq!(fs::read_to_string(dir.join("a.md")).unwrap(), "unrelated");
    assert!(!dir.join("libri/a.txt.annulus-novum").exists());
}

#[test]
fn audits_are_never_overwritten() {
    let dir = house("acta");
    let program = dir.join("opus.annulus");
    let first = audit_path(&program, None);
    assert_eq!(first, dir.join("opus.acta"));
    let audit: Chain<Label> = Chain::new();
    write_audit(&first, &audit, &test_wand(), &[]).unwrap();
    assert!(write_audit(&first, &audit, &test_wand(), &[]).is_err());
    assert_eq!(audit_path(&program, None), dir.join("opus.2.acta"));
    assert!(fs::read_to_string(&first)
        .unwrap()
        .starts_with("# made with the test wand"));
}

#[test]
fn no_store_holds_a_hidden_character() {
    let dir = house("hidden");
    seal_fresh(&dir, &every_kind(), &BTreeSet::new());
    for name in [INDEX, VESSEL, KEYS] {
        let text = fs::read_to_string(dir.join(name)).unwrap();
        assert!(
            !text
                .chars()
                .any(|c| c != '\n' && c != '\t' && annulus::lingua::exsequi::hidden(c)),
            "{name}"
        );
    }
}
