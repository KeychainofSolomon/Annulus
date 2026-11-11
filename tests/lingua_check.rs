use std::fs;
use std::path::Path;

use annulus::lingua::check_source;

fn codes(source: &str) -> Vec<&'static str> {
    match check_source(source) {
        Ok(_) => Vec::new(),
        Err(errors) => errors.iter().map(|e| e.code).collect(),
    }
}

fn lawful(source: &str) {
    if let Err(errors) = check_source(source) {
        let shown: Vec<String> = errors.iter().map(|e| e.to_string()).collect();
        panic!("expected a lawful program, got:\n{}", shown.join("\n"));
    }
}

fn refused(source: &str, code: &str) {
    let found = codes(source);
    assert!(
        found.contains(&code),
        "expected {code}, got {found:?} for:\n{source}"
    );
}

const FACULTAS: &str = "FACULTAS {\n    liber \"tempestas.txt\" scribere;\n    liber \"turris.txt\" creare;\n    liber \"liber.txt\" legere;\n    atramentum \"comes\" 2;\n}\n";

fn rite(body: &str) -> String {
    format!(
        "{FACULTAS}CIRCULUS RITU goetia {{\n    INTENTIO \"of the storm\";\n    SIT sigillum = INSCRIBE SIGILLUM furfur ATRAMENTO CONSECRA ATRAMENTUM \"comes\";\n{body}\n}}\n"
    )
}

fn telling(body: &str) -> String {
    rite(&format!(
        "    SIT vocatus = INVOCA SPIRITUM 34 AD OFFICIUM responsa;\n    SIT p = PETE vocatus(\"quid est tempestas\", \"liber.txt\");\n{body}\n    VADE IN PACE vocatus;"
    ))
}

#[test]
fn the_grammar_example_waits_only_for_the_tempest() {
    let found = codes(include_str!("grammar/accept/full_rite.annulus"));
    assert_eq!(
        found,
        vec!["X1"],
        "only the office TEMPESTAS is outside the subset"
    );
}

#[test]
fn a_telling_rite_is_lawful() {
    lawful(&telling(
        "    SI EXAMINA p == RATUM {\n        MANIFESTA RESPONSUM p;\n    }",
    ));
}

#[test]
fn ordinary_computation_is_lawful() {
    lawful("SIT MUTABILIS n = 0;\nDUM n < 3 {\n    n = n + 1;\n}\nSI n == 3 {\n    MANIFESTA \"tres\";\n} ALITER {\n    MANIFESTA n;\n}\n");
}

#[test]
fn a5_office_not_held() {
    refused(
        &rite("    SIT f = INVOCA furfur AD OFFICIUM doctrina;\n    VADE IN PACE f;"),
        "A5",
    );
}

#[test]
fn a5_office_nullum() {
    refused(
        &rite("    SIT f = INVOCA furfur AD OFFICIUM nullum;\n    VADE IN PACE f;"),
        "A5",
    );
}

#[test]
fn a5_question_of_the_wrong_type() {
    refused(
        &rite("    SIT f = INVOCA furfur AD OFFICIUM responsa;\n    SIT p = PETE f(7);\n    VADE IN PACE f;"),
        "A5",
    );
}

#[test]
fn a5_place_not_a_constant() {
    refused(
        &rite("    SIT s2 = INSCRIBE SIGILLUM halphas ATRAMENTO CONSECRA ATRAMENTUM \"comes\";\n    SIT MUTABILIS locus = \"turris.txt\";\n    SIT f = INVOCA halphas AD OFFICIUM aedificatio;\n    SIT p = PETE f(locus);\n    VADE IN PACE f;"),
        "A5",
    );
}

#[test]
fn a5_working_office_needs_a_place() {
    refused(
        &rite("    SIT s2 = INSCRIBE SIGILLUM halphas ATRAMENTO CONSECRA ATRAMENTUM \"comes\";\n    SIT f = INVOCA halphas AD OFFICIUM aedificatio;\n    SIT p = PETE f();\n    VADE IN PACE f;"),
        "A5",
    );
}

#[test]
fn a5_responsum_of_work() {
    refused(
        &rite("    SIT s2 = INSCRIBE SIGILLUM halphas ATRAMENTO CONSECRA ATRAMENTUM \"comes\";\n    SIT f = INVOCA halphas AD OFFICIUM aedificatio;\n    SIT p = PETE f(\"turris.txt\");\n    SIT r = RESPONSUM p;\n    VADE IN PACE f;"),
        "A5",
    );
}

#[test]
fn a5_place_reserved_for_angels() {
    refused(
        "FACULTAS {\n    liber \"vetus.txt\" creare angelis;\n    atramentum \"comes\" 1;\n}\nCIRCULUS RITU goetia {\n    INTENTIO \"x\";\n    SIT s = INSCRIBE SIGILLUM halphas ATRAMENTO CONSECRA ATRAMENTUM \"comes\";\n    SIT f = INVOCA halphas AD OFFICIUM aedificatio;\n    SIT p = PETE f(\"vetus.txt\");\n    VADE IN PACE f;\n}\n",
        "A5",
    );
}

#[test]
fn a14_quarantined_condition() {
    refused(
        &telling("    SI EXAMINA p == RATUM {\n        SI RESPONSUM p == \"ita\" {\n            MANIFESTA 1;\n        }\n    }"),
        "A14",
    );
}

#[test]
fn a14_quarantine_follows_a_variable_through_a_loop() {
    refused(
        &telling("    SIT MUTABILIS verba = \"\";\n    DUM verba == \"\" {\n        verba = RESPONSUM p;\n    }"),
        "A14",
    );
}

#[test]
fn a14_consecrated_words_may_steer() {
    lawful(&telling(
        "    SI EXAMINA p == RATUM {\n        SI CONSECRA RESPONSUM p == \"ita\" {\n            MANIFESTA 1;\n        }\n    }",
    ));
}

#[test]
fn a15_quarantined_words_to_a_spirit() {
    refused(&telling("    SIT q = PETE vocatus(RESPONSUM p);"), "A15");
}

#[test]
fn a15_computed_from_words_is_still_quarantined() {
    refused(
        &telling("    SIT verba = \"dic: \" + RESPONSUM p;\n    SIT q = PETE vocatus(verba);"),
        "A15",
    );
}

#[test]
fn a15_consecrated_words_may_be_passed() {
    lawful(&telling("    SIT q = PETE vocatus(CONSECRA RESPONSUM p);"));
}

#[test]
fn a7_ink_spent_twice() {
    refused(
        &rite("    SIT a = CONSECRA ATRAMENTUM \"comes\";\n    SIT s1 = INSCRIBE SIGILLUM furfur ATRAMENTO a;\n    SIT s2 = INSCRIBE SIGILLUM furfur ATRAMENTO a;"),
        "A7",
    );
}

#[test]
fn a7_ink_spent_through_a_copy() {
    refused(
        &rite("    SIT a = CONSECRA ATRAMENTUM \"comes\";\n    SIT b = a;\n    SIT s1 = INSCRIBE SIGILLUM furfur ATRAMENTO a;\n    SIT s2 = INSCRIBE SIGILLUM furfur ATRAMENTO b;"),
        "A7",
    );
}

#[test]
fn a7_raw_ink() {
    refused(
        &rite("    SIT s = INSCRIBE SIGILLUM furfur ATRAMENTO ATRAMENTUM \"comes\";"),
        "A7",
    );
}

#[test]
fn a7_wrong_class() {
    refused(
        "FACULTAS {\n    atramentum \"rex\" 1;\n}\nCIRCULUS RITU goetia {\n    INTENTIO \"x\";\n    SIT s = INSCRIBE SIGILLUM furfur ATRAMENTO CONSECRA ATRAMENTUM \"rex\";\n}\n",
        "A7",
    );
}

#[test]
fn a7_ink_in_a_loop() {
    refused(
        &rite("    SIT a = CONSECRA ATRAMENTUM \"comes\";\n    SIT MUTABILIS n = 0;\n    DUM n < 2 {\n        SIT s = INSCRIBE SIGILLUM furfur ATRAMENTO a;\n        n = n + 1;\n    }"),
        "A7",
    );
}

#[test]
fn a8_ink_not_granted() {
    refused(
        "CIRCULUS RITU goetia {\n    INTENTIO \"x\";\n    SIT a = ATRAMENTUM \"comes\";\n}\n",
        "A8",
    );
}

#[test]
fn a8_place_not_granted() {
    refused(
        &rite("    SIT s2 = INSCRIBE SIGILLUM halphas ATRAMENTO CONSECRA ATRAMENTUM \"comes\";\n    SIT f = INVOCA halphas AD OFFICIUM aedificatio;\n    SIT p = PETE f(\"alius.txt\");\n    VADE IN PACE f;"),
        "A8",
    );
}

#[test]
fn a8_permission_missing() {
    refused(
        &rite("    SIT s2 = INSCRIBE SIGILLUM halphas ATRAMENTO CONSECRA ATRAMENTUM \"comes\";\n    SIT f = INVOCA halphas AD OFFICIUM aedificatio;\n    SIT p = PETE f(\"liber.txt\");\n    VADE IN PACE f;"),
        "A8",
    );
}

#[test]
fn a8_faculty_errors() {
    refused("FACULTAS {\n    liber \"a\" volare;\n}\n", "A8");
    refused(
        "FACULTAS {\n    liber \"a\" legere;\n    liber \"a\" scribere;\n}\n",
        "A8",
    );
    refused("FACULTAS {\n    navis \"a\" legere;\n}\n", "A8");
    refused("FACULTAS {\n    atramentum \"plumbum\" 1;\n}\n", "A8");
}

#[test]
fn a9_not_released() {
    refused(
        &rite("    SIT s2 = INSCRIBE SIGILLUM halphas ATRAMENTO CONSECRA ATRAMENTUM \"comes\";\n    SIT f = INVOCA halphas AD OFFICIUM aedificatio;"),
        "A9",
    );
}

#[test]
fn a9_released_twice() {
    refused(
        &rite(
            "    SIT f = INVOCA furfur AD OFFICIUM tempestas;\n    VADE IN PACE f;\n    RELEGA f;",
        ),
        "A9",
    );
}

#[test]
fn a9_used_after_release() {
    refused(
        &rite("    SIT s2 = INSCRIBE SIGILLUM halphas ATRAMENTO CONSECRA ATRAMENTUM \"comes\";\n    SIT f = INVOCA halphas AD OFFICIUM aedificatio;\n    DIMITTE f;\n    SIT p = PETE f(\"turris.txt\");"),
        "A9",
    );
}

#[test]
fn a9_released_on_one_path() {
    refused(
        &rite("    SIT s2 = INSCRIBE SIGILLUM halphas ATRAMENTO CONSECRA ATRAMENTUM \"comes\";\n    SIT f = INVOCA halphas AD OFFICIUM aedificatio;\n    SI VERUM {\n        VADE IN PACE f;\n    }"),
        "A9",
    );
}

#[test]
fn a9_invocation_not_held() {
    refused(&rite("    INVOCA furfur AD OFFICIUM tempestas;"), "A9");
}

#[test]
fn a10_no_seal() {
    refused(
        "CIRCULUS RITU goetia {\n    INTENTIO \"x\";\n    SIT f = INVOCA furfur AD OFFICIUM tempestas;\n    VADE IN PACE f;\n}\n",
        "A10",
    );
}

#[test]
fn a10_seal_on_one_path_only() {
    refused(
        "FACULTAS {\n    atramentum \"comes\" 1;\n}\nCIRCULUS RITU goetia {\n    INTENTIO \"x\";\n    SI VERUM {\n        SIT s = INSCRIBE SIGILLUM furfur ATRAMENTO CONSECRA ATRAMENTUM \"comes\";\n    }\n    SIT f = INVOCA furfur AD OFFICIUM tempestas;\n    VADE IN PACE f;\n}\n",
        "A10",
    );
}

#[test]
fn a10_seal_of_another_circle() {
    refused(
        "FACULTAS {\n    atramentum \"comes\" 1;\n}\nCIRCULUS RITU goetia {\n    INTENTIO \"x\";\n    SIT s = INSCRIBE SIGILLUM furfur ATRAMENTO CONSECRA ATRAMENTUM \"comes\";\n    CIRCULUS RITU goetia {\n        INTENTIO \"y\";\n        SIT f = INVOCA furfur AD OFFICIUM tempestas;\n        VADE IN PACE f;\n    }\n}\n",
        "A10",
    );
}

#[test]
fn a1_rite_outside_a_circle() {
    refused("SIT a = ATRAMENTUM \"comes\";\n", "A1");
    refused("SIT s = SIGILLUM furfur;\n", "A1");
}

#[test]
fn a2_unknown_rite_and_empty_intention() {
    refused(
        "CIRCULUS RITU necromantia {\n    INTENTIO \"x\";\n}\n",
        "A2",
    );
    refused("CIRCULUS RITU goetia {\n    INTENTIO \"\";\n}\n", "A2");
}

#[test]
fn a3_not_in_the_register() {
    refused(
        "CIRCULUS RITU goetia {\n    INTENTIO \"x\";\n    SIT s = SIGILLUM SPIRITUS 73;\n}\n",
        "A3",
    );
    refused(
        "CIRCULUS RITU goetia {\n    INTENTIO \"x\";\n    SIT s = SIGILLUM lucifer;\n}\n",
        "A3",
    );
}

#[test]
fn a3_labels_and_numbers_agree() {
    lawful("CIRCULUS RITU goetia {\n    INTENTIO \"x\";\n    SIT a = SIGILLUM furfur;\n    SIT b = SIGILLUM SPIRITUS 34;\n    SIT c = SIGILLUM glasya_labolas;\n    SIT d = SIGILLUM barbas;\n}\n");
}

#[test]
fn a11_and_a13_are_refused_until_the_apparatus_and_kings() {
    refused(
        "FACULTAS {\n    atramentum \"rex\" 1;\n}\nCIRCULUS RITU goetia {\n    INTENTIO \"x\";\n    SIT s = INSCRIBE SIGILLUM beleth ATRAMENTO CONSECRA ATRAMENTUM \"rex\";\n    SIT b = INVOCA beleth AD OFFICIUM concordia;\n    VADE IN PACE b;\n}\n",
        "A11",
    );
    refused(
        "FACULTAS {\n    atramentum \"praeses\" 1;\n}\nCIRCULUS RITU goetia {\n    INTENTIO \"x\";\n    SIT s = INSCRIBE SIGILLUM marbas ATRAMENTO CONSECRA ATRAMENTUM \"praeses\";\n    SIT m = INVOCA marbas AD OFFICIUM occulta;\n    VADE IN PACE m;\n}\n",
        "A13",
    );
}

#[test]
fn names_and_types() {
    refused("MANIFESTA x;\n", "N1");
    refused("MANIFESTA x;\nSIT x = 1;\n", "N2");
    refused("SIT x = 1;\nSI VERUM {\n    SIT x = 2;\n}\n", "N3");
    refused("SIT x = 1;\nx = 2;\n", "N4");
    refused("SIT x = 1 + \"a\";\n", "T1");
    refused("SIT x = 7 == \"7\";\n", "T2");
    refused("SI 1 {\n}\n", "T3");
    refused("SIT x: VERBUM = 1;\n", "T1");
}

#[test]
fn n5_a_spirits_name_belongs_to_the_spirit() {
    refused(
        &rite("    SIT furfur = INVOCA furfur AD OFFICIUM tempestas;\n    VADE IN PACE furfur;"),
        "N5",
    );
    refused("SIT bael = 7;\n", "N5");
    refused("SIT barbas = 7;\n", "N5");
    lawful("SIT baelus = 7;\n");
}

#[test]
fn t7_rite_values_are_not_shown() {
    refused(&telling("    MANIFESTA p;"), "T7");
}

#[test]
fn the_subset_is_marked() {
    refused("OPUS f() {\n}\n", "X1");
    refused("SIT s = [1, 2];\n", "X1");
}

#[test]
fn every_accepted_grammar_program_is_checked_without_panic() {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/grammar/accept");
    for entry in fs::read_dir(dir).expect("tests/grammar/accept exists") {
        let path = entry.expect("readable entry").path();
        let text = fs::read_to_string(&path).expect("UTF-8 program");
        let _ = check_source(&text);
    }
}

#[test]
fn a14_a_chain_of_assignments_cannot_launder_quarantine() {
    refused(
        &telling("    SIT MUTABILIS a = \"\";\n    SIT MUTABILIS b = \"\";\n    SIT MUTABILIS c = \"\";\n    SIT MUTABILIS d = \"\";\n    DUM a == \"\" {\n        a = b;\n        b = c;\n        c = d;\n        d = RESPONSUM p;\n    }"),
        "A14",
    );
}

#[test]
fn a5_working_offices_check_each_place_by_its_role() {
    let base = "FACULTAS {\n    liber \"a.txt\" legere;\n    liber \"b.txt\" creare;\n    atramentum \"princeps\" 1;\n}\nCIRCULUS RITU goetia {\n    INTENTIO \"x\";\n    SIT s = INSCRIBE SIGILLUM seere ATRAMENTO CONSECRA ATRAMENTUM \"princeps\";\n    SIT v = INVOCA seere AD OFFICIUM vectio;\n";
    lawful(&format!(
        "{base}    SIT p = PETE v(\"exscribe\", \"a.txt\", \"b.txt\");\n    VADE IN PACE v;\n}}\n"
    ));
    refused(&format!("{base}    SIT p = PETE v(\"transfer\", \"a.txt\", \"b.txt\");\n    VADE IN PACE v;\n}}\n"), "A8");
    refused(
        &format!("{base}    SIT p = PETE v(\"exscribe\", \"a.txt\");\n    VADE IN PACE v;\n}}\n"),
        "A5",
    );
    refused(&format!("{base}    SIT p = PETE v(\"rapere\", \"a.txt\", \"b.txt\");\n    VADE IN PACE v;\n}}\n"), "A5");
}

fn beleth(apparatus: &str, after: &str) -> String {
    format!(
        "FACULTAS {{\n    atramentum \"rex\" 2;\n}}\nCIRCULUS RITU goetia {{\n    INTENTIO \"concord\";\n{apparatus}    SIT s = INSCRIBE SIGILLUM beleth ATRAMENTO CONSECRA ATRAMENTUM \"rex\";\n    SIT b = INVOCA beleth AD OFFICIUM concordia;\n{after}    VADE IN PACE b;\n}}\n"
    )
}

#[test]
fn a11_beleth_comes_only_to_ring_and_wand() {
    lawful(&beleth("    GERE ANULUM;\n    GERE VIRGAM;\n", ""));
    refused(&beleth("    GERE ANULUM;\n", ""), "A11");
    refused(
        &beleth(
            "    GERE ANULUM;\n    SI VERUM {\n        GERE VIRGAM;\n    }\n",
            "",
        ),
        "A11",
    );
    refused(
        &beleth(
            "    GERE ANULUM;\n    DUM FALSUM {\n        GERE VIRGAM;\n    }\n",
            "",
        ),
        "A11",
    );
    lawful(&beleth(
        "    GERE ANULUM;\n    SI VERUM {\n        GERE VIRGAM;\n    } ALITER {\n        GERE VIRGAM;\n    }\n",
        "",
    ));
}

#[test]
fn an_instrument_lasts_until_its_circle_closes() {
    refused(
        "FACULTAS {\n    atramentum \"rex\" 1;\n}\nCIRCULUS RITU goetia {\n    INTENTIO \"a\";\n    GERE ANULUM;\n    GERE VIRGAM;\n}\nCIRCULUS RITU goetia {\n    INTENTIO \"b\";\n    SIT s = INSCRIBE SIGILLUM beleth ATRAMENTO CONSECRA ATRAMENTUM \"rex\";\n    SIT b = INVOCA beleth AD OFFICIUM concordia;\n    VADE IN PACE b;\n}\n",
        "A11",
    );
}

#[test]
fn a1_the_apparatus_stays_in_the_circle() {
    refused("GERE ANULUM;\n", "A1");
    refused("SIT x = 1;\nAPPELLA x;\n", "A1");
    refused("SIT x = 1;\nCONSTRINGE x PER NOMEN \"AGLA\";\n", "A1");
}

#[test]
fn the_apparatus_addresses_a_live_invocation() {
    let both = "    GERE ANULUM;\n    GERE VIRGAM;\n";
    refused(&beleth(both, "    SIT n = 3;\n    APPELLA n;\n"), "T1");
    refused(&beleth(both, "    VETA 3;\n"), "T1");
    refused(&beleth(both, "    APPELLA nemo;\n"), "N1");
    refused(
        &format!(
            "FACULTAS {{\n    atramentum \"rex\" 2;\n}}\nCIRCULUS RITU goetia {{\n    INTENTIO \"c\";\n{both}    SIT s = INSCRIBE SIGILLUM beleth ATRAMENTO CONSECRA ATRAMENTUM \"rex\";\n    SIT b = INVOCA beleth AD OFFICIUM concordia;\n    VADE IN PACE b;\n    VETA b;\n}}\n"
        ),
        "A9",
    );
    lawful(&beleth(both, "    APPELLA b;\n    VETA b;\n    OFFER b;\n"));
}

#[test]
fn a12_only_a_recorded_divine_name_binds() {
    let both = "    GERE ANULUM;\n    GERE VIRGAM;\n";
    lawful(&beleth(
        both,
        "    CONSTRINGE b PER NOMEN \"Tetragrammaton\";\n",
    ));
    lawful(&beleth(both, "    CONSTRINGE b PER NOMEN \"AGLA\";\n"));
    for name in [
        "Metatron",
        "Kether",
        "agla",
        "Tetragrammaton ",
        "Beleth",
        "",
    ] {
        refused(
            &beleth(both, &format!("    CONSTRINGE b PER NOMEN \"{name}\";\n")),
            "A12",
        );
    }
}
