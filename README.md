# App Task — Lista de tareas (Rust + Slint)

Checklist + panel de detalles, tema claro/oscuro y sección Acerca de.

Licencia: **GPL-3.0-only** (`LICENSE`). Copyright (C) 2026 Alfredo Riveros.
Slint se usa bajo su opción GPL-3.0, compatible con este proyecto.

## Estructura UI (Slint refactorizado)
```
ui/
├── app.slint                  # Ventana raíz: solo estado + composición
├── theme.slint                # Global AppTheme: paleta claro/oscuro centralizada
├── models.slint               # Struct TaskData compartido
├── components/
│   ├── top_bar.slint          # Botón ☰ + título de lista + navegación + tema + idioma
│   ├── menu_panel.slint       # Desplegable ☰: listas, renombrar, exportar
│   ├── task_row.slint         # Una fila: checkbox + título + preview + ✎/★/🗑
│   ├── pending_header.slint   # Cabecera "Pending" + contador + colapso
│   ├── add_task_bar.slint     # Input + botón Añadir
│   └── filter_bar.slint       # Filtros Todas/Pendientes/Hechas/Destacadas
└── views/
    ├── task_list_view.slint   # Compone header + alta + filtros + lista
    ├── details_panel.slint    # Ver/editar título, estado y descripción
    └── about_view.slint       # Autor, proyecto, GitHub, portafolio, donaciones
src/
├── main.rs       # Punto de entrada: crea App, modelo, store y registra handlers
├── model.rs      # TaskItem/TaskListData/StoreData, defaults, helpers + tests
├── store.rs      # Rutas, load/save, migración legacy, slugify, export + tests
├── config.rs     # AppConfig (tema, idioma), load/save, selección de idioma
├── about.rs      # Constantes de Acerca de (edita aquí tus datos)
└── handlers.rs   # register_tasks / register_lists / register_ui + helpers UI
```

Reglas del refactor:
- `app.slint` no tiene estilos ni lógica visual, solo compone.
- Los colores solo viven en `theme.slint` (vía `AppTheme.*`).
- Los componentes no acceden al `root` de `App`: reciben `in`/`in-out properties` y emiten `callback`s.
- `dark-mode` vive en `App` (para que Rust lo lea/guarde) y se propaga al global con `changed dark-mode => { AppTheme.dark-mode = ...; }`, porque el compilador Slint no exporta globales de ficheros importados a Rust.

## Funciones
- Múltiples listas (menú ☰): crear, renombrar, eliminar, cambiar de lista
- Exportar lista activa a JSON (`~/Documentos/app-task-<lista>.json`)
- Checklist: checkbox, título, preview de descripción, ✎ seleccionar, ★ destacar, 🗑 borrar
- Clic en la fila o en el checkbox → muestra la descripción en el panel derecho
- Cabecera `Pending` con contador + colapso
- Filtros + alta de tareas (Enter o Añadir)
- Tema claro/oscuro persistido en `~/.local/share/app-task/config.json`
- Tareas en `lists.json` (multi-lista; en proyecto o `~/.local/share/app-task/`).
  Migra automáticamente el `tasks.json` anterior.
  `cargo test` verifica store, migración, slug y modelo.
- Vista Acerca de: edita tus enlaces en `src/about.rs` (`GITHUB`, `PORTFOLIO`, `DONATE`, `LINKEDIN`, `DEV`)

## Idiomas (ES/EN automático)
- Todos los textos visibles usan `@tr()` en los `.slint`.
- Traducciones empaquetadas en el binario vía `build.rs`
  (`with_bundled_translations("lang")`): sin archivos `.mo` externos.
- `lang/app-task.pot` (plantilla, regenerar con `slint-tr-extractor`),
  `lang/en/LC_MESSAGES/app-task.po` (inglés). El español es el idioma fuente.
- Detección automática por locale del sistema (`es_*` → español,
  `en_*` → inglés); botones **ES/EN** en la barra superior para cambio
  manual (persistido en `config.json` como `lang: auto|es|en`).
- Agregar un idioma: `msginit -l fr -i lang/app-task.pot -o lang/fr/LC_MESSAGES/app-task.po`,
  traducir, recompilar. Requiere `cargo install slint-tr-extractor` y `gettext`.

## Ejecutar / instalar (Fedora)
```bash
cd /home/Alfredo/Documentos/app-task
cargo run                  # desarrollo
cargo build --release && cp target/release/app-task ~/.local/bin/
```
Lanzador: `~/.local/share/applications/io.github.silvioalfredo.apptask.desktop`
→ busca **App Task** en el menú.

## Publicar en la Software de Fedora (vía Flathub)
App ID: `io.github.silvioalfredo.apptask` (minúsculas, exigido por Flathub).
- `LICENSE` (GPL-3.0-only) + `license` en `Cargo.toml` + cabeceras SPDX en fuentes.
- `data/*.desktop` validado con `desktop-file-validate` (sin avisos).
- `data/*.metainfo.xml` validado con `appstreamcli validate` (limpio).
- `io.github.silvioalfredo.apptask.yml`: manifiesto Flatpak (runtime
  freedesktop 26.08, sin red, `--persist` de datos y `xdg-documents` para exportar).
- Pendiente antes del PR a Flathub:
  1. Crear el repo público `github.com/SilvioAlfredo/app-task` y subir el código.
  2. Capturar 2 screenshots (lista + acerca de) en `data/screenshots/` y
     descomentar el bloque `<screenshots>` del metainfo.
  3. Generar `cargo-sources.json` (`flatpak-cargo-generator.py Cargo.lock`).
  4. Tag `v0.1.0`, compilar con `flatpak-builder --install`, probar,
     y abrir el PR en `github.com/flathub`.
