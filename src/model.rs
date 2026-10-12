// App Task — task-list manager (Rust + Slint).
// Copyright (C) 2026 Alfredo Riveros.
// SPDX-License-Identifier: GPL-3.0-only
// See LICENSE for details.
//! Modelo de datos: tareas, listas y helpers del modelo Slint.
//! Sin E/S ni UI: todo aquí es testeable sin display.

use serde::{Deserialize, Serialize};
use slint::{Model, VecModel};

use crate::TaskData;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskItem {
    pub title: String,
    #[serde(default)]
    pub details: String,
    #[serde(default)]
    pub completed: bool,
    #[serde(default)]
    pub starred: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskListData {
    pub name: String,
    #[serde(default)]
    pub tasks: Vec<TaskItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StoreData {
    #[serde(default)]
    pub active: usize,
    #[serde(default)]
    pub lists: Vec<TaskListData>,
}

impl StoreData {
    pub fn active_list(&self) -> Option<&TaskListData> {
        self.lists.get(self.active)
    }

    pub fn active_list_mut(&mut self) -> Option<&mut TaskListData> {
        let a = self.active;
        self.lists.get_mut(a)
    }

    /// Deja el store en un estado válido (≥1 lista, active en rango).
    pub fn normalize(&mut self) {
        if self.lists.is_empty() {
            self.lists.push(TaskListData { name: "Actualización apps".into(), tasks: Vec::new() });
        }
        if self.active >= self.lists.len() {
            self.active = 0;
        }
    }
}

pub fn default_tasks() -> Vec<TaskItem> {
    vec![
        TaskItem { title: "Completar portafolios".into(), details: String::new(), completed: false, starred: false },
        TaskItem { title: "completar biblioteca horizonte".into(), details: String::new(), completed: false, starred: false },
        TaskItem { title: "Completar metronomo".into(), details: String::new(), completed: false, starred: false },
        TaskItem { title: "completar gestion inventario".into(), details: String::new(), completed: false, starred: false },
        TaskItem { title: "completar peluqueria canina".into(), details: String::new(), completed: false, starred: false },
        TaskItem { title: "revisar navegador".into(), details: String::new(), completed: false, starred: false },
        TaskItem { title: "Completar gestion estacionamiento".into(), details: String::new(), completed: false, starred: false },
        TaskItem { title: "completar gestion empleados".into(), details: String::new(), completed: false, starred: false },
        TaskItem { title: "revisar proyecto braille".into(), details: String::new(), completed: false, starred: false },
        TaskItem { title: "Revisar juego del ahorcado".into(), details: String::new(), completed: false, starred: false },
        TaskItem { title: "versión de viajes codo a codo".into(), details: String::new(), completed: false, starred: false },
        TaskItem { title: "Completar challenge de hotel".into(), details: String::new(), completed: false, starred: false },
        TaskItem { title: "Crear el ecommerce de alura".into(), details: String::new(), completed: false, starred: false },
        TaskItem { title: "Modernizar xilofon y crear nuevos instrumentos".into(), details: String::new(), completed: false, starred: false },
    ]
}

pub fn default_store() -> StoreData {
    StoreData {
        active: 0,
        lists: vec![TaskListData { name: "Actualización apps".into(), tasks: default_tasks() }],
    }
}

pub fn to_slint(t: &TaskItem) -> TaskData {
    TaskData {
        title: t.title.clone().into(),
        details: t.details.clone().into(),
        completed: t.completed,
        starred: t.starred,
    }
}

pub fn from_slint(d: &TaskData) -> TaskItem {
    TaskItem {
        title: d.title.to_string(),
        details: d.details.to_string(),
        completed: d.completed,
        starred: d.starred,
    }
}

/// Lee el modelo Slint hacia Vec<TaskItem>.
pub fn read_model(model: &VecModel<TaskData>) -> Vec<TaskItem> {
    (0..model.row_count())
        .filter_map(|i| model.row_data(i))
        .map(|d| from_slint(&d))
        .collect()
}

/// Carga tareas en el modelo en su lugar (conserva el Rc que usa la UI).
pub fn load_model(model: &VecModel<TaskData>, tasks: &[TaskItem]) {
    while model.row_count() > 0 {
        model.remove(0);
    }
    for t in tasks {
        model.push(to_slint(t));
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::rc::Rc;

    #[test]
    fn store_round_trip() {
        let store = StoreData {
            active: 1,
            lists: vec![
                TaskListData { name: "A".into(), tasks: default_tasks() },
                TaskListData { name: "B".into(), tasks: vec![] },
            ],
        };
        let json = serde_json::to_string_pretty(&store).unwrap();
        let back: StoreData = serde_json::from_str(&json).unwrap();
        assert_eq!(back.active, 1);
        assert_eq!(back.lists.len(), 2);
        assert_eq!(back.lists[0].tasks.len(), 14);
        assert!(back.lists[1].tasks.is_empty());
    }

    #[test]
    fn legacy_tasks_parse_as_store() {
        let legacy = r#"[{"title":"T1","details":"d","completed":true,"starred":false}]"#;
        let items: Vec<TaskItem> = serde_json::from_str(legacy).unwrap();
        assert_eq!(items.len(), 1);
        let store = StoreData {
            active: 0,
            lists: vec![TaskListData { name: "Actualización apps".into(), tasks: items }],
        };
        assert_eq!(store.lists[0].tasks[0].title, "T1");
        assert!(store.lists[0].tasks[0].completed);
    }

    #[test]
    fn model_drain_and_fill_keeps_rc() {
        let model = Rc::new(VecModel::from(
            default_tasks().iter().map(to_slint).collect::<Vec<_>>(),
        ));
        assert_eq!(model.row_count(), 14);
        load_model(&model, &[]);
        assert_eq!(model.row_count(), 0);
        load_model(&model, &default_tasks()[..2]);
        assert_eq!(model.row_count(), 2);
        let items = read_model(&model);
        assert_eq!(items[0].title, "Completar portafolios");
    }

    #[test]
    fn normalize_repairs_store() {
        let mut s = StoreData { active: 5, lists: vec![] };
        s.normalize();
        assert_eq!(s.lists.len(), 1);
        assert_eq!(s.active, 0);
    }
}
