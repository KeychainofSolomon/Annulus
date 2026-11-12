use annulus::horae::*;

fn at(s: &str) -> f64 {
    parse_hora(s).unwrap() as f64
}

fn near(a: f64, b: f64, minutes: f64) -> bool {
    (a - b).abs() <= minutes * 60.0
}

#[test]
fn new_moons_fall_where_the_eclipses_were() {
    for (eclipse, after) in [
        ("2024-04-08T18:21:00Z", "2024-04-10T00:00:00Z"),
        ("2017-08-21T18:30:00Z", "2017-08-22T00:00:00Z"),
        ("2000-01-06T18:14:00Z", "2000-01-07T00:00:00Z"),
    ] {
        let found = last_new_moon(at(after));
        assert!(
            near(found, at(eclipse), 10.0),
            "{eclipse}: off by {} min",
            (found - at(eclipse)) / 60.0
        );
    }
    assert_eq!(aetas_lunae(at("2024-04-12T18:30:00Z")), 4);
    assert_eq!(aetas_lunae(at("2024-04-08T18:00:00Z")), 29);
}

#[test]
fn the_sun_rises_and_sets_where_it_did() {
    let berlin = (52.52, 13.405);
    let s = sol(
        local_day(at("2026-06-21T12:00:00Z"), berlin.1),
        berlin.0,
        berlin.1,
    )
    .unwrap();
    assert!(
        near(s.ortus, at("2026-06-21T02:43:00Z"), 4.0),
        "rise {}",
        (s.ortus - at("2026-06-21T02:43:00Z")) / 60.0
    );
    assert!(
        near(s.occasus, at("2026-06-21T19:33:00Z"), 4.0),
        "set {}",
        (s.occasus - at("2026-06-21T19:33:00Z")) / 60.0
    );
    let w = sol(
        local_day(at("2026-12-21T12:00:00Z"), berlin.1),
        berlin.0,
        berlin.1,
    )
    .unwrap();
    assert!(near(w.ortus, at("2026-12-21T07:15:00Z"), 4.0));
    assert!(near(w.occasus, at("2026-12-21T14:54:00Z"), 4.0));
    assert!(w.aurora < w.ortus && w.crepusculum > w.occasus);
    assert!(sol(local_day(at("2026-12-21T12:00:00Z"), 15.0), 78.2, 15.0).is_none());
}

fn caelum(hora: &str, serenum: Option<bool>) -> Caelum {
    Caelum {
        hora: parse_hora(hora),
        locus: Some((41.9, 12.5)),
        serenum,
    }
}

fn rank(r: &str) -> Vec<String> {
    vec![r.to_string()]
}

#[test]
fn each_rank_keeps_its_hours() {
    let day = "2024-04-13";
    let open = |r: &str, hhmm: &str, sky: Option<bool>| {
        admit(&rank(r), &caelum(&format!("{day}T{hhmm}:00+02:00"), sky)).is_ok()
    };
    assert!(open("earl", "03:00", None));
    assert!(open("king", "10:30", None) && open("king", "16:30", None));
    assert!(
        !open("king", "08:00", None)
            && !open("king", "13:30", None)
            && !open("king", "21:00", None)
    );
    assert!(
        open("marquis", "16:30", None)
            && open("marquis", "20:00", None)
            && open("marquis", "04:00", None)
    );
    assert!(!open("marquis", "10:00", None));
    assert!(open("duke", "09:00", Some(true)));
    assert!(
        !open("duke", "09:00", Some(false))
            && !open("duke", "09:00", None)
            && !open("duke", "15:00", Some(true))
    );
    assert!(open("prince", "12:00", None) && !open("prince", "20:15", None));
    assert!(open("knight", "06:15", None) || open("knight", "06:20", None));
    assert!(!open("knight", "12:00", None));
    assert!(!open("president", "12:00", None));
    let both = vec!["duke".to_string(), "earl".to_string()];
    assert!(admit(&both, &caelum(&format!("{day}T03:00:00+02:00"), None)).is_ok());
}

#[test]
fn the_moon_must_be_of_an_even_age_to_fourteen() {
    let earl = rank("earl");
    assert!(admit(&earl, &caelum("2024-04-12T20:00:00Z", None)).is_ok());
    let e = admit(&earl, &caelum("2024-04-11T20:00:00Z", None)).unwrap_err();
    assert!(e.contains("3 days old"), "{e}");
    assert!(admit(&earl, &caelum("2024-04-25T20:00:00Z", None)).is_err());
}

#[test]
fn without_a_place_only_earls_may_come() {
    let c = Caelum {
        hora: parse_hora("2024-04-13T14:00:00Z"),
        locus: None,
        serenum: Some(true),
    };
    assert!(admit(&rank("earl"), &c).is_ok());
    for r in ["king", "duke", "prince", "marquis", "knight"] {
        assert!(
            admit(&rank(r), &c)
                .unwrap_err()
                .contains("place is not declared"),
            "{r}"
        );
    }
    let unfixed = Caelum { hora: None, ..c };
    assert!(admit(&rank("earl"), &unfixed).is_err());
}

#[test]
fn times_and_places_are_read_strictly() {
    assert_eq!(parse_hora("1970-01-01T00:00:00Z"), Some(0));
    assert_eq!(
        parse_hora("2024-04-12T20:00:00+02:00"),
        parse_hora("2024-04-12T18:00:00Z")
    );
    assert_eq!(
        parse_hora("2024-04-12T18:00Z"),
        parse_hora("2024-04-12T18:00:00Z")
    );
    for bad in [
        "2024-04-12",
        "2024-13-01T00:00:00Z",
        "2024-04-12T18:00:00",
        "noon",
    ] {
        assert_eq!(parse_hora(bad), None, "{bad}");
    }
    assert_eq!(parse_locus("41.9, 12.5"), Some((41.9, 12.5)));
    assert_eq!(parse_locus("91,0"), None);
    assert_eq!(parse_locus("41.9"), None);
}
