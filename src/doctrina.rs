//! `DOCTRINA`: teaching an art, with a certificate (`testimonium`) that the
//! art's library checks (`library.md` §3, `offices.md` §2.2).
//!
//! A teaching answer is free prose, then a line `TESTIMONIUM`, then one claim
//! per line. A claim is a computation stated with its result, in one of the
//! forms below, and the library redoes it:
//!
//! - arithmetic: `numerus: EXPR = VALUE`, exact over the rationals;
//! - geometry: `geometria: FIGURE ARGS = VALUE`;
//! - astronomy, by the calendar and the Moon: `dies: … = VALUE`.
//!
//! The Operator asks in the same forms, with `?` for the value. The examiner
//! approves only when every claim asked is answered and every claim given is
//! true, so a true but idle certificate cannot pass for an answer.

use crate::nucleus::Verdict;

/// The line that opens the certificate.
pub const TESTIMONIUM: &str = "TESTIMONIUM";

/// The arts a teaching office may name in the register (`offices.md` §5.1).
pub const ARTES: [&str; 13] = [
    "grammatica",
    "dialectica",
    "rhetorica",
    "arithmetica",
    "geometria",
    "musica",
    "astronomia",
    "astrologia",
    "mechanica",
    "philosophia",
    "poetica",
    "chiromantia",
    "pyromantia",
];

/// The kinds of claim a library can check, and the art each belongs to.
pub const KINDS: [(&str, &str); 3] = [
    ("numerus", "arithmetica"),
    ("geometria", "geometria"),
    ("dies", "astronomia"),
];

/// The art a claim's kind belongs to.
pub fn art_of(kind: &str) -> Option<&'static str> {
    KINDS.iter().find(|(k, _)| *k == kind).map(|(_, a)| *a)
}

const MAX_CLAIM: usize = 200;

/// One claim, split into its kind, what is computed, and the value given
/// (`None` where the value is asked: `?`).
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Claim {
    pub kind: String,
    pub what: String,
    pub value: Option<String>,
}

fn normal(s: &str) -> String {
    s.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// Reads one line as a claim, if it has the form `kind: what = value`.
pub fn claim(line: &str) -> Option<Claim> {
    let line = line.trim();
    if line.len() > MAX_CLAIM {
        return None;
    }
    let (kind, rest) = line.split_once(':')?;
    let kind = kind.trim();
    art_of(kind)?;
    let (what, value) = rest.rsplit_once('=')?;
    let what = normal(what);
    let value = value.trim();
    if what.is_empty() || value.is_empty() {
        return None;
    }
    Some(Claim {
        kind: kind.to_string(),
        what,
        value: (value != "?").then(|| value.to_string()),
    })
}

/// The claims a question asks: every line or `;`-separated part of the form
/// `kind: what = ?`.
pub fn asked(quaestio: &str) -> Vec<Claim> {
    quaestio
        .split(['\n', ';'])
        .filter_map(claim)
        .filter(|c| c.value.is_none())
        .collect()
}

/// Splits an answer into its prose and its certificate's lines, if it has one.
pub fn certificate(text: &str) -> Option<Vec<&str>> {
    let mut lines = text.lines();
    lines.by_ref().find(|l| l.trim() == TESTIMONIUM)?;
    Some(lines.filter(|l| !l.trim().is_empty()).collect())
}

/// What the library makes of a claim with a value.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Proba {
    Verum,
    Falsum,
    /// The library cannot check it: an unknown form, or beyond its reach.
    Ignotum,
}

/// Checks one claim.
pub fn proba(c: &Claim) -> Proba {
    let Some(value) = &c.value else {
        return Proba::Ignotum;
    };
    let right = match c.kind.as_str() {
        "numerus" => numerus::check(&c.what, value),
        "geometria" => geometria::check(&c.what, value),
        "dies" => dies::check(&c.what, value),
        _ => None,
    };
    match right {
        Some(true) => Proba::Verum,
        Some(false) => Proba::Falsum,
        None => Proba::Ignotum,
    }
}

/// The true value of an asked claim, as the library writes it, if it can
/// compute one. Used for the simulated will and for tests.
pub fn solve(c: &Claim) -> Option<String> {
    match c.kind.as_str() {
        "numerus" => numerus::solve(&c.what),
        "geometria" => geometria::solve(&c.what),
        "dies" => dies::solve(&c.what),
        _ => None,
    }
}

/// The verdict on a teaching answer (`library.md` §3.1): with no certificate,
/// or one the library cannot check, `HOLD`; with a false claim, `REJECT`; with
/// every asked claim answered and every claim true, `APPROVE`. A claim in an
/// art the spirit does not teach cannot be checked for it, so it holds.
pub fn verdict(quaestio: &str, text: &str, artes: &[String]) -> Verdict {
    if text.ends_with(crate::officia::INCOMPLETE) {
        return Verdict::Hold;
    }
    let Some(lines) = certificate(text) else {
        return Verdict::Hold;
    };
    let mut given = Vec::new();
    let mut doubt = false;
    for line in lines {
        let Some(c) = claim(line) else {
            doubt = true;
            continue;
        };
        let taught = art_of(&c.kind).is_some_and(|a| artes.iter().any(|t| t == a));
        match proba(&c) {
            Proba::Falsum => return Verdict::Reject,
            Proba::Verum if taught => given.push(c),
            _ => doubt = true,
        }
    }
    let wanted = asked(quaestio);
    let answered = wanted
        .iter()
        .all(|w| given.iter().any(|g| g.kind == w.kind && g.what == w.what));
    if doubt || wanted.is_empty() || !answered {
        Verdict::Hold
    } else {
        Verdict::Approve
    }
}

/// Arithmetic, exact over the rationals with 128-bit parts. A result too large
/// for them cannot be checked, and holds.
pub mod numerus {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Q {
        pub n: i128,
        pub d: i128,
    }

    fn gcd(mut a: i128, mut b: i128) -> i128 {
        while b != 0 {
            (a, b) = (b, a % b);
        }
        a.abs()
    }

    impl Q {
        pub fn new(n: i128, d: i128) -> Option<Q> {
            if d == 0 {
                return None;
            }
            let g = gcd(n, d).max(1);
            let s = if d < 0 { -1 } else { 1 };
            Some(Q {
                n: n.checked_div(g)?.checked_mul(s)?,
                d: d.checked_div(g)?.checked_mul(s)?,
            })
        }
        fn add(self, o: Q) -> Option<Q> {
            Q::new(
                self.n
                    .checked_mul(o.d)?
                    .checked_add(o.n.checked_mul(self.d)?)?,
                self.d.checked_mul(o.d)?,
            )
        }
        fn neg(self) -> Option<Q> {
            Q::new(self.n.checked_neg()?, self.d)
        }
        fn mul(self, o: Q) -> Option<Q> {
            Q::new(self.n.checked_mul(o.n)?, self.d.checked_mul(o.d)?)
        }
        fn div(self, o: Q) -> Option<Q> {
            Q::new(self.n.checked_mul(o.d)?, self.d.checked_mul(o.n)?)
        }
        fn pow(self, e: i128) -> Option<Q> {
            if !(-64..=64).contains(&e) {
                return None;
            }
            let base = if e < 0 { Q::new(self.d, self.n)? } else { self };
            let mut out = Q { n: 1, d: 1 };
            for _ in 0..e.abs() {
                out = out.mul(base)?;
            }
            Some(out)
        }
        /// Rounded to `places` decimals, half away from zero, as a numerator
        /// over 10^places.
        fn rounded(self, places: u32) -> Option<i128> {
            let scale = 10i128.checked_pow(places)?;
            let x = self.n.checked_mul(scale)?;
            let (q, r) = (x / self.d, x % self.d);
            Some(if r.abs().checked_mul(2)? >= self.d {
                q + x.signum()
            } else {
                q
            })
        }
    }

    struct P<'a> {
        s: &'a [u8],
        i: usize,
    }

    impl P<'_> {
        fn ws(&mut self) {
            while self.s.get(self.i) == Some(&b' ') {
                self.i += 1;
            }
        }
        fn eat(&mut self, c: u8) -> bool {
            self.ws();
            if self.s.get(self.i) == Some(&c) {
                self.i += 1;
                true
            } else {
                false
            }
        }
        fn expr(&mut self, depth: u32) -> Option<Q> {
            let mut v = self.term(depth)?;
            loop {
                if self.eat(b'+') {
                    v = v.add(self.term(depth)?)?;
                } else if self.eat(b'-') {
                    v = v.add(self.term(depth)?.neg()?)?;
                } else {
                    return Some(v);
                }
            }
        }
        fn term(&mut self, depth: u32) -> Option<Q> {
            let mut v = self.power(depth)?;
            loop {
                if self.eat(b'*') {
                    v = v.mul(self.power(depth)?)?;
                } else if self.eat(b'/') {
                    v = v.div(self.power(depth)?)?;
                } else {
                    return Some(v);
                }
            }
        }
        fn power(&mut self, depth: u32) -> Option<Q> {
            let base = self.unary(depth)?;
            if self.eat(b'^') {
                let e = self.unary(depth)?;
                return if e.d == 1 { base.pow(e.n) } else { None };
            }
            Some(base)
        }
        fn unary(&mut self, depth: u32) -> Option<Q> {
            if depth > 32 {
                return None;
            }
            if self.eat(b'-') {
                return self.unary(depth + 1)?.neg();
            }
            if self.eat(b'(') {
                let v = self.expr(depth + 1)?;
                return self.eat(b')').then_some(v);
            }
            self.ws();
            let start = self.i;
            while self
                .s
                .get(self.i)
                .is_some_and(|c| c.is_ascii_digit() || *c == b'.')
            {
                self.i += 1;
            }
            decimal(std::str::from_utf8(&self.s[start..self.i]).ok()?)
        }
    }

    /// A plain decimal number, such as `12` or `3.25`.
    fn decimal(s: &str) -> Option<Q> {
        let (whole, frac) = s.split_once('.').unwrap_or((s, ""));
        if whole.is_empty()
            || !whole
                .bytes()
                .chain(frac.bytes())
                .all(|c| c.is_ascii_digit())
        {
            return None;
        }
        if s.contains('.') && frac.is_empty() {
            return None;
        }
        let digits = format!("{whole}{frac}");
        Q::new(
            digits.parse().ok()?,
            10i128.checked_pow(u32::try_from(frac.len()).ok()?)?,
        )
    }

    /// Evaluates an expression of numbers, `+ - * / ^` and brackets.
    pub fn eval(expr: &str) -> Option<Q> {
        let mut p = P {
            s: expr.as_bytes(),
            i: 0,
        };
        let v = p.expr(0)?;
        p.ws();
        (p.i == p.s.len()).then_some(v)
    }

    /// Whether `value` is the true value of `expr`. A whole number or a
    /// fraction `a/b` must be exact; a decimal must be the true value rounded
    /// to as many places as it gives.
    pub fn check(expr: &str, value: &str) -> Option<bool> {
        let truth = eval(expr)?;
        let value = value.trim();
        let (neg, body) = match value.strip_prefix('-') {
            Some(rest) => (true, rest.trim()),
            None => (false, value),
        };
        let sign = |q: Q| if neg { q.neg() } else { Some(q) };
        if let Some((a, b)) = body.split_once('/') {
            let q = sign(decimal(a.trim())?.div(decimal(b.trim())?)?)?;
            return Some(q == truth);
        }
        let q = sign(decimal(body)?)?;
        match body.split_once('.') {
            None => Some(q == truth),
            Some((_, frac)) => {
                let places = u32::try_from(frac.len()).ok()?;
                let given = q.n.checked_mul(10i128.checked_pow(places)?)? / q.d;
                Some(truth.rounded(places)? == given)
            }
        }
    }

    /// The exact value, as a whole number or a fraction `a/b`.
    pub fn solve(expr: &str) -> Option<String> {
        let q = eval(expr)?;
        Some(if q.d == 1 {
            q.n.to_string()
        } else {
            format!("{}/{}", q.n, q.d)
        })
    }
}

/// Geometry of the plane. Values are decimals, right to as many places as
/// they give (at most six).
pub mod geometria {
    fn numbers(args: &[&str]) -> Option<Vec<f64>> {
        args.iter()
            .map(|a| {
                let v: f64 = a.parse().ok()?;
                v.is_finite().then_some(v)
            })
            .collect()
    }

    /// The true value of a figure, if the form is known and the figure exists.
    pub fn value(what: &str) -> Option<f64> {
        let words: Vec<&str> = what.split_whitespace().collect();
        let v = match words.as_slice() {
            ["hypotenusa", rest @ ..] => match numbers(rest)?.as_slice() {
                [a, b] if *a >= 0.0 && *b >= 0.0 => a.hypot(*b),
                _ => return None,
            },
            ["distantia", rest @ ..] => match numbers(rest)?.as_slice() {
                [x1, y1, x2, y2] => (x2 - x1).hypot(y2 - y1),
                _ => return None,
            },
            ["area", "triangulum", rest @ ..] => match numbers(rest)?.as_slice() {
                [a, b, c] if *a > 0.0 && *b > 0.0 && *c > 0.0 => {
                    let s = (a + b + c) / 2.0;
                    let q = s * (s - a) * (s - b) * (s - c);
                    if q < 0.0 {
                        return None;
                    }
                    q.sqrt()
                }
                _ => return None,
            },
            ["area", "rectangulum", rest @ ..] => match numbers(rest)?.as_slice() {
                [a, b] if *a >= 0.0 && *b >= 0.0 => a * b,
                _ => return None,
            },
            ["area", "circulus", rest @ ..] => match numbers(rest)?.as_slice() {
                [r] if *r >= 0.0 => std::f64::consts::PI * r * r,
                _ => return None,
            },
            ["ambitus", "circulus", rest @ ..] => match numbers(rest)?.as_slice() {
                [r] if *r >= 0.0 => 2.0 * std::f64::consts::PI * r,
                _ => return None,
            },
            _ => return None,
        };
        (v.is_finite() && v.abs() < 1e12).then_some(v)
    }

    pub fn check(what: &str, given: &str) -> Option<bool> {
        let truth = value(what)?;
        let value = given.trim();
        let places = value.split_once('.').map_or(0, |(_, f)| f.len());
        if places > 6 {
            return None;
        }
        let given: f64 = value.parse().ok()?;
        let scale = 10f64.powi(places as i32);
        Some((truth * scale).round() == (given * scale).round() && given.is_finite())
    }

    /// The value to four places.
    pub fn solve(what: &str) -> Option<String> {
        Some(format!("{:.4}", value(what)?))
    }
}

/// The calendar and the Moon: Easter by the computus, weekdays, Julian dates
/// and the Moon's age, recomputed from the tables.
pub mod dies {
    const WEEK: [&str; 7] = [
        "Iovis", "Veneris", "Saturni", "Solis", "Lunae", "Martis", "Mercurii",
    ];

    /// A date `YYYY-MM-DD`, valid in the Gregorian calendar, or in the Julian
    /// one where `gregorian` is false.
    fn date(s: &str, gregorian: bool) -> Option<(i64, i64, i64)> {
        let mut parts = s.trim().splitn(3, '-');
        let y: i64 = parts.next()?.parse().ok()?;
        let m: i64 = parts.next()?.parse().ok()?;
        let d: i64 = parts.next()?.parse().ok()?;
        ((1..=9999).contains(&y)
            && (1..=12).contains(&m)
            && d >= 1
            && d <= month_len(y, m, gregorian))
        .then_some((y, m, d))
    }

    fn leap(y: i64, gregorian: bool) -> bool {
        y % 4 == 0 && (!gregorian || y % 100 != 0 || y % 400 == 0)
    }

    fn month_len(y: i64, m: i64, gregorian: bool) -> i64 {
        match m {
            2 if leap(y, gregorian) => 29,
            2 => 28,
            4 | 6 | 9 | 11 => 30,
            _ => 31,
        }
    }

    /// Days since 1970-01-01 of a proleptic Gregorian date.
    fn days(y: i64, m: i64, d: i64) -> i64 {
        let y = if m <= 2 { y - 1 } else { y };
        let era = y.div_euclid(400);
        let yoe = y - era * 400;
        let mp = (m + 9) % 12;
        let doy = (153 * mp + 2) / 5 + d - 1;
        let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
        era * 146_097 + doe - 719_468
    }

    fn civil(z: i64) -> (i64, i64, i64) {
        let z = z + 719_468;
        let era = z.div_euclid(146_097);
        let doe = z - era * 146_097;
        let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
        let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
        let mp = (5 * doy + 2) / 153;
        let d = doy - (153 * mp + 2) / 5 + 1;
        let m = if mp < 10 { mp + 3 } else { mp - 9 };
        (
            if m <= 2 {
                yoe + era * 400 + 1
            } else {
                yoe + era * 400
            },
            m,
            d,
        )
    }

    /// Days since 1970-01-01 of a Julian-calendar date.
    fn julian_days(y: i64, m: i64, d: i64) -> i64 {
        let a = (14 - m) / 12;
        let yy = y + 4800 - a;
        let mm = m + 12 * a - 3;
        let jdn = d + (153 * mm + 2) / 5 + 365 * yy + yy / 4 - 32_083;
        jdn - 2_440_588
    }

    fn fmt((y, m, d): (i64, i64, i64)) -> String {
        format!("{y:04}-{m:02}-{d:02}")
    }

    /// Easter Sunday in the Gregorian calendar (the anonymous Gregorian
    /// algorithm), from 1583.
    fn pascha(y: i64) -> Option<(i64, i64, i64)> {
        if !(1583..=9999).contains(&y) {
            return None;
        }
        let a = y % 19;
        let b = y / 100;
        let c = y % 100;
        let d = b / 4;
        let e = b % 4;
        let f = (b + 8) / 25;
        let g = (b - f + 1) / 3;
        let h = (19 * a + b - d - g + 15) % 30;
        let i = c / 4;
        let k = c % 4;
        let l = (32 + 2 * e + 2 * i - h - k) % 7;
        let m = (a + 11 * h + 22 * l) / 451;
        let month = (h + l - 7 * m + 114) / 31;
        let day = (h + l - 7 * m + 114) % 31 + 1;
        Some((y, month, day))
    }

    /// Easter Sunday in the Julian calendar (Meeus's Julian algorithm).
    fn pascha_iuliana(y: i64) -> Option<(i64, i64, i64)> {
        if !(1..=9999).contains(&y) {
            return None;
        }
        let a = y % 4;
        let b = y % 7;
        let c = y % 19;
        let d = (19 * c + 15) % 30;
        let e = (2 * a + 4 * b - d + 34) % 7;
        let month = (d + e + 114) / 31;
        let day = (d + e + 114) % 31 + 1;
        Some((y, month, day))
    }

    pub fn solve(what: &str) -> Option<String> {
        let words: Vec<&str> = what.split_whitespace().collect();
        Some(match words.as_slice() {
            ["pascha", y] => fmt(pascha(y.parse().ok()?)?),
            ["pascha", "iuliana", y] => fmt(pascha_iuliana(y.parse().ok()?)?),
            ["hebdomadis", d] => {
                let (y, m, d) = date(d, true)?;
                WEEK[days(y, m, d).rem_euclid(7) as usize].to_string()
            }
            ["iuliana", d] => {
                let (y, m, d) = date(d, false)?;
                fmt(civil(julian_days(y, m, d)))
            }
            ["luna", d] => {
                let (y, m, d) = date(d, true)?;
                let noon = (days(y, m, d) * 86_400 + 43_200) as f64;
                crate::horae::aetas_lunae(noon).to_string()
            }
            _ => return None,
        })
    }

    pub fn check(what: &str, value: &str) -> Option<bool> {
        let truth = solve(what)?;
        let value = value.trim();
        Some(match date(value, true).or_else(|| date(value, false)) {
            Some(d) => fmt(d) == truth,
            None => value.eq_ignore_ascii_case(&truth),
        })
    }
}
