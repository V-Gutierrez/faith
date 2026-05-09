use faith::reference::{parse, ParsedRef};

fn r(book: &str, chapter: u16, verse: Option<u16>) -> ParsedRef {
    ParsedRef {
        book: book.into(),
        chapter,
        verse,
        end_chapter: None,
        end_verse: None,
    }
}

fn rr(book: &str, c1: u16, v1: u16, c2: u16, v2: u16) -> ParsedRef {
    ParsedRef {
        book: book.into(),
        chapter: c1,
        verse: Some(v1),
        end_chapter: Some(c2),
        end_verse: Some(v2),
    }
}

#[test]
fn english_corpus_30_cases() {
    let cases: &[(&str, ParsedRef)] = &[
        ("Genesis 1:1", r("GEN", 1, Some(1))),
        ("Gen 1:1", r("GEN", 1, Some(1))),
        ("Gn 1:1", r("GEN", 1, Some(1))),
        ("Exodus 20:3", r("EXO", 20, Some(3))),
        ("Ex 20:3", r("EXO", 20, Some(3))),
        ("Leviticus 19:18", r("LEV", 19, Some(18))),
        ("Numbers 6:24", r("NUM", 6, Some(24))),
        ("Deuteronomy 6:4", r("DEU", 6, Some(4))),
        ("Joshua 1:9", r("JOS", 1, Some(9))),
        ("Judges 6:12", r("JDG", 6, Some(12))),
        ("Ruth 1:16", r("RUT", 1, Some(16))),
        ("1 Samuel 17:45", r("1SA", 17, Some(45))),
        ("1Sam 17:45", r("1SA", 17, Some(45))),
        ("2 Kings 6:17", r("2KI", 6, Some(17))),
        ("Psalms 23", r("PSA", 23, None)),
        ("Ps 23:1", r("PSA", 23, Some(1))),
        ("Proverbs 3:5", r("PRO", 3, Some(5))),
        ("Isaiah 40:31", r("ISA", 40, Some(31))),
        ("Jeremiah 29:11", r("JER", 29, Some(11))),
        ("Daniel 3:17", r("DAN", 3, Some(17))),
        ("Matthew 5:3", r("MAT", 5, Some(3))),
        ("Mt 5:3", r("MAT", 5, Some(3))),
        ("Mark 1:1", r("MRK", 1, Some(1))),
        ("Luke 2:11", r("LUK", 2, Some(11))),
        ("John 3:16", r("JHN", 3, Some(16))),
        ("Jn 3.16", r("JHN", 3, Some(16))),
        ("Acts 2:38", r("ACT", 2, Some(38))),
        ("Romans 8:28", r("ROM", 8, Some(28))),
        ("1 Corinthians 13:4", r("1CO", 13, Some(4))),
        ("Galatians 5:22", r("GAL", 5, Some(22))),
        ("Ephesians 2:8", r("EPH", 2, Some(8))),
        ("Philippians 4:13", r("PHP", 4, Some(13))),
        ("Hebrews 11:1", r("HEB", 11, Some(1))),
        ("James 1:5", r("JAS", 1, Some(5))),
        ("Revelation 21:4", r("REV", 21, Some(4))),
        ("John 3:16-17", rr("JHN", 3, 16, 3, 17)),
        ("John 3:35-4:2", rr("JHN", 3, 35, 4, 2)),
    ];
    assert!(
        cases.len() >= 30,
        "need >=30 EN cases, have {}",
        cases.len()
    );
    for (input, expected) in cases {
        let got = parse(input).unwrap_or_else(|e| panic!("EN parse failed for {input:?}: {e}"));
        assert_eq!(&got, expected, "mismatch for {input:?}");
    }
}

#[test]
fn portuguese_corpus_30_cases() {
    let cases: &[(&str, ParsedRef)] = &[
        ("Gênesis 1:1", r("GEN", 1, Some(1))),
        ("Genesis 1:1", r("GEN", 1, Some(1))),
        ("Gn 1:1", r("GEN", 1, Some(1))),
        ("Êxodo 20:3", r("EXO", 20, Some(3))),
        ("Ex 20:3", r("EXO", 20, Some(3))),
        ("Levítico 19:18", r("LEV", 19, Some(18))),
        ("Lv 19:18", r("LEV", 19, Some(18))),
        ("Números 6:24", r("NUM", 6, Some(24))),
        ("Deuteronômio 6:4", r("DEU", 6, Some(4))),
        ("Josué 1:9", r("JOS", 1, Some(9))),
        ("Juízes 6:12", r("JDG", 6, Some(12))),
        ("Rute 1:16", r("RUT", 1, Some(16))),
        ("1 Samuel 17:45", r("1SA", 17, Some(45))),
        ("Primeira Samuel 17:45", r("1SA", 17, Some(45))),
        ("2 Reis 6:17", r("2KI", 6, Some(17))),
        ("Salmos 23", r("PSA", 23, None)),
        ("Sl 23:1", r("PSA", 23, Some(1))),
        ("Provérbios 3:5", r("PRO", 3, Some(5))),
        ("Isaías 40:31", r("ISA", 40, Some(31))),
        ("Jeremias 29:11", r("JER", 29, Some(11))),
        ("Daniel 3:17", r("DAN", 3, Some(17))),
        ("Mateus 5:3", r("MAT", 5, Some(3))),
        ("Mt 5:3", r("MAT", 5, Some(3))),
        ("Marcos 1:1", r("MRK", 1, Some(1))),
        ("Lucas 2:11", r("LUK", 2, Some(11))),
        ("João 3:16", r("JHN", 3, Some(16))),
        ("Joao 3:16", r("JHN", 3, Some(16))),
        ("Jo 3.16", r("JHN", 3, Some(16))),
        ("Atos 2:38", r("ACT", 2, Some(38))),
        ("Romanos 8:28", r("ROM", 8, Some(28))),
        ("1 Coríntios 13:4", r("1CO", 13, Some(4))),
        ("Gálatas 5:22", r("GAL", 5, Some(22))),
        ("Efésios 2:8", r("EPH", 2, Some(8))),
        ("Filipenses 4:13", r("PHP", 4, Some(13))),
        ("Hebreus 11:1", r("HEB", 11, Some(1))),
        ("Tiago 1:5", r("JAS", 1, Some(5))),
        ("Apocalipse 21:4", r("REV", 21, Some(4))),
        ("João 3:16-17", rr("JHN", 3, 16, 3, 17)),
        ("João 3:35-4:2", rr("JHN", 3, 35, 4, 2)),
    ];
    assert!(
        cases.len() >= 30,
        "need >=30 PT cases, have {}",
        cases.len()
    );
    for (input, expected) in cases {
        let got = parse(input).unwrap_or_else(|e| panic!("PT parse failed for {input:?}: {e}"));
        assert_eq!(&got, expected, "mismatch for {input:?}");
    }
}
