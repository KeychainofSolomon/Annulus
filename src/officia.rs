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
        "PRAESENTIA" | "FUTURA" | "OCCULTA" | "THESAURI" | "AMISSA" | "RESPONSA" => telling,
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
        ("PRAESENTIA" | "FUTURA" | "OCCULTA" | "THESAURI" | "AMISSA" | "RESPONSA", _) => {
            Some(all(Legere))
        }
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

pub fn cited(text: &str) -> Vec<String> {
    text.split("[locus: ")
        .skip(1)
        .filter_map(|rest| rest.split_once(']').map(|(p, _)| p.to_string()))
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
