use std::collections::BTreeMap;
use std::sync::OnceLock;

use super::json::{self, Json};

pub const GOETIA: &str = include_str!("../../docs/registers/goetia.json");
const NOMINA: &str = include_str!("../../docs/registers/nomina.json");

pub const RITES: &[&str] = &[
    "goetia",
    "rudd",
    "arbatel",
    "heptameron",
    "clavicula",
    "iuratus",
    "janua",
];

pub const INK_CLASSES: &[&str] = &[
    "rex",
    "marchio",
    "dux",
    "princeps",
    "eques",
    "praeses",
    "comes",
    "vespertilio",
];

pub const INSTRUMENTS_REQUIRED: &[(u32, &[&str])] = &[
    (13, &["ANULUM", "VIRGAM"]),
    (28, &["ANULUM"]),
    (29, &["ANULUM"]),
];

#[derive(Clone, Debug)]
pub struct Spirit {
    pub n: u32,
    pub name: String,
    pub label: String,
    pub ranks: Vec<String>,
    pub offices: Vec<String>,
    /// The arts its teaching offices name (`offices.md` §5.1), in register
    /// order, each once.
    pub artes: Vec<String>,
    pub legions: u32,
    pub veracity: String,
    pub veracity_condition: String,
    pub malice: usize,
}

impl Spirit {
    pub fn seal(&self, rite: &str) -> String {
        format!("{rite}:{}:{}", self.n, self.label)
    }

    pub fn ink_class(&self) -> Option<&'static str> {
        match self.ranks.as_slice() {
            [rank] => rank_ink(rank),
            _ => None,
        }
    }

    pub fn holds(&self, office: &str) -> bool {
        self.offices.iter().any(|o| o == office)
    }

    pub fn only_president(&self) -> bool {
        self.ranks.iter().all(|r| r == "president")
    }

    pub fn instruments(&self) -> &'static [&'static str] {
        INSTRUMENTS_REQUIRED
            .iter()
            .find(|(n, _)| *n == self.n)
            .map(|(_, i)| *i)
            .unwrap_or(&[])
    }
}

pub fn rank_ink(rank: &str) -> Option<&'static str> {
    Some(match rank {
        "king" => "rex",
        "marquis" => "marchio",
        "duke" => "dux",
        "prince" => "princeps",
        "knight" => "eques",
        "president" => "praeses",
        "earl" => "comes",
        _ => return None,
    })
}

#[derive(Clone, Debug)]
pub struct Register {
    pub rite: &'static str,
    pub spirits: Vec<Spirit>,
    pub labels: BTreeMap<String, u32>,
}

impl Register {
    pub fn by_number(&self, n: i64) -> Option<&Spirit> {
        self.spirits.iter().find(|s| i64::from(s.n) == n)
    }

    pub fn by_label(&self, label: &str) -> Option<&Spirit> {
        self.labels
            .get(label)
            .and_then(|n| self.by_number(i64::from(*n)))
    }

    pub fn is_label(&self, name: &str) -> bool {
        self.labels.contains_key(name)
    }
}

pub fn label_of(name: &str) -> Option<String> {
    let label: String = name
        .to_lowercase()
        .chars()
        .map(|c| if c == '-' || c == ' ' { '_' } else { c })
        .collect();
    let mut chars = label.chars();
    let first = chars.next()?;
    (first.is_ascii_lowercase()
        && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_'))
    .then_some(label)
}

pub fn load(rite: &'static str, source: &str) -> Result<Register, String> {
    let root = json::parse(source)?;
    let mut spirits = Vec::new();
    let mut spellings: BTreeMap<String, Vec<u32>> = BTreeMap::new();
    for entry in root.list() {
        let n = entry
            .get("n")
            .and_then(Json::number)
            .ok_or("an entry without a number")? as u32;
        let name = entry
            .get("name")
            .and_then(Json::text)
            .ok_or("an entry without a name")?
            .to_string();
        let label = label_of(&name).ok_or_else(|| format!("no label for {name}"))?;
        let ranks = entry
            .get("rank")
            .and_then(Json::text)
            .unwrap_or("")
            .split('/')
            .map(str::to_string)
            .collect();
        let offices = entry
            .get("offices")
            .map(Json::list)
            .unwrap_or(&[])
            .iter()
            .filter_map(|o| o.get("kind").and_then(Json::text).map(str::to_string))
            .collect();
        let mut artes: Vec<String> = Vec::new();
        for office in entry.get("offices").map(Json::list).unwrap_or(&[]) {
            for art in office.get("artes").map(Json::list).unwrap_or(&[]) {
                if let Some(a) = art.text() {
                    if !artes.iter().any(|x| x == a) {
                        artes.push(a.to_string());
                    }
                }
            }
        }
        let aliases = entry
            .get("aliases")
            .map(Json::list)
            .unwrap_or(&[])
            .iter()
            .filter_map(Json::text)
            .filter_map(label_of);
        for spelling in std::iter::once(label.clone()).chain(aliases) {
            let owners = spellings.entry(spelling).or_default();
            if !owners.contains(&n) {
                owners.push(n);
            }
        }
        let text = |key: &str| {
            entry
                .get(key)
                .and_then(Json::text)
                .unwrap_or("")
                .to_string()
        };
        spirits.push(Spirit {
            n,
            name,
            label,
            ranks,
            offices,
            artes,
            legions: entry.get("legions").and_then(Json::number).unwrap_or(1.0) as u32,
            veracity: text("veracity"),
            veracity_condition: text("veracity_condition"),
            malice: entry.get("malice").map(Json::list).unwrap_or(&[]).len(),
        });
    }
    let mut labels = BTreeMap::new();
    for spirit in &spirits {
        labels.insert(spirit.label.clone(), spirit.n);
    }
    for (spelling, owners) in spellings {
        if let [only] = owners.as_slice() {
            labels.entry(spelling).or_insert(*only);
        }
    }
    Ok(Register {
        rite,
        spirits,
        labels,
    })
}

pub fn goetia() -> &'static Register {
    static REGISTER: OnceLock<Register> = OnceLock::new();
    REGISTER.get_or_init(|| load("goetia", GOETIA).expect("the goetia register is well formed"))
}

pub fn divine_names() -> &'static std::collections::BTreeSet<String> {
    static NAMES: OnceLock<std::collections::BTreeSet<String>> = OnceLock::new();
    NAMES.get_or_init(|| {
        let root = json::parse(NOMINA).expect("the register of names is well formed");
        root.list()
            .iter()
            .filter(|e| e.get("kind").and_then(Json::text) == Some("deus"))
            .flat_map(|e| e.get("forms").map(Json::list).unwrap_or(&[]).to_vec())
            .filter_map(|f| f.get("text").and_then(Json::text).map(str::to_string))
            .collect()
    })
}
