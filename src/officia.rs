use crate::nucleus::Perm;

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
