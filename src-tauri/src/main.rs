// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::menu::{MenuItemBuilder, MenuItemKind, PredefinedMenuItem};

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn md_to_html(md: &str) -> String {
    let mut options = comrak::Options::default();
    options.extension.autolink = true;
    options.extension.description_lists = true;
    options.extension.footnotes = true;
    options.extension.front_matter_delimiter = Some("---".to_owned());
    options.extension.math_code = true;
    options.extension.math_dollars = true;
    options.extension.multiline_block_quotes = true;
    options.extension.shortcodes = true;
    options.extension.strikethrough = true;
    options.extension.superscript = true;
    options.extension.table = true;
    options.extension.tagfilter = true;
    options.extension.tasklist = true;
    options.extension.header_ids = Some("nav-item-".to_string());
    comrak::markdown_to_html(md, &options)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![md_to_html])
        .setup(|app| {
            #[cfg(target_os = "macos")]
            if let Some(menu) = app.menu() {
                if let Ok(items) = menu.items() {
                    for item in &items {
                        println!("{:#?}", item.id());
                    }
                    let separator = PredefinedMenuItem::separator(app)?;
                    let open = MenuItemBuilder::with_id("open", "Open")
                        .accelerator("CmdOrCtrl+O")
                        .build(app)?;
                    if let Some(MenuItemKind::Submenu(file_menu)) = items.get(1) {
                        file_menu.insert(&separator, 0)?;
                        file_menu.insert(&open, 0)?;
                    }
                }
            }
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
