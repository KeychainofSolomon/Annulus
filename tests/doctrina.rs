use annulus::doctrina::{asked, claim, proba, solve, verdict, Proba};
use annulus::nucleus::Verdict;

fn p(line: &str) -> Proba {
    proba(&claim(line).unwrap_or_else(|| panic!("{line} is not a claim")))
}

#[test]
fn arithmetic_is_exact() {
    assert_eq!(p("numerus: 17 * 23 = 391"), Proba::Verum);
    assert_eq!(p("numerus: 17*23 = 392"), Proba::Falsum);
    assert_eq!(p("numerus: (1 + 2) ^ 10 - 59049 = 0"), Proba::Verum);
    assert_eq!(p("numerus: 7 / 2 = 7/2"), Proba::Verum);
    assert_eq!(p("numerus: 7 / 2 = 3.5"), Proba::Verum);
    assert_eq!(p("numerus: 7 / 2 = 3"), Proba::Falsum);
    assert_eq!(p("numerus: 1 / 3 = 0.33"), Proba::Verum);
    assert_eq!(p("numerus: 2 / 3 = 0.67"), Proba::Verum);
    assert_eq!(p("numerus: 2 / 3 = 0.66"), Proba::Falsum);
    assert_eq!(p("numerus: -2 / 3 = -0.67"), Proba::Verum);
    assert_eq!(p("numerus: 2 ^ -2 = 1/4"), Proba::Verum);
    assert_eq!(p("numerus: 0.1 + 0.2 = 0.3"), Proba::Verum);
    // Beyond reach, or not well formed: the library cannot say.
    assert_eq!(p("numerus: 1 / 0 = 0"), Proba::Ignotum);
    assert_eq!(p("numerus: 10 ^ 99 = 0"), Proba::Ignotum);
    assert_eq!(p("numerus: two and two = 4"), Proba::Ignotum);
    assert_eq!(
        solve(&claim("numerus: 22 / 7 = ?").unwrap()).unwrap(),
        "22/7"
    );
}

#[test]
fn geometry_is_right_to_the_places_given() {
    assert_eq!(p("geometria: hypotenusa 3 4 = 5"), Proba::Verum);
    assert_eq!(p("geometria: hypotenusa 1 1 = 1.4142"), Proba::Verum);
    assert_eq!(p("geometria: hypotenusa 1 1 = 1.4143"), Proba::Falsum);
    assert_eq!(p("geometria: area circulus 1 = 3.14"), Proba::Verum);
    assert_eq!(p("geometria: area triangulum 3 4 5 = 6"), Proba::Verum);
    assert_eq!(p("geometria: distantia 0 0 3 4 = 5.0"), Proba::Verum);
    assert_eq!(p("geometria: ambitus circulus 1 = 6.2832"), Proba::Verum);
    assert_eq!(p("geometria: area rectangulum 2 3 = 7"), Proba::Falsum);
    // No such triangle.
    assert_eq!(p("geometria: area triangulum 1 1 5 = 0"), Proba::Ignotum);
}

#[test]
fn the_calendar_and_the_moon_are_recomputed() {
    assert_eq!(p("dies: pascha 2026 = 2026-04-05"), Proba::Verum);
    assert_eq!(p("dies: pascha 2024 = 2024-03-31"), Proba::Verum);
    assert_eq!(p("dies: pascha 2026 = 2026-04-12"), Proba::Falsum);
    // Orthodox Easter 2026 is 12 April (Gregorian), 30 March in the Julian calendar.
    assert_eq!(p("dies: pascha iuliana 2026 = 2026-03-30"), Proba::Verum);
    assert_eq!(p("dies: hebdomadis 2026-11-13 = Veneris"), Proba::Verum);
    assert_eq!(p("dies: hebdomadis 1970-01-01 = Iovis"), Proba::Verum);
    assert_eq!(p("dies: hebdomadis 2026-11-13 = Lunae"), Proba::Falsum);
    // The Gregorian reform: Julian 5 October 1582 was Gregorian 15 October.
    assert_eq!(p("dies: iuliana 1582-10-05 = 1582-10-15"), Proba::Verum);
    // A Julian leap day the Gregorian calendar lacks.
    assert_eq!(p("dies: iuliana 1900-02-29 = 1900-03-13"), Proba::Verum);
    // The Moon was new at about 18:20 UTC on 2024-04-08 (the eclipse): at
    // noon the next day it is 0 days old; at noon that day, the old Moon's 29.
    assert_eq!(p("dies: luna 2024-04-09 = 0"), Proba::Verum);
    assert_eq!(p("dies: luna 2024-04-08 = 29"), Proba::Verum);
    assert_eq!(p("dies: luna 2024-04-09 = 14"), Proba::Falsum);
    assert_eq!(p("dies: pascha 1500 = 1500-04-19"), Proba::Ignotum);
}

fn artes(a: &[&str]) -> Vec<String> {
    a.iter().map(|s| s.to_string()).collect()
}

#[test]
fn the_verdict_follows_the_certificate() {
    let all = artes(&["arithmetica", "geometria", "astronomia"]);
    let q = "numerus: 17*23 = ?; geometria: hypotenusa 3 4 = ?";
    let good = "Hear me.\nTESTIMONIUM\nnumerus: 17*23 = 391\ngeometria: hypotenusa 3 4 = 5";
    assert_eq!(verdict(q, good, &all), Verdict::Approve);
    // Spacing does not matter.
    let spaced = "TESTIMONIUM\nnumerus:  17*23 = 391\ngeometria: hypotenusa  3  4 = 5";
    assert_eq!(verdict(q, spaced, &all), Verdict::Approve);
    // One false claim rejects, even an extra one nobody asked for.
    assert_eq!(
        verdict(q, &format!("{good}\nnumerus: 1+1 = 3"), &all),
        Verdict::Reject
    );
    // No certificate, a missing answer, or idle truths: held.
    assert_eq!(verdict(q, "It is 391, and 5.", &all), Verdict::Hold);
    assert_eq!(
        verdict(q, "TESTIMONIUM\nnumerus: 17*23 = 391", &all),
        Verdict::Hold
    );
    assert_eq!(
        verdict("teach me rhetoric", "TESTIMONIUM\nnumerus: 1+1 = 2", &all),
        Verdict::Hold
    );
    // An art the spirit does not teach cannot be checked for it.
    assert_eq!(verdict(q, good, &artes(&["arithmetica"])), Verdict::Hold);
    // Marked incomplete.
    assert_eq!(verdict(q, &format!("{good} [...]"), &all), Verdict::Hold);
    // A line that is no claim leaves doubt.
    assert_eq!(
        verdict(q, &format!("{good}\nand that is all"), &all),
        Verdict::Hold
    );
    assert_eq!(asked(q).len(), 2);
}
