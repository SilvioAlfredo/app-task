// App Task — task-list manager (Rust + Slint).
// Copyright (C) 2026 Alfredo Riveros.
// SPDX-License-Identifier: GPL-3.0-only
// See LICENSE for details.
//! Persistencia: rutas, carga/guardado del store multi-lista,
//! migración del formato legacy y exportación a JSON.

use crate::model::{StoreData, TaskItem, TaskListData, default_store};

/// Directorio de datos XDG (`~/.local/share/app-task/`).
pub fn data_dir() -> std::path::PathBuf {
    let base = std::env::var("XDG_DATA_HOME")
        .map(std::path::PathBuf::from)
        .unwrap_or_else(|_| {
            std::env::var("HOME")
                .map(|h| std::path::PathBuf::from(h).join(".local/share"))
                .unwrap_or_else(|_| std::path::PathBuf::from("."))
        });
    let dir = base.join("app-task");
    let _ = std::fs::create_dir_all(&dir);
    dir
}

/// Si se ejecuta desde el proyecto (hay `Cargo.toml`), guarda al lado del
/// código para no mezclar desarrollo con la app instalada.
fn project_dev_dir() -> Option<std::path::PathBuf> {
    std::env::current_dir().ok().filter(|cwd| cwd.join("Cargo.toml").exists())
}

fn store_file_path() -> std::path::PathBuf {
    project_dev_dir()
        .map(|d| d.join("lists.json"))
        .unwrap_or_else(|| data_dir().join("lists.json"))
}

/// Ruta legacy (formato anterior: array directo de tareas). Solo para migrar.
fn legacy_tasks_path() -> std::path::PathBuf {
    project_dev_dir()
        .map(|d| d.join("tasks.json"))
        .unwrap_or_else(|| data_dir().join("tasks.json"))
}

pub fn load_store() -> StoreData {
    // 1) Formato actual multi-lista
    if let Ok(data) = std::fs::read_to_string(store_file_path()) {
        if let Ok(mut store) = serde_json::from_str::<StoreData>(&data) {
            store.normalize();
            return store;
        }
    }
    // 2) Migración desde formato legacy
    if let Ok(data) = std::fs::read_to_string(legacy_tasks_path()) {
        if let Ok(items) = serde_json::from_str::<Vec<TaskItem>>(&data) {
            let store = StoreData {
                active: 0,
                lists: vec![TaskListData { name: "Actualización apps".into(), tasks: items }],
            };
            save_store_data(&store);
            return store;
        }
    }
    default_store()
}

pub fn save_store_data(store: &StoreData) {
    if let Ok(json) = serde_json::to_string_pretty(store) {
        let _ = std::fs::write(store_file_path(), json);
    }
}

/// Nombre seguro para el archivo exportado.
pub fn slugify(name: &str) -> String {
    let s: String = name
        .to_lowercase()
        .chars()
        .filter_map(|c| {
            if c.is_alphanumeric() {
                Some(c)
            } else if c == ' ' || c == '-' || c == '_' {
                Some('-')
            } else {
                None
            }
        })
        .collect();
    let s = s.trim_matches('-').to_string();
    if s.is_empty() { "lista".into() } else { s }
}

/// Carpeta destino de la exportación: `~/Documentos` si existe, si no `$HOME`.
pub fn export_dir() -> std::path::PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        let docs = std::path::PathBuf::from(&home).join("Documentos");
        if docs.is_dir() {
            return docs;
        }
        return std::path::PathBuf::from(home);
    }
    data_dir()
}

/// Exporta una lista a `~/Documentos/app-task-<slug>.json`. Devuelve la ruta.
pub fn export_list(name: &str, tasks: &[TaskItem]) -> Option<String> {
    let path = export_dir().join(format!("app-task-{}.json", slugify(name)));
    let payload = serde_json::json!({ "name": name, "tasks": tasks });
    let json = serde_json::to_string_pretty(&payload).ok()?;
    std::fs::write(&path, json).ok()?;
    Some(path.to_string_lossy().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugify_cases() {
        assert_eq!(slugify("Actualización apps"), "actualización-apps");
        assert_eq!(slugify("  Hola Mundo!  "), "hola-mundo");
        assert_eq!(slugify("!!!"), "lista");
        assert_eq!(slugify("Mis tareas 2026"), "mis-tareas-2026");
    }

    #[test]
    fn export_payload_shape() {
        let tasks = vec![TaskItem {
            title: "T".into(),
            details: "".into(),
            completed: false,
            starred: true,
        }];
        let payload = serde_json::json!({ "name": "L", "tasks": tasks });
        assert_eq!(payload["name"], "L");
        assert_eq!(payload["tasks"][0]["starred"], true);
    }
}
