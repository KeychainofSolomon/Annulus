use crate::nucleus::{Effect, Perm, Verb};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Class {
    Telling,
    Working,
}

#[derive(Clone, Copy, Debug)]
pub struct Signature {
    pub class: Class,
    pub modes: &'static [&'static str],
    pub quaestio: bool,
}

pub fn signature(kind: &str) -> Option<Signature> {
    let telling = Signature {
        class: Class::Telling,
        modes: &[],
        quaestio: true,
    };
    let working = |modes| Signature {
        class: Class::Working,
        modes,
        quaestio: false,
    };
    Some(match kind {
        "PRAESENTIA" | "FUTURA" | "OCCULTA" | "THESAURI" | "AMISSA" | "RESPONSA" | "DOCTRINA" => {
            telling
        }
        "PRAETERITA" => telling,
        "VIRTUTES" => Signature {
            class: Class::Telling,
            modes: &[],
            quaestio: false,
        },
        "AEDIFICATIO" | "EVERSIO" | "CONCORDIA" | "DISCORDIA" => working(&[]),
        "VECTIO" => working(&["transfer", "exscribe"]),
        "FURTUM" => working(&["aufer"]),
        _ => return None,
    })
}

pub fn places(kind: &str, mode: Option<&str>, n: usize) -> Option<Vec<Vec<Perm>>> {
    use Perm::*;
    let all = |p: Perm| vec![vec![p]; n];
    match (kind, mode) {
        (
            "PRAESENTIA" | "FUTURA" | "OCCULTA" | "THESAURI" | "AMISSA" | "RESPONSA" | "VIRTUTES"
            | "PRAETERITA" | "DOCTRINA",
            _,
        ) => Some(all(Legere)),
        ("AEDIFICATIO", _) if n >= 1 => Some(all(Creare)),
        ("EVERSIO", _) if n >= 1 => Some(all(Delere)),
        ("VECTIO", Some("transfer")) | ("FURTUM", Some("aufer")) if n == 2 => {
            Some(vec![vec![Legere, Delere], vec![Creare]])
        }
        ("VECTIO", Some("exscribe")) if n == 2 => Some(vec![vec![Legere], vec![Creare]]),
        ("CONCORDIA", _) if n >= 2 => {
            let mut out = vec![vec![Legere]; n - 1];
            out.push(vec![Scribere]);
            Some(out)
        }
        ("DISCORDIA", _) if n >= 2 => {
            let mut out = vec![vec![Legere]];
            out.extend(vec![vec![Creare]; n - 1]);
            Some(out)
        }
        _ => None,
    }
}

pub fn shape(kind: &str, mode: Option<&str>) -> &'static str {
    match (kind, mode) {
        ("VECTIO" | "FURTUM", _) => "one source, then one new target",
        ("CONCORDIA", _) => "sources, then one target to write",
        ("DISCORDIA", _) => "one source, then new targets",
        ("AEDIFICATIO", _) | ("EVERSIO", _) => "at least one place",
        _ => "places",
    }
}

pub const INCOMPLETE: &str = " [...]";

pub const ABSENS: &str = "absens";

pub fn virtus(place: &str, data: Option<&str>) -> String {
    match data {
        Some(d) => format!("{place}: {} bytes, {} lines", d.len(), d.lines().count()),
        None => format!("{place}: {ABSENS}"),
    }
}

pub fn virtutes(places: &[String], read: impl Fn(&str) -> Option<String>) -> String {
    places
        .iter()
        .map(|p| virtus(p, read(p).as_deref()))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn same_account(given: &str, truth: &str) -> bool {
    let lines = |s: &str| {
        let mut v: Vec<String> = s
            .lines()
            .map(|l| l.trim().to_string())
            .filter(|l| !l.is_empty())
            .collect();
        v.sort();
        v
    };
    lines(given) == lines(truth)
}

pub const HISTORY_WORDS: [&str; 7] = [
    "invocatus",
    "recusatus",
    "petitum",
    "iudicatum",
    "commissum",
    "dimissus",
    "retentus",
];

pub fn account_of_past(given: &str, history: &[String]) -> Option<bool> {
    let given = given.strip_suffix(INCOMPLETE).unwrap_or(given);
    let claims: Vec<&str> = given
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect();
    if claims.is_empty() {
        return None;
    }
    Some(claims.iter().all(|c| history.iter().any(|h| h == c)))
}

pub fn cited(text: &str) -> Vec<String> {
    text.split(['[', '\u{ff3b}'])
        .skip(1)
        .filter_map(|rest| {
            let (inner, _) = rest.split_once([']', '\u{ff3d}']).unwrap_or((rest, ""));
            let inner = inner.replace(['\u{ff1a}', '\u{fe55}', '\u{a789}'], ":");
            let (word, place) = inner.split_once(':')?;
            let word: String = word.chars().filter(|c| !c.is_whitespace()).collect();
            matches!(
                word.to_lowercase().as_str(),
                "locus" | "loci" | "loco" | "locum"
            )
            .then(|| place.trim().to_string())
        })
        .collect()
}

pub fn merge(sources: &[String]) -> String {
    let split: Vec<Vec<&str>> = sources.iter().map(|s| s.lines().collect()).collect();
    let longest = split.iter().map(Vec::len).max().unwrap_or(0);
    (0..longest)
        .filter_map(|i| split.iter().find_map(|lines| lines.get(i).copied()))
        .collect::<Vec<&str>>()
        .join("\n")
}

pub fn expected(
    kind: &str,
    mode: Option<&str>,
    places: &[String],
    read: impl Fn(&str) -> String,
) -> Vec<Effect> {
    let effect = |verb, place: &String, data: String| Effect {
        verb,
        place: place.clone(),
        data,
    };
    match (kind, mode) {
        ("AEDIFICATIO", _) => places
            .iter()
            .map(|p| effect(Verb::Crea, p, String::new()))
            .collect(),
        ("EVERSIO", _) => places
            .iter()
            .map(|p| effect(Verb::Dele, p, String::new()))
            .collect(),
        ("VECTIO", Some("transfer")) if places.len() == 2 => vec![
            effect(Verb::Transfer, &places[1], read(&places[0])),
            effect(Verb::Dele, &places[0], String::new()),
        ],
        ("VECTIO", _) if places.len() == 2 => {
            vec![effect(Verb::Exscribe, &places[1], read(&places[0]))]
        }
        ("FURTUM", _) if places.len() == 2 => vec![
            effect(Verb::Exscribe, &places[1], read(&places[0])),
            effect(Verb::Aufer, &places[0], String::new()),
        ],
        ("CONCORDIA", _) if places.len() >= 2 => {
            let (sources, target) = places.split_at(places.len() - 1);
            let texts: Vec<String> = sources.iter().map(|p| read(p)).collect();
            vec![effect(Verb::Coniunge, &target[0], merge(&texts))]
        }
        ("DISCORDIA", _) if places.len() >= 2 => {
            let source = read(&places[0]);
            places[1..]
                .iter()
                .map(|p| effect(Verb::Scinde, p, source.clone()))
                .collect()
        }
        _ => Vec::new(),
    }
}
