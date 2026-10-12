// App Task — task-list manager (Rust + Slint).
// Copyright (C) 2026 Alfredo Riveros.
// SPDX-License-Identifier: GPL-3.0-only
// See LICENSE for details.
//! Cableado UI ↔ estado: registra los callbacks Slint en la App.
//! Tres grupos: tareas de la lista activa, gestión de listas y UI global
//! (tema, idioma, enlaces). Los helpers sincronizan modelo ↔ store ↔ UI.

use std::cell::RefCell;
use std::rc::Rc;

use slint::{ComponentHandle, Model, ModelRc, SharedString, VecModel};

use crate::config::{apply_language, effective_lang, update_config};
use crate::model::{StoreData, TaskItem, TaskListData, load_model, read_model};
use crate::store;
use crate::{App, TaskData};

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn update_pending(app: &App, model: &Rc<VecModel<TaskData>>) {
    let pending = (0..model.row_count())
        .filter_map(|i| model.row_data(i))
        .filter(|d| !d.completed)
        .count() as i32;
    app.set_pending_count(pending);
}

/// Refresca nombres de listas, activa, título y pendientes en la UI.
fn refresh_lists_ui(app: &App, model: &Rc<VecModel<TaskData>>, store: &StoreData) {
    let names: Vec<SharedString> =
        store.lists.iter().map(|l| l.name.clone().into()).collect();
    app.set_lists_names(ModelRc::from(Rc::new(VecModel::from(names))));
    app.set_active_list(store.active as i32);
    if let Some(list) = store.active_list() {
        app.set_list_name(list.name.clone().into());
    }
    app.set_selected_index(-1);
    app.set_export_path("".into());
    update_pending(app, model);
}

/// Vuelca el modelo a la lista activa y persiste todo el store.
fn sync_and_save(model: &VecModel<TaskData>, store: &Rc<RefCell<StoreData>>) {
    let items = read_model(model);
    let mut s = store.borrow_mut();
    if let Some(list) = s.active_list_mut() {
        list.tasks = items;
    }
    store::save_store_data(&s);
}

fn show_task_details(app: &App, d: &TaskData, idx: i32) {
    app.set_selected_index(idx);
    app.set_detail_title(d.title.clone());
    app.set_detail_text(d.details.clone());
    app.set_detail_completed(d.completed);
    app.set_detail_starred(d.starred);
}

// ---------------------------------------------------------------------------
// Tareas de la lista activa
// ---------------------------------------------------------------------------

/// Estado inicial de la UI de listas (nombres, título, pendientes, selección).
/// Llamar una vez desde `main` después de crear App y modelo.
pub fn init_ui(app: &App, model: &Rc<VecModel<TaskData>>, store: &StoreData) {
    refresh_lists_ui(app, model, store);
}

pub fn register_tasks(
    app: &App,
    model: &Rc<VecModel<TaskData>>,
    store: &Rc<RefCell<StoreData>>,
) {
    // Añadir
    {
        let app_weak = app.as_weak();
        let (model, store) = (model.clone(), store.clone());
        app.on_add_task(move || {
            let Some(app) = app_weak.upgrade() else { return };
            let title = app.get_new_task_title().trim().to_string();
            if title.is_empty() {
                return;
            }
            model.push(TaskData {
                title: title.into(),
                details: "".into(),
                completed: false,
                starred: false,
            });
            app.set_new_task_title("".into());
            update_pending(&app, &model);
            sync_and_save(&model, &store);
        });
    }

    // Marcar/desmarcar (además muestra la descripción)
    {
        let app_weak = app.as_weak();
        let (model, store) = (model.clone(), store.clone());
        app.on_toggle_completed(move |idx| {
            let Some(app) = app_weak.upgrade() else { return };
            if let Some(mut d) = model.row_data(idx as usize) {
                d.completed = !d.completed;
                model.set_row_data(idx as usize, d.clone());
                show_task_details(&app, &d, idx);
                update_pending(&app, &model);
                sync_and_save(&model, &store);
            }
        });
    }

    // Destacar
    {
        let app_weak = app.as_weak();
        let (model, store) = (model.clone(), store.clone());
        app.on_toggle_star(move |idx| {
            let Some(app) = app_weak.upgrade() else { return };
            if let Some(mut d) = model.row_data(idx as usize) {
                d.starred = !d.starred;
                model.set_row_data(idx as usize, d.clone());
                if app.get_selected_index() == idx {
                    app.set_detail_starred(d.starred);
                }
                sync_and_save(&model, &store);
            }
        });
    }

    // Borrar tarea
    {
        let app_weak = app.as_weak();
        let (model, store) = (model.clone(), store.clone());
        app.on_delete_task(move |idx| {
            let Some(app) = app_weak.upgrade() else { return };
            if idx < 0 || idx as usize >= model.row_count() {
                return;
            }
            model.remove(idx as usize);
            let sel = app.get_selected_index();
            if sel == idx {
                app.set_selected_index(-1);
            } else if sel > idx {
                app.set_selected_index(sel - 1);
            }
            update_pending(&app, &model);
            sync_and_save(&model, &store);
        });
    }

    // Seleccionar (cargar detalles)
    {
        let app_weak = app.as_weak();
        let model = model.clone();
        app.on_select_task(move |idx| {
            let Some(app) = app_weak.upgrade() else { return };
            if let Some(d) = model.row_data(idx as usize) {
                show_task_details(&app, &d, idx);
            }
        });
    }

    // Guardar detalles
    {
        let app_weak = app.as_weak();
        let (model, store) = (model.clone(), store.clone());
        app.on_save_details(move || {
            let Some(app) = app_weak.upgrade() else { return };
            let idx = app.get_selected_index();
            if idx < 0 || idx as usize >= model.row_count() {
                return;
            }
            model.set_row_data(idx as usize, TaskData {
                title: app.get_detail_title(),
                details: app.get_detail_text(),
                completed: app.get_detail_completed(),
                starred: app.get_detail_starred(),
            });
            update_pending(&app, &model);
            sync_and_save(&model, &store);
        });
    }

    // Limpiar selección + estrella del panel (se guarda con Guardar)
    {
        let app_weak = app.as_weak();
        app.on_clear_selection(move || {
            if let Some(app) = app_weak.upgrade() {
                app.set_selected_index(-1);
            }
        });
    }
    {
        let app_weak = app.as_weak();
        app.on_toggle_detail_star(move || {
            if let Some(app) = app_weak.upgrade() {
                app.set_detail_starred(!app.get_detail_starred());
            }
        });
    }
}

// ---------------------------------------------------------------------------
// Gestión de listas
// ---------------------------------------------------------------------------

pub fn register_lists(
    app: &App,
    model: &Rc<VecModel<TaskData>>,
    store: &Rc<RefCell<StoreData>>,
) {
    // Cambiar de lista (guarda la actual antes)
    {
        let app_weak = app.as_weak();
        let (model, store) = (model.clone(), store.clone());
        app.on_switch_list(move |idx| {
            let Some(app) = app_weak.upgrade() else { return };
            if idx < 0 {
                return;
            }
            sync_and_save(&model, &store);
            let tasks: Vec<TaskItem> = {
                let mut s = store.borrow_mut();
                let i = idx as usize;
                if i >= s.lists.len() {
                    return;
                }
                s.active = i;
                store::save_store_data(&s);
                s.lists[i].tasks.clone()
            };
            load_model(&model, &tasks);
            refresh_lists_ui(&app, &model, &store.borrow());
        });
    }

    // Crear lista (empieza vacía y pasa a activa)
    {
        let app_weak = app.as_weak();
        let (model, store) = (model.clone(), store.clone());
        app.on_create_list(move |name| {
            let Some(app) = app_weak.upgrade() else { return };
            let name = name.trim().to_string();
            sync_and_save(&model, &store);
            {
                let mut s = store.borrow_mut();
                s.lists.push(TaskListData {
                    name: if name.is_empty() { "Nueva lista".into() } else { name },
                    tasks: Vec::new(),
                });
                s.active = s.lists.len() - 1;
                store::save_store_data(&s);
            }
            load_model(&model, &[]);
            refresh_lists_ui(&app, &model, &store.borrow());
        });
    }

    // Renombrar lista activa
    {
        let app_weak = app.as_weak();
        let (model, store) = (model.clone(), store.clone());
        app.on_rename_list(move |name| {
            let Some(app) = app_weak.upgrade() else { return };
            let name = name.trim().to_string();
            if name.is_empty() {
                return;
            }
            {
                let mut s = store.borrow_mut();
                if let Some(list) = s.active_list_mut() {
                    list.name = name;
                }
                store::save_store_data(&s);
            }
            refresh_lists_ui(&app, &model, &store.borrow());
        });
    }

    // Eliminar lista (nunca la única)
    {
        let app_weak = app.as_weak();
        let (model, store) = (model.clone(), store.clone());
        app.on_delete_list(move |idx| {
            let Some(app) = app_weak.upgrade() else { return };
            if idx < 0 {
                return;
            }
            let tasks: Vec<TaskItem> = {
                let mut s = store.borrow_mut();
                if s.lists.len() <= 1 {
                    return;
                }
                let i = (idx as usize).min(s.lists.len() - 1);
                s.lists.remove(i);
                if s.active == i {
                    s.active = i.min(s.lists.len() - 1);
                } else if s.active > i {
                    s.active -= 1;
                }
                store::save_store_data(&s);
                s.active_list().map(|l| l.tasks.clone()).unwrap_or_default()
            };
            load_model(&model, &tasks);
            refresh_lists_ui(&app, &model, &store.borrow());
        });
    }

    // Exportar lista activa a JSON
    {
        let app_weak = app.as_weak();
        let (model, store) = (model.clone(), store.clone());
        app.on_export_list(move || {
            let Some(app) = app_weak.upgrade() else { return };
            sync_and_save(&model, &store);
            let s = store.borrow();
            let Some(list) = s.active_list() else { return };
            if let Some(path) = store::export_list(&list.name, &list.tasks) {
                app.set_export_path(path.into());
            }
        });
    }
}

// ---------------------------------------------------------------------------
// UI global: tema, idioma, enlaces
// ---------------------------------------------------------------------------

pub fn register_ui(app: &App) {
    {
        let app_weak = app.as_weak();
        app.on_toggle_theme(move || {
            if let Some(app) = app_weak.upgrade() {
                let next = !app.get_dark_mode();
                app.set_dark_mode(next);
                update_config(|c| c.dark_mode = next);
            }
        });
    }
    {
        let app_weak = app.as_weak();
        app.on_set_language(move |lang| {
            if let Some(app) = app_weak.upgrade() {
                let lang = lang.to_string();
                apply_language(&lang);
                app.set_current_lang(effective_lang(&lang).into());
                update_config(|c| c.lang = lang.clone());
            }
        });
    }
    app.on_open_url(|url| {
        let _ = std::process::Command::new("xdg-open")
            .arg(url.to_string())
            .spawn();
    });
}
