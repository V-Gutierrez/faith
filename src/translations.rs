//! Translation alias catalog.
//!
//! Maps the public uppercase short alias (`KJV`, `ONBV`) to the upstream
//! HelloAO ID and license metadata. Adding a translation here is the only
//! change needed to make `faith install <ALIAS>` work.

#[derive(Debug, Clone, Copy)]
pub struct TranslationDef {
    pub alias: &'static str,
    pub helloao_id: &'static str,
    pub name: &'static str,
    pub english_name: &'static str,
    pub language: &'static str,
    pub direction: &'static str,
    pub license: &'static str,
    pub source_url: &'static str,
}

pub const CATALOG: &[TranslationDef] = &[
    TranslationDef {
        alias: "KJV",
        helloao_id: "eng_kjv",
        name: "King James (Authorized) Version",
        english_name: "King James Version",
        language: "eng",
        direction: "ltr",
        license: "Public Domain",
        source_url: "https://bible.helloao.org/api/eng_kjv/complete.json",
    },
    TranslationDef {
        alias: "ONBV",
        helloao_id: "por_onbv",
        name: "Biblica® Open Nova Bíblia Viva 2007",
        english_name: "Portuguese Open Nova Bíblia Viva",
        language: "por",
        direction: "ltr",
        license: "CC BY-SA 4.0",
        source_url: "https://bible.helloao.org/api/por_onbv/complete.json",
    },
    TranslationDef {
        alias: "BLJ",
        helloao_id: "por_blj",
        name: "Bíblia Livre",
        english_name: "Portuguese Bíblia Livre",
        language: "por",
        direction: "ltr",
        license: "CC BY-SA 4.0",
        source_url: "https://bible.helloao.org/api/por_blj/complete.json",
    },
    TranslationDef {
        alias: "BSL",
        helloao_id: "por_bsl",
        name: "Bíblia Portuguesa Mundial",
        english_name: "World Portuguese Bible",
        language: "por",
        direction: "ltr",
        license: "CC BY-SA 4.0",
        source_url: "https://bible.helloao.org/api/por_bsl/complete.json",
    },
    TranslationDef {
        alias: "BLT",
        helloao_id: "por_blt",
        name: "Biblia Livre Para Todos",
        english_name: "Portuguese Free Bible for All",
        language: "por",
        direction: "ltr",
        license: "CC BY-SA 4.0",
        source_url: "https://bible.helloao.org/api/por_blt/complete.json",
    },
    TranslationDef {
        alias: "TFT",
        helloao_id: "por_tft",
        name: "A Bíblia Sagrada, Tradução para Tradutores",
        english_name: "Portuguese Translation for Translators",
        language: "por",
        direction: "ltr",
        license: "CC BY-SA 4.0",
        source_url: "https://bible.helloao.org/api/por_tft/complete.json",
    },
    TranslationDef {
        alias: "RVR09",
        helloao_id: "spa_r09",
        name: "Santa Biblia — Reina Valera 1909",
        english_name: "Spanish Reina Valera 1909",
        language: "spa",
        direction: "ltr",
        license: "Public Domain",
        source_url: "https://bible.helloao.org/api/spa_r09/complete.json",
    },
    TranslationDef {
        alias: "LSG",
        helloao_id: "fra_lsg",
        name: "Louis Segond 1910",
        english_name: "French Louis Segond 1910",
        language: "fra",
        direction: "ltr",
        license: "Public Domain",
        source_url: "https://bible.helloao.org/api/fra_lsg/complete.json",
    },
    TranslationDef {
        alias: "LUT",
        helloao_id: "deu_l12",
        name: "Lutherbibel 1912",
        english_name: "German Luther Bible 1912",
        language: "deu",
        direction: "ltr",
        license: "Public Domain",
        source_url: "https://bible.helloao.org/api/deu_l12/complete.json",
    },
    TranslationDef {
        alias: "SBLGNT",
        helloao_id: "grc_sbl",
        name: "Η Καινή Διαθήκη",
        english_name: "Greek SBL New Testament",
        language: "grc",
        direction: "ltr",
        license: "CC BY-SA 4.0",
        source_url: "https://bible.helloao.org/api/grc_sbl/complete.json",
    },
    TranslationDef {
        alias: "WLC",
        helloao_id: "heb_wlc",
        name: "Westminster Leningrad Codex",
        english_name: "Hebrew Westminster Leningrad Codex",
        language: "heb",
        direction: "rtl",
        license: "Public Domain",
        source_url: "https://bible.helloao.org/api/heb_wlc/complete.json",
    },
];

pub fn by_alias(alias: &str) -> Option<&'static TranslationDef> {
    let upper = alias.to_ascii_uppercase();
    CATALOG.iter().find(|t| t.alias == upper)
}

pub fn lang_code_to_iso2(iso3: &str) -> &'static str {
    match iso3 {
        "eng" => "en",
        "por" => "pt",
        "spa" => "es",
        "fra" | "fre" => "fr",
        "deu" | "ger" => "de",
        "grc" => "grc",
        "heb" => "he",
        _ => "und",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn catalog_has_kjv_and_onbv() {
        assert!(by_alias("KJV").is_some());
        assert!(by_alias("kjv").is_some());
        assert!(by_alias("ONBV").is_some());
        assert!(by_alias("BLJ").is_some());
        assert!(by_alias("BSL").is_some());
        assert!(by_alias("BLT").is_some());
        assert!(by_alias("TFT").is_some());
        assert!(by_alias("RVR09").is_some());
        assert!(by_alias("LSG").is_some());
        assert!(by_alias("LUT").is_some());
        assert!(by_alias("SBLGNT").is_some());
        assert!(by_alias("WLC").is_some());
        assert!(by_alias("XYZ").is_none());
    }

    #[test]
    fn aliases_are_short_uppercase_ascii() {
        for t in CATALOG {
            assert_eq!(t.alias, t.alias.to_ascii_uppercase());
            assert!(t.alias.len() <= 6);
            assert!(t.alias.chars().all(|c| c.is_ascii_alphanumeric()));
        }
    }
}
