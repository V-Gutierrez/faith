use std::io::Cursor;

use faith::cli;
use faith::store::{Store, StoredTranslation};
use tempfile::TempDir;

fn fresh_store() -> (Store, TempDir) {
    let d = tempfile::tempdir().unwrap();
    let p = d.path().join("bible.db");
    let mut s = Store::open(&p).unwrap();

    s.upsert_translation(&StoredTranslation {
        id: "KJV".into(),
        name: "King James (Authorized) Version".into(),
        english_name: "King James Version".into(),
        language: "eng".into(),
        direction: "ltr".into(),
        license: "Public Domain".into(),
        source_url: "https://bible.helloao.org/api/eng_kjv/complete.json".into(),
        installed_at: "2026-05-09T00:00:00Z".into(),
        books: 0,
        verses: 0,
    })
    .unwrap();

    s.upsert_translation(&StoredTranslation {
        id: "ONBV".into(),
        name: "Biblica® Open Nova Bíblia Viva 2007".into(),
        english_name: "Portuguese Open Nova Bíblia Viva".into(),
        language: "por".into(),
        direction: "ltr".into(),
        license: "CC BY-SA 4.0".into(),
        source_url: "https://bible.helloao.org/api/por_onbv/complete.json".into(),
        installed_at: "2026-05-09T00:00:00Z".into(),
        books: 0,
        verses: 0,
    })
    .unwrap();

    s.replace_verses(
        "KJV",
        &[
            ("JHN".into(), 3, 16, "For God so loved the world, that he gave his only begotten Son, that whosoever believeth in him should not perish, but have everlasting life.".into()),
            ("JHN".into(), 3, 17, "For God sent not his Son into the world to condemn the world; but that the world through him might be saved.".into()),
            ("JHN".into(), 3, 18, "He that believeth on him is not condemned.".into()),
            ("JHN".into(), 4, 1, "When therefore the Lord knew how the Pharisees had heard...".into()),
            ("JHN".into(), 4, 2, "(Though Jesus himself baptized not, but his disciples,)".into()),
            ("PSA".into(), 23, 1, "The LORD is my shepherd; I shall not want.".into()),
            ("ROM".into(), 8, 28, "And we know that all things work together for good to them that love God.".into()),
        ],
    )
    .unwrap();

    s.replace_verses(
        "ONBV",
        &[
            ("JHN".into(), 3, 16, "Porque Deus amou tanto o mundo que deu o seu Filho unigênito, para que todo aquele que nele crê não pereça, mas tenha a vida eterna.".into()),
            ("PSA".into(), 23, 1, "O SENHOR é meu pastor; nada me faltará.".into()),
        ],
    )
    .unwrap();

    (s, d)
}

fn run_get(store: &Store, reference: &str, trs: &[&str], text: bool) -> (i32, String) {
    let mut buf = Cursor::new(Vec::<u8>::new());
    let trs_owned: Vec<String> = trs.iter().map(|s| s.to_string()).collect();
    let code = cli::get::run(store, reference, &trs_owned, text, &mut buf).unwrap();
    (code, String::from_utf8(buf.into_inner()).unwrap())
}

#[test]
fn get_single_verse_kjv_json_snapshot() {
    let (s, _d) = fresh_store();
    let (code, out) = run_get(&s, "John 3:16", &["KJV"], false);
    assert_eq!(code, 0);
    insta::assert_snapshot!(out);
}

#[test]
fn get_verse_range_same_chapter_snapshot() {
    let (s, _d) = fresh_store();
    let (code, out) = run_get(&s, "John 3:16-18", &["KJV"], false);
    assert_eq!(code, 0);
    insta::assert_snapshot!(out);
}

#[test]
fn get_chapter_only_snapshot() {
    let (s, _d) = fresh_store();
    let (code, out) = run_get(&s, "Psalms 23", &["KJV"], false);
    assert_eq!(code, 0);
    insta::assert_snapshot!(out);
}

#[test]
fn get_cross_chapter_range_snapshot() {
    let (s, _d) = fresh_store();
    let (code, out) = run_get(&s, "John 3:18-4:2", &["KJV"], false);
    assert_eq!(code, 0);
    insta::assert_snapshot!(out);
}

#[test]
fn get_multi_translation_snapshot() {
    let (s, _d) = fresh_store();
    let (code, out) = run_get(&s, "John 3:16", &["KJV", "ONBV"], false);
    assert_eq!(code, 0);
    insta::assert_snapshot!(out);
}

#[test]
fn get_unknown_translation_returns_error_object() {
    let (s, _d) = fresh_store();
    let (code, out) = run_get(&s, "John 3:16", &["XYZ"], false);
    assert_eq!(code, 4);
    assert!(out.contains("E_TRANSLATION_MISSING"), "stdout: {out}");
}

#[test]
fn get_bad_reference_returns_parse_error() {
    let (s, _d) = fresh_store();
    let (code, out) = run_get(&s, "Florbal 99:99", &["KJV"], false);
    assert_eq!(code, 2);
    assert!(out.contains("E_REF_PARSE"), "stdout: {out}");
}

#[test]
fn get_text_format_snapshot() {
    let (s, _d) = fresh_store();
    let (code, out) = run_get(&s, "John 3:16", &["KJV"], true);
    assert_eq!(code, 0);
    insta::assert_snapshot!(out);
}

#[test]
fn batch_preserves_order_snapshot() {
    let (s, _d) = fresh_store();
    let mut stdin =
        Cursor::new(br#"["John 3:16","Psalms 23:1","Romans 8:28","Florbal 1:1"]"#.to_vec());
    let mut buf = Cursor::new(Vec::<u8>::new());
    let code = cli::batch::run(&s, "KJV", false, &mut stdin, &mut buf).unwrap();
    assert_eq!(code, 2);
    insta::assert_snapshot!(String::from_utf8(buf.into_inner()).unwrap());
}

#[test]
fn list_translations_snapshot() {
    let (s, _d) = fresh_store();
    let mut buf = Cursor::new(Vec::<u8>::new());
    cli::list::run_translations(&s, None, false, &mut buf).unwrap();
    let out = String::from_utf8(buf.into_inner()).unwrap();
    let normalized = out.replace(
        "\"installed_at\":\"2026-05-09T00:00:00Z\"",
        "\"installed_at\":\"<TS>\"",
    );
    insta::assert_snapshot!(normalized);
}

#[test]
fn list_books_snapshot() {
    let (s, _d) = fresh_store();
    let mut buf = Cursor::new(Vec::<u8>::new());
    cli::list::run_books(&s, "KJV", &mut buf).unwrap();
    insta::assert_snapshot!(String::from_utf8(buf.into_inner()).unwrap());
}

#[test]
fn info_book_no_translation_snapshot() {
    let (s, _d) = fresh_store();
    let mut buf = Cursor::new(Vec::<u8>::new());
    let code = cli::info::run(&s, "JHN", None, &mut buf).unwrap();
    assert_eq!(code, 0);
    insta::assert_snapshot!(String::from_utf8(buf.into_inner()).unwrap());
}

#[test]
fn info_book_with_translation_snapshot() {
    let (s, _d) = fresh_store();
    let mut buf = Cursor::new(Vec::<u8>::new());
    let code = cli::info::run(&s, "JHN", Some("KJV"), &mut buf).unwrap();
    assert_eq!(code, 0);
    insta::assert_snapshot!(String::from_utf8(buf.into_inner()).unwrap());
}

#[test]
fn info_unknown_book_returns_parse_error() {
    let (s, _d) = fresh_store();
    let mut buf = Cursor::new(Vec::<u8>::new());
    let code = cli::info::run(&s, "ZZZ", None, &mut buf).unwrap();
    assert_eq!(code, 2);
    let out = String::from_utf8(buf.into_inner()).unwrap();
    assert!(out.contains("E_REF_PARSE"), "stdout: {out}");
}

#[test]
fn info_unknown_translation_returns_missing_error() {
    let (s, _d) = fresh_store();
    let mut buf = Cursor::new(Vec::<u8>::new());
    let code = cli::info::run(&s, "JHN", Some("XYZ"), &mut buf).unwrap();
    assert_eq!(code, 4);
    let out = String::from_utf8(buf.into_inner()).unwrap();
    assert!(out.contains("E_TRANSLATION_MISSING"), "stdout: {out}");
}

#[test]
fn manifest_snapshot() {
    let (s, _d) = fresh_store();
    let mut buf = Cursor::new(Vec::<u8>::new());
    cli::manifest::run(&s, &mut buf).unwrap();
    let out = String::from_utf8(buf.into_inner()).unwrap();
    let normalized = out.replace(
        "\"installed_at\":\"2026-05-09T00:00:00Z\"",
        "\"installed_at\":\"<TS>\"",
    );
    let normalized = normalize_data_dir(&normalized);
    let normalized = normalized.replace(
        &format!("\"version\":\"{}\"", env!("CARGO_PKG_VERSION")),
        "\"version\":\"<VER>\"",
    );
    insta::assert_snapshot!(normalized);
}

fn normalize_data_dir(s: &str) -> String {
    let re = regex::Regex::new(r#""data_dir":"[^"]*""#).unwrap();
    re.replace_all(s, "\"data_dir\":\"<PATH>\"").into_owned()
}
