//! The binding (`operator.md` §8.8): a key the master vouched for at a calling may grant a
//! faculty that only reads; anything more still needs the master.

use std::collections::{BTreeMap, BTreeSet};

use annulus::baculum::{sign_for_test, test_key};
use annulus::nucleus::state::{digest_constitution, digest_faculty, digest_ligatio, Ligatio};
use annulus::nucleus::*;

const UNTIL: i64 = 1_800_000_000;

fn wand() -> Wand {
    Wand {
        fingerprint: test_key("test-wand"),
        present: true,
        master_unlocked: false,
        test: true,
    }
}

fn ligatio(cert_by: &str) -> Ligatio {
    let key = test_key("ligatio");
    Ligatio {
        signature: sign_for_test(cert_by, &digest_ligatio(&key, UNTIL)),
        key,
        until: UNTIL,
    }
}

/// Launches with `faculty` granted by `signer`, carrying `lig`.
fn launch(
    faculty: &[Cap],
    reserved: &[Cap],
    signer: &str,
    lig: Option<Ligatio>,
) -> Result<(), String> {
    let constitution = Constitution {
        version: 1,
        ..Default::default()
    };
    let faculty: BTreeSet<Cap> = faculty.iter().cloned().collect();
    let reserved: BTreeSet<Cap> = reserved.iter().cloned().collect();
    let stores = Stores::install(&wand());
    let auth = Auth {
        faculty_signature: sign_for_test(
            signer,
            &digest_faculty(&faculty, &reserved, &stores.roots()),
        ),
        owner: test_key("owner-wand"),
        manifest_signature: sign_for_test("owner-wand", &digest_constitution(&constitution, &[])),
        revoked: Vec::new(),
        roots: stores.roots(),
        wand: wand(),
        ligatio: lig,
    };
    let world: BTreeMap<Place, (String, Mark)> = [(
        "notes.txt".to_string(),
        ("hello".to_string(), Mark::Outside),
    )]
    .into_iter()
    .collect();
    let mut s = State::initial(Launch {
        simulated: true,
        constitution,
        auth,
        faculty,
        reserved,
        ink: BTreeMap::new(),
        world,
        stores,
    });
    s.launch().map_err(|e| format!("{e:?}"))
}

fn read() -> Cap {
    Cap::new("notes.txt", Perm::Legere)
}

#[test]
fn a_binding_grants_a_faculty_that_only_reads() {
    assert_eq!(
        launch(&[read()], &[], "ligatio", Some(ligatio("test-wand"))),
        Ok(())
    );
}

#[test]
fn a_binding_never_grants_a_work() {
    for perm in [Perm::Scribere, Perm::Creare, Perm::Delere] {
        let r = launch(
            &[read(), Cap::new("notes.txt", perm)],
            &[],
            "ligatio",
            Some(ligatio("test-wand")),
        );
        assert!(r.unwrap_err().contains("not signed"), "{perm:?}");
    }
    let r = launch(
        &[read()],
        &[Cap::new("notes.txt", Perm::Scribere)],
        "ligatio",
        Some(ligatio("test-wand")),
    );
    assert!(r.unwrap_err().contains("not signed"));
}

#[test]
fn a_binding_the_master_did_not_vouch_for_grants_nothing() {
    let r = launch(&[read()], &[], "ligatio", Some(ligatio("another-wand")));
    assert!(r.unwrap_err().contains("not signed"));
    let r = launch(&[read()], &[], "ligatio", None);
    assert!(r.unwrap_err().contains("not signed"));
}

#[test]
fn the_master_still_grants_works_as_before() {
    let write = Cap::new("notes.txt", Perm::Scribere);
    assert_eq!(launch(&[read(), write], &[], "test-wand", None), Ok(()));
}
