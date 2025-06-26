// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::{
    Manager,
    menu::{MenuBuilder, MenuItemBuilder, SubmenuBuilder},
};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_positioner::init())
        .setup(|app| {
            #[cfg(target_os = "macos")]
            {
                let app_menu = SubmenuBuilder::new(app, "Vortex AI Chat")
                    .items(&[
                        &MenuItemBuilder::with_id("about", "About Vortex AI Chat").build(app)?,
                        &MenuItemBuilder::with_id("preferences", "Preferences...").accelerator("Cmd+,").build(app)?,
                        &MenuItemBuilder::with_id("quit", "Quit").accelerator("Cmd+Q").build(app)?,
                    ])
                    .build()?;

                let edit_menu = SubmenuBuilder::new(app, "Edit")
                    .items(&[
                        &MenuItemBuilder::with_id("undo", "Undo").accelerator("Cmd+Z").build(app)?,
                        &MenuItemBuilder::with_id("redo", "Redo").accelerator("Cmd+Shift+Z").build(app)?,
                        &MenuItemBuilder::with_id("cut", "Cut").accelerator("Cmd+X").build(app)?,
                        &MenuItemBuilder::with_id("copy", "Copy").accelerator("Cmd+C").build(app)?,
                        &MenuItemBuilder::with_id("paste", "Paste").accelerator("Cmd+V").build(app)?,
                        &MenuItemBuilder::with_id("select_all", "Select All").accelerator("Cmd+A").build(app)?,
                    ])
                    .build()?;

                let view_menu = SubmenuBuilder::new(app, "View")
                    .items(&[
                        &MenuItemBuilder::with_id("submit", "Submit Message").accelerator("Cmd+Enter").build(app)?,
                        &MenuItemBuilder::with_id("toggle_sidebar", "Toggle Sidebar").accelerator("Cmd+B").build(app)?,
                    ])
                    .build()?;

                let window_menu = SubmenuBuilder::new(app, "Window")
                    .items(&[
                        &MenuItemBuilder::with_id("minimize", "Minimize").accelerator("Cmd+M").build(app)?,
                        &MenuItemBuilder::with_id("zoom", "Zoom").build(app)?,
                        &MenuItemBuilder::with_id("close", "Close").accelerator("Cmd+W").build(app)?,
                    ])
                    .build()?;

                let help_menu = SubmenuBuilder::new(app, "Help")
                    .items(&[
                        &MenuItemBuilder::with_id("help", "Vortex AI Chat Help").accelerator("Cmd+?").build(app)?,
                    ])
                    .build()?;

                let menu = MenuBuilder::new(app)
                    .items(&[&app_menu, &edit_menu, &view_menu, &window_menu, &help_menu])
                    .build()?;

                app.set_menu(menu)?;

                app.on_menu_event(|app, event| {
                    let window = app.get_webview_window("main").unwrap();
                    match event.id().as_ref() {
                        "submit" => {
                            window.eval("document.dispatchEvent(new KeyboardEvent('keydown', { key: 'Enter', metaKey: true }))").unwrap();
                        }
                        "toggle_sidebar" => {
                            window.eval("document.dispatchEvent(new KeyboardEvent('keydown', { key: 'b', metaKey: true }))").unwrap();
                        }
                        "quit" => {
                            std::process::exit(0);
                        }
                        _ => {}
                    }
                });
            }

            // Handle keyboard events through JavaScript
            if let Some(window) = app.get_webview_window("main") {
                window.eval(r#"
                    document.addEventListener('keydown', (e) => {
                        if (e.key === 'Enter') {
                            if (e.metaKey || e.ctrlKey) {
                                // Cmd+Enter: Submit message
                                document.dispatchEvent(new CustomEvent('submit-message'));
                            } else if (!e.shiftKey) {
                                // Enter: Submit message (when not recording)
                                document.dispatchEvent(new CustomEvent('submit-message'));
                            }
                            // Shift+Enter: New line (handled by default)
                        }
                    });
                "#).unwrap();
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
