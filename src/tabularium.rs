use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;
use std::fs;
use std::io::Write;
use std::path::{Component, Path, PathBuf};

use crate::nucleus::sha256::hash_hex;
use crate::nucleus::state::{fingerprint_effects, sign_for_test, verify, Chain, Roots};
use crate::nucleus::*;

pub const INDEX: &str = "annulus.proscriptio";
pub const VESSEL: &str = "annulus.vas";
pub const KEYS: &str = "annulus.claves";
pub const ROOTS: &str = "annulus.radices";
pub const PETITIONS: &str = "annulus.petitiones";
pub const MARKS: &str = "annulus.signa";
const STAGED: &str = ".annulus-novum";

pub struct Domus {
    pub dir: PathBuf,
}

pub struct Opened {
    pub stores: Stores,
    pub roots: Option<Roots>,
    pub counts: [usize; 3],
    pub marks: BTreeSet<Place>,
}

fn escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '\\' => out.push_str("\\\\"),
            '\t' => out.push_str("\\t"),
            '\n' => out.push_str("\\n"),
            '\r' => out.push_str("\\r"),
            _ => out.push(c),
        }
    }
    out
}

fn unescape(s: &str) -> Result<String, String> {
    let mut out = String::with_capacity(s.len());
    let mut chars = s.chars();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next() {
            Some('\\') => out.push('\\'),
            Some('t') => out.push('\t'),
            Some('n') => out.push('\n'),
            Some('r') => out.push('\r'),
            _ => return Err("a broken escape".into()),
        }
    }
    Ok(out)
}

fn join(fields: &[String]) -> String {
    fields
        .iter()
        .map(|f| escape(f))
        .collect::<Vec<_>>()
        .join("\t")
}

fn split(line: &str) -> Result<Vec<String>, String> {
    line.split('\t').map(unescape).collect()
}

fn verb_name(v: Verb) -> &'static str {
    match v {
        Verb::Scribe => "scribe",
        Verb::Crea => "crea",
        Verb::Dele => "dele",
        Verb::Exscribe => "exscribe",
        Verb::Transfer => "transfer",
        Verb::Coniunge => "coniunge",
        Verb::Scinde => "scinde",
        Verb::Aufer => "aufer",
    }
}

fn verb_of(s: &str) -> Result<Verb, String> {
    Ok(match s {
        "scribe" => Verb::Scribe,
        "crea" => Verb::Crea,
        "dele" => Verb::Dele,
        "exscribe" => Verb::Exscribe,
        "transfer" => Verb::Transfer,
        "coniunge" => Verb::Coniunge,
        "scinde" => Verb::Scinde,
        "aufer" => Verb::Aufer,
        _ => return Err(format!("an unknown verb {s}")),
    })
}

fn number(s: &str) -> Result<u64, String> {
    s.parse().map_err(|_| format!("not a number: {s}"))
}

pub trait Record: Sized + Debug {
    fn encode(&self) -> Vec<String>;
    fn decode(fields: &[String]) -> Result<Self, String>;
}

impl Record for IxEntry {
    fn encode(&self) -> Vec<String> {
        match self {
            IxEntry::Damn(s) => vec!["damn".into(), s.clone()],
            IxEntry::Proscribe(s, reach) => {
                let mut f = vec!["proscribe".into(), s.clone()];
                for r in reach {
                    f.push(verb_name(r.verb).into());
                    f.push(r.place.clone());
                }
                f
            }
            IxEntry::Lift(s) => vec!["lift".into(), s.clone()],
            IxEntry::Suspect(s) => vec!["suspect".into(), s.clone()],
            IxEntry::Clear(s) => vec!["clear".into(), s.clone()],
        }
    }

    fn decode(f: &[String]) -> Result<Self, String> {
        let seal = f.get(1).cloned().ok_or("an entry without a seal")?;
        Ok(match (f[0].as_str(), f.len()) {
            ("damn", 2) => IxEntry::Damn(seal),
            ("lift", 2) => IxEntry::Lift(seal),
            ("suspect", 2) => IxEntry::Suspect(seal),
            ("clear", 2) => IxEntry::Clear(seal),
            ("proscribe", n) if n % 2 == 0 => {
                let reach = f[2..]
                    .chunks(2)
                    .map(|c| {
                        Ok(Reach {
                            verb: verb_of(&c[0])?,
                            place: c[1].clone(),
                        })
                    })
                    .collect::<Result<Vec<_>, String>>()?;
                IxEntry::Proscribe(seal, reach)
            }
            _ => return Err(format!("an unknown entry of the Index: {}", f[0])),
        })
    }
}

impl Record for KeyEntry {
    fn encode(&self) -> Vec<String> {
        match self {
            KeyEntry::Petition(s) => vec!["petition".into(), s.clone()],
            KeyEntry::Confirmed(s) => vec!["confirmed".into(), s.clone()],
            KeyEntry::Spend(n) => vec!["spend".into(), n.to_string()],
        }
    }

    fn decode(f: &[String]) -> Result<Self, String> {
        match (f[0].as_str(), f.get(1), f.len()) {
            ("petition", Some(s), 2) => Ok(KeyEntry::Petition(s.clone())),
            ("confirmed", Some(s), 2) => Ok(KeyEntry::Confirmed(s.clone())),
            ("spend", Some(n), 2) => Ok(KeyEntry::Spend(number(n)? as usize)),
            _ => Err(format!(
                "an unknown entry of the sealed petitions: {}",
                f[0]
            )),
        }
    }
}

impl Record for Sealed {
    fn encode(&self) -> Vec<String> {
        match self {
            Sealed::History(s) => vec!["history".into(), s.clone()],
            Sealed::Datum(s) => vec!["datum".into(), s.clone()],
            Sealed::Content(pid, content) => {
                let mut f = vec!["content".into(), pid.to_string()];
                match content {
                    Content::Empty => f.push("empty".into()),
                    Content::Response(v, text) => {
                        f.push("response".into());
                        f.push(v.to_string());
                        f.push(text.clone());
                    }
                    Content::Work(effects) => {
                        f.push("work".into());
                        for e in effects {
                            f.push(verb_name(e.verb).into());
                            f.push(e.place.clone());
                            f.push(e.data.clone());
                        }
                    }
                }
                f
            }
        }
    }

    fn decode(f: &[String]) -> Result<Self, String> {
        match (f[0].as_str(), f.len()) {
            ("history", 2) => Ok(Sealed::History(f[1].clone())),
            ("datum", 2) => Ok(Sealed::Datum(f[1].clone())),
            ("content", n) if n >= 3 => {
                let pid = number(&f[1])?;
                let content = match (f[2].as_str(), n) {
                    ("empty", 3) => Content::Empty,
                    ("response", 5) => Content::Response(number(&f[3])?, f[4].clone()),
                    ("work", m) if (m - 3) % 3 == 0 => Content::Work(
                        f[3..]
                            .chunks(3)
                            .map(|c| {
                                Ok(Effect {
                                    verb: verb_of(&c[0])?,
                                    place: c[1].clone(),
                                    data: c[2].clone(),
                                })
                            })
                            .collect::<Result<Vec<_>, String>>()?,
                    ),
                    _ => return Err("a sealed content of unknown form".into()),
                };
                Ok(Sealed::Content(pid, content))
            }
            _ => Err(format!("an unknown entry of the Vessel: {}", f[0])),
        }
    }
}

fn digest_roots(roots: &Roots, marks: &str) -> String {
    hash_hex(
        format!(
            "radices|{}|{}|{}|{}",
            roots.index, roots.vessel, roots.keys, marks
        )
        .as_bytes(),
    )
}

fn encode_marks(marks: &BTreeSet<Place>) -> String {
    marks.iter().map(|p| format!("{}\n", escape(p))).collect()
}

fn decode_marks(text: &str) -> Result<BTreeSet<Place>, String> {
    text.lines()
        .map(|l| unescape(l).map_err(|e| format!("{MARKS}: {e}")))
        .collect()
}

impl Domus {
    pub fn new(dir: &Path) -> Domus {
        Domus {
            dir: dir.to_path_buf(),
        }
    }

    fn path(&self, name: &str) -> PathBuf {
        self.dir.join(name)
    }

    fn read_chain<T: Record>(&self, name: &str) -> Result<Chain<T>, String> {
        let mut chain = Chain::new();
        let path = self.path(name);
        let text = fs::read_to_string(&path)
            .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
        for (n, line) in text.lines().enumerate() {
            let (hash, rest) = line
                .split_once('\t')
                .ok_or_else(|| format!("{name}, line {}: no hash", n + 1))?;
            let fields = split(rest).map_err(|e| format!("{name}, line {}: {e}", n + 1))?;
            let entry = T::decode(&fields).map_err(|e| format!("{name}, line {}: {e}", n + 1))?;
            chain.push(entry);
            if chain.root() != hash {
                return Err(format!(
                    "{name} has been tampered with: line {} does not match its chain",
                    n + 1
                ));
            }
        }
        Ok(chain)
    }

    fn last_roots(&self, wand: &Wand) -> Result<(Roots, String), String> {
        let path = self.path(ROOTS);
        let text = fs::read_to_string(&path)
            .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
        let line = text
            .lines()
            .last()
            .ok_or("the record of the stores' roots is empty")?;
        let f = split(line)?;
        let [index, vessel, keys, marks, signer, signature] = f.as_slice() else {
            return Err("the record of the stores' roots is malformed".into());
        };
        let roots = Roots {
            index: index.clone(),
            vessel: vessel.clone(),
            keys: keys.clone(),
        };
        if !verify(signer, &digest_roots(&roots, marks), signature) {
            return Err("the record of the stores' roots is not signed".into());
        }
        if wand.present && *signer != wand.fingerprint {
            return Err("the stores were sealed by another wand".into());
        }
        Ok((roots, marks.clone()))
    }

    pub fn installed(&self) -> bool {
        [INDEX, VESSEL, KEYS, ROOTS, MARKS]
            .iter()
            .any(|n| self.path(n).exists())
    }

    pub fn open(&self, wand: &Wand) -> Result<Opened, String> {
        if !self.installed() {
            let stores = Stores::install(wand);
            return Ok(Opened {
                stores,
                roots: None,
                counts: [0, 0, 0],
                marks: BTreeSet::new(),
            });
        }
        let index: Chain<IxEntry> = self.read_chain(INDEX)?;
        let vessel: Chain<Sealed> = self.read_chain(VESSEL)?;
        let keys: Chain<KeyEntry> = self.read_chain(KEYS)?;
        let (roots, marks_digest) = self.last_roots(wand)?;
        let marks_text = match fs::read_to_string(self.path(MARKS)) {
            Ok(t) => t,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
            Err(e) => return Err(format!("cannot read {MARKS}: {e}")),
        };
        if hash_hex(marks_text.as_bytes()) != marks_digest {
            return Err("the record of quarantined places has been tampered with".into());
        }
        let marks = decode_marks(&marks_text)?;
        if index.root() != roots.index || vessel.root() != roots.vessel || keys.root() != roots.keys
        {
            return Err("the stores do not match the last signed record of their roots".into());
        }
        let counts = [index.len(), vessel.len(), keys.len()];
        Ok(Opened {
            stores: Stores {
                index,
                vessel,
                keys,
            },
            roots: Some(roots),
            counts,
            marks,
        })
    }

    fn append_chain<T: Record>(
        &self,
        name: &str,
        chain: &Chain<T>,
        from: usize,
    ) -> Result<(), String> {
        let mut prefix: Chain<T> = Chain::new();
        let mut lines = String::new();
        for (n, entry) in chain.entries().iter().enumerate() {
            let encoded = join(&entry.encode());
            let decoded = T::decode(&split(&encoded)?)?;
            prefix.push(decoded);
            if n >= from {
                lines.push_str(&format!("{}\t{}\n", prefix.root(), encoded));
            }
        }
        if prefix.root() != chain.root() {
            return Err(format!("{name}: an entry cannot be written faithfully"));
        }
        let path = self.path(name);
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|e| format!("cannot open {}: {e}", path.display()))?;
        file.write_all(lines.as_bytes())
            .and_then(|_| file.sync_all())
            .map_err(|e| format!("cannot write {}: {e}", path.display()))
    }

    pub fn seal(
        &self,
        stores: &Stores,
        counts: [usize; 3],
        marks: &BTreeSet<Place>,
        wand: &Wand,
    ) -> Result<(), String> {
        if !wand.present {
            return Err("without the wand the stores cannot be sealed".into());
        }
        fs::create_dir_all(&self.dir).map_err(|e| format!("cannot make the house: {e}"))?;
        self.append_chain(INDEX, &stores.index, counts[0])?;
        self.append_chain(VESSEL, &stores.vessel, counts[1])?;
        self.append_chain(KEYS, &stores.keys, counts[2])?;
        let marks_text = encode_marks(marks);
        let marks_digest = hash_hex(marks_text.as_bytes());
        replace_file(&self.path(MARKS), marks_text.as_bytes())?;
        let roots = stores.roots();
        let signature = sign_for_test(&wand.fingerprint, &digest_roots(&roots, &marks_digest));
        let line = join(&[
            roots.index,
            roots.vessel,
            roots.keys,
            marks_digest,
            wand.fingerprint.clone(),
            signature,
        ]);
        let path = self.path(ROOTS);
        let mut file = fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .map_err(|e| format!("cannot open {}: {e}", path.display()))?;
        file.write_all(format!("{line}\n").as_bytes())
            .and_then(|_| file.sync_all())
            .map_err(|e| format!("cannot write {}: {e}", path.display()))
    }

    pub fn note_petitions(&self, audit: &[Label]) -> Result<(), String> {
        let mut lines = String::new();
        for label in audit {
            if let Label::Commit(_, effects, CommitResult::AwaitsKey) = label {
                let what: Vec<String> = effects
                    .iter()
                    .map(|e| format!("{} {}", verb_name(e.verb), e.place))
                    .collect();
                lines.push_str(&format!(
                    "{}\t{}\n",
                    fingerprint_effects(effects),
                    escape(&what.join(", "))
                ));
            }
        }
        if lines.is_empty() {
            return Ok(());
        }
        let path = self.path(PETITIONS);
        fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
            .and_then(|mut f| f.write_all(lines.as_bytes()))
            .map_err(|e| format!("cannot write {}: {e}", path.display()))
    }

    pub fn petitions(&self, stores: &Stores) -> Vec<(String, String)> {
        let described: BTreeMap<String, String> = fs::read_to_string(self.path(PETITIONS))
            .unwrap_or_default()
            .lines()
            .filter_map(|l| l.split_once('\t'))
            .map(|(f, d)| (f.to_string(), unescape(d).unwrap_or_default()))
            .collect();
        let entries = stores.keys.entries();
        let mut last: BTreeMap<&String, (usize, bool)> = BTreeMap::new();
        for (n, e) in entries.iter().enumerate() {
            match e {
                KeyEntry::Petition(f) => {
                    last.insert(f, (n, true));
                }
                KeyEntry::Confirmed(f) => {
                    last.insert(f, (n, false));
                }
                KeyEntry::Spend(_) => {}
            }
        }
        let mut pending: Vec<(usize, &String)> = last
            .into_iter()
            .filter(|(_, (_, open))| *open)
            .map(|(f, (n, _))| (n, f))
            .collect();
        pending.sort();
        pending
            .into_iter()
            .map(|(_, f)| (f.clone(), described.get(f).cloned().unwrap_or_default()))
            .collect()
    }
}

pub fn audit_path(program: &Path, chosen: Option<&Path>) -> PathBuf {
    if let Some(p) = chosen {
        return p.to_path_buf();
    }
    let stem = program
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| "annulus".into());
    let dir = program.parent().unwrap_or(Path::new("."));
    let first = dir.join(format!("{stem}.acta"));
    if !first.exists() {
        return first;
    }
    (2u64..)
        .map(|n| dir.join(format!("{stem}.{n}.acta")))
        .find(|p| !p.exists())
        .unwrap_or(first)
}

pub fn write_audit(path: &Path, audit: &Chain<Label>, wand: &Wand) -> Result<(), String> {
    let mut text = String::new();
    if wand.test {
        text.push_str(
            "# made with the test wand (virga probationis): not valid for a real installation\n",
        );
    }
    let mut replay: Chain<&Label> = Chain::new();
    for label in audit.entries() {
        replay.push(label);
        text.push_str(&format!(
            "{}\t{}\n",
            replay.root(),
            escape(&format!("{label:?}"))
        ));
    }
    let mut file = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(path)
        .map_err(|e| format!("cannot create the audit {}: {e}", path.display()))?;
    file.write_all(text.as_bytes())
        .and_then(|_| file.sync_all())
        .map_err(|e| format!("cannot write the audit {}: {e}", path.display()))
}

pub fn lawful_place(place: &str) -> bool {
    let path = Path::new(place);
    !place.is_empty()
        && !place.contains(':')
        && !place.contains('\\')
        && !place.ends_with(STAGED)
        && path.components().all(|c| matches!(c, Component::Normal(_)))
}

fn through_link(root: &Path, place: &str) -> bool {
    let mut path = root.to_path_buf();
    Path::new(place).components().any(|c| {
        path.push(c);
        fs::symlink_metadata(&path).is_ok_and(|m| m.file_type().is_symlink())
    })
}

fn guard(root: &Path, place: &str) -> Result<PathBuf, String> {
    if !lawful_place(place) {
        return Err(format!(
            "the place {place:?} leaves the Circle's folder or is not a plain relative path"
        ));
    }
    if through_link(root, place) {
        return Err(format!(
            "the place {place:?} passes through a link and is refused"
        ));
    }
    Ok(root.join(place))
}

pub fn guard_places(root: &Path, house: &Path, places: &BTreeSet<String>) -> Result<(), String> {
    let root = fs::canonicalize(root)
        .map_err(|e| format!("cannot find the Circle's folder {}: {e}", root.display()))?;
    let house = fs::canonicalize(house)
        .map_err(|e| format!("cannot find the house {}: {e}", house.display()))?;
    for place in places {
        let path = guard(&root, place)?;
        if path.starts_with(&house) {
            return Err(format!(
                "the place {place:?} lies in the house of the stores"
            ));
        }
    }
    Ok(())
}

fn read_place(path: &Path, place: &str) -> Result<Option<String>, String> {
    if path.is_dir() {
        return Err(format!("the place {place:?} is a directory"));
    }
    match fs::read(path) {
        Ok(bytes) => String::from_utf8(bytes)
            .map(Some)
            .map_err(|_| format!("the place {place:?} is not UTF-8 text")),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(format!("cannot read the place {place:?}: {e}")),
    }
}

fn replace_file(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut staged = path.as_os_str().to_owned();
    staged.push(STAGED);
    let staged = PathBuf::from(staged);
    let written = fs::OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&staged)
        .and_then(|mut f| f.write_all(bytes).and_then(|_| f.sync_all()));
    if let Err(e) = written {
        if e.kind() != std::io::ErrorKind::AlreadyExists {
            let _ = fs::remove_file(&staged);
        }
        return Err(format!("cannot stage {}: {e}", path.display()));
    }
    fs::rename(&staged, path).map_err(|e| {
        let _ = fs::remove_file(&staged);
        format!("cannot write {}: {e}", path.display())
    })
}

pub fn read_world(
    root: &Path,
    places: &BTreeSet<String>,
    marks: &BTreeSet<Place>,
) -> Result<BTreeMap<Place, (String, Mark)>, String> {
    let mut world = BTreeMap::new();
    for place in places {
        let path = guard(root, place)?;
        if let Some(text) = read_place(&path, place)? {
            let mark = if marks.contains(place) {
                Mark::Quarantined
            } else {
                Mark::Outside
            };
            world.insert(place.clone(), (text, mark));
        }
    }
    Ok(world)
}

pub fn marks_after(
    marks: &BTreeSet<Place>,
    after: &BTreeMap<Place, (String, Mark)>,
) -> BTreeSet<Place> {
    let mut next = marks.clone();
    next.extend(
        after
            .iter()
            .filter(|(_, (_, m))| *m == Mark::Quarantined)
            .map(|(p, _)| p.clone()),
    );
    next
}

pub fn write_world(
    root: &Path,
    before: &BTreeMap<Place, (String, Mark)>,
    after: &BTreeMap<Place, (String, Mark)>,
    places: &BTreeSet<String>,
) -> Result<Vec<String>, String> {
    let mut changed = Vec::new();
    for place in places {
        let was = before.get(place).map(|(d, _)| d);
        let now = after.get(place).map(|(d, _)| d);
        if was == now {
            continue;
        }
        let path = guard(root, place)?;
        if read_place(&path, place)?.as_ref() != was {
            return Err(format!(
                "the place {place:?} changed on disk during the run and was left as it is"
            ));
        }
        match now {
            Some(data) => {
                if let Some(parent) = path.parent() {
                    fs::create_dir_all(parent)
                        .map_err(|e| format!("cannot make the folder for {place:?}: {e}"))?;
                }
                replace_file(&path, data.as_bytes())?;
                changed.push(format!("written: {place}"));
            }
            None => {
                fs::remove_file(&path)
                    .map_err(|e| format!("cannot delete the place {place:?}: {e}"))?;
                changed.push(format!("deleted: {place}"));
            }
        }
    }
    Ok(changed)
}
