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
