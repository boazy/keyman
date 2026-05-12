/*
 * Keyman is copyright (C) SIL Global. MIT License.
 *
 * Wire-format types for `--json` output. The shape of these structs is
 * the documented JSON schema; field names and types here are stable.
 * Adding fields is allowed; renaming or removing is a breaking change.
 */

use serde::Serialize;

use crate::keyboard::{
    ActivateOutcome, ImState, Keyboard, KeyboardId, Language, SelectOutcome, Status,
};

#[derive(Debug, Serialize)]
pub struct LanguageJson {
    pub name: String,
    pub id: String,
}

impl From<&Language> for LanguageJson {
    fn from(l: &Language) -> Self {
        Self {
            name: l.name.clone(),
            id: l.id.clone(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct KeyboardJson {
    pub id: String,
    pub name: String,
    pub package: String,
    pub selected: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub language: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub languages: Option<Vec<LanguageJson>>,
}

impl KeyboardJson {
    pub fn from_keyboard(k: &Keyboard, include_all_languages: bool) -> Self {
        let language = if k.languages.len() == 1 {
            Some(k.languages[0].name.clone())
        } else {
            None
        };
        let languages = if include_all_languages && !k.languages.is_empty() {
            Some(k.languages.iter().map(LanguageJson::from).collect())
        } else {
            None
        };
        Self {
            id: k.id.as_str().to_string(),
            name: k.name.clone(),
            package: k.package.clone(),
            selected: k.selected,
            language,
            languages,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct ListJson {
    pub keyboards: Vec<KeyboardJson>,
}

#[derive(Debug, Serialize)]
pub struct StatusJson {
    pub im_registered: bool,
    pub im_selected: bool,
    pub im_process_running: bool,
    pub selected_keyboard: Option<KeyboardJson>,
}

impl StatusJson {
    pub fn from_status(s: &Status, include_all_languages: bool) -> Self {
        Self {
            im_registered: s.im_state.im_registered,
            im_selected: s.im_state.im_selected,
            im_process_running: s.im_state.im_process_running,
            selected_keyboard: s
                .selected_keyboard
                .as_ref()
                .map(|k| KeyboardJson::from_keyboard(k, include_all_languages)),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct SelectJson {
    pub selected: KeyboardJson,
    pub im_activated: bool,
    pub previous_selection: Option<String>,
}

impl SelectJson {
    pub fn from_outcome(o: &SelectOutcome, include_all_languages: bool) -> Self {
        Self {
            selected: KeyboardJson::from_keyboard(&o.selected, include_all_languages),
            im_activated: o.im_activated,
            previous_selection: o.previous_selection.as_ref().map(KeyboardId::to_string),
        }
    }
}

#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Serialize)]
pub struct ActivateJson {
    pub im_registered_before: bool,
    pub im_selected_before: bool,
    pub im_registered_after: bool,
    pub im_selected_after: bool,
    pub changed: bool,
}

impl From<&ActivateOutcome> for ActivateJson {
    fn from(o: &ActivateOutcome) -> Self {
        let changed = (o.im_registered_before != o.im_registered_after)
            || (o.im_selected_before != o.im_selected_after);
        Self {
            im_registered_before: o.im_registered_before,
            im_selected_before: o.im_selected_before,
            im_registered_after: o.im_registered_after,
            im_selected_after: o.im_selected_after,
            changed,
        }
    }
}

pub fn format_im_state(s: ImState) -> String {
    format!(
        "input-method: registered={} selected={} process-running={}",
        yn(s.im_registered),
        yn(s.im_selected),
        yn(s.im_process_running)
    )
}

fn yn(b: bool) -> &'static str {
    if b {
        "yes"
    } else {
        "no"
    }
}
