use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::baculum::Baculum;
use crate::lingua::exsequi::{Optiones, Responsum, Voluntas};
use crate::lingua::{check_source, json, run_source_with};
use crate::nucleus::state::{digest_constitution, verify};
use crate::nucleus::Outcome;
use crate::ritus::{constitution, faculty_of, no_wand, test_wand};
use crate::tabularium::{self as tab, Domus};

pub const HOUSE: &str = "annulus.domus";

pub const USAGE: &str = "usage: annulus <source.annulus>
       annulus ritus <program.annulus> [--virga STICK | --virga-probationis] [--semen N] [--sors OUTCOME]
                     [--responsa FILE] [--acta FILE] [--domus DIR] [--mundus DIR]
                     [--hora 2026-11-12T15:00:00+01:00] [--locus LAT,LON] [--caelum serenum|turbidum]
                     [--tentus N@2026-11-12T15:00:00+01:00]
       annulus sanci [--virga STICK | --virga-probationis] [--domus DIR]
       annulus confirma <fingerprint> [--virga STICK | --virga-probationis] [--domus DIR]
       annulus petitiones [--virga STICK | --virga-probationis] [--domus DIR]
       annulus adopta [--virga STICK | --virga-probationis] [--domus DIR]
       annulus acta <audit.acta> [--virga STICK | --virga-probationis] [--domus DIR]
       annulus --version
The wand (made with `virga fac`) is found by itself when one is plugged in; --virga names
its drive, for example --virga E:\\. Without a wand nothing is sealed or written.";

/// The environment variable that may name the wand's stick.
pub const VIRGA_ENV: &str = "ANNULUS_VIRGA";

pub const EXIT_USAGE: u8 = 2;
pub const EXIT_REFUSED: u8 = 2;

pub struct Usage(pub String);

#[derive(Default)]
struct Flags {
    test_wand: bool,
    virga: Option<PathBuf>,
    semen: u64,
    sors: Option<Outcome>,
    responsa: Option<PathBuf>,
    acta: Option<PathBuf>,
    domus: Option<PathBuf>,
    mundus: Option<PathBuf>,
    caelum: crate::horae::Caelum,
}

fn outcome(word: &str) -> Option<Outcome> {
    Some(match word.to_ascii_lowercase().as_str() {
        "obsequium" => Outcome::Obsequium,
        "pars" => Outcome::Pars,
        "mora" => Outcome::Mora,
        "recusatio" => Outcome::Recusatio,
        "mendacium" => Outcome::Mendacium,
        "malitia" => Outcome::Malitia,
        _ => return None,
    })
}

fn flags(args: &[String], allowed: &[&str]) -> Result<(Vec<String>, Flags), Usage> {
    let mut f = Flags::default();
    let mut plain = Vec::new();
    let mut seen = BTreeSet::new();
    let mut it = args.iter();
    while let Some(a) = it.next() {
        if !a.starts_with("--") {
            plain.push(a.clone());
            continue;
        }
        if !allowed.contains(&a.as_str()) || !seen.insert(a.clone()) {
            return Err(Usage(format!(
                "the flag {a} is unknown here or given twice"
            )));
        }
        if a == "--virga-probationis" {
            f.test_wand = true;
            continue;
        }
        let value = it
            .next()
            .ok_or_else(|| Usage(format!("the flag {a} needs a value")))?;
        match a.as_str() {
            "--semen" => {
                f.semen = value
                    .parse()
                    .map_err(|_| Usage(format!("--semen needs a whole number, not {value}")))?
            }
            "--sors" => {
                f.sors = Some(outcome(value).ok_or_else(|| {
                    Usage(format!(
                        "--sors takes obsequium, pars, mora, recusatio, mendacium or malitia, not {value}"
                    ))
                })?)
            }
            "--responsa" => f.responsa = Some(value.into()),
            "--virga" => f.virga = Some(value.into()),
            "--acta" => f.acta = Some(value.into()),
            "--domus" => f.domus = Some(value.into()),
            "--mundus" => f.mundus = Some(value.into()),
            "--hora" => {
                f.caelum.hora = Some(crate::horae::parse_hora(value).ok_or_else(|| {
                    Usage(format!("--hora takes a time like 2026-11-12T15:00:00+01:00, not {value}"))
                })?)
            }
            "--locus" => {
                f.caelum.locus = Some(crate::horae::parse_locus(value).ok_or_else(|| {
                    Usage(format!("--locus takes latitude,longitude like 52.52,13.40, not {value}"))
                })?)
            }
            "--caelum" => {
                f.caelum.serenum = Some(match value.as_str() {
                    "serenum" => true,
                    "turbidum" => false,
                    _ => return Err(Usage(format!("--caelum takes serenum or turbidum, not {value}"))),
                })
            }
            "--tentus" => {
                f.caelum.tentus = Some(crate::horae::parse_tentus(value).ok_or_else(|| {
                    Usage(format!(
                        "--tentus takes the spirit's number and the hour of its calling, like 2@2026-11-12T15:00:00+01:00, not {value}"
                    ))
                })?)
            }
            _ => unreachable!(),
        }
    }
    Ok((plain, f))
}

/// Which of the wand's secrets an act needs.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Need {
    /// A rite: the master words (to sign the faculty at launch), then the rite words.
    Ritus,
    /// An act outside any rite: the master words only.
    Magister,
    /// Only reading: no words.
    Nihil,
}

fn find_virga(f: &Flags) -> Result<Option<PathBuf>, String> {
    if let Some(p) = &f.virga {
        return Ok(Some(p.clone()));
    }
    if let Some(p) = std::env::var_os(VIRGA_ENV) {
        return Ok(Some(PathBuf::from(p)));
    }
    match virga::quaere().as_slice() {
        [] => Ok(None),
        [one] => Ok(Some(one.clone())),
        _ => Err("more than one wand is plugged in: name the one you mean with --virga".into()),
    }
}

/// Takes up the wand: the test wand if asked for, else the real wand if one is plugged in
/// or named, with the words the act needs, else no wand.
fn wand(f: &Flags, need: Need, err: &mut dyn Write) -> Result<Baculum, String> {
    if f.test_wand {
        if f.virga.is_some() {
            return Err("--virga and --virga-probationis exclude each other".into());
        }
        return Ok(test_wand());
    }
    let Some(stick) = find_virga(f)? else {
        return Ok(no_wand());
    };
    let v = virga::Virga::aperi(&stick)?;
    let _ = writeln!(err, "the wand {}", virga::breviter(v.signum()));
    let _ = err.flush();
    // the prompts go straight to the terminal; what was said above must reach it first
    let _ = std::io::stderr().flush();
    let title = format!("Take up the wand {}", virga::breviter(v.signum()));
    let (m, r) = match need {
        Need::Ritus => {
            let mut w =
                virga::verba::roga_plura(&title, &["master words", "rite words"])?.into_iter();
            (w.next(), w.next())
        }
        Need::Magister => (
            Some(virga::verba::roga_plura(&title, &["master words"])?.remove(0)),
            None,
        ),
        Need::Nihil => (None, None),
    };
    Baculum::verum(v, m.as_deref(), r.as_deref())
}

/// Whether the house's ratification is the owner's signature on this Constitution.
fn ratified(wand: &Baculum) -> bool {
    wand.lex.as_ref().is_some_and(|lex| {
        wand.trusts(&lex.owner)
            && verify(
                &lex.owner,
                &digest_constitution(&constitution(), &wand.revocatae()),
                &lex.signature,
            )
    })
}

pub fn read_responsa(path: &Path) -> Result<Vec<Responsum>, String> {
    let text = fs::read_to_string(path)
        .map_err(|e| format!("cannot read the answers {}: {e}", path.display()))?;
    let root = json::parse(&text).map_err(|e| format!("{}: {e}", path.display()))?;
    let json::Json::List(items) = &root else {
        return Err(format!("{}: the answers must be a list", path.display()));
    };
    items
        .iter()
        .enumerate()
        .map(|(n, item)| {
            let field = |k: &str| {
                item.get(k)
                    .and_then(|v| v.text())
                    .map(str::to_string)
                    .ok_or_else(|| format!("{}: answer {}: no text {k:?}", path.display(), n + 1))
            };
            let spirit = item
                .get("spiritus")
                .and_then(|v| v.number())
                .filter(|x| x.fract() == 0.0 && *x >= 1.0 && *x <= 72.0)
                .ok_or_else(|| {
                    format!(
                        "{}: answer {}: \"spiritus\" must be a number from 1 to 72",
                        path.display(),
                        n + 1
                    )
                })?;
            Ok(Responsum {
                spirit: spirit as u32,
                office: field("officium")?,
                quaestio: field("quaestio")?,
                verum: field("verum")?,
                falsum: match item.get("falsum") {
                    None => None,
                    Some(_) => Some(field("falsum")?),
                },
            })
        })
        .collect()
}

fn read_program(path: &Path) -> Result<String, String> {
    use std::io::Read;
    let mut source = String::new();
    fs::File::open(path)
        .and_then(|f| {
            f.take(crate::lingua::lexis::MAX_SOURCE_BYTES as u64 + 1)
                .read_to_string(&mut source)
        })
        .map_err(|e| format!("cannot read {}: {e}", path.display()))?;
    if source.len() > crate::lingua::lexis::MAX_SOURCE_BYTES {
        return Err(format!(
            "{} is larger than {} bytes",
            path.display(),
            crate::lingua::lexis::MAX_SOURCE_BYTES
        ));
    }
    Ok(source)
}

fn render(path: &Path, errors: &[crate::lingua::Error]) -> String {
    errors
        .iter()
        .map(|e| format!("{}:{e}", path.display()))
        .collect::<Vec<_>>()
        .join("\n")
}

pub fn ritus(args: &[String], out: &mut dyn Write, err: &mut dyn Write) -> Result<u8, Usage> {
    ritus_with(args, out, err, None)
}

pub fn ritus_with(
    args: &[String],
    out: &mut dyn Write,
    err: &mut dyn Write,
    mut will: Option<&mut dyn Voluntas>,
) -> Result<u8, Usage> {
    let notes: Vec<String> = will
        .as_ref()
        .map(|w| {
            vec![format!(
                "the will of the spirits: {}; each recorded outcome is the model's own declaration, not a known fact",
                w.nomen()
            )]
        })
        .unwrap_or_default();
    let (plain, f) = flags(
        args,
        &[
            "--virga-probationis",
            "--virga",
            "--semen",
            "--sors",
            "--responsa",
            "--acta",
            "--domus",
            "--mundus",
            "--hora",
            "--locus",
            "--caelum",
            "--tentus",
        ],
    )?;
    let [program] = plain.as_slice() else {
        return Err(Usage("ritus takes exactly one program".into()));
    };
    let program = PathBuf::from(program);
    let mut say = |text: String| {
        let _ = writeln!(err, "{text}");
    };
    let source = match read_program(&program) {
        Ok(s) => s,
        Err(e) => {
            say(format!("error: {e}"));
            return Ok(1);
        }
    };
    let checked = match check_source(&source) {
        Ok(c) => c,
        Err(errors) => {
            say(render(&program, &errors));
            return Ok(1);
        }
    };
    let responsa = match &f.responsa {
        Some(p) => match read_responsa(p) {
            Ok(r) => r,
            Err(e) => {
                say(format!("error: {e}"));
                return Ok(1);
            }
        },
        None => Vec::new(),
    };
    let beside = program
        .parent()
        .filter(|p| !p.as_os_str().is_empty())
        .unwrap_or(Path::new("."))
        .to_path_buf();
    let mundus = f.mundus.clone().unwrap_or_else(|| beside.clone());
    let domus_dir = f.domus.clone().unwrap_or_else(|| beside.join(HOUSE));
    let (faculty, reserved, _) = faculty_of(&checked);
    let places: BTreeSet<String> = faculty
        .iter()
        .chain(reserved.iter())
        .map(|c| c.place.clone())
        .collect();
    let refuse = |say: &mut dyn FnMut(String), why: String| {
        say(format!("the launch is refused: {why}"));
        EXIT_REFUSED
    };
    if let Err(e) = fs::create_dir_all(&domus_dir) {
        return Ok(refuse(&mut say, format!("cannot make the house: {e}")));
    }
    if let Err(e) = tab::guard_places(&mundus, &domus_dir, &places) {
        return Ok(refuse(&mut say, e));
    }
    let domus = Domus::new(&domus_dir);
    let _held = match domus.lock() {
        Ok(l) => l,
        Err(e) => return Ok(refuse(&mut say, e)),
    };
    let taken = wand(&f, Need::Ritus, &mut std::io::stderr());
    let mut wand = match taken {
        Ok(w) => w,
        Err(e) => return Ok(refuse(&mut say, e)),
    };
    let opened = match domus.open(&mut wand) {
        Ok(o) => o,
        Err(e) => return Ok(refuse(&mut say, e)),
    };
    if wand.verum_est() && !ratified(&wand) {
        return Ok(refuse(
            &mut say,
            "this Constitution is not ratified by your wand for this house: ratify it once with `annulus sanci`".into(),
        ));
    }
    let before = match tab::read_world(&mundus, &places, &opened.marks) {
        Ok(w) => w,
        Err(e) => return Ok(refuse(&mut say, e)),
    };
    let options = Optiones {
        semen: f.semen,
        sors: f.sors,
        responsa,
        radices: opened.roots.clone(),
        caelum: crate::horae::Caelum {
            hora: Some(f.caelum.hora.unwrap_or_else(crate::horae::now)),
            ..f.caelum
        },
    };
    let mut notes = notes;
    notes.push(format!(
        "the sky: {} at {}, {}; the Moon {} days old",
        crate::horae::iso(options.caelum.hora.unwrap_or(0)),
        options
            .caelum
            .locus
            .map(|(a, b)| format!("{a},{b}"))
            .unwrap_or_else(|| "an undeclared place".into()),
        match options.caelum.serenum {
            Some(true) => "clear",
            Some(false) => "not clear",
            None => "the weather undeclared",
        },
        crate::horae::aetas_lunae(options.caelum.hora.unwrap_or(0) as f64)
    ));
    if let Some((n, t0)) = options.caelum.tentus {
        notes.push(format!(
            "spirit {n} is held since its calling at {}, and has not had licence to depart; its hours are those of its calling",
            crate::horae::iso(t0)
        ));
    }
    let exitus = match run_source_with(
        &source,
        wand,
        opened.stores,
        before.clone(),
        &options,
        will.take(),
    ) {
        Ok(e) => e,
        Err(errors) => {
            say(render(&program, &errors));
            return Ok(1);
        }
    };
    for line in &exitus.output {
        if writeln!(out, "{line}").is_err() {
            break;
        }
    }
    let _ = out.flush();
    if let Some(m) = &exitus.message {
        say(format!(
            "class {}: {}",
            exitus.class,
            crate::lingua::exsequi::visible(m).replace('\n', " ")
        ));
    }
    let mut class = exitus.class;
    let state = &exitus.state;
    let wand = &exitus.baculum;
    let acta = tab::audit_path(&program, f.acta.as_deref());
    let seq = match domus.next_seal() {
        Ok(n) => n,
        Err(e) => {
            say(format!("error: {e}"));
            say("the stores are not sealed and the world is not written".into());
            return Ok(class.max(1));
        }
    };
    match tab::write_audit(&acta, state.audit(), wand, &notes, seq) {
        Ok(()) => say(format!("audit: {}", acta.display())),
        Err(e) => {
            say(format!("error: {e}"));
            say("the stores are not sealed and the world is not written".into());
            return Ok(class.max(1));
        }
    }
    if !wand.present {
        say("without the wand nothing is sealed and nothing is written".into());
        return Ok(class);
    }
    let marks = tab::marks_after(&opened.marks, state.world());
    if let Err(e) = domus.seal(state.stores(), opened.counts, &marks, wand) {
        say(format!("error: {e}"));
        say("the world is not written".into());
        return Ok(class.max(1));
    }
    if let Err(e) = domus.note_petitions(state.audit().entries()) {
        say(format!("error: {e}"));
        class = class.max(1);
    }
    for (fingerprint, what) in domus.petitions(state.stores()) {
        say(format!("awaits the second key: {fingerprint} ({what})"));
    }
    match tab::write_world(&mundus, &before, state.world(), &places) {
        Ok(changed) => changed.into_iter().for_each(&mut say),
        Err(e) => {
            say(format!("error: {e}"));
            class = class.max(1);
        }
    }
    Ok(class)
}

fn house(f: &Flags) -> PathBuf {
    f.domus.clone().unwrap_or_else(|| PathBuf::from(HOUSE))
}

pub fn confirma(args: &[String], out: &mut dyn Write, err: &mut dyn Write) -> Result<u8, Usage> {
    let (plain, f) = flags(args, &["--virga-probationis", "--virga", "--domus"])?;
    let [fingerprint] = plain.as_slice() else {
        return Err(Usage("confirma takes exactly one fingerprint".into()));
    };
    let mut wand = match wand(&f, Need::Magister, err) {
        Ok(w) => w,
        Err(e) => {
            let _ = writeln!(err, "the wand is refused: {e}");
            return Ok(EXIT_REFUSED);
        }
    };
    if !wand.present {
        let _ = writeln!(err, "the second key is turned only with the wand");
        return Ok(EXIT_REFUSED);
    }
    let domus = Domus::new(&house(&f));
    if !domus.installed() {
        let _ = writeln!(err, "there are no stores in {}", domus.dir.display());
        return Ok(EXIT_REFUSED);
    }
    let _held = match domus.lock() {
        Ok(l) => l,
        Err(e) => {
            let _ = writeln!(err, "the stores are refused: {e}");
            return Ok(EXIT_REFUSED);
        }
    };
    let mut opened = match domus.open(&mut wand) {
        Ok(o) => o,
        Err(e) => {
            let _ = writeln!(err, "the stores are refused: {e}");
            return Ok(EXIT_REFUSED);
        }
    };
    let pending = domus
        .petitions(&opened.stores)
        .iter()
        .any(|(p, _)| p == fingerprint);
    let Some(what) = domus.faithful_description(fingerprint) else {
        let _ = writeln!(
            err,
            "the petition {fingerprint} has no faithful description: nothing is confirmed that cannot be read"
        );
        return Ok(EXIT_REFUSED);
    };
    if !pending || !opened.stores.confirm_petition(fingerprint, &wand) {
        let _ = writeln!(
            err,
            "no sealed petition awaits with the fingerprint {fingerprint}"
        );
        return Ok(EXIT_REFUSED);
    }
    if let Err(e) = domus.seal(&opened.stores, opened.counts, &opened.marks, &wand) {
        let _ = writeln!(err, "error: {e}");
        return Ok(1);
    }
    let _ = writeln!(out, "confirmed: {fingerprint}\t{what}");
    Ok(0)
}

pub fn adopta(args: &[String], out: &mut dyn Write, err: &mut dyn Write) -> Result<u8, Usage> {
    let (plain, f) = flags(args, &["--virga-probationis", "--virga", "--domus"])?;
    if !plain.is_empty() {
        return Err(Usage("adopta takes no program".into()));
    }
    let mut wand = match wand(&f, Need::Magister, err) {
        Ok(w) => w,
        Err(e) => {
            let _ = writeln!(err, "the wand is refused: {e}");
            return Ok(EXIT_REFUSED);
        }
    };
    if !wand.present {
        let _ = writeln!(err, "a house is adopted only with the wand");
        return Ok(EXIT_REFUSED);
    }
    let domus = Domus::new(&house(&f));
    let _held = match domus.lock() {
        Ok(l) => l,
        Err(e) => {
            let _ = writeln!(err, "the stores are refused: {e}");
            return Ok(EXIT_REFUSED);
        }
    };
    match domus.adopt(&mut wand) {
        Ok(()) => {
            let _ = writeln!(out, "adopted: {}", domus.dir.display());
            Ok(0)
        }
        Err(e) => {
            let _ = writeln!(err, "the house is not adopted: {e}");
            Ok(EXIT_REFUSED)
        }
    }
}

pub fn petitiones(args: &[String], out: &mut dyn Write, err: &mut dyn Write) -> Result<u8, Usage> {
    let (plain, f) = flags(args, &["--virga-probationis", "--virga", "--domus"])?;
    if !plain.is_empty() {
        return Err(Usage("petitiones takes no program".into()));
    }
    let domus = Domus::new(&house(&f));
    if !domus.installed() {
        return Ok(0);
    }
    let mut wand = match wand(&f, Need::Nihil, err) {
        Ok(w) => w,
        Err(e) => {
            let _ = writeln!(err, "the wand is refused: {e}");
            return Ok(EXIT_REFUSED);
        }
    };
    let opened = match domus.open(&mut wand) {
        Ok(o) => o,
        Err(e) => {
            let _ = writeln!(err, "the stores are refused: {e}");
            return Ok(EXIT_REFUSED);
        }
    };
    for (fingerprint, what) in domus.petitions(&opened.stores) {
        let _ = writeln!(out, "{fingerprint}\t{what}");
    }
    Ok(0)
}

/// `annulus acta`: reads an audit back and checks it whole: its chain, the signature of a
/// wand the house trusts, and the seal it was bound to (core audit 1, #5).
pub fn acta(args: &[String], out: &mut dyn Write, err: &mut dyn Write) -> Result<u8, Usage> {
    let (plain, f) = flags(args, &["--virga-probationis", "--virga", "--domus"])?;
    let [file] = plain.as_slice() else {
        return Err(Usage("acta takes exactly one audit".into()));
    };
    let domus = Domus::new(&house(&f));
    if !domus.installed() {
        let _ = writeln!(err, "there are no stores in {}", domus.dir.display());
        return Ok(EXIT_REFUSED);
    }
    let mut wand = match wand(&f, Need::Nihil, err) {
        Ok(w) => w,
        Err(e) => {
            let _ = writeln!(err, "the wand is refused: {e}");
            return Ok(EXIT_REFUSED);
        }
    };
    if let Err(e) = domus.open(&mut wand) {
        let _ = writeln!(err, "the stores are refused: {e}");
        return Ok(EXIT_REFUSED);
    }
    match domus.read_audit(Path::new(file), &wand) {
        Ok(a) => {
            let _ = writeln!(
                out,
                "{file}: whole; {} entries, root {}; bound to seal {}; signed by the wand {}",
                a.entries, a.root, a.seq, a.master
            );
            Ok(0)
        }
        Err(e) => {
            let _ = writeln!(err, "{file}: refused: {e}");
            Ok(EXIT_REFUSED)
        }
    }
}

/// `annulus sanci`: the owner ratifies the Constitution of this Annulus for a house, with
/// the master secret, outside any run (`authority.md` §2.5). Until then a real wand's
/// house refuses to launch. A revocation changes the Constitution's digest, so the
/// remaining wand ratifies again after one.
pub fn sanci(args: &[String], out: &mut dyn Write, err: &mut dyn Write) -> Result<u8, Usage> {
    let (plain, f) = flags(args, &["--virga-probationis", "--virga", "--domus"])?;
    if !plain.is_empty() {
        return Err(Usage("sanci takes no program".into()));
    }
    let mut wand = match wand(&f, Need::Magister, err) {
        Ok(w) => w,
        Err(e) => {
            let _ = writeln!(err, "the wand is refused: {e}");
            return Ok(EXIT_REFUSED);
        }
    };
    if !wand.present {
        let _ = writeln!(err, "the Constitution is ratified only with the wand");
        return Ok(EXIT_REFUSED);
    }
    let domus = Domus::new(&house(&f));
    let _held = match domus.lock() {
        Ok(l) => l,
        Err(e) => {
            let _ = writeln!(err, "the stores are refused: {e}");
            return Ok(EXIT_REFUSED);
        }
    };
    if let Err(e) = domus.open(&mut wand) {
        let _ = writeln!(err, "the stores are refused: {e}");
        return Ok(EXIT_REFUSED);
    }
    let c = constitution();
    let digest = digest_constitution(&c, &wand.revocatae());
    if let Err(e) = domus.ratify(&wand, &digest) {
        let _ = writeln!(err, "the Constitution is not ratified: {e}");
        return Ok(EXIT_REFUSED);
    }
    let _ = writeln!(
        out,
        "ratified: the Constitution, version {}, digest {digest}, for {}",
        c.version,
        domus.dir.display()
    );
    Ok(0)
}
