/*
 * Keyman is copyright (C) SIL Global. MIT License.
 *
 * Read keyboard display names and BCP-47 language tags from `kmp.json`
 * package descriptors.
 *
 * Each package lives at
 * `~/Library/Application Support/keyman.inputmethod.Keyman/Keyman-Keyboards/<package>/`.
 * The descriptor `kmp.json` contains a `keyboards` array; each entry
 * carries an `id` (matches the .kmx stem), a `name` (display name
 * shown in Keyman's menu), and a `languages` array of `{id, name}`
 * pairs. If the file is missing or unparseable we return defaults;
 * the caller falls back to the keyboard stem and an empty language
 * list. This is a best-effort path, not a correctness path.
 */

use std::fs;
use std::path::Path;

use serde::Deserialize;

use crate::keyboard::{KeyboardId, Language};

#[derive(Debug, Default)]
pub struct PackageInfo {
    pub display_name: Option<String>,
    pub languages: Vec<Language>,
}

#[derive(Debug, Deserialize)]
struct KmpJson {
    #[serde(default)]
    keyboards: Vec<KmpKeyboard>,
    #[serde(default)]
    info: Option<KmpInfo>,
}

#[derive(Debug, Deserialize)]
struct KmpKeyboard {
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    languages: Vec<KmpLanguage>,
}

#[derive(Debug, Deserialize)]
struct KmpLanguage {
    #[serde(default)]
    id: Option<String>,
    #[serde(default)]
    name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct KmpInfo {
    #[serde(default)]
    name: Option<KmpInfoEntry>,
}

#[derive(Debug, Deserialize)]
struct KmpInfoEntry {
    #[serde(default)]
    description: Option<String>,
}

pub fn info_for(id: &KeyboardId, root: &Path) -> PackageInfo {
    let Some(pkg) = id.package() else {
        return PackageInfo::default();
    };
    let Some(stem) = id.stem() else {
        return PackageInfo::default();
    };
    let kmp_path = root.join(pkg).join("kmp.json");
    let Ok(bytes) = fs::read(&kmp_path) else {
        return PackageInfo::default();
    };
    let Ok(parsed) = serde_json::from_slice::<KmpJson>(&bytes) else {
        return PackageInfo::default();
    };

    let entry = parsed.keyboards.iter().find(|kb| {
        kb.id
            .as_deref()
            .is_some_and(|kb_id| kb_id.eq_ignore_ascii_case(stem))
    });

    let display_name = entry
        .and_then(|kb| kb.name.clone())
        .filter(|n| !n.is_empty())
        .or_else(|| {
            parsed
                .info
                .and_then(|i| i.name)
                .and_then(|n| n.description)
                .filter(|s| !s.is_empty())
        });

    let languages = entry
        .map(|kb| {
            kb.languages
                .iter()
                .filter_map(|l| {
                    let lang_id = l.id.clone()?;
                    let lang_name = l.name.clone().unwrap_or_else(|| lang_id.clone());
                    Some(Language {
                        name: lang_name,
                        id: lang_id,
                    })
                })
                .collect()
        })
        .unwrap_or_default();

    PackageInfo {
        display_name,
        languages,
    }
}
