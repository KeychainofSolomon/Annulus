# The 72 Seals and their Angels — Occultist Explorer

A single reference for the vault's **72 seals**. In Occultist Explorer each of the 72 Goetic spirits has a domain of work, and its data is sealed under its own encryption sub-key (one of 72, derived from the master key — "the Ring"). Each seal is **labelled with the angel that binds that spirit** — the *Testament of Solomon*'s question "by what angel thou art frustrated." This file lists every spirit with its binding angel, and gives the same data as JSON so it can be dropped straight into the code.

## Provenance (read before trusting this)

- **The pairing (which angel binds which demon)** follows **Thomas Rudd's Goetia**, British Library **Harley MS 6483**, as documented in **Skinner & Rankine, *The Goetia of Dr Rudd* (2007)**. In Rudd's system the 72 Goetic demons and the 72 angels of the Shem HaMephorash are both mapped, in order, onto the 72 five-degree "quinances" of the zodiac. So the binding angel of Goetia spirit **n** is Shem angel **n**. This sequential scheme is the authentic one; a facsimile of Harley 6483 has **not** yet been examined directly (open lead).
- **The angel names** are given in their common transliteration, cross-checked against Benebell Wen's "72 Shem Angels Reference Handout". **Transliteration varies widely** between traditions (Vehuiah / Vahaviah, Jeliel / Yelayel, …); the spelling here is the common Kabbalistic/Lenain form. The Hebrew, not the Latin letters, is the true name.
- **Demon names and ranks** match the project's verified `spirits.js` (de Laurence 1916 Goetia).
- Two angels are both commonly transliterated **"Lauviah"** (11 and 17); they are **distinct** angels with different Hebrew, not a duplication error.

## How this is used in the code (important)

The encryption keys are derived from the **spirit's number**, never from the angel name:
`sub-key(n) = HKDF-SHA256(Ring, info = "OCCV1/seal/spirit/" + n)`.
The angel is a **descriptive label only** — shown in the logs, the manifests and the Seals page ("sealed by Mahasiah, who binds Marbas"). Because the angel never enters the key, these names can be corrected or re-transliterated at any time **without breaking any existing archive**. Do not move the angel into the key derivation.

## The 72

| # | Spirit (demon) | Rank | Binding angel |
|--:|----------------|------|---------------|
| 1 | Bael | King | Vehuiah |
| 2 | Agares | Duke | Jeliel |
| 3 | Vassago | Prince | Sitael |
| 4 | Samigina | Marquis | Elemiah |
| 5 | Marbas | President | Mahasiah |
| 6 | Valefor | Duke | Lelahel |
| 7 | Amon | Marquis | Achaiah |
| 8 | Barbatos | Duke | Cahetel |
| 9 | Paimon | King | Haziel |
| 10 | Buer | President | Aladiah |
| 11 | Gusion | Duke | Lauviah |
| 12 | Sitri | Prince | Hahaiah |
| 13 | Beleth | King | Iezalel |
| 14 | Leraje | Marquis | Mebahel |
| 15 | Eligos | Duke | Hariel |
| 16 | Zepar | Duke | Hakamiah |
| 17 | Botis | President / Count | Lauviah (II) |
| 18 | Bathin | Duke | Caliel |
| 19 | Sallos | Duke | Leuviah |
| 20 | Purson | King | Pahaliah |
| 21 | Morax (Marax) | President / Count | Nelchael |
| 22 | Ipos | Count / Prince | Yeiayel |
| 23 | Aim | Duke | Melahel |
| 24 | Naberius | Marquis | Haheuiah |
| 25 | Glasya-Labolas | President / Count | Nith-Haiah |
| 26 | Bune | Duke | Haaiah |
| 27 | Ronove | Marquis / Count | Yerathel |
| 28 | Berith | Duke | Seheiah |
| 29 | Astaroth | Duke | Reiyel |
| 30 | Forneus | Marquis | Omael |
| 31 | Foras | President | Lecabel |
| 32 | Asmoday | King | Vasariah |
| 33 | Gaap | Prince / President | Yehuiah |
| 34 | Furfur | Count | Lehahiah |
| 35 | Marchosias | Marquis | Chavakiah |
| 36 | Stolas | Prince | Menadel |
| 37 | Phenex | Marquis | Aniel |
| 38 | Halphas | Count | Haamiah |
| 39 | Malphas | President | Rehael |
| 40 | Raum | Count | Ieiazel |
| 41 | Focalor | Duke | Hahahel |
| 42 | Vepar | Duke | Mikael |
| 43 | Sabnock | Marquis | Veualiah |
| 44 | Shax | Marquis | Yelahiah |
| 45 | Vine | King / Count | Sealiah |
| 46 | Bifrons | Count | Ariel |
| 47 | Vual | Duke | Asaliah |
| 48 | Haagenti | President | Mihael |
| 49 | Crocell | Duke | Vehuel |
| 50 | Furcas | Knight | Daniel |
| 51 | Balam | King | Hahasiah |
| 52 | Alloces | Duke | Imamiah |
| 53 | Caim | President | Nanael |
| 54 | Murmur | Duke / Count | Nithael |
| 55 | Orobas | Prince | Mebahiah |
| 56 | Gremory | Duke | Poyel |
| 57 | Ose | President | Nemamiah |
| 58 | Amy | President | Yeialel |
| 59 | Orias | Marquis | Harahel |
| 60 | Vapula | Duke | Mitzrael |
| 61 | Zagan | King / President | Umabel |
| 62 | Valac | President | Iahhel |
| 63 | Andras | Marquis | Anauel |
| 64 | Flauros (Haures) | Duke | Mehiel |
| 65 | Andrealphus | Marquis | Damabiah |
| 66 | Kimaris (Cimejes) | Marquis | Manakel |
| 67 | Amdusias | Duke | Eyael |
| 68 | Belial | King | Habuhiah |
| 69 | Decarabia | Marquis | Rochel |
| 70 | Seere | Prince | Jabamiah |
| 71 | Dantalion | Duke | Haiaiel |
| 72 | Andromalius | Count | Mumiah |

## Machine-readable (drop into spirits.js as the `angel` field)

```json
{
  "scheme": "Rudd sequential: Goetia spirit n is bound by Shem HaMephorash angel n",
  "source": "Skinner & Rankine, The Goetia of Dr Rudd (BL Harley MS 6483); angel names in common Lenain transliteration",
  "note": "Label only. Keys derive from spirit number, never from the angel name.",
  "pairs": [
    {"n": 1, "spirit": "Bael", "angel": "Vehuiah"},
    {"n": 2, "spirit": "Agares", "angel": "Jeliel"},
    {"n": 3, "spirit": "Vassago", "angel": "Sitael"},
    {"n": 4, "spirit": "Samigina", "angel": "Elemiah"},
    {"n": 5, "spirit": "Marbas", "angel": "Mahasiah"},
    {"n": 6, "spirit": "Valefor", "angel": "Lelahel"},
    {"n": 7, "spirit": "Amon", "angel": "Achaiah"},
    {"n": 8, "spirit": "Barbatos", "angel": "Cahetel"},
    {"n": 9, "spirit": "Paimon", "angel": "Haziel"},
    {"n": 10, "spirit": "Buer", "angel": "Aladiah"},
    {"n": 11, "spirit": "Gusion", "angel": "Lauviah"},
    {"n": 12, "spirit": "Sitri", "angel": "Hahaiah"},
    {"n": 13, "spirit": "Beleth", "angel": "Iezalel"},
    {"n": 14, "spirit": "Leraje", "angel": "Mebahel"},
    {"n": 15, "spirit": "Eligos", "angel": "Hariel"},
    {"n": 16, "spirit": "Zepar", "angel": "Hakamiah"},
    {"n": 17, "spirit": "Botis", "angel": "Lauviah II"},
    {"n": 18, "spirit": "Bathin", "angel": "Caliel"},
    {"n": 19, "spirit": "Sallos", "angel": "Leuviah"},
    {"n": 20, "spirit": "Purson", "angel": "Pahaliah"},
    {"n": 21, "spirit": "Morax", "angel": "Nelchael"},
    {"n": 22, "spirit": "Ipos", "angel": "Yeiayel"},
    {"n": 23, "spirit": "Aim", "angel": "Melahel"},
    {"n": 24, "spirit": "Naberius", "angel": "Haheuiah"},
    {"n": 25, "spirit": "Glasya-Labolas", "angel": "Nith-Haiah"},
    {"n": 26, "spirit": "Bune", "angel": "Haaiah"},
    {"n": 27, "spirit": "Ronove", "angel": "Yerathel"},
    {"n": 28, "spirit": "Berith", "angel": "Seheiah"},
    {"n": 29, "spirit": "Astaroth", "angel": "Reiyel"},
    {"n": 30, "spirit": "Forneus", "angel": "Omael"},
    {"n": 31, "spirit": "Foras", "angel": "Lecabel"},
    {"n": 32, "spirit": "Asmoday", "angel": "Vasariah"},
    {"n": 33, "spirit": "Gaap", "angel": "Yehuiah"},
    {"n": 34, "spirit": "Furfur", "angel": "Lehahiah"},
    {"n": 35, "spirit": "Marchosias", "angel": "Chavakiah"},
    {"n": 36, "spirit": "Stolas", "angel": "Menadel"},
    {"n": 37, "spirit": "Phenex", "angel": "Aniel"},
    {"n": 38, "spirit": "Halphas", "angel": "Haamiah"},
    {"n": 39, "spirit": "Malphas", "angel": "Rehael"},
    {"n": 40, "spirit": "Raum", "angel": "Ieiazel"},
    {"n": 41, "spirit": "Focalor", "angel": "Hahahel"},
    {"n": 42, "spirit": "Vepar", "angel": "Mikael"},
    {"n": 43, "spirit": "Sabnock", "angel": "Veualiah"},
    {"n": 44, "spirit": "Shax", "angel": "Yelahiah"},
    {"n": 45, "spirit": "Vine", "angel": "Sealiah"},
    {"n": 46, "spirit": "Bifrons", "angel": "Ariel"},
    {"n": 47, "spirit": "Vual", "angel": "Asaliah"},
    {"n": 48, "spirit": "Haagenti", "angel": "Mihael"},
    {"n": 49, "spirit": "Crocell", "angel": "Vehuel"},
    {"n": 50, "spirit": "Furcas", "angel": "Daniel"},
    {"n": 51, "spirit": "Balam", "angel": "Hahasiah"},
    {"n": 52, "spirit": "Alloces", "angel": "Imamiah"},
    {"n": 53, "spirit": "Caim", "angel": "Nanael"},
    {"n": 54, "spirit": "Murmur", "angel": "Nithael"},
    {"n": 55, "spirit": "Orobas", "angel": "Mebahiah"},
    {"n": 56, "spirit": "Gremory", "angel": "Poyel"},
    {"n": 57, "spirit": "Ose", "angel": "Nemamiah"},
    {"n": 58, "spirit": "Amy", "angel": "Yeialel"},
    {"n": 59, "spirit": "Orias", "angel": "Harahel"},
    {"n": 60, "spirit": "Vapula", "angel": "Mitzrael"},
    {"n": 61, "spirit": "Zagan", "angel": "Umabel"},
    {"n": 62, "spirit": "Valac", "angel": "Iahhel"},
    {"n": 63, "spirit": "Andras", "angel": "Anauel"},
    {"n": 64, "spirit": "Flauros", "angel": "Mehiel"},
    {"n": 65, "spirit": "Andrealphus", "angel": "Damabiah"},
    {"n": 66, "spirit": "Kimaris", "angel": "Manakel"},
    {"n": 67, "spirit": "Amdusias", "angel": "Eyael"},
    {"n": 68, "spirit": "Belial", "angel": "Habuhiah"},
    {"n": 69, "spirit": "Decarabia", "angel": "Rochel"},
    {"n": 70, "spirit": "Seere", "angel": "Jabamiah"},
    {"n": 71, "spirit": "Dantalion", "angel": "Haiaiel"},
    {"n": 72, "spirit": "Andromalius", "angel": "Mumiah"}
  ]
}
```

## For the other Claude

To wire this in: add an `angel` field to each entry in `engine/spirits.js` from the `pairs` above (match on `n`), and show it wherever a seal is named — the Seals page, the spirit log ("sealed by {angel}, who binds {spirit}"), and the execution manifest. **Do not** put the angel name into `app/vault.js`'s key derivation; the seal key stays `HKDF(Ring, "OCCV1/seal/spirit/" + n)`. Keep the provenance note above with the data, and keep the open lead (verify against a Harley 6483 facsimile) in `docs/sources/LOG.md`.
