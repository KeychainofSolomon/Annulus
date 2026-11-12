use std::f64::consts::PI;

pub const DAY: f64 = 86_400.0;
pub const LUNAR_DAYS: [u32; 7] = [2, 4, 6, 8, 10, 12, 14];

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Caelum {
    pub hora: Option<i64>,
    pub locus: Option<(f64, f64)>,
    pub serenum: Option<bool>,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sol {
    pub aurora: f64,
    pub ortus: f64,
    pub occasus: f64,
    pub crepusculum: f64,
}

impl Sol {
    pub fn hora(&self) -> f64 {
        (self.occasus - self.ortus) / 12.0
    }
}

fn rad(d: f64) -> f64 {
    d * PI / 180.0
}

fn deg(r: f64) -> f64 {
    r * 180.0 / PI
}

fn julian(unix: f64) -> f64 {
    unix / DAY + 2_440_587.5
}

fn unix(jd: f64) -> f64 {
    (jd - 2_440_587.5) * DAY
}

fn event(j_star: f64, lat: f64, altitude: f64) -> Option<(f64, f64)> {
    let m = (357.5291 + 0.985_600_28 * j_star).rem_euclid(360.0);
    let c = 1.9148 * rad(m).sin() + 0.02 * rad(2.0 * m).sin() + 0.0003 * rad(3.0 * m).sin();
    let lambda = (m + c + 180.0 + 102.9372).rem_euclid(360.0);
    let transit = 2_451_545.0 + j_star + 0.0053 * rad(m).sin() - 0.0069 * rad(2.0 * lambda).sin();
    let sin_d = rad(lambda).sin() * rad(23.4397).sin();
    let cos_d = sin_d.asin().cos();
    let cos_w = (rad(altitude).sin() - rad(lat).sin() * sin_d) / (rad(lat).cos() * cos_d);
    if !(-1.0..=1.0).contains(&cos_w) {
        return None;
    }
    let w = deg(cos_w.acos()) / 360.0;
    Some((unix(transit - w), unix(transit + w)))
}

pub fn local_day(t: f64, lon: f64) -> i64 {
    ((t + lon / 360.0 * DAY) / DAY).floor() as i64
}

pub fn sol(day: i64, lat: f64, lon: f64) -> Option<Sol> {
    let noon = day as f64 * DAY + DAY / 2.0 - lon / 360.0 * DAY;
    let j_star = julian(noon) - 2_451_545.0;
    let (ortus, occasus) = event(j_star, lat, -0.833)?;
    let (aurora, crepusculum) = event(j_star, lat, -6.0)?;
    Some(Sol {
        aurora,
        ortus,
        occasus,
        crepusculum,
    })
}

fn new_moon(k: f64) -> f64 {
    let t = k / 1236.85;
    let jde = 2_451_550.097_66 + 29.530_588_861 * k + 0.000_154_37 * t * t
        - 0.000_000_150 * t * t * t
        + 0.000_000_000_73 * t * t * t * t;
    let e = 1.0 - 0.002_516 * t - 0.000_007_4 * t * t;
    let m = rad(2.5534 + 29.105_356_70 * k - 0.000_001_4 * t * t);
    let mp = rad(201.5643 + 385.816_935_28 * k + 0.010_758_2 * t * t + 0.000_012_38 * t * t * t);
    let f = rad(160.7108 + 390.670_502_84 * k - 0.001_611_8 * t * t - 0.000_002_27 * t * t * t);
    let om = rad(124.7746 - 1.563_755_88 * k + 0.002_067_2 * t * t);
    let correction = -0.40720 * mp.sin()
        + 0.17241 * e * m.sin()
        + 0.01608 * (2.0 * mp).sin()
        + 0.01039 * (2.0 * f).sin()
        + 0.00739 * e * (mp - m).sin()
        - 0.00514 * e * (mp + m).sin()
        + 0.00208 * e * e * (2.0 * m).sin()
        - 0.00111 * (mp - 2.0 * f).sin()
        - 0.00057 * (mp + 2.0 * f).sin()
        + 0.00056 * e * (2.0 * mp + m).sin()
        - 0.00042 * (3.0 * mp).sin()
        + 0.00042 * e * (m + 2.0 * f).sin()
        + 0.00038 * e * (m - 2.0 * f).sin()
        - 0.00024 * e * (2.0 * mp - m).sin()
        - 0.00017 * om.sin()
        - 0.00007 * (mp + 2.0 * m).sin()
        + 0.00004 * (2.0 * mp - 2.0 * f).sin()
        + 0.00004 * (3.0 * m).sin()
        + 0.00003 * (mp + m - 2.0 * f).sin()
        + 0.00003 * (2.0 * mp + 2.0 * f).sin()
        - 0.00003 * (mp + m + 2.0 * f).sin()
        + 0.00003 * (mp - m + 2.0 * f).sin()
        - 0.00002 * (mp - m - 2.0 * f).sin()
        - 0.00002 * (3.0 * mp + m).sin()
        + 0.00002 * (4.0 * mp).sin();
    unix(jde + correction - 69.0 / DAY)
}

pub fn last_new_moon(t: f64) -> f64 {
    let jd = julian(t);
    let mut k = ((jd - 2_451_550.097_66) / 29.530_588_861).floor();
    while new_moon(k) > t {
        k -= 1.0;
    }
    while new_moon(k + 1.0) <= t {
        k += 1.0;
    }
    new_moon(k)
}

pub fn aetas_lunae(t: f64) -> u32 {
    ((t - last_new_moon(t)) / DAY).floor() as u32
}

fn hours(t: f64, lon: f64, lat: f64) -> Option<(Sol, f64)> {
    let s = sol(local_day(t, lon), lat, lon)?;
    Some((s, s.hora()))
}

pub fn window(rank: &str, t: f64, caelum: &Caelum) -> Result<(), String> {
    let Some((lat, lon)) = caelum.locus else {
        return match rank {
            "earl" => Ok(()),
            _ => Err("the place is not declared, so the hours cannot be known".into()),
        };
    };
    let in_day = |f: &dyn Fn(&Sol, f64) -> bool| -> Result<bool, String> {
        hours(t, lon, lat).map(|(s, h)| f(&s, h)).ok_or_else(|| {
            "the sun neither rises nor sets here today, so the hours cannot be known".into()
        })
    };
    let open = match rank {
        "earl" => true,
        "king" => in_day(&|s, h| {
            (t >= s.ortus + 2.0 * h && t < s.ortus + 6.0 * h)
                || (t >= s.ortus + 8.0 * h && t < s.occasus)
        })?,
        "marquis" => in_day(&|s, h| t < s.ortus || t >= s.ortus + 8.0 * h)?,
        "duke" => {
            if caelum.serenum != Some(true) {
                return Err(match caelum.serenum {
                    None => "the sky is undeclared, and a Duke comes only in clear weather".into(),
                    Some(_) => {
                        "the sky is not clear, and a Duke comes only in clear weather".into()
                    }
                });
            }
            in_day(&|s, h| t >= s.ortus && t < s.ortus + 6.0 * h)?
        }
        "prince" => in_day(&|s, _| t >= s.ortus && t < s.occasus)?,
        "knight" => {
            in_day(&|s, _| (t >= s.aurora && t < s.ortus) || (t >= s.occasus && t < s.crepusculum))?
        }
        "president" => {
            return Err(
                "a President comes only after his king is called, and the kings are not built yet"
                    .into(),
            )
        }
        _ => false,
    };
    if open {
        Ok(())
    } else {
        Err(format!("this is not the hour of a {rank}"))
    }
}

pub fn lunar(t: f64) -> Result<(), String> {
    let age = aetas_lunae(t);
    if LUNAR_DAYS.contains(&age) {
        Ok(())
    } else {
        Err(format!(
            "the Moon is {age} days old, and the work is done only when she is 2, 4, 6, 8, 10, 12 or 14 days old"
        ))
    }
}

impl Caelum {
    pub fn probatio() -> Caelum {
        Caelum {
            hora: Some(1_713_018_600),
            locus: Some((41.9, 12.5)),
            serenum: None,
        }
    }
}

pub fn iso(t: i64) -> String {
    let days = t.div_euclid(86_400);
    let secs = t.rem_euclid(86_400);
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z - era * 146_097;
    let yoe = (doe - doe / 1460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let d = doy - (153 * mp + 2) / 5 + 1;
    let m = if mp < 10 { mp + 3 } else { mp - 9 };
    let y = yoe + era * 400 + i64::from(m <= 2);
    format!(
        "{y:04}-{m:02}-{d:02}T{:02}:{:02}:{:02}Z",
        secs / 3600,
        secs % 3600 / 60,
        secs % 60
    )
}

pub fn admit(ranks: &[String], caelum: &Caelum) -> Result<(), String> {
    let t = caelum.hora.ok_or("the hour is not fixed")? as f64;
    lunar(t)?;
    let mut why = Vec::new();
    for rank in ranks {
        match window(rank, t, caelum) {
            Ok(()) => return Ok(()),
            Err(e) => why.push(e),
        }
    }
    Err(why.join("; "))
}

pub fn now() -> i64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs() as i64)
        .unwrap_or(0)
}

pub fn parse_hora(s: &str) -> Option<i64> {
    let s = s.trim();
    let (main, offset) = if let Some(m) = s.strip_suffix('Z') {
        (m, 0)
    } else if s.len() > 6 && matches!(&s[s.len() - 6..s.len() - 5], "+" | "-") {
        let (m, o) = s.split_at(s.len() - 6);
        let sign = if o.starts_with('-') { -1 } else { 1 };
        let (hh, mm) = o[1..].split_once(':')?;
        (
            m,
            sign * (hh.parse::<i64>().ok()? * 3600 + mm.parse::<i64>().ok()? * 60),
        )
    } else {
        return None;
    };
    let (date, time) = main.split_once('T')?;
    let d: Vec<i64> = date
        .split('-')
        .map(|x| x.parse().ok())
        .collect::<Option<_>>()?;
    let tm: Vec<i64> = time
        .split(':')
        .map(|x| x.parse().ok())
        .collect::<Option<_>>()?;
    let ([y, mo, da], [h, mi, se]) = (d.as_slice(), tm.as_slice()) else {
        let ([y, mo, da], [h, mi]) = (d.as_slice(), tm.as_slice()) else {
            return None;
        };
        return civil(*y, *mo, *da, *h, *mi, 0).map(|t| t - offset);
    };
    civil(*y, *mo, *da, *h, *mi, *se).map(|t| t - offset)
}

fn civil(y: i64, m: i64, d: i64, h: i64, mi: i64, s: i64) -> Option<i64> {
    if !(1..=12).contains(&m)
        || !(1..=31).contains(&d)
        || !(0..24).contains(&h)
        || !(0..60).contains(&mi)
        || !(0..61).contains(&s)
    {
        return None;
    }
    let (y2, m2) = if m <= 2 { (y - 1, m + 12) } else { (y, m) };
    let era = y2.div_euclid(400);
    let yoe = y2 - era * 400;
    let doy = (153 * (m2 - 3) + 2) / 5 + d - 1;
    let doe = yoe * 365 + yoe / 4 - yoe / 100 + doy;
    let days = era * 146_097 + doe - 719_468;
    Some(days * 86_400 + h * 3600 + mi * 60 + s)
}

pub fn parse_locus(s: &str) -> Option<(f64, f64)> {
    let (a, b) = s.split_once(',')?;
    let (lat, lon) = (a.trim().parse::<f64>().ok()?, b.trim().parse::<f64>().ok()?);
    ((-90.0..=90.0).contains(&lat) && (-180.0..=180.0).contains(&lon)).then_some((lat, lon))
}
