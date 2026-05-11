//! USFM 66-book canonical table with HelloAO ID and locale name mapping.
//!
//! `canonical_id` is the USFM 3-letter book ID (`GEN`, `JHN`, `1CO`, …).
//! `helloao_id` matches what the upstream API returns under `book.id`.
//! `language` is ISO 639-3 of the surface language for the localized names
//! present in `aliases`.

use std::collections::HashMap;
use std::sync::OnceLock;

#[derive(Debug, Clone)]
pub struct BookEntry {
    pub canonical_id: &'static str,
    pub helloao_id: &'static str,
    pub order: u8,
    pub name_en: &'static str,
    pub name_pt: &'static str,
    pub name_es: &'static str,
    pub name_fr: &'static str,
    pub name_de: &'static str,
    pub name_grc: &'static str,
    pub name_heb: &'static str,
    pub aliases_en: &'static [&'static str],
    pub aliases_pt: &'static [&'static str],
    pub aliases_es: &'static [&'static str],
    pub aliases_fr: &'static [&'static str],
    pub aliases_de: &'static [&'static str],
    pub aliases_grc: &'static [&'static str],
    pub aliases_heb: &'static [&'static str],
    /// Canonical Protestant chapter count (KJV-style 66-book canon).
    pub chapters: u16,
    /// `"OT"` (orders 1..=39) or `"NT"` (orders 40..=66).
    pub testament: &'static str,
}

/// Combined alias list (all languages, deduped, lowercased) — useful for `info` output.
pub fn aliases_lower(b: &BookEntry) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut push = |s: &str| {
        let lo = s.to_ascii_lowercase();
        if !out.iter().any(|e| e == &lo) {
            out.push(lo);
        }
    };
    for a in b.aliases_en.iter()
        .chain(b.aliases_pt.iter())
        .chain(b.aliases_es.iter())
        .chain(b.aliases_fr.iter())
        .chain(b.aliases_de.iter())
        .chain(b.aliases_grc.iter())
        .chain(b.aliases_heb.iter())
    {
        push(a);
    }
    push(b.canonical_id);
    out
}

pub fn all() -> &'static [BookEntry] {
    BOOKS
}

pub fn by_canonical_id(id: &str) -> Option<&'static BookEntry> {
    let upper = id.to_ascii_uppercase();
    BOOKS.iter().find(|b| b.canonical_id == upper)
}

pub fn alias_index() -> &'static HashMap<String, &'static str> {
    static IDX: OnceLock<HashMap<String, &'static str>> = OnceLock::new();
    IDX.get_or_init(|| {
        let mut m: HashMap<String, &'static str> = HashMap::new();
        for b in BOOKS {
            // Insert full names for all languages
            insert_alias(&mut m, b.name_en, b.canonical_id);
            insert_alias(&mut m, b.name_pt, b.canonical_id);
            insert_alias(&mut m, b.name_es, b.canonical_id);
            insert_alias(&mut m, b.name_fr, b.canonical_id);
            insert_alias(&mut m, b.name_de, b.canonical_id);
            insert_alias(&mut m, b.name_grc, b.canonical_id);
            insert_alias(&mut m, b.name_heb, b.canonical_id);
            // Insert all aliases for all languages
            for a in b.aliases_en.iter()
                .chain(b.aliases_pt.iter())
                .chain(b.aliases_es.iter())
                .chain(b.aliases_fr.iter())
                .chain(b.aliases_de.iter())
                .chain(b.aliases_grc.iter())
                .chain(b.aliases_heb.iter())
            {
                insert_alias(&mut m, a, b.canonical_id);
            }
            insert_alias(&mut m, b.canonical_id, b.canonical_id);
        }
        m
    })
}

fn insert_alias(m: &mut HashMap<String, &'static str>, raw: &str, canonical: &'static str) {
    let key = crate::reference::normalize(raw);
    if !key.is_empty() {
        m.insert(key, canonical);
    }
}

#[rustfmt::skip]
const BOOKS: &[BookEntry] = &[
    BookEntry {
        canonical_id: "GEN", helloao_id: "GEN", order:  1,
        name_en: "Genesis",
        name_pt: "Gênesis",
        name_es: "Génesis",
        name_fr: "Genèse",
        name_de: "Genesis",
        name_grc: "Γένεσις",
        name_heb: "בראשית",
        aliases_en: &["Gen", "Gn"],
        aliases_pt: &["Gn", "Gen"],
        aliases_es: &["Gén", "Gen"],
        aliases_fr: &["Gen", "Gn"],
        aliases_de: &["Gen", "1Mo"],
        aliases_grc: &["Γεν"],
        aliases_heb: &["בר"],
        chapters: 50, testament: "OT"
    },
    BookEntry {
        canonical_id: "EXO", helloao_id: "EXO", order:  2,
        name_en: "Exodus",
        name_pt: "Êxodo",
        name_es: "Éxodo",
        name_fr: "Exode",
        name_de: "Exodus",
        name_grc: "Ἔξοδος",
        name_heb: "שמות",
        aliases_en: &["Exod", "Exo", "Ex"],
        aliases_pt: &["Ex", "Exo", "Exod"],
        aliases_es: &["Éx", "Ex"],
        aliases_fr: &["Ex", "Exo"],
        aliases_de: &["Ex", "2Mo"],
        aliases_grc: &["Ἔξ"],
        aliases_heb: &["שמ"],
        chapters: 40, testament: "OT"
    },
    BookEntry {
        canonical_id: "LEV", helloao_id: "LEV", order:  3,
        name_en: "Leviticus",
        name_pt: "Levítico",
        name_es: "Levítico",
        name_fr: "Lévitique",
        name_de: "Levitikus",
        name_grc: "Λευιτικόν",
        name_heb: "ויקרא",
        aliases_en: &["Lev", "Lv"],
        aliases_pt: &["Lv", "Lev"],
        aliases_es: &["Lev", "Lv"],
        aliases_fr: &["Lév", "Lv"],
        aliases_de: &["Lev", "3Mo"],
        aliases_grc: &["Λευ"],
        aliases_heb: &["ויק"],
        chapters: 27, testament: "OT"
    },
    BookEntry {
        canonical_id: "NUM", helloao_id: "NUM", order:  4,
        name_en: "Numbers",
        name_pt: "Números",
        name_es: "Números",
        name_fr: "Nombres",
        name_de: "Numeri",
        name_grc: "Ἀριθμοί",
        name_heb: "במדבר",
        aliases_en: &["Num", "Nm", "Nu"],
        aliases_pt: &["Nm", "Num"],
        aliases_es: &["Núm", "Nm"],
        aliases_fr: &["Nom", "Nb"],
        aliases_de: &["Num", "4Mo"],
        aliases_grc: &["Ἀρ"],
        aliases_heb: &["במד"],
        chapters: 36, testament: "OT"
    },
    BookEntry {
        canonical_id: "DEU", helloao_id: "DEU", order:  5,
        name_en: "Deuteronomy",
        name_pt: "Deuteronômio",
        name_es: "Deuteronomio",
        name_fr: "Deutéronome",
        name_de: "Deuteronomium",
        name_grc: "Δευτερονόμιον",
        name_heb: "דברים",
        aliases_en: &["Deut", "Deu", "Dt"],
        aliases_pt: &["Dt", "Deut", "Deu"],
        aliases_es: &["Deut", "Dt"],
        aliases_fr: &["Deut", "Dt"],
        aliases_de: &["Dtn", "5Mo"],
        aliases_grc: &["Δευτ"],
        aliases_heb: &["דבר"],
        chapters: 34, testament: "OT"
    },
    BookEntry {
        canonical_id: "JOS", helloao_id: "JOS", order:  6,
        name_en: "Joshua",
        name_pt: "Josué",
        name_es: "Josué",
        name_fr: "Josué",
        name_de: "Josua",
        name_grc: "Ἰησοῦς",
        name_heb: "יהושע",
        aliases_en: &["Josh", "Jos"],
        aliases_pt: &["Js", "Jos", "Josué"],
        aliases_es: &["Jos", "Js"],
        aliases_fr: &["Jos", "Jus"],
        aliases_de: &["Jos"],
        aliases_grc: &["Ἰησ"],
        aliases_heb: &["יהו"],
        chapters: 24, testament: "OT"
    },
    BookEntry {
        canonical_id: "JDG", helloao_id: "JDG", order:  7,
        name_en: "Judges",
        name_pt: "Juízes",
        name_es: "Jueces",
        name_fr: "Juges",
        name_de: "Richter",
        name_grc: "Κριταί",
        name_heb: "שופטים",
        aliases_en: &["Judg", "Jdg", "Jdgs"],
        aliases_pt: &["Jz", "Juízes", "Juizes"],
        aliases_es: &["Jue", "Jc"],
        aliases_fr: &["Jug", "Jg"],
        aliases_de: &["Ri"],
        aliases_grc: &["Κρ"],
        aliases_heb: &["שופ"],
        chapters: 21, testament: "OT"
    },
    BookEntry {
        canonical_id: "RUT", helloao_id: "RUT", order:  8,
        name_en: "Ruth",
        name_pt: "Rute",
        name_es: "Rut",
        name_fr: "Ruth",
        name_de: "Ruth",
        name_grc: "Ῥούθ",
        name_heb: "רות",
        aliases_en: &["Rut", "Ru"],
        aliases_pt: &["Rt", "Rute"],
        aliases_es: &["Rt"],
        aliases_fr: &["Rt", "Ru"],
        aliases_de: &["Rt", "Rut"],
        aliases_grc: &["Ῥ"],
        aliases_heb: &["רו"],
        chapters: 4, testament: "OT"
    },
    BookEntry {
        canonical_id: "1SA", helloao_id: "1SA", order:  9,
        name_en: "1 Samuel",
        name_pt: "1 Samuel",
        name_es: "1 Samuel",
        name_fr: "1 Samuel",
        name_de: "1 Samuel",
        name_grc: "1 Samuel",
        name_heb: "1 Samuel",
        aliases_en: &["1Sam", "1 Sam", "1Sa", "I Samuel", "First Samuel"],
        aliases_pt: &["1Sm", "1 Sm", "1 Sam", "I Samuel", "Primeiro Samuel", "Primeira Samuel"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 31, testament: "OT"
    },
    BookEntry {
        canonical_id: "2SA", helloao_id: "2SA", order: 10,
        name_en: "2 Samuel",
        name_pt: "2 Samuel",
        name_es: "2 Samuel",
        name_fr: "2 Samuel",
        name_de: "2 Samuel",
        name_grc: "2 Samuel",
        name_heb: "2 Samuel",
        aliases_en: &["2Sam", "2 Sam", "2Sa", "II Samuel", "Second Samuel"],
        aliases_pt: &["2Sm", "2 Sm", "2 Sam", "II Samuel", "Segundo Samuel", "Segunda Samuel"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 24, testament: "OT"
    },
    BookEntry {
        canonical_id: "1KI", helloao_id: "1KI", order: 11,
        name_en: "1 Kings",
        name_pt: "1 Reis",
        name_es: "1 Kings",
        name_fr: "1 Kings",
        name_de: "1 Kings",
        name_grc: "1 Kings",
        name_heb: "1 Kings",
        aliases_en: &["1Kgs", "1 Kgs", "1Ki", "I Kings", "First Kings"],
        aliases_pt: &["1Rs", "1 Rs", "1 Reis", "I Reis", "Primeiro Reis", "Primeira Reis"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 22, testament: "OT"
    },
    BookEntry {
        canonical_id: "2KI", helloao_id: "2KI", order: 12,
        name_en: "2 Kings",
        name_pt: "2 Reis",
        name_es: "2 Kings",
        name_fr: "2 Kings",
        name_de: "2 Kings",
        name_grc: "2 Kings",
        name_heb: "2 Kings",
        aliases_en: &["2Kgs", "2 Kgs", "2Ki", "II Kings", "Second Kings"],
        aliases_pt: &["2Rs", "2 Rs", "2 Reis", "II Reis", "Segundo Reis", "Segunda Reis"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 25, testament: "OT"
    },
    BookEntry {
        canonical_id: "1CH", helloao_id: "1CH", order: 13,
        name_en: "1 Chronicles",
        name_pt: "1 Crônicas",
        name_es: "1 Chronicles",
        name_fr: "1 Chronicles",
        name_de: "1 Chronicles",
        name_grc: "1 Chronicles",
        name_heb: "1 Chronicles",
        aliases_en: &["1Chr", "1 Chr", "1Ch", "I Chronicles", "First Chronicles"],
        aliases_pt: &["1Cr", "1 Cr", "1 Crônicas", "1 Cronicas", "I Crônicas", "Primeira Crônicas"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 29, testament: "OT"
    },
    BookEntry {
        canonical_id: "2CH", helloao_id: "2CH", order: 14,
        name_en: "2 Chronicles",
        name_pt: "2 Crônicas",
        name_es: "2 Chronicles",
        name_fr: "2 Chronicles",
        name_de: "2 Chronicles",
        name_grc: "2 Chronicles",
        name_heb: "2 Chronicles",
        aliases_en: &["2Chr", "2 Chr", "2Ch", "II Chronicles", "Second Chronicles"],
        aliases_pt: &["2Cr", "2 Cr", "2 Crônicas", "2 Cronicas", "II Crônicas", "Segunda Crônicas"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 36, testament: "OT"
    },
    BookEntry {
        canonical_id: "EZR", helloao_id: "EZR", order: 15,
        name_en: "Ezra",
        name_pt: "Esdras",
        name_es: "Ezra",
        name_fr: "Ezra",
        name_de: "Ezra",
        name_grc: "Ezra",
        name_heb: "Ezra",
        aliases_en: &["Ezr"],
        aliases_pt: &["Ed", "Esd"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 10, testament: "OT"
    },
    BookEntry {
        canonical_id: "NEH", helloao_id: "NEH", order: 16,
        name_en: "Nehemiah",
        name_pt: "Neemias",
        name_es: "Nehemiah",
        name_fr: "Nehemiah",
        name_de: "Nehemiah",
        name_grc: "Nehemiah",
        name_heb: "Nehemiah",
        aliases_en: &["Neh", "Ne"],
        aliases_pt: &["Ne", "Neh"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 13, testament: "OT"
    },
    BookEntry {
        canonical_id: "EST", helloao_id: "EST", order: 17,
        name_en: "Esther",
        name_pt: "Ester",
        name_es: "Esther",
        name_fr: "Esther",
        name_de: "Esther",
        name_grc: "Esther",
        name_heb: "Esther",
        aliases_en: &["Est", "Es"],
        aliases_pt: &["Et", "Est"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 10, testament: "OT"
    },
    BookEntry {
        canonical_id: "JOB", helloao_id: "JOB", order: 18,
        name_en: "Job",
        name_pt: "Jó",
        name_es: "Job",
        name_fr: "Job",
        name_de: "Job",
        name_grc: "Job",
        name_heb: "Job",
        aliases_en: &["Jb"],
        aliases_pt: &["Jó", "Jo"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 42, testament: "OT"
    },
    BookEntry {
        canonical_id: "PSA", helloao_id: "PSA", order: 19,
        name_en: "Psalms",
        name_pt: "Salmos",
        name_es: "Psalms",
        name_fr: "Psalms",
        name_de: "Psalms",
        name_grc: "Psalms",
        name_heb: "Psalms",
        aliases_en: &["Ps", "Psa", "Psalm", "Pss"],
        aliases_pt: &["Sl", "Sal", "Salmo"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 150, testament: "OT"
    },
    BookEntry {
        canonical_id: "PRO", helloao_id: "PRO", order: 20,
        name_en: "Proverbs",
        name_pt: "Provérbios",
        name_es: "Proverbs",
        name_fr: "Proverbs",
        name_de: "Proverbs",
        name_grc: "Proverbs",
        name_heb: "Proverbs",
        aliases_en: &["Prov", "Pro", "Pr", "Prv"],
        aliases_pt: &["Pv", "Prov", "Provérbios", "Proverbios"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 31, testament: "OT"
    },
    BookEntry {
        canonical_id: "ECC", helloao_id: "ECC", order: 21,
        name_en: "Ecclesiastes",
        name_pt: "Eclesiastes",
        name_es: "Ecclesiastes",
        name_fr: "Ecclesiastes",
        name_de: "Ecclesiastes",
        name_grc: "Ecclesiastes",
        name_heb: "Ecclesiastes",
        aliases_en: &["Eccl", "Ecc", "Ec", "Qoh"],
        aliases_pt: &["Ec", "Ecl"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 12, testament: "OT"
    },
    BookEntry {
        canonical_id: "SNG", helloao_id: "SNG", order: 22,
        name_en: "Song of Solomon",
        name_pt: "Cantares",
        name_es: "Song of Solomon",
        name_fr: "Song of Solomon",
        name_de: "Song of Solomon",
        name_grc: "Song of Solomon",
        name_heb: "Song of Solomon",
        aliases_en: &["Song", "Sng", "Sos", "Song of Songs", "Canticles"],
        aliases_pt: &["Ct", "Cantares", "Cânticos", "Canticos", "Cântico dos Cânticos"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 8, testament: "OT"
    },
    BookEntry {
        canonical_id: "ISA", helloao_id: "ISA", order: 23,
        name_en: "Isaiah",
        name_pt: "Isaías",
        name_es: "Isaiah",
        name_fr: "Isaiah",
        name_de: "Isaiah",
        name_grc: "Isaiah",
        name_heb: "Isaiah",
        aliases_en: &["Isa", "Is"],
        aliases_pt: &["Is", "Isa"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 66, testament: "OT"
    },
    BookEntry {
        canonical_id: "JER", helloao_id: "JER", order: 24,
        name_en: "Jeremiah",
        name_pt: "Jeremias",
        name_es: "Jeremiah",
        name_fr: "Jeremiah",
        name_de: "Jeremiah",
        name_grc: "Jeremiah",
        name_heb: "Jeremiah",
        aliases_en: &["Jer", "Je"],
        aliases_pt: &["Jr", "Jer"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 52, testament: "OT"
    },
    BookEntry {
        canonical_id: "LAM", helloao_id: "LAM", order: 25,
        name_en: "Lamentations",
        name_pt: "Lamentações",
        name_es: "Lamentations",
        name_fr: "Lamentations",
        name_de: "Lamentations",
        name_grc: "Lamentations",
        name_heb: "Lamentations",
        aliases_en: &["Lam", "La"],
        aliases_pt: &["Lm", "Lam", "Lamentacoes"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 5, testament: "OT"
    },
    BookEntry {
        canonical_id: "EZK", helloao_id: "EZK", order: 26,
        name_en: "Ezekiel",
        name_pt: "Ezequiel",
        name_es: "Ezekiel",
        name_fr: "Ezekiel",
        name_de: "Ezekiel",
        name_grc: "Ezekiel",
        name_heb: "Ezekiel",
        aliases_en: &["Ezek", "Ezk", "Eze"],
        aliases_pt: &["Ez", "Eze", "Ezeq"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 48, testament: "OT"
    },
    BookEntry {
        canonical_id: "DAN", helloao_id: "DAN", order: 27,
        name_en: "Daniel",
        name_pt: "Daniel",
        name_es: "Daniel",
        name_fr: "Daniel",
        name_de: "Daniel",
        name_grc: "Daniel",
        name_heb: "Daniel",
        aliases_en: &["Dan", "Dn"],
        aliases_pt: &["Dn", "Dan"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 12, testament: "OT"
    },
    BookEntry {
        canonical_id: "HOS", helloao_id: "HOS", order: 28,
        name_en: "Hosea",
        name_pt: "Oséias",
        name_es: "Hosea",
        name_fr: "Hosea",
        name_de: "Hosea",
        name_grc: "Hosea",
        name_heb: "Hosea",
        aliases_en: &["Hos", "Ho"],
        aliases_pt: &["Os", "Oséias", "Oseias"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 14, testament: "OT"
    },
    BookEntry {
        canonical_id: "JOL", helloao_id: "JOL", order: 29,
        name_en: "Joel",
        name_pt: "Joel",
        name_es: "Joel",
        name_fr: "Joel",
        name_de: "Joel",
        name_grc: "Joel",
        name_heb: "Joel",
        aliases_en: &["Joel", "Jl"],
        aliases_pt: &["Jl", "Joel"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 3, testament: "OT"
    },
    BookEntry {
        canonical_id: "AMO", helloao_id: "AMO", order: 30,
        name_en: "Amos",
        name_pt: "Amós",
        name_es: "Amos",
        name_fr: "Amos",
        name_de: "Amos",
        name_grc: "Amos",
        name_heb: "Amos",
        aliases_en: &["Am"],
        aliases_pt: &["Am", "Amós", "Amos"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 9, testament: "OT"
    },
    BookEntry {
        canonical_id: "OBA", helloao_id: "OBA", order: 31,
        name_en: "Obadiah",
        name_pt: "Obadias",
        name_es: "Obadiah",
        name_fr: "Obadiah",
        name_de: "Obadiah",
        name_grc: "Obadiah",
        name_heb: "Obadiah",
        aliases_en: &["Obad", "Oba", "Ob"],
        aliases_pt: &["Ob", "Obad"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 1, testament: "OT"
    },
    BookEntry {
        canonical_id: "JON", helloao_id: "JON", order: 32,
        name_en: "Jonah",
        name_pt: "Jonas",
        name_es: "Jonah",
        name_fr: "Jonah",
        name_de: "Jonah",
        name_grc: "Jonah",
        name_heb: "Jonah",
        aliases_en: &["Jon", "Jnh"],
        aliases_pt: &["Jn", "Jon"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 4, testament: "OT"
    },
    BookEntry {
        canonical_id: "MIC", helloao_id: "MIC", order: 33,
        name_en: "Micah",
        name_pt: "Miquéias",
        name_es: "Micah",
        name_fr: "Micah",
        name_de: "Micah",
        name_grc: "Micah",
        name_heb: "Micah",
        aliases_en: &["Mic", "Mi"],
        aliases_pt: &["Mq", "Miquéias", "Miqueias"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 7, testament: "OT"
    },
    BookEntry {
        canonical_id: "NAM", helloao_id: "NAM", order: 34,
        name_en: "Nahum",
        name_pt: "Naum",
        name_es: "Nahum",
        name_fr: "Nahum",
        name_de: "Nahum",
        name_grc: "Nahum",
        name_heb: "Nahum",
        aliases_en: &["Nah", "Nam", "Na"],
        aliases_pt: &["Na", "Nau", "Naum"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 3, testament: "OT"
    },
    BookEntry {
        canonical_id: "HAB", helloao_id: "HAB", order: 35,
        name_en: "Habakkuk",
        name_pt: "Habacuque",
        name_es: "Habakkuk",
        name_fr: "Habakkuk",
        name_de: "Habakkuk",
        name_grc: "Habakkuk",
        name_heb: "Habakkuk",
        aliases_en: &["Hab", "Hb"],
        aliases_pt: &["Hc", "Hab", "Habacuque"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 3, testament: "OT"
    },
    BookEntry {
        canonical_id: "ZEP", helloao_id: "ZEP", order: 36,
        name_en: "Zephaniah",
        name_pt: "Sofonias",
        name_es: "Zephaniah",
        name_fr: "Zephaniah",
        name_de: "Zephaniah",
        name_grc: "Zephaniah",
        name_heb: "Zephaniah",
        aliases_en: &["Zeph", "Zep", "Zp"],
        aliases_pt: &["Sf", "Sof"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 3, testament: "OT"
    },
    BookEntry {
        canonical_id: "HAG", helloao_id: "HAG", order: 37,
        name_en: "Haggai",
        name_pt: "Ageu",
        name_es: "Haggai",
        name_fr: "Haggai",
        name_de: "Haggai",
        name_grc: "Haggai",
        name_heb: "Haggai",
        aliases_en: &["Hag", "Hg"],
        aliases_pt: &["Ag", "Ageu"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 2, testament: "OT"
    },
    BookEntry {
        canonical_id: "ZEC", helloao_id: "ZEC", order: 38,
        name_en: "Zechariah",
        name_pt: "Zacarias",
        name_es: "Zechariah",
        name_fr: "Zechariah",
        name_de: "Zechariah",
        name_grc: "Zechariah",
        name_heb: "Zechariah",
        aliases_en: &["Zech", "Zec", "Zc"],
        aliases_pt: &["Zc", "Zac"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 14, testament: "OT"
    },
    BookEntry {
        canonical_id: "MAL", helloao_id: "MAL", order: 39,
        name_en: "Malachi",
        name_pt: "Malaquias",
        name_es: "Malachi",
        name_fr: "Malachi",
        name_de: "Malachi",
        name_grc: "Malachi",
        name_heb: "Malachi",
        aliases_en: &["Mal", "Ml"],
        aliases_pt: &["Ml", "Mal", "Malaquias"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 4, testament: "OT"
    },
    BookEntry {
        canonical_id: "MAT", helloao_id: "MAT", order: 40,
        name_en: "Matthew",
        name_pt: "Mateus",
        name_es: "Mateo",
        name_fr: "Matthieu",
        name_de: "Matthäus",
        name_grc: "Κατὰ Μαθθαῖον",
        name_heb: "מתי",
        aliases_en: &["Matt", "Mat", "Mt"],
        aliases_pt: &["Mt", "Mat", "Mateus"],
        aliases_es: &["Mat", "Mt"],
        aliases_fr: &["Mat", "Mt"],
        aliases_de: &["Mt", "Matt"],
        aliases_grc: &["Μτ", "Ματθ"],
        aliases_heb: &["מת"],
        chapters: 28, testament: "NT"
    },
    BookEntry {
        canonical_id: "MRK", helloao_id: "MRK", order: 41,
        name_en: "Mark",
        name_pt: "Marcos",
        name_es: "Mark",
        name_fr: "Mark",
        name_de: "Mark",
        name_grc: "Mark",
        name_heb: "Mark",
        aliases_en: &["Mark", "Mrk", "Mk"],
        aliases_pt: &["Mc", "Mar", "Marcos"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 16, testament: "NT"
    },
    BookEntry {
        canonical_id: "LUK", helloao_id: "LUK", order: 42,
        name_en: "Luke",
        name_pt: "Lucas",
        name_es: "Luke",
        name_fr: "Luke",
        name_de: "Luke",
        name_grc: "Luke",
        name_heb: "Luke",
        aliases_en: &["Luk", "Lk"],
        aliases_pt: &["Lc", "Luc", "Lucas"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 24, testament: "NT"
    },
    BookEntry {
        canonical_id: "JHN", helloao_id: "JHN", order: 43,
        name_en: "John",
        name_pt: "João",
        name_es: "Juan",
        name_fr: "Jean",
        name_de: "Johannes",
        name_grc: "Κατὰ Ἰωάννην",
        name_heb: "יוחנן",
        aliases_en: &["John", "Jhn", "Jn", "Joh"],
        aliases_pt: &["Jo", "João", "Joao"],
        aliases_es: &["Jn", "Jua"],
        aliases_fr: &["Jn", "Jea"],
        aliases_de: &["Joh", "Jn"],
        aliases_grc: &["Ἰω", "Ἰωαν"],
        aliases_heb: &["יוח"],
        chapters: 21, testament: "NT"
    },
    BookEntry {
        canonical_id: "ACT", helloao_id: "ACT", order: 44,
        name_en: "Acts",
        name_pt: "Atos",
        name_es: "Acts",
        name_fr: "Acts",
        name_de: "Acts",
        name_grc: "Acts",
        name_heb: "Acts",
        aliases_en: &["Act", "Ac"],
        aliases_pt: &["At", "Atos", "Ato"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 28, testament: "NT"
    },
    BookEntry {
        canonical_id: "ROM", helloao_id: "ROM", order: 45,
        name_en: "Romans",
        name_pt: "Romanos",
        name_es: "Romans",
        name_fr: "Romans",
        name_de: "Romans",
        name_grc: "Romans",
        name_heb: "Romans",
        aliases_en: &["Rom", "Ro", "Rm"],
        aliases_pt: &["Rm", "Rom", "Romanos"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 16, testament: "NT"
    },
    BookEntry {
        canonical_id: "1CO", helloao_id: "1CO", order: 46,
        name_en: "1 Corinthians",
        name_pt: "1 Coríntios",
        name_es: "1 Corinthians",
        name_fr: "1 Corinthians",
        name_de: "1 Corinthians",
        name_grc: "1 Corinthians",
        name_heb: "1 Corinthians",
        aliases_en: &["1Cor", "1 Cor", "1Co", "I Corinthians", "First Corinthians"],
        aliases_pt: &["1Co", "1 Co", "1 Cor", "1 Coríntios", "1 Corintios", "I Coríntios", "Primeira Coríntios", "Primeiro Coríntios"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 16, testament: "NT"
    },
    BookEntry {
        canonical_id: "2CO", helloao_id: "2CO", order: 47,
        name_en: "2 Corinthians",
        name_pt: "2 Coríntios",
        name_es: "2 Corinthians",
        name_fr: "2 Corinthians",
        name_de: "2 Corinthians",
        name_grc: "2 Corinthians",
        name_heb: "2 Corinthians",
        aliases_en: &["2Cor", "2 Cor", "2Co", "II Corinthians", "Second Corinthians"],
        aliases_pt: &["2Co", "2 Co", "2 Cor", "2 Coríntios", "2 Corintios", "II Coríntios", "Segunda Coríntios", "Segundo Coríntios"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 13, testament: "NT"
    },
    BookEntry {
        canonical_id: "GAL", helloao_id: "GAL", order: 48,
        name_en: "Galatians",
        name_pt: "Gálatas",
        name_es: "Galatians",
        name_fr: "Galatians",
        name_de: "Galatians",
        name_grc: "Galatians",
        name_heb: "Galatians",
        aliases_en: &["Gal", "Ga"],
        aliases_pt: &["Gl", "Gal", "Gálatas", "Galatas"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 6, testament: "NT"
    },
    BookEntry {
        canonical_id: "EPH", helloao_id: "EPH", order: 49,
        name_en: "Ephesians",
        name_pt: "Efésios",
        name_es: "Ephesians",
        name_fr: "Ephesians",
        name_de: "Ephesians",
        name_grc: "Ephesians",
        name_heb: "Ephesians",
        aliases_en: &["Eph", "Ep"],
        aliases_pt: &["Ef", "Ef", "Efésios", "Efesios"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 6, testament: "NT"
    },
    BookEntry {
        canonical_id: "PHP", helloao_id: "PHP", order: 50,
        name_en: "Philippians",
        name_pt: "Filipenses",
        name_es: "Philippians",
        name_fr: "Philippians",
        name_de: "Philippians",
        name_grc: "Philippians",
        name_heb: "Philippians",
        aliases_en: &["Phil", "Php", "Phi"],
        aliases_pt: &["Fp", "Fl", "Fil", "Filipenses"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 4, testament: "NT"
    },
    BookEntry {
        canonical_id: "COL", helloao_id: "COL", order: 51,
        name_en: "Colossians",
        name_pt: "Colossenses",
        name_es: "Colossians",
        name_fr: "Colossians",
        name_de: "Colossians",
        name_grc: "Colossians",
        name_heb: "Colossians",
        aliases_en: &["Col"],
        aliases_pt: &["Cl", "Col", "Colossenses"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 4, testament: "NT"
    },
    BookEntry {
        canonical_id: "1TH", helloao_id: "1TH", order: 52,
        name_en: "1 Thessalonians",
        name_pt: "1 Tessalonicenses",
        name_es: "1 Thessalonians",
        name_fr: "1 Thessalonians",
        name_de: "1 Thessalonians",
        name_grc: "1 Thessalonians",
        name_heb: "1 Thessalonians",
        aliases_en: &["1Thess", "1 Thess", "1Th", "I Thessalonians", "First Thessalonians"],
        aliases_pt: &["1Ts", "1 Ts", "1 Tess", "1 Tessalonicenses", "I Tessalonicenses", "Primeira Tessalonicenses"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 5, testament: "NT"
    },
    BookEntry {
        canonical_id: "2TH", helloao_id: "2TH", order: 53,
        name_en: "2 Thessalonians",
        name_pt: "2 Tessalonicenses",
        name_es: "2 Thessalonians",
        name_fr: "2 Thessalonians",
        name_de: "2 Thessalonians",
        name_grc: "2 Thessalonians",
        name_heb: "2 Thessalonians",
        aliases_en: &["2Thess", "2 Thess", "2Th", "II Thessalonians", "Second Thessalonians"],
        aliases_pt: &["2Ts", "2 Ts", "2 Tess", "2 Tessalonicenses", "II Tessalonicenses", "Segunda Tessalonicenses"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 3, testament: "NT"
    },
    BookEntry {
        canonical_id: "1TI", helloao_id: "1TI", order: 54,
        name_en: "1 Timothy",
        name_pt: "1 Timóteo",
        name_es: "1 Timothy",
        name_fr: "1 Timothy",
        name_de: "1 Timothy",
        name_grc: "1 Timothy",
        name_heb: "1 Timothy",
        aliases_en: &["1Tim", "1 Tim", "1Ti", "I Timothy", "First Timothy"],
        aliases_pt: &["1Tm", "1 Tm", "1 Tim", "1 Timóteo", "1 Timoteo", "I Timóteo", "Primeira Timóteo"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 6, testament: "NT"
    },
    BookEntry {
        canonical_id: "2TI", helloao_id: "2TI", order: 55,
        name_en: "2 Timothy",
        name_pt: "2 Timóteo",
        name_es: "2 Timothy",
        name_fr: "2 Timothy",
        name_de: "2 Timothy",
        name_grc: "2 Timothy",
        name_heb: "2 Timothy",
        aliases_en: &["2Tim", "2 Tim", "2Ti", "II Timothy", "Second Timothy"],
        aliases_pt: &["2Tm", "2 Tm", "2 Tim", "2 Timóteo", "2 Timoteo", "II Timóteo", "Segunda Timóteo"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 4, testament: "NT"
    },
    BookEntry {
        canonical_id: "TIT", helloao_id: "TIT", order: 56,
        name_en: "Titus",
        name_pt: "Tito",
        name_es: "Titus",
        name_fr: "Titus",
        name_de: "Titus",
        name_grc: "Titus",
        name_heb: "Titus",
        aliases_en: &["Tit"],
        aliases_pt: &["Tt", "Tito"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 3, testament: "NT"
    },
    BookEntry {
        canonical_id: "PHM", helloao_id: "PHM", order: 57,
        name_en: "Philemon",
        name_pt: "Filemom",
        name_es: "Philemon",
        name_fr: "Philemon",
        name_de: "Philemon",
        name_grc: "Philemon",
        name_heb: "Philemon",
        aliases_en: &["Phlm", "Phm", "Pm"],
        aliases_pt: &["Fm", "File", "Filemom", "Filemon"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 1, testament: "NT"
    },
    BookEntry {
        canonical_id: "HEB", helloao_id: "HEB", order: 58,
        name_en: "Hebrews",
        name_pt: "Hebreus",
        name_es: "Hebrews",
        name_fr: "Hebrews",
        name_de: "Hebrews",
        name_grc: "Hebrews",
        name_heb: "Hebrews",
        aliases_en: &["Heb", "He"],
        aliases_pt: &["Hb", "Heb", "Hebreus"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 13, testament: "NT"
    },
    BookEntry {
        canonical_id: "JAS", helloao_id: "JAS", order: 59,
        name_en: "James",
        name_pt: "Tiago",
        name_es: "James",
        name_fr: "James",
        name_de: "James",
        name_grc: "James",
        name_heb: "James",
        aliases_en: &["Jas", "Jam"],
        aliases_pt: &["Tg", "Tia", "Tiago"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 5, testament: "NT"
    },
    BookEntry {
        canonical_id: "1PE", helloao_id: "1PE", order: 60,
        name_en: "1 Peter",
        name_pt: "1 Pedro",
        name_es: "1 Peter",
        name_fr: "1 Peter",
        name_de: "1 Peter",
        name_grc: "1 Peter",
        name_heb: "1 Peter",
        aliases_en: &["1Pet", "1 Pet", "1Pe", "I Peter", "First Peter"],
        aliases_pt: &["1Pe", "1 Pe", "1 Ped", "1 Pedro", "I Pedro", "Primeira Pedro", "Primeiro Pedro"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 5, testament: "NT"
    },
    BookEntry {
        canonical_id: "2PE", helloao_id: "2PE", order: 61,
        name_en: "2 Peter",
        name_pt: "2 Pedro",
        name_es: "2 Peter",
        name_fr: "2 Peter",
        name_de: "2 Peter",
        name_grc: "2 Peter",
        name_heb: "2 Peter",
        aliases_en: &["2Pet", "2 Pet", "2Pe", "II Peter", "Second Peter"],
        aliases_pt: &["2Pe", "2 Pe", "2 Ped", "2 Pedro", "II Pedro", "Segunda Pedro", "Segundo Pedro"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 3, testament: "NT"
    },
    BookEntry {
        canonical_id: "1JN", helloao_id: "1JN", order: 62,
        name_en: "1 John",
        name_pt: "1 João",
        name_es: "1 John",
        name_fr: "1 John",
        name_de: "1 John",
        name_grc: "1 John",
        name_heb: "1 John",
        aliases_en: &["1John", "1 John", "1Jn", "1Jo", "I John", "First John"],
        aliases_pt: &["1Jo", "1 Jo", "1 João", "1 Joao", "I João", "Primeira João", "Primeiro João"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 5, testament: "NT"
    },
    BookEntry {
        canonical_id: "2JN", helloao_id: "2JN", order: 63,
        name_en: "2 John",
        name_pt: "2 João",
        name_es: "2 John",
        name_fr: "2 John",
        name_de: "2 John",
        name_grc: "2 John",
        name_heb: "2 John",
        aliases_en: &["2John", "2 John", "2Jn", "2Jo", "II John", "Second John"],
        aliases_pt: &["2Jo", "2 Jo", "2 João", "2 Joao", "II João", "Segunda João", "Segundo João"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 1, testament: "NT"
    },
    BookEntry {
        canonical_id: "3JN", helloao_id: "3JN", order: 64,
        name_en: "3 John",
        name_pt: "3 João",
        name_es: "3 John",
        name_fr: "3 John",
        name_de: "3 John",
        name_grc: "3 John",
        name_heb: "3 John",
        aliases_en: &["3John", "3 John", "3Jn", "3Jo", "III John", "Third John"],
        aliases_pt: &["3Jo", "3 Jo", "3 João", "3 Joao", "III João", "Terceira João", "Terceiro João"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 1, testament: "NT"
    },
    BookEntry {
        canonical_id: "JUD", helloao_id: "JUD", order: 65,
        name_en: "Jude",
        name_pt: "Judas",
        name_es: "Jude",
        name_fr: "Jude",
        name_de: "Jude",
        name_grc: "Jude",
        name_heb: "Jude",
        aliases_en: &["Jud", "Jude"],
        aliases_pt: &["Jd", "Judas"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 1, testament: "NT"
    },
    BookEntry {
        canonical_id: "REV", helloao_id: "REV", order: 66,
        name_en: "Revelation",
        name_pt: "Apocalipse",
        name_es: "Revelation",
        name_fr: "Revelation",
        name_de: "Revelation",
        name_grc: "Revelation",
        name_heb: "Revelation",
        aliases_en: &["Rev", "Re", "Apoc", "Apocalypse"],
        aliases_pt: &["Ap", "Apoc", "Apocalipse"],
        aliases_es: &[],
        aliases_fr: &[],
        aliases_de: &[],
        aliases_grc: &[],
        aliases_heb: &[],
        chapters: 22, testament: "NT"
    },
];

// Note: Some book names use English fallback for languages without data yet.
// TODO: Populate full multilingual names for all 66 books.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn has_66_books() {
        assert_eq!(BOOKS.len(), 66);
    }

    #[test]
    fn canonical_ids_are_unique() {
        let mut seen = std::collections::HashSet::new();
        for b in BOOKS {
            assert!(seen.insert(b.canonical_id), "duplicate {}", b.canonical_id);
        }
    }

    #[test]
    fn canonical_lookup_round_trips() {
        assert_eq!(by_canonical_id("JHN").unwrap().name_en, "John");
        assert_eq!(by_canonical_id("jhn").unwrap().canonical_id, "JHN");
        assert!(by_canonical_id("ZZZ").is_none());
    }

    #[test]
    fn alias_index_resolves_pt_and_en() {
        let idx = alias_index();
        assert_eq!(idx.get("john").copied(), Some("JHN"));
        assert_eq!(idx.get("joao").copied(), Some("JHN"));
        assert_eq!(idx.get("jhn").copied(), Some("JHN"));
        assert_eq!(idx.get("1cor").copied(), Some("1CO"));
        assert_eq!(idx.get("1corintios").copied(), Some("1CO"));
        assert_eq!(idx.get("primeiracorintios").copied(), Some("1CO"));
        assert_eq!(idx.get("psalms").copied(), Some("PSA"));
        assert_eq!(idx.get("salmos").copied(), Some("PSA"));
    }

    #[test]
    fn order_is_1_through_66_in_sequence() {
        for (i, b) in BOOKS.iter().enumerate() {
            assert_eq!(
                b.order as usize,
                i + 1,
                "out of order at {}",
                b.canonical_id
            );
        }
    }
}
