// App Task — task-list manager (Rust + Slint).
// Copyright (C) 2026 Alfredo Riveros.
// SPDX-License-Identifier: GPL-3.0-only
// See LICENSE for details.
//! Configuración persistida (tema, idioma) y helpers de idioma.

use serde::{Deserialize, Serialize};

use crate::store::data_dir;

fn config_file_path() -> std::path::PathBuf {
    data_dir().join("config.json")
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct AppConfig {
    #[serde(default = "default_dark")]
    pub dark_mode: bool,
    /// "auto" (según locale del sistema), "es" o "en"
    #[serde(default = "default_lang")]
    pub lang: String,
}

fn default_dark() -> bool {
    true
}

fn default_lang() -> String {
    "auto".into()
}

pub fn load_config() -> AppConfig {
    std::fs::read_to_string(config_file_path())
        .ok()
        .and_then(|d| serde_json::from_str(&d).ok())
        .unwrap_or(AppConfig { dark_mode: true, lang: default_lang() })
}

fn save_config(cfg: &AppConfig) {
    if let Ok(json) = serde_json::to_string_pretty(cfg) {
        let _ = std::fs::write(config_file_path(), json);
    }
}

pub fn update_config(f: impl FnOnce(&mut AppConfig)) {
    let mut cfg = load_config();
    f(&mut cfg);
    save_config(&cfg);
}

/// Aplica un idioma empaquetado: "en" → inglés, cualquier otro → español (fuente).
pub fn apply_language(lang: &str) {
    if lang == "en" {
        let _ = slint::select_bundled_translation("en");
    } else {
        let _ = slint::select_bundled_translation("");
    }
}

/// Idioma efectivo para resaltar el botón ES/EN ("es" o "en").
pub fn effective_lang(pref: &str) -> String {
    if pref == "en" || pref == "es" {
        return pref.into();
    }
    // "auto": deducir del locale como hace Slint (sys-locale lee LANG/LANGUAGE/...)
    for var in ["LANGUAGE", "LC_ALL", "LC_MESSAGES", "LANG"] {
        if let Ok(v) = std::env::var(var) {
            let base = v.split(['_', '-', '@', '.']).next().unwrap_or("").to_lowercase();
            if base == "en" {
                return "en".into();
            }
            if !base.is_empty() {
                return "es".into();
            }
        }
    }
    "es".into()
}
