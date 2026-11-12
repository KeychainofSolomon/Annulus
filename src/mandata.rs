use std::collections::BTreeSet;
use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};

use crate::lingua::exsequi::{Optiones, Responsum, Voluntas};
use crate::lingua::{check_source, json, run_source_with};
use crate::nucleus::{Outcome, Wand};
use crate::ritus::{faculty_of, no_wand, test_wand};
use crate::tabularium::{self as tab, Domus};

pub const HOUSE: &str = "annulus.domus";

pub const USAGE: &str = "usage: annulus <source.annulus>
       annulus ritus <program.annulus> [--virga-probationis] [--semen N] [--sors OUTCOME]
                     [--responsa FILE] [--acta FILE] [--domus DIR] [--mundus DIR]
       annulus confirma <fingerprint> [--virga-probationis] [--domus DIR]
       annulus petitiones [--virga-probationis] [--domus DIR]
       annulus --version";

pub const EXIT_USAGE: u8 = 2;
pub const EXIT_REFUSED: u8 = 2;

pub struct Usage(pub String);

#[derive(Default)]
struct Flags {
    test_wand: bool,
    semen: u64,
    sors: Option<Outcome>,
    responsa: Option<PathBuf>,
    acta: Option<PathBuf>,
    domus: Option<PathBuf>,
    mundus: Option<PathBuf>,
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
            "--acta" => f.acta = Some(value.into()),
            "--domus" => f.domus = Some(value.into()),
            "--mundus" => f.mundus = Some(value.into()),
            _ => unreachable!(),
        }
    }
    Ok((plain, f))
}

fn wand(f: &Flags) -> Wand {
    if f.test_wand {
        test_wand()
    } else {
        no_wand()
    }
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
            "--semen",
            "--sors",
            "--responsa",
            "--acta",
            "--domus",
            "--mundus",
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
    let wand = wand(&f);
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
    let opened = match domus.open(&wand) {
        Ok(o) => o,
        Err(e) => return Ok(refuse(&mut say, e)),
    };
    let before = match tab::read_world(&mundus, &places, &opened.marks) {
        Ok(w) => w,
        Err(e) => return Ok(refuse(&mut say, e)),
    };
    let options = Optiones {
        semen: f.semen,
        sors: f.sors,
        responsa,
        radices: opened.roots.clone(),
    };
    let exitus = match run_source_with(
        &source,
        wand.clone(),
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
    let acta = tab::audit_path(&program, f.acta.as_deref());
    match tab::write_audit(&acta, state.audit(), &wand, &notes) {
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
    if let Err(e) = domus.seal(state.stores(), opened.counts, &marks, &wand) {
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
    let (plain, f) = flags(args, &["--virga-probationis", "--domus"])?;
    let [fingerprint] = plain.as_slice() else {
        return Err(Usage("confirma takes exactly one fingerprint".into()));
    };
    let wand = wand(&f);
    if !wand.present {
        let _ = writeln!(err, "the second key is turned only with the wand");
        return Ok(EXIT_REFUSED);
    }
    let domus = Domus::new(&house(&f));
    if !domus.installed() {
        let _ = writeln!(err, "there are no stores in {}", domus.dir.display());
        return Ok(EXIT_REFUSED);
    }
    let mut opened = match domus.open(&wand) {
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
    let _ = writeln!(out, "confirmed: {fingerprint}");
    Ok(0)
}

pub fn petitiones(args: &[String], out: &mut dyn Write, err: &mut dyn Write) -> Result<u8, Usage> {
    let (plain, f) = flags(args, &["--virga-probationis", "--domus"])?;
    if !plain.is_empty() {
        return Err(Usage("petitiones takes no program".into()));
    }
    let domus = Domus::new(&house(&f));
    if !domus.installed() {
        return Ok(0);
    }
    let opened = match domus.open(&wand(&f)) {
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
