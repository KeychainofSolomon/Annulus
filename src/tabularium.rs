use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Debug;
use std::fs;
use std::io::Write;
use std::path::{Component, Path, PathBuf};

use crate::baculum::{Baculum, Lex};
use crate::nucleus::sha256::hash_hex;
use crate::nucleus::state::{fingerprint_effects, verify, Chain, Roots};
use crate::nucleus::*;

pub const INDEX: &str = "annulus.proscriptio";
pub const VESSEL: &str = "annulus.vas";
pub const KEYS: &str = "annulus.claves";
pub const ROOTS: &str = "annulus.radices";
pub const PETITIONS: &str = "annulus.petitiones";
pub const MARKS: &str = "annulus.signa";
pub const LOCK: &str = "annulus.sera";
/// The house's copy of its wands' lineage (a real wand's house only).
pub const LINEAGE: &str = "annulus.virgae";
/// The owner's ratification of the Constitution.
pub const LEX: &str = "annulus.lex";
/// The journal of a seal in progress: the stores' lengths before it began, so an
/// interrupted seal is undone at the next opening (audit V06).
pub const JOURNAL: &str = "annulus.iter";
const STAGED: &str = ".annulus-novum";
pub const MAX_STORE: u64 = 64 << 20;
pub const MAX_PLACE: u64 = 8 << 20;

fn read_bounded(path: &Path, max: u64) -> std::io::Result<String> {
    let len = fs::metadata(path)?.len();
    if len > max {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("larger than {max} bytes"),
        ));
    }
    fs::read_to_string(path)
}

pub struct Domus {
    pub dir: PathBuf,
    ancorae: Option<PathBuf>,
}

/// Whom a house trusts to have sealed its stores.
enum Fides {
    /// No wand in hand and no lineage: any valid signature is read (nothing is sealed).
    Any,
    /// A house without a lineage, opened with this (test) wand.
    Wand(String),
    /// A real wand's house: its lineage decides.
    Stirps(virga::Stirps),
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
            c if crate::lingua::exsequi::hidden(c) => {
                out.push_str(&format!("\\u{{{:x}}}", c as u32))
            }
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
            Some('u') => {
                let rest: String = chars.by_ref().take_while(|c| *c != '}').collect();
                let code = rest
                    .strip_prefix('{')
                    .and_then(|h| u32::from_str_radix(h, 16).ok())
                    .and_then(char::from_u32)
                    .ok_or("a broken escape")?;
                out.push(code);
            }
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

/// The digest a roots record signs. `seq` is the record's line number, so a signed record
/// copied to the end of the file to replay older stores does not verify there.
fn digest_roots(roots: &Roots, marks: &str, seq: usize) -> String {
    hash_hex(
        format!(
            "radices|2|{seq}|{}|{}|{}|{}",
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

/// Test builds only: stops the program right after step `n` of a seal, as a power cut
/// would, when `ANNULUS_PROBATIO_CADE` is `n` (crash tests).
#[cfg(feature = "probatio")]
fn cade(n: u32) {
    if std::env::var("ANNULUS_PROBATIO_CADE")
        .ok()
        .and_then(|v| v.parse().ok())
        == Some(n)
    {
        std::process::exit(99);
    }
}

#[cfg(not(feature = "probatio"))]
fn cade(_: u32) {}

impl Domus {
    pub fn new(dir: &Path) -> Domus {
        Domus {
            dir: dir.to_path_buf(),
            ancorae: ancorae_default(),
        }
    }

    pub fn anchored(dir: &Path, ancorae: &Path) -> Domus {
        Domus {
            dir: dir.to_path_buf(),
            ancorae: Some(ancorae.to_path_buf()),
        }
    }

    fn anchor_path(&self) -> Result<PathBuf, String> {
        let base = self
            .ancorae
            .as_ref()
            .ok_or("no place outside the house to keep its anchor: set ANNULUS_ANCORAE")?;
        if !base.is_absolute() {
            return Err("the anchors' folder must be an absolute path".into());
        }
        let real = real_path(&self.dir)?;
        let name = real.to_string_lossy().to_lowercase();
        let base_real = real_path(base)?;
        if folded(&base_real).starts_with(&folded(&real)) {
            return Err("the anchors' folder lies inside the house it guards".into());
        }
        Ok(base.join(hash_hex(name.as_bytes())))
    }

    /// A real wand keeps the house's anchor on its own stick; otherwise the anchors'
    /// folder outside the house is used.
    fn anchor_path_for(&self, b: &Baculum) -> Result<PathBuf, String> {
        match b.ancorae() {
            Some(base) => {
                let real = real_path(&self.dir)?;
                let name = real.to_string_lossy().to_lowercase();
                Ok(base.join(hash_hex(name.as_bytes())))
            }
            None => self.anchor_path(),
        }
    }

    fn anchor(&self) -> Result<Option<String>, String> {
        self.anchor_at(&self.anchor_path()?)
    }

    fn anchor_at(&self, path: &Path) -> Result<Option<String>, String> {
        match read_bounded(path, MAX_STORE) {
            Ok(t) => Ok(Some(t.trim().to_string())),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(format!("cannot read the house's anchor: {e}")),
        }
    }

    /// Whether the anchor's line is in the record of roots: an anchor is `n<TAB>hash`,
    /// the number and hash of the last line this wand sealed (an older anchor holds only
    /// the hash of the last line).
    fn anchor_holds(&self, anchor: &str) -> Result<bool, String> {
        let path = self.path(ROOTS);
        let text = read_bounded(&path, MAX_STORE)
            .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
        let lines: Vec<&str> = text.lines().collect();
        let mut f = anchor.split('\t');
        Ok(match (f.next(), f.next()) {
            (Some(n), Some(h)) => n
                .parse::<usize>()
                .ok()
                .and_then(|n| n.checked_sub(1))
                .and_then(|i| lines.get(i))
                .is_some_and(|l| hash_hex(l.as_bytes()) == h),
            _ => lines
                .last()
                .is_some_and(|l| hash_hex(l.as_bytes()) == anchor),
        })
    }

    fn roots_count(&self) -> Result<usize, String> {
        let path = self.path(ROOTS);
        let text = read_bounded(&path, MAX_STORE)
            .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
        Ok(text.lines().count())
    }

    /// Writes the anchor: the number and hash of the last roots record, and for a real wand
    /// also the length and head of the lineage it has seen, so neither can later be shown
    /// to it rolled back.
    fn write_anchor(&self, b: &Baculum) -> Result<(), String> {
        let anchor = self.anchor_path_for(b)?;
        if let Some(parent) = anchor.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("cannot keep the anchor: {e}"))?;
        }
        let line = self.last_roots_line()?;
        let n = self.roots_count()?;
        let mut text = format!("{n}\t{}", hash_hex(line.as_bytes()));
        if let Some(st) = &b.stirps {
            let len = st.longitudo();
            text.push_str(&format!("\t{len}\t{}", st.caput(len).unwrap_or("")));
        }
        replace_file(&anchor, text.as_bytes())
    }

    /// Whether the lineage now known still contains the one the anchor recorded.
    fn anchor_lineage_holds(anchor: &str, st: &virga::Stirps) -> bool {
        let f: Vec<&str> = anchor.split('\t').collect();
        match f.as_slice() {
            [_, _, len, head] => len
                .parse::<usize>()
                .ok()
                .and_then(|n| st.caput(n))
                .is_some_and(|h| h == *head),
            _ => true,
        }
    }

    fn file_len(&self, name: &str) -> Result<u64, String> {
        match fs::metadata(self.path(name)) {
            Ok(m) => Ok(m.len()),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(0),
            Err(e) => Err(format!("cannot read {name}: {e}")),
        }
    }

    /// Undoes a seal that was interrupted: each store is cut back to its length before the
    /// seal, and the quarantine record restored. A journal can only cut back, so a planted
    /// one can at most roll the house back, which the wand's anchor then refuses.
    fn recover(&self) -> Result<(), String> {
        let path = self.path(JOURNAL);
        let text = match read_bounded(&path, MAX_STORE) {
            Ok(t) => t,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(()),
            Err(e) => return Err(format!("cannot read {JOURNAL}: {e}")),
        };
        let mut lines = text.lines();
        let lens: Vec<u64> = lines
            .next()
            .ok_or("the journal of an interrupted seal is empty")?
            .split('\t')
            .map(|n| {
                n.parse()
                    .map_err(|_| "the journal of an interrupted seal is malformed")
            })
            .collect::<Result<_, _>>()?;
        let [li, lv, lk, lr] = lens.as_slice() else {
            return Err("the journal of an interrupted seal is malformed".into());
        };
        let marks = lines
            .next()
            .ok_or("the journal of an interrupted seal is malformed")?;
        for (name, len) in [(INDEX, li), (VESSEL, lv), (KEYS, lk), (ROOTS, lr)] {
            let now = self.file_len(name)?;
            if now < *len {
                return Err(format!(
                    "{name} is shorter than before the interrupted seal: the house has been changed"
                ));
            }
            if now > *len {
                let f = fs::OpenOptions::new()
                    .write(true)
                    .open(self.path(name))
                    .map_err(|e| format!("cannot repair {name}: {e}"))?;
                f.set_len(*len)
                    .and_then(|_| f.sync_all())
                    .map_err(|e| format!("cannot repair {name}: {e}"))?;
            }
        }
        if marks == "-" {
            let _ = fs::remove_file(self.path(MARKS));
        } else {
            replace_file(&self.path(MARKS), unescape(marks)?.as_bytes())?;
        }
        fs::remove_file(&path).map_err(|e| format!("cannot clear {JOURNAL}: {e}"))
    }

    fn house_lineage(&self) -> Result<Option<virga::Stirps>, String> {
        match read_bounded(&self.path(LINEAGE), MAX_STORE) {
            Ok(t) => virga::Stirps::decode(&t)
                .map(Some)
                .map_err(|e| format!("the house's copy of the lineage is refused: {e}")),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(format!("cannot read {LINEAGE}: {e}")),
        }
    }

    /// Whom this house trusts to have sealed it, given the wand in hand. A real wand's
    /// lineage is joined with the house's copy, and the later one is written back to both.
    fn fides(&self, b: &mut Baculum) -> Result<Fides, String> {
        let house = self.house_lineage()?;
        if b.verum_est() {
            let mine = b.stirps.clone().ok_or("the wand has no lineage")?;
            let joined = match &house {
                Some(h) => virga::Stirps::concilia(&mine, h).map_err(|e| {
                    if h.genus() != mine.genus() {
                        "this house belongs to another Operator's wands".to_string()
                    } else {
                        e
                    }
                })?,
                None => {
                    if self.installed() {
                        return Err("this house was set up without a real wand (with the test wand, or before the wand was made): a real wand keeps only a house of its own, so make a new house".into());
                    }
                    mine.clone()
                }
            };
            if house.as_ref() != Some(&joined) {
                fs::create_dir_all(&self.dir).map_err(|e| format!("cannot make the house: {e}"))?;
                replace_file(&self.path(LINEAGE), joined.encode().as_bytes())?;
            }
            if joined != mine {
                if let Some(v) = b.virga.as_mut() {
                    v.scribe_stirpem(&joined)?;
                }
            }
            b.stirps = Some(joined.clone());
            if !joined.vivit(&b.fingerprint) {
                return Err("this wand has been revoked: it opens nothing".into());
            }
            Ok(Fides::Stirps(joined))
        } else if let Some(h) = house {
            if b.present {
                return Err("this house is kept by a real wand: the test wand is refused".into());
            }
            Ok(Fides::Stirps(h))
        } else if b.present {
            Ok(Fides::Wand(b.fingerprint.clone()))
        } else {
            Ok(Fides::Any)
        }
    }

    /// The owner's ratification of the Constitution, if the house has one.
    pub fn lex(&self) -> Result<Option<Lex>, String> {
        match read_bounded(&self.path(LEX), MAX_STORE) {
            Ok(t) => {
                let (owner, signature) = t
                    .trim()
                    .split_once('\t')
                    .ok_or("the ratification of the Constitution is malformed")?;
                Ok(Some(Lex {
                    owner: owner.to_string(),
                    signature: signature.to_string(),
                }))
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(format!("cannot read {LEX}: {e}")),
        }
    }

    /// Ratifies the Constitution: the master key signs its digest (outside any run).
    pub fn ratify(&self, b: &Baculum, digest: &str) -> Result<(), String> {
        let signature = b.signa_magister(digest)?;
        fs::create_dir_all(&self.dir).map_err(|e| format!("cannot make the house: {e}"))?;
        replace_file(
            &self.path(LEX),
            format!("{}\t{signature}\n", b.fingerprint).as_bytes(),
        )
    }

    fn last_roots_line(&self) -> Result<String, String> {
        let path = self.path(ROOTS);
        let text = read_bounded(&path, MAX_STORE)
            .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
        text.lines()
            .last()
            .map(str::to_string)
            .ok_or_else(|| "the record of the stores' roots is empty".into())
    }

    fn path(&self, name: &str) -> PathBuf {
        self.dir.join(name)
    }

    fn read_chain<T: Record>(&self, name: &str) -> Result<Chain<T>, String> {
        let mut chain = Chain::new();
        let path = self.path(name);
        let text = read_bounded(&path, MAX_STORE)
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

    fn last_roots(&self, fides: &Fides, adopting: bool) -> Result<(Roots, String), String> {
        let path = self.path(ROOTS);
        let text = read_bounded(&path, MAX_STORE)
            .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
        let line = text
            .lines()
            .last()
            .ok_or("the record of the stores' roots is empty")?;
        let f = split(line)?;
        let [index, vessel, keys, marks, master, signer, bond, signature] = f.as_slice() else {
            return Err("the record of the stores' roots is malformed, or of an older kind".into());
        };
        let roots = Roots {
            index: index.clone(),
            vessel: vessel.clone(),
            keys: keys.clone(),
        };
        if signer != master && !virga::vinculum_valet(master, signer, bond) {
            return Err(
                "the record of the stores' roots is signed by a key no wand vouches for".into(),
            );
        }
        if !verify(
            signer,
            &digest_roots(&roots, marks, text.lines().count()),
            signature,
        ) {
            return Err("the record of the stores' roots is not signed".into());
        }
        match fides {
            Fides::Any => {}
            Fides::Wand(w) => {
                if master != w {
                    return Err("the stores were sealed by another wand".into());
                }
            }
            Fides::Stirps(s) => {
                if !s.vivit(master) && !(adopting && s.revocata(master)) {
                    return Err(if s.revocata(master) {
                        "the stores were last sealed by a revoked wand: look them over, then adopt them with a remaining wand (`annulus adopta`)".into()
                    } else {
                        "the stores were sealed by a wand that is not of this house's lineage"
                            .into()
                    });
                }
            }
        }
        Ok((roots, marks.clone()))
    }

    pub fn installed(&self) -> bool {
        [INDEX, VESSEL, KEYS, ROOTS, MARKS]
            .iter()
            .any(|n| self.path(n).exists())
    }

    pub fn lock(&self) -> Result<fs::File, String> {
        fs::create_dir_all(&self.dir).map_err(|e| format!("cannot make the house: {e}"))?;
        let path = self.path(LOCK);
        let file = fs::OpenOptions::new()
            .create(true)
            .truncate(false)
            .write(true)
            .open(&path)
            .map_err(|e| format!("cannot open {}: {e}", path.display()))?;
        file.try_lock()
            .map_err(|_| "another run holds this house; wait for it to end".to_string())?;
        Ok(file)
    }

    /// Adopts a house the Operator has looked over: its stores are checked, then anchored
    /// to this wand. A real wand re-seals them with its master key, which also takes back
    /// a house last sealed by a wand since revoked.
    pub fn adopt(&self, wand: &mut Baculum) -> Result<(), String> {
        self.recover()?;
        if !self.installed() {
            return Err("there are no stores here to adopt".into());
        }
        let fides = self.fides(wand)?;
        if !wand.verum_est() {
            if self.anchor()?.is_some() {
                return Err("this house already has its anchor".into());
            }
            self.verified(&fides, false)?;
            return self.write_anchor(wand);
        }
        let opened = self.verified(&fides, true)?;
        self.seal(&opened.stores, opened.counts, &opened.marks, wand)
    }

    pub fn open(&self, wand: &mut Baculum) -> Result<Opened, String> {
        self.recover()?;
        let fides = self.fides(wand)?;
        wand.lex = self.lex()?;
        if wand.verum_est() {
            let anchor = self.anchor_at(&self.anchor_path_for(wand)?)?;
            if !self.installed() {
                if anchor.is_some() {
                    return Err("this house was sealed before, and its stores are gone: refusing to start afresh".into());
                }
                return Ok(Opened {
                    stores: Stores::install(wand),
                    roots: None,
                    counts: [0, 0, 0],
                    marks: BTreeSet::new(),
                });
            }
            // a house this wand has never seen, or whose anchor is gone, is not taken on
            // trust: the Operator looks it over and adopts it (audit V01)
            let a = anchor.ok_or(
                "this wand has no anchor for this house (it has not seen it before, or its anchor was removed): look the house over, then adopt it with `annulus adopta`",
            )?;
            if !self.anchor_holds(&a)? {
                return Err("the stores are not the ones this wand last saw: they have been rolled back or replaced".into());
            }
            if let Some(st) = &wand.stirps {
                if !Self::anchor_lineage_holds(&a, st) {
                    return Err("the wands' lineage is older than this wand last saw: it has been rolled back".into());
                }
            }
            let opened = self.verified(&fides, false)?;
            // the wand remembers the latest stores it has seen, whoever sealed them
            self.write_anchor(wand)?;
            return Ok(opened);
        }
        if matches!(fides, Fides::Stirps(_)) {
            // a real wand's house, opened without the wand: its anchors are on the wand,
            // and nothing will be sealed
            if !self.installed() {
                return Err("a real wand's house is opened only with the wand".into());
            }
            return self.verified(&fides, false);
        }
        let anchor = self.anchor()?;
        if self.installed() && anchor.is_none() {
            return Err(
                "this house has no anchor: if it is yours and untouched, adopt it once with `annulus adopta`"
                    .into(),
            );
        }
        if !self.installed() && anchor.is_some() {
            return Err(
                "this house was sealed before, and its stores are gone: refusing to start afresh"
                    .into(),
            );
        }
        if let Some(a) = &anchor {
            if !self.anchor_holds(a)? {
                return Err(
                    "the stores are not the last ones sealed: they have been rolled back or replaced"
                        .into(),
                );
            }
        }
        if !self.installed() {
            let stores = Stores::install(wand);
            return Ok(Opened {
                stores,
                roots: None,
                counts: [0, 0, 0],
                marks: BTreeSet::new(),
            });
        }
        self.verified(&fides, false)
    }

    fn verified(&self, fides: &Fides, adopting: bool) -> Result<Opened, String> {
        let index: Chain<IxEntry> = self.read_chain(INDEX)?;
        let vessel: Chain<Sealed> = self.read_chain(VESSEL)?;
        let keys: Chain<KeyEntry> = self.read_chain(KEYS)?;
        let (roots, marks_digest) = self.last_roots(fides, adopting)?;
        let marks_text = match read_bounded(&self.path(MARKS), MAX_STORE) {
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
        wand: &Baculum,
    ) -> Result<(), String> {
        if !wand.present {
            return Err("without the wand the stores cannot be sealed".into());
        }
        fs::create_dir_all(&self.dir).map_err(|e| format!("cannot make the house: {e}"))?;
        let old_marks = match read_bounded(&self.path(MARKS), MAX_STORE) {
            Ok(t) => escape(&t),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => "-".to_string(),
            Err(e) => return Err(format!("cannot read {MARKS}: {e}")),
        };
        let journal = format!(
            "{}\t{}\t{}\t{}\n{old_marks}\n",
            self.file_len(INDEX)?,
            self.file_len(VESSEL)?,
            self.file_len(KEYS)?,
            self.file_len(ROOTS)?
        );
        replace_file(&self.path(JOURNAL), journal.as_bytes())?;
        cade(1);
        self.append_chain(INDEX, &stores.index, counts[0])?;
        cade(2);
        self.append_chain(VESSEL, &stores.vessel, counts[1])?;
        cade(3);
        self.append_chain(KEYS, &stores.keys, counts[2])?;
        cade(4);
        let marks_text = encode_marks(marks);
        let marks_digest = hash_hex(marks_text.as_bytes());
        replace_file(&self.path(MARKS), marks_text.as_bytes())?;
        cade(5);
        let roots = stores.roots();
        let seq = match self.roots_count() {
            Ok(n) => n + 1,
            Err(_) if !self.path(ROOTS).exists() => 1,
            Err(e) => return Err(e),
        };
        let (signer, signature) = wand.signa_ritus(&digest_roots(&roots, &marks_digest, seq))?;
        let bond = if signer == wand.fingerprint {
            "-".to_string()
        } else {
            wand.vinculum().to_string()
        };
        let line = join(&[
            roots.index,
            roots.vessel,
            roots.keys,
            marks_digest,
            wand.fingerprint.clone(),
            signer,
            bond,
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
            .map_err(|e| format!("cannot write {}: {e}", path.display()))?;
        cade(6);
        // the seal is whole: the journal goes
        fs::remove_file(self.path(JOURNAL)).map_err(|e| format!("cannot clear {JOURNAL}: {e}"))?;
        cade(7);
        if let Some(s) = &wand.stirps {
            replace_file(&self.path(LINEAGE), s.encode().as_bytes())?;
        }
        cade(8);
        self.write_anchor(wand)?;
        cade(9);
        Ok(())
    }

    pub fn note_petitions(&self, audit: &[Label]) -> Result<(), String> {
        let mut lines = String::new();
        for label in audit {
            if let Label::Commit(_, effects, CommitResult::AwaitsKey) = label {
                let mut fields = vec![fingerprint_effects(effects)];
                for e in effects {
                    fields.push(verb_name(e.verb).to_string());
                    fields.push(e.place.clone());
                    fields.push(e.data.clone());
                }
                lines.push_str(&format!("{}\n", join(&fields)));
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

    pub fn faithful_description(&self, fingerprint: &str) -> Option<String> {
        read_bounded(&self.path(PETITIONS), MAX_STORE)
            .ok()?
            .lines()
            .filter_map(|l| described_petition(l).ok())
            .find(|(f, _)| f == fingerprint)
            .map(|(_, d)| d)
    }

    pub fn petitions(&self, stores: &Stores) -> Vec<(String, String)> {
        let described: BTreeMap<String, String> = read_bounded(&self.path(PETITIONS), MAX_STORE)
            .unwrap_or_default()
            .lines()
            .filter_map(|l| described_petition(l).ok())
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
            .map(|(_, f)| {
                (
                    f.clone(),
                    described.get(f).cloned().unwrap_or_else(|| {
                        "no faithful description: do not confirm what you cannot read".into()
                    }),
                )
            })
            .collect()
    }
}

fn described_petition(line: &str) -> Result<(String, String), String> {
    let fields = split(line)?;
    let (fp, rest) = fields.split_first().ok_or("an empty petition line")?;
    if rest.is_empty() || rest.len() % 3 != 0 {
        return Err("a malformed petition line".into());
    }
    let effects = rest
        .chunks(3)
        .map(|c| {
            Ok(Effect {
                verb: verb_of(&c[0])?,
                place: c[1].clone(),
                data: c[2].clone(),
            })
        })
        .collect::<Result<Vec<_>, String>>()?;
    if fingerprint_effects(&effects) != *fp {
        return Err("a petition's description does not match its fingerprint".into());
    }
    let what: Vec<String> = effects
        .iter()
        .map(|e| format!("{} {}", verb_name(e.verb), e.place))
        .collect();
    Ok((fp.clone(), what.join(", ")))
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

pub fn write_audit(
    path: &Path,
    audit: &Chain<Label>,
    wand: &Wand,
    notes: &[String],
) -> Result<(), String> {
    let mut text = String::new();
    if wand.test {
        text.push_str(
            "# made with the test wand (virga probationis): not valid for a real installation\n",
        );
    }
    for note in notes {
        text.push_str(&format!("# {}\n", escape(note)));
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
        && place.split('/').all(plain_name)
}

fn plain_name(name: &str) -> bool {
    const DEVICES: [&str; 22] = [
        "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7", "COM8",
        "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
    ];
    let stem = name
        .split('.')
        .next()
        .unwrap_or("")
        .trim_end()
        .to_uppercase();
    !name.is_empty()
        && !name.ends_with('.')
        && !name.ends_with(' ')
        && !name.starts_with(' ')
        && !name
            .chars()
            .any(|c| c.is_control() || "~*?\"<>|".contains(c))
        && !DEVICES.contains(&stem.as_str())
}

fn folded(path: &Path) -> Vec<String> {
    path.components()
        .map(|c| c.as_os_str().to_string_lossy().to_lowercase())
        .collect()
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
        if path.starts_with(&house) || folded(&path).starts_with(&folded(&house)) {
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
    if fs::metadata(path).is_ok_and(|m| m.len() > MAX_PLACE) {
        return Err(format!(
            "the place {place:?} is larger than {MAX_PLACE} bytes"
        ));
    }
    match fs::read(path) {
        Ok(bytes) => String::from_utf8(bytes)
            .map(Some)
            .map_err(|_| format!("the place {place:?} is not UTF-8 text")),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
        Err(e) => Err(format!("cannot read the place {place:?}: {e}")),
    }
}

fn real_path(path: &Path) -> Result<PathBuf, String> {
    let absolute =
        std::path::absolute(path).map_err(|e| format!("cannot name {}: {e}", path.display()))?;
    let mut rest = Vec::new();
    let mut cur = absolute.as_path();
    loop {
        if let Ok(real) = fs::canonicalize(cur) {
            return Ok(rest.iter().rev().fold(real, |acc: PathBuf, n| acc.join(n)));
        }
        let name = cur
            .file_name()
            .ok_or_else(|| format!("cannot find any part of {}", path.display()))?;
        rest.push(name.to_os_string());
        cur = cur
            .parent()
            .ok_or_else(|| format!("cannot find any part of {}", path.display()))?;
    }
}

pub fn ancorae_default() -> Option<PathBuf> {
    if let Some(d) = std::env::var_os("ANNULUS_ANCORAE") {
        return Some(PathBuf::from(d));
    }
    if let Some(d) = std::env::var_os("APPDATA") {
        return Some(PathBuf::from(d).join("Annulus").join("ancorae"));
    }
    std::env::var_os("HOME").map(|d| PathBuf::from(d).join(".annulus").join("ancorae"))
}

fn replace_file(path: &Path, bytes: &[u8]) -> Result<(), String> {
    let mut staged = path.as_os_str().to_owned();
    staged.push(STAGED);
    let staged = PathBuf::from(staged);
    if fs::symlink_metadata(&staged).is_ok() {
        fs::remove_file(&staged).map_err(|e| {
            format!(
                "cannot clear a stale staging file for {}: {e}",
                path.display()
            )
        })?;
    }
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
