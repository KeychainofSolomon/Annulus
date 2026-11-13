//! The wand in the Operator's hand (`baculum`): the nucleus's view of it ([`Wand`]) and the
//! keys it holds for this one act.
//!
//! - The **master** key is unlocked with the master words only for acts outside any rite:
//!   at launch, to sign the faculty before any Circle opens, and for the second key, the
//!   ratification of the Constitution and the adoption of a house. [`launch`] takes it and
//!   drops it before the nucleus starts, so it is never held while a spirit is invoked
//!   (`operator.md` §8.3); the nucleus refuses to launch while it is held.
//! - The **rite** key is unlocked with the rite words and is held for the run: it is what
//!   makes the wand present for `GERE VIRGAM`, and it seals the stores afterwards.
//!
//! Every signature is Ed25519 (`virga`). The **test wand** (`virga probationis`) uses the
//! same signatures with keys grown from a published seed: anyone can forge it, it says so
//! on every audit, and a house kept by a real wand refuses it.
//!
//! [`launch`]: crate::ritus::launch

use std::ops::Deref;
use std::path::PathBuf;

use crate::nucleus::sha256::digest as hash;
use crate::nucleus::Wand;

/// Every message Annulus signs is a digest of its own, inside this frame, so that no
/// signature made for Annulus means anything to the wand's own lineage, and the reverse.
pub fn message(digest: &str) -> Vec<u8> {
    format!("annulus|1|{digest}").into_bytes()
}

pub fn signa(key: &virga::Clavis, digest: &str) -> String {
    key.signa(&message(digest))
}

/// The ratification of the Constitution kept in a house: which wand signed, and its
/// signature on the Constitution's digest.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Lex {
    pub owner: String,
    pub signature: String,
}

fn probatio_key(name: &str) -> virga::Clavis {
    virga::Clavis::ex_semine(hash(format!("annulus-virga-probationis|{name}").as_bytes()))
}

pub struct Baculum {
    wand: Wand,
    ritus: String,
    vinculum: String,
    master: Option<virga::Clavis>,
    rite: Option<virga::Clavis>,
    probatio: Option<String>,
    /// The wand on its stick (real wands only).
    pub virga: Option<virga::Virga>,
    /// The lineage this wand follows, joined with the house's copy once a house is opened.
    pub stirps: Option<virga::Stirps>,
    /// The house's ratification of the Constitution, once read.
    pub lex: Option<Lex>,
    /// The binding made at a calling (§8.8): its key, and the master's word for it.
    ligatio: Option<(virga::Clavis, crate::nucleus::state::Ligatio)>,
}

impl std::fmt::Debug for Baculum {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Baculum")
            .field("wand", &self.wand)
            .field("master_held", &self.master.is_some())
            .field("rite_held", &self.rite.is_some())
            .finish()
    }
}

impl Deref for Baculum {
    type Target = Wand;
    fn deref(&self) -> &Wand {
        &self.wand
    }
}

impl Baculum {
    /// No wand: no faculty, nothing sealed, nothing written.
    pub fn nullum() -> Baculum {
        Baculum {
            wand: Wand {
                fingerprint: String::new(),
                present: false,
                master_unlocked: false,
                test: false,
            },
            ritus: String::new(),
            vinculum: String::new(),
            master: None,
            rite: None,
            probatio: None,
            virga: None,
            stirps: None,
            lex: None,
            ligatio: None,
        }
    }

    /// The test wand, accepted only with `--virga-probationis`.
    pub fn probationis() -> Baculum {
        Baculum::probationis_nominis("virga-probationis")
    }

    /// Another test wand, with its own published keys (for tests of "another wand").
    pub fn probationis_nominis(name: &str) -> Baculum {
        let master = probatio_key(&format!("{name}|magister"));
        let rite = probatio_key(&format!("{name}|ritus"));
        let ritus = rite.publica();
        Baculum {
            wand: Wand {
                fingerprint: master.publica(),
                present: true,
                master_unlocked: false,
                test: true,
            },
            vinculum: virga::vincula(&master, &ritus),
            ritus,
            master: None,
            rite: Some(rite),
            probatio: Some(name.to_string()),
            virga: None,
            stirps: None,
            lex: None,
            ligatio: None,
        }
    }

    /// The real wand: opens both secrets with the Operator's words. The master words may
    /// be left out for acts that need only the rite secret.
    pub fn verum(
        v: virga::Virga,
        verba_magistri: Option<&str>,
        verba_ritus: Option<&str>,
    ) -> Result<Baculum, String> {
        if v.status() != virga::Status::Vivit {
            return Err("this wand has been revoked: it opens nothing".into());
        }
        let master = match verba_magistri {
            Some(w) => Some(
                v.solve(virga::Secretum::Magister, w)
                    .map_err(|e| format!("the master secret: {e}"))?,
            ),
            None => None,
        };
        let rite = match verba_ritus {
            Some(w) => Some(
                v.solve(virga::Secretum::Ritus, w)
                    .map_err(|e| format!("the rite secret: {e}"))?,
            ),
            None => None,
        };
        Ok(Baculum {
            wand: Wand {
                fingerprint: v.magister.clone(),
                present: master.is_some() || rite.is_some(),
                master_unlocked: false,
                test: false,
            },
            ritus: v.ritus.clone(),
            vinculum: v.vinculum.clone(),
            master,
            rite,
            probatio: None,
            stirps: Some(v.stirps.clone()),
            virga: Some(v),
            lex: None,
            ligatio: None,
        })
    }

    /// The nucleus's view, as it stands now: `master_unlocked` is true while the master
    /// key is held.
    pub fn wand(&self) -> Wand {
        Wand {
            master_unlocked: self.master.is_some(),
            ..self.wand.clone()
        }
    }

    pub fn ritus(&self) -> &str {
        &self.ritus
    }

    pub fn vinculum(&self) -> &str {
        &self.vinculum
    }

    /// Signs with the master key: only outside a rite. The test wand's master is always at
    /// hand; a real wand's only if its words were given and it has not been laid down.
    pub fn signa_magister(&self, digest: &str) -> Result<String, String> {
        if let Some(k) = &self.master {
            return Ok(signa(k, digest));
        }
        if let Some(name) = &self.probatio {
            return Ok(signa(&probatio_key(&format!("{name}|magister")), digest));
        }
        Err("this act needs the master secret, opened with the master words".into())
    }

    /// Signs with the rite key, or with the master key when only the master is open.
    /// Returns the signer's public key and the signature.
    pub fn signa_ritus(&self, digest: &str) -> Result<(String, String), String> {
        if let Some(k) = &self.rite {
            return Ok((k.publica(), signa(k, digest)));
        }
        if self.master.is_some() || self.probatio.is_some() {
            return Ok((self.wand.fingerprint.clone(), self.signa_magister(digest)?));
        }
        Err("the wand is not present".into())
    }

    /// Makes the binding (`operator.md` §8.8): a fresh key, vouched for by the master
    /// until `until` (Unix seconds). Only while the master is open, so only at a calling.
    pub fn liga(&mut self, until: i64) -> Result<(), String> {
        use crate::nucleus::state::{digest_ligatio, Ligatio};
        let key = virga::Clavis::nova()?;
        let public = key.publica();
        let signature = self.signa_magister(&digest_ligatio(&public, until))?;
        self.ligatio = Some((
            key,
            Ligatio {
                key: public,
                until,
                signature,
            },
        ));
        Ok(())
    }

    /// The binding, if one was made.
    pub fn ligatio(&self) -> Option<&crate::nucleus::state::Ligatio> {
        self.ligatio.as_ref().map(|(_, l)| l)
    }

    /// Signs with the binding key: what a faculty that only reads may be granted by.
    pub fn signa_ligatione(&self, digest: &str) -> Option<String> {
        self.ligatio.as_ref().map(|(k, _)| signa(k, digest))
    }

    /// Lays the master key down: it is wiped, and the wand is no longer unlocked as master.
    pub fn dimitte_magistrum(&mut self) {
        self.master = None;
    }

    /// Whether this is a real wand on a stick.
    pub fn verum_est(&self) -> bool {
        self.virga.is_some()
    }

    /// The wands revoked in this wand's lineage, as the nucleus records them.
    pub fn revocatae(&self) -> Vec<String> {
        self.stirps
            .as_ref()
            .map(|s| s.revocatae().to_vec())
            .unwrap_or_default()
    }

    /// Whether `owner` may speak as the owner: a living wand of this wand's lineage, or,
    /// for a test wand, the test wand itself.
    pub fn trusts(&self, owner: &str) -> bool {
        match &self.stirps {
            Some(s) => s.vivit(owner),
            None => self.wand.test && owner == self.wand.fingerprint,
        }
    }

    /// Where this wand keeps a house's anchor: on the stick for a real wand.
    pub fn ancorae(&self) -> Option<PathBuf> {
        self.virga.as_ref().map(|v| v.ancorae())
    }
}

/// The public key of a named test key (tests only).
#[cfg(feature = "probatio")]
pub fn test_key(name: &str) -> String {
    probatio_key(name).publica()
}

/// Signs with a named test key; also accepts the public key of one of the names below in
/// place of its name (tests only).
#[cfg(feature = "probatio")]
pub fn sign_for_test(name_or_key: &str, digest: &str) -> String {
    const NAMES: [&str; 4] = ["test-wand", "owner-wand", "alia-virga", "virga-probationis"];
    let name = NAMES
        .iter()
        .find(|n| test_key(n) == name_or_key)
        .copied()
        .unwrap_or(name_or_key);
    signa(&probatio_key(name), digest)
}
