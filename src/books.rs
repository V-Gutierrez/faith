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
    pub aliases_en: &'static [&'static str],
    pub aliases_pt: &'static [&'static str],
    /// Canonical Protestant chapter count (KJV-style 66-book canon).
    pub chapters: u16,
    /// `"OT"` (orders 1..=39) or `"NT"` (orders 40..=66).
    pub testament: &'static str,
}

/// Combined alias list (EN + PT, deduped, lowercased) — useful for `info` output.
pub fn aliases_lower(b: &BookEntry) -> Vec<String> {
    let mut out: Vec<String> = Vec::new();
    let mut push = |s: &str| {
        let lo = s.to_ascii_lowercase();
        if !out.iter().any(|e| e == &lo) {
            out.push(lo);
        }
    };
    for a in b.aliases_en.iter().chain(b.aliases_pt.iter()) {
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
            insert_alias(&mut m, b.name_en, b.canonical_id);
            insert_alias(&mut m, b.name_pt, b.canonical_id);
            for a in b.aliases_en.iter().chain(b.aliases_pt.iter()) {
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
    BookEntry { canonical_id: "GEN", helloao_id: "GEN", order:  1, name_en: "Genesis",         name_pt: "Gênesis",
        aliases_en: &["Gen", "Gn"], aliases_pt: &["Gn", "Gen"], chapters: 50, testament: "OT" },
    BookEntry { canonical_id: "EXO", helloao_id: "EXO", order:  2, name_en: "Exodus",          name_pt: "Êxodo",
        aliases_en: &["Exod", "Exo", "Ex"], aliases_pt: &["Ex", "Exo", "Exod"], chapters: 40, testament: "OT" },
    BookEntry { canonical_id: "LEV", helloao_id: "LEV", order:  3, name_en: "Leviticus",       name_pt: "Levítico",
        aliases_en: &["Lev", "Lv"], aliases_pt: &["Lv", "Lev"], chapters: 27, testament: "OT" },
    BookEntry { canonical_id: "NUM", helloao_id: "NUM", order:  4, name_en: "Numbers",         name_pt: "Números",
        aliases_en: &["Num", "Nm", "Nu"], aliases_pt: &["Nm", "Num"], chapters: 36, testament: "OT" },
    BookEntry { canonical_id: "DEU", helloao_id: "DEU", order:  5, name_en: "Deuteronomy",     name_pt: "Deuteronômio",
        aliases_en: &["Deut", "Deu", "Dt"], aliases_pt: &["Dt", "Deut", "Deu"], chapters: 34, testament: "OT" },
    BookEntry { canonical_id: "JOS", helloao_id: "JOS", order:  6, name_en: "Joshua",          name_pt: "Josué",
        aliases_en: &["Josh", "Jos"], aliases_pt: &["Js", "Jos", "Josué"], chapters: 24, testament: "OT" },
    BookEntry { canonical_id: "JDG", helloao_id: "JDG", order:  7, name_en: "Judges",          name_pt: "Juízes",
        aliases_en: &["Judg", "Jdg", "Jdgs"], aliases_pt: &["Jz", "Juízes", "Juizes"], chapters: 21, testament: "OT" },
    BookEntry { canonical_id: "RUT", helloao_id: "RUT", order:  8, name_en: "Ruth",            name_pt: "Rute",
        aliases_en: &["Rut", "Ru"], aliases_pt: &["Rt", "Rute"], chapters: 4, testament: "OT" },
    BookEntry { canonical_id: "1SA", helloao_id: "1SA", order:  9, name_en: "1 Samuel",        name_pt: "1 Samuel",
        aliases_en: &["1Sam", "1 Sam", "1Sa", "I Samuel", "First Samuel"],
        aliases_pt: &["1Sm", "1 Sm", "1 Sam", "I Samuel", "Primeiro Samuel", "Primeira Samuel"], chapters: 31, testament: "OT" },
    BookEntry { canonical_id: "2SA", helloao_id: "2SA", order: 10, name_en: "2 Samuel",        name_pt: "2 Samuel",
        aliases_en: &["2Sam", "2 Sam", "2Sa", "II Samuel", "Second Samuel"],
        aliases_pt: &["2Sm", "2 Sm", "2 Sam", "II Samuel", "Segundo Samuel", "Segunda Samuel"], chapters: 24, testament: "OT" },
    BookEntry { canonical_id: "1KI", helloao_id: "1KI", order: 11, name_en: "1 Kings",         name_pt: "1 Reis",
        aliases_en: &["1Kgs", "1 Kgs", "1Ki", "I Kings", "First Kings"],
        aliases_pt: &["1Rs", "1 Rs", "1 Reis", "I Reis", "Primeiro Reis", "Primeira Reis"], chapters: 22, testament: "OT" },
    BookEntry { canonical_id: "2KI", helloao_id: "2KI", order: 12, name_en: "2 Kings",         name_pt: "2 Reis",
        aliases_en: &["2Kgs", "2 Kgs", "2Ki", "II Kings", "Second Kings"],
        aliases_pt: &["2Rs", "2 Rs", "2 Reis", "II Reis", "Segundo Reis", "Segunda Reis"], chapters: 25, testament: "OT" },
    BookEntry { canonical_id: "1CH", helloao_id: "1CH", order: 13, name_en: "1 Chronicles",    name_pt: "1 Crônicas",
        aliases_en: &["1Chr", "1 Chr", "1Ch", "I Chronicles", "First Chronicles"],
        aliases_pt: &["1Cr", "1 Cr", "1 Crônicas", "1 Cronicas", "I Crônicas", "Primeira Crônicas"], chapters: 29, testament: "OT" },
    BookEntry { canonical_id: "2CH", helloao_id: "2CH", order: 14, name_en: "2 Chronicles",    name_pt: "2 Crônicas",
        aliases_en: &["2Chr", "2 Chr", "2Ch", "II Chronicles", "Second Chronicles"],
        aliases_pt: &["2Cr", "2 Cr", "2 Crônicas", "2 Cronicas", "II Crônicas", "Segunda Crônicas"], chapters: 36, testament: "OT" },
    BookEntry { canonical_id: "EZR", helloao_id: "EZR", order: 15, name_en: "Ezra",            name_pt: "Esdras",
        aliases_en: &["Ezr"], aliases_pt: &["Ed", "Esd"], chapters: 10, testament: "OT" },
    BookEntry { canonical_id: "NEH", helloao_id: "NEH", order: 16, name_en: "Nehemiah",        name_pt: "Neemias",
        aliases_en: &["Neh", "Ne"], aliases_pt: &["Ne", "Neh"], chapters: 13, testament: "OT" },
    BookEntry { canonical_id: "EST", helloao_id: "EST", order: 17, name_en: "Esther",          name_pt: "Ester",
        aliases_en: &["Est", "Es"], aliases_pt: &["Et", "Est"], chapters: 10, testament: "OT" },
    BookEntry { canonical_id: "JOB", helloao_id: "JOB", order: 18, name_en: "Job",             name_pt: "Jó",
        aliases_en: &["Jb"], aliases_pt: &["Jó", "Jo"], chapters: 42, testament: "OT" },
    BookEntry { canonical_id: "PSA", helloao_id: "PSA", order: 19, name_en: "Psalms",          name_pt: "Salmos",
        aliases_en: &["Ps", "Psa", "Psalm", "Pss"], aliases_pt: &["Sl", "Sal", "Salmo"], chapters: 150, testament: "OT" },
    BookEntry { canonical_id: "PRO", helloao_id: "PRO", order: 20, name_en: "Proverbs",        name_pt: "Provérbios",
        aliases_en: &["Prov", "Pro", "Pr", "Prv"], aliases_pt: &["Pv", "Prov", "Provérbios", "Proverbios"], chapters: 31, testament: "OT" },
    BookEntry { canonical_id: "ECC", helloao_id: "ECC", order: 21, name_en: "Ecclesiastes",    name_pt: "Eclesiastes",
        aliases_en: &["Eccl", "Ecc", "Ec", "Qoh"], aliases_pt: &["Ec", "Ecl"], chapters: 12, testament: "OT" },
    BookEntry { canonical_id: "SNG", helloao_id: "SNG", order: 22, name_en: "Song of Solomon", name_pt: "Cantares",
        aliases_en: &["Song", "Sng", "Sos", "Song of Songs", "Canticles"],
        aliases_pt: &["Ct", "Cantares", "Cânticos", "Canticos", "Cântico dos Cânticos"], chapters: 8, testament: "OT" },
    BookEntry { canonical_id: "ISA", helloao_id: "ISA", order: 23, name_en: "Isaiah",          name_pt: "Isaías",
        aliases_en: &["Isa", "Is"], aliases_pt: &["Is", "Isa"], chapters: 66, testament: "OT" },
    BookEntry { canonical_id: "JER", helloao_id: "JER", order: 24, name_en: "Jeremiah",        name_pt: "Jeremias",
        aliases_en: &["Jer", "Je"], aliases_pt: &["Jr", "Jer"], chapters: 52, testament: "OT" },
    BookEntry { canonical_id: "LAM", helloao_id: "LAM", order: 25, name_en: "Lamentations",    name_pt: "Lamentações",
        aliases_en: &["Lam", "La"], aliases_pt: &["Lm", "Lam", "Lamentacoes"], chapters: 5, testament: "OT" },
    BookEntry { canonical_id: "EZK", helloao_id: "EZK", order: 26, name_en: "Ezekiel",         name_pt: "Ezequiel",
        aliases_en: &["Ezek", "Ezk", "Eze"], aliases_pt: &["Ez", "Eze", "Ezeq"], chapters: 48, testament: "OT" },
    BookEntry { canonical_id: "DAN", helloao_id: "DAN", order: 27, name_en: "Daniel",          name_pt: "Daniel",
        aliases_en: &["Dan", "Dn"], aliases_pt: &["Dn", "Dan"], chapters: 12, testament: "OT" },
    BookEntry { canonical_id: "HOS", helloao_id: "HOS", order: 28, name_en: "Hosea",           name_pt: "Oséias",
        aliases_en: &["Hos", "Ho"], aliases_pt: &["Os", "Oséias", "Oseias"], chapters: 14, testament: "OT" },
    BookEntry { canonical_id: "JOL", helloao_id: "JOL", order: 29, name_en: "Joel",            name_pt: "Joel",
        aliases_en: &["Joel", "Jl"], aliases_pt: &["Jl", "Joel"], chapters: 3, testament: "OT" },
    BookEntry { canonical_id: "AMO", helloao_id: "AMO", order: 30, name_en: "Amos",            name_pt: "Amós",
        aliases_en: &["Am"], aliases_pt: &["Am", "Amós", "Amos"], chapters: 9, testament: "OT" },
    BookEntry { canonical_id: "OBA", helloao_id: "OBA", order: 31, name_en: "Obadiah",         name_pt: "Obadias",
        aliases_en: &["Obad", "Oba", "Ob"], aliases_pt: &["Ob", "Obad"], chapters: 1, testament: "OT" },
    BookEntry { canonical_id: "JON", helloao_id: "JON", order: 32, name_en: "Jonah",           name_pt: "Jonas",
        aliases_en: &["Jon", "Jnh"], aliases_pt: &["Jn", "Jon"], chapters: 4, testament: "OT" },
    BookEntry { canonical_id: "MIC", helloao_id: "MIC", order: 33, name_en: "Micah",           name_pt: "Miquéias",
        aliases_en: &["Mic", "Mi"], aliases_pt: &["Mq", "Miquéias", "Miqueias"], chapters: 7, testament: "OT" },
    BookEntry { canonical_id: "NAM", helloao_id: "NAM", order: 34, name_en: "Nahum",           name_pt: "Naum",
        aliases_en: &["Nah", "Nam", "Na"], aliases_pt: &["Na", "Nau", "Naum"], chapters: 3, testament: "OT" },
    BookEntry { canonical_id: "HAB", helloao_id: "HAB", order: 35, name_en: "Habakkuk",        name_pt: "Habacuque",
        aliases_en: &["Hab", "Hb"], aliases_pt: &["Hc", "Hab", "Habacuque"], chapters: 3, testament: "OT" },
    BookEntry { canonical_id: "ZEP", helloao_id: "ZEP", order: 36, name_en: "Zephaniah",       name_pt: "Sofonias",
        aliases_en: &["Zeph", "Zep", "Zp"], aliases_pt: &["Sf", "Sof"], chapters: 3, testament: "OT" },
    BookEntry { canonical_id: "HAG", helloao_id: "HAG", order: 37, name_en: "Haggai",          name_pt: "Ageu",
        aliases_en: &["Hag", "Hg"], aliases_pt: &["Ag", "Ageu"], chapters: 2, testament: "OT" },
    BookEntry { canonical_id: "ZEC", helloao_id: "ZEC", order: 38, name_en: "Zechariah",       name_pt: "Zacarias",
        aliases_en: &["Zech", "Zec", "Zc"], aliases_pt: &["Zc", "Zac"], chapters: 14, testament: "OT" },
    BookEntry { canonical_id: "MAL", helloao_id: "MAL", order: 39, name_en: "Malachi",         name_pt: "Malaquias",
        aliases_en: &["Mal", "Ml"], aliases_pt: &["Ml", "Mal", "Malaquias"], chapters: 4, testament: "OT" },
    BookEntry { canonical_id: "MAT", helloao_id: "MAT", order: 40, name_en: "Matthew",         name_pt: "Mateus",
        aliases_en: &["Matt", "Mat", "Mt"], aliases_pt: &["Mt", "Mat", "Mateus"], chapters: 28, testament: "NT" },
    BookEntry { canonical_id: "MRK", helloao_id: "MRK", order: 41, name_en: "Mark",            name_pt: "Marcos",
        aliases_en: &["Mark", "Mrk", "Mk"], aliases_pt: &["Mc", "Mar", "Marcos"], chapters: 16, testament: "NT" },
    BookEntry { canonical_id: "LUK", helloao_id: "LUK", order: 42, name_en: "Luke",            name_pt: "Lucas",
        aliases_en: &["Luk", "Lk"], aliases_pt: &["Lc", "Luc", "Lucas"], chapters: 24, testament: "NT" },
    BookEntry { canonical_id: "JHN", helloao_id: "JHN", order: 43, name_en: "John",            name_pt: "João",
        aliases_en: &["John", "Jhn", "Jn", "Joh"], aliases_pt: &["Jo", "João", "Joao"], chapters: 21, testament: "NT" },
    BookEntry { canonical_id: "ACT", helloao_id: "ACT", order: 44, name_en: "Acts",            name_pt: "Atos",
        aliases_en: &["Act", "Ac"], aliases_pt: &["At", "Atos", "Ato"], chapters: 28, testament: "NT" },
    BookEntry { canonical_id: "ROM", helloao_id: "ROM", order: 45, name_en: "Romans",          name_pt: "Romanos",
        aliases_en: &["Rom", "Ro", "Rm"], aliases_pt: &["Rm", "Rom", "Romanos"], chapters: 16, testament: "NT" },
    BookEntry { canonical_id: "1CO", helloao_id: "1CO", order: 46, name_en: "1 Corinthians",   name_pt: "1 Coríntios",
        aliases_en: &["1Cor", "1 Cor", "1Co", "I Corinthians", "First Corinthians"],
        aliases_pt: &["1Co", "1 Co", "1 Cor", "1 Coríntios", "1 Corintios", "I Coríntios", "Primeira Coríntios", "Primeiro Coríntios"], chapters: 16, testament: "NT" },
    BookEntry { canonical_id: "2CO", helloao_id: "2CO", order: 47, name_en: "2 Corinthians",   name_pt: "2 Coríntios",
        aliases_en: &["2Cor", "2 Cor", "2Co", "II Corinthians", "Second Corinthians"],
        aliases_pt: &["2Co", "2 Co", "2 Cor", "2 Coríntios", "2 Corintios", "II Coríntios", "Segunda Coríntios", "Segundo Coríntios"], chapters: 13, testament: "NT" },
    BookEntry { canonical_id: "GAL", helloao_id: "GAL", order: 48, name_en: "Galatians",       name_pt: "Gálatas",
        aliases_en: &["Gal", "Ga"], aliases_pt: &["Gl", "Gal", "Gálatas", "Galatas"], chapters: 6, testament: "NT" },
    BookEntry { canonical_id: "EPH", helloao_id: "EPH", order: 49, name_en: "Ephesians",       name_pt: "Efésios",
        aliases_en: &["Eph", "Ep"], aliases_pt: &["Ef", "Ef", "Efésios", "Efesios"], chapters: 6, testament: "NT" },
    BookEntry { canonical_id: "PHP", helloao_id: "PHP", order: 50, name_en: "Philippians",     name_pt: "Filipenses",
        aliases_en: &["Phil", "Php", "Phi"], aliases_pt: &["Fp", "Fl", "Fil", "Filipenses"], chapters: 4, testament: "NT" },
    BookEntry { canonical_id: "COL", helloao_id: "COL", order: 51, name_en: "Colossians",      name_pt: "Colossenses",
        aliases_en: &["Col"], aliases_pt: &["Cl", "Col", "Colossenses"], chapters: 4, testament: "NT" },
    BookEntry { canonical_id: "1TH", helloao_id: "1TH", order: 52, name_en: "1 Thessalonians", name_pt: "1 Tessalonicenses",
        aliases_en: &["1Thess", "1 Thess", "1Th", "I Thessalonians", "First Thessalonians"],
        aliases_pt: &["1Ts", "1 Ts", "1 Tess", "1 Tessalonicenses", "I Tessalonicenses", "Primeira Tessalonicenses"], chapters: 5, testament: "NT" },
    BookEntry { canonical_id: "2TH", helloao_id: "2TH", order: 53, name_en: "2 Thessalonians", name_pt: "2 Tessalonicenses",
        aliases_en: &["2Thess", "2 Thess", "2Th", "II Thessalonians", "Second Thessalonians"],
        aliases_pt: &["2Ts", "2 Ts", "2 Tess", "2 Tessalonicenses", "II Tessalonicenses", "Segunda Tessalonicenses"], chapters: 3, testament: "NT" },
    BookEntry { canonical_id: "1TI", helloao_id: "1TI", order: 54, name_en: "1 Timothy",       name_pt: "1 Timóteo",
        aliases_en: &["1Tim", "1 Tim", "1Ti", "I Timothy", "First Timothy"],
        aliases_pt: &["1Tm", "1 Tm", "1 Tim", "1 Timóteo", "1 Timoteo", "I Timóteo", "Primeira Timóteo"], chapters: 6, testament: "NT" },
    BookEntry { canonical_id: "2TI", helloao_id: "2TI", order: 55, name_en: "2 Timothy",       name_pt: "2 Timóteo",
        aliases_en: &["2Tim", "2 Tim", "2Ti", "II Timothy", "Second Timothy"],
        aliases_pt: &["2Tm", "2 Tm", "2 Tim", "2 Timóteo", "2 Timoteo", "II Timóteo", "Segunda Timóteo"], chapters: 4, testament: "NT" },
    BookEntry { canonical_id: "TIT", helloao_id: "TIT", order: 56, name_en: "Titus",           name_pt: "Tito",
        aliases_en: &["Tit"], aliases_pt: &["Tt", "Tito"], chapters: 3, testament: "NT" },
    BookEntry { canonical_id: "PHM", helloao_id: "PHM", order: 57, name_en: "Philemon",        name_pt: "Filemom",
        aliases_en: &["Phlm", "Phm", "Pm"], aliases_pt: &["Fm", "File", "Filemom", "Filemon"], chapters: 1, testament: "NT" },
    BookEntry { canonical_id: "HEB", helloao_id: "HEB", order: 58, name_en: "Hebrews",         name_pt: "Hebreus",
        aliases_en: &["Heb", "He"], aliases_pt: &["Hb", "Heb", "Hebreus"], chapters: 13, testament: "NT" },
    BookEntry { canonical_id: "JAS", helloao_id: "JAS", order: 59, name_en: "James",           name_pt: "Tiago",
        aliases_en: &["Jas", "Jam"], aliases_pt: &["Tg", "Tia", "Tiago"], chapters: 5, testament: "NT" },
    BookEntry { canonical_id: "1PE", helloao_id: "1PE", order: 60, name_en: "1 Peter",         name_pt: "1 Pedro",
        aliases_en: &["1Pet", "1 Pet", "1Pe", "I Peter", "First Peter"],
        aliases_pt: &["1Pe", "1 Pe", "1 Ped", "1 Pedro", "I Pedro", "Primeira Pedro", "Primeiro Pedro"], chapters: 5, testament: "NT" },
    BookEntry { canonical_id: "2PE", helloao_id: "2PE", order: 61, name_en: "2 Peter",         name_pt: "2 Pedro",
        aliases_en: &["2Pet", "2 Pet", "2Pe", "II Peter", "Second Peter"],
        aliases_pt: &["2Pe", "2 Pe", "2 Ped", "2 Pedro", "II Pedro", "Segunda Pedro", "Segundo Pedro"], chapters: 3, testament: "NT" },
    BookEntry { canonical_id: "1JN", helloao_id: "1JN", order: 62, name_en: "1 John",          name_pt: "1 João",
        aliases_en: &["1John", "1 John", "1Jn", "1Jo", "I John", "First John"],
        aliases_pt: &["1Jo", "1 Jo", "1 João", "1 Joao", "I João", "Primeira João", "Primeiro João"], chapters: 5, testament: "NT" },
    BookEntry { canonical_id: "2JN", helloao_id: "2JN", order: 63, name_en: "2 John",          name_pt: "2 João",
        aliases_en: &["2John", "2 John", "2Jn", "2Jo", "II John", "Second John"],
        aliases_pt: &["2Jo", "2 Jo", "2 João", "2 Joao", "II João", "Segunda João", "Segundo João"], chapters: 1, testament: "NT" },
    BookEntry { canonical_id: "3JN", helloao_id: "3JN", order: 64, name_en: "3 John",          name_pt: "3 João",
        aliases_en: &["3John", "3 John", "3Jn", "3Jo", "III John", "Third John"],
        aliases_pt: &["3Jo", "3 Jo", "3 João", "3 Joao", "III João", "Terceira João", "Terceiro João"], chapters: 1, testament: "NT" },
    BookEntry { canonical_id: "JUD", helloao_id: "JUD", order: 65, name_en: "Jude",            name_pt: "Judas",
        aliases_en: &["Jud", "Jude"], aliases_pt: &["Jd", "Judas"], chapters: 1, testament: "NT" },
    BookEntry { canonical_id: "REV", helloao_id: "REV", order: 66, name_en: "Revelation",      name_pt: "Apocalipse",
        aliases_en: &["Rev", "Re", "Apoc", "Apocalypse"], aliases_pt: &["Ap", "Apoc", "Apocalipse"], chapters: 22, testament: "NT" },
];

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
