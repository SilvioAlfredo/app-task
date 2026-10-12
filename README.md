# App Task

Gestor de listas de tareas de escritorio: checklist con descripciones, múltiples listas, filtros, tema claro/oscuro y guardado local automático. Desarrollado en **Rust** con interfaz declarativa en **Slint**, para Linux.

<!-- Capturas pendientes: data/screenshots/tasks.png y data/screenshots/about.png
![Lista de tareas](data/screenshots/tasks.png)
![Acerca de](data/screenshots/about.png)
-->

## Funciones

- **Múltiples listas**: crear, renombrar, eliminar y cambiar de lista desde el menú ☰.
- **Checklist**: marcar, destacar ★, borrar, y ver la descripción con un clic.
- **Panel de detalles**: título, estado y notas por tarea.
- **Filtros**: todas, pendientes, hechas y destacadas, más contador de pendientes.
- **Exportar** la lista activa a JSON (`~/Documentos/app-task-<lista>.json`).
- **Tema claro/oscuro** y **español/inglés** (automático según el sistema, con cambio manual).
- **Sin cuentas ni nube**: todo queda en tu equipo (`~/.local/share/app-task/`).

## Instalar

**Fedora Software (Flathub)** — en camino (`io.github.silvioalfredo.apptask`).

**Desde el código fuente** (Fedora):
```bash
sudo dnf install -y cargo gcc libX11-devel libXcursor-devel libXrandr-devel \
  libXi-devel libGL-devel fontconfig-devel dbus-devel
git clone https://github.com/SilvioAlfredo/app-task.git
cd app-task
cargo build --release
cp target/release/app-task ~/.local/bin/
sed "s|^Exec=app-task$|Exec=$HOME/.local/bin/app-task|" \
  data/io.github.silvioalfredo.apptask.desktop \
  > ~/.local/share/applications/io.github.silvioalfredo.apptask.desktop
mkdir -p ~/.local/share/icons/hicolor/scalable/apps ~/.local/share/metainfo
cp data/icons/hicolor/scalable/apps/io.github.silvioalfredo.apptask.svg \
  ~/.local/share/icons/hicolor/scalable/apps/
cp data/io.github.silvioalfredo.apptask.metainfo.xml ~/.local/share/metainfo/
update-desktop-database ~/.local/share/applications
```
Luego busca **App Task** en el menú.

## Uso

- **☰**: gestiona tus listas y exporta a JSON.
- **Clic en una tarea**: muestra su descripción a la derecha; **Guardar** la persiste.
- **ES/EN**: cambia el idioma (se recuerda); **☀/🌙**: cambia el tema.
- Tus listas migran solas del formato anterior (`tasks.json` → `lists.json`).

## Desarrollo

```bash
cargo run     # desarrollo
cargo test    # 6 tests: modelo, store, migración, slug, export
```

Estructura:

- `ui/` — componentes Slint (`app.slint` compone; `theme.slint` centraliza colores; `components/` y `views/` separan piezas).
- `src/` — `main.rs` (entrada), `model.rs` (datos), `store.rs` (persistencia), `config.rs` (ajustes), `about.rs` (tus datos), `handlers.rs` (callbacks).
- `lang/` — plantilla `app-task.pot` + inglés en `en/LC_MESSAGES/app-task.po` (español de fuente, empaquetado en el binario). Nuevo idioma: `msginit -l fr -i lang/app-task.pot -o lang/fr/LC_MESSAGES/app-task.po`, traducir, recompilar.

## Licencia

**GPL-3.0-only** — ver `LICENSE`. Copyright (C) 2026 Alfredo Riveros.

## Autor

- GitHub: <https://github.com/SilvioAlfredo>
- Portafolio: <https://silvioalfredo.github.io/Portafolio-Riveros-Silvio/>
- LinkedIn: <https://www.linkedin.com/in/silvioriveros/>
- Apoya el proyecto: <https://paypal.me/SilvioRiverosDev>
