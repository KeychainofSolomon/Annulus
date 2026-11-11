use std::fs;
use std::path::Path;

use annulus::lingua::parse_source;

fn programs(kind: &str) -> Vec<(String, String)> {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/grammar")
        .join(kind);
    let mut out: Vec<(String, String)> = fs::read_dir(dir)
        .expect("tests/grammar exists")
        .map(|e| e.expect("readable entry").path())
        .filter(|p| p.extension().is_some_and(|x| x == "annulus"))
        .map(|p| {
            (
                p.file_name().unwrap().to_string_lossy().into_owned(),
                fs::read_to_string(&p).expect("UTF-8 program"),
            )
        })
        .collect();
    out.sort();
    out
}

#[test]
fn every_accept_program_parses() {
    let all = programs("accept");
    assert!(all.len() >= 28);
    for (name, text) in all {
        if let Err(e) = parse_source(&text) {
            panic!("accept/{name} was rejected: {e}");
        }
    }
}

#[test]
fn every_reject_program_is_refused() {
    let all = programs("reject");
    assert!(all.len() >= 41);
    for (name, text) in all {
        match parse_source(&text) {
            Ok(_) => panic!("reject/{name} was accepted"),
            Err(e) => println!("reject/{name}: {e}"),
        }
    }
}

#[test]
fn deep_nesting_is_refused_not_a_crash() {
    let deep = format!("SIT x = {}1{};", "(".repeat(5000), ")".repeat(5000));
    assert!(parse_source(&deep).is_err());
    let negations = format!("SIT x = {}1;", "- ".repeat(5000));
    assert!(parse_source(&negations).is_err());
    let blocks = format!("{}{}", "SI VERUM { ".repeat(5000), "}".repeat(5000));
    assert!(parse_source(&blocks).is_err());
    let fine = format!("SIT x = {}1{};", "(".repeat(100), ")".repeat(100));
    assert!(parse_source(&fine).is_ok());
}

#[test]
fn the_rust_parser_agrees_with_the_reference_to_the_column() {
    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/grammar/positions.json");
    let text = fs::read_to_string(path).expect("positions.json, written by tools/grammar_check.py");
    let record = annulus::lingua::json::parse(&text).expect("valid JSON");
    let annulus::lingua::json::Json::Object(map) = record else {
        panic!("positions.json is an object");
    };
    let mut seen = 0;
    for kind in ["accept", "reject"] {
        for (name, program) in programs(kind) {
            let key = format!("{kind}/{name}");
            let expected = map
                .get(&key)
                .unwrap_or_else(|| panic!("{key} missing from positions.json"));
            let found = match parse_source(&program) {
                Ok(_) => None,
                Err(e) => Some((e.pos.line as f64, e.pos.column as f64)),
            };
            let wanted = match expected.list() {
                [l, c] => Some((l.number().unwrap(), c.number().unwrap())),
                _ => None,
            };
            assert_eq!(found, wanted, "{key}: the two parsers disagree");
            seen += 1;
        }
    }
    assert_eq!(
        seen,
        map.len(),
        "positions.json lists a program that no longer exists"
    );
}
