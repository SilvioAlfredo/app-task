// App Task — task-list manager (Rust + Slint).
// Copyright (C) 2026 Alfredo Riveros.
// SPDX-License-Identifier: GPL-3.0-only
// See LICENSE for details.
//! App Task — punto de entrada.
//! La lógica vive en módulos: `model` (datos), `store` (persistencia),
//! `config` (ajustes), `about` (info del proyecto) y `handlers` (callbacks).

use slint::{ModelRc, VecModel};
use std::cell::RefCell;
use std::rc::Rc;

mod about;
mod config;
mod handlers;
mod model;
mod store;

slint::include_modules!();

use config::{apply_language, effective_lang, load_config};
use model::to_slint;

fn main() -> Result<(), slint::PlatformError> {
    let app = App::new()?;

    // Store multi-lista + modelo de la lista activa
    let store: Rc<RefCell<model::StoreData>> = Rc::new(RefCell::new(store::load_store()));
    let active_tasks = store.borrow().active_list().map(|l| l.tasks.clone()).unwrap_or_default();
    let model: Rc<VecModel<TaskData>> =
        Rc::new(VecModel::from(active_tasks.iter().map(to_slint).collect::<Vec<_>>()));
    app.set_tasks(ModelRc::from(model.clone()));

    // Tema, idioma y datos fijos
    let cfg = load_config();
    app.set_dark_mode(cfg.dark_mode);
    if cfg.lang == "en" || cfg.lang == "es" {
        apply_language(&cfg.lang);
    }
    app.set_current_lang(effective_lang(&cfg.lang).into());
    app.set_about_dev(about::DEV.into());
    app.set_about_project(about::PROJECT.into());
    app.set_about_version(about::VERSION.into());
    app.set_about_github(about::GITHUB.into());
    app.set_about_portfolio(about::PORTFOLIO.into());
    app.set_about_donate(about::DONATE.into());
    app.set_about_linkedin(about::LINKEDIN.into());

    handlers::register_tasks(&app, &model, &store);
    handlers::register_lists(&app, &model, &store);
    handlers::register_ui(&app);
    handlers::init_ui(&app, &model, &store.borrow());

    app.run()
}
