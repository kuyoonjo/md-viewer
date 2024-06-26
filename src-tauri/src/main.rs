// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use tauri::{
    menu::{CheckMenuItemBuilder, MenuId, MenuItemKind, PredefinedMenuItem},
    AppHandle, Manager,
};
use tauri_plugin_deep_link::DeepLinkExt;

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

#[cfg(any(target_os = "windows", target_os = "linux"))]
fn args() -> Vec<String> {
    let args = std::env::args();
    args.into_iter().collect()
}

#[tauri::command]
fn url_to_path(url: &str) -> String {
    if let Ok(url) = url::Url::parse(url) {
        if let Ok(url) = url.to_file_path() {
            if let Some(url) = url.to_str() {
                return url.to_string();
            }
        }
    }
    "".to_string()
}

#[tauri::command]
fn join(url: &str, path: &str) -> String {
    if let Ok(url) = url::Url::parse(url) {
        if let Ok(url) = url.join(path) {
            return url.to_string();
        }
    }
    "".to_string()
}

#[tauri::command]
fn decode_hex(hex: &str) -> String {
    if let Ok(v) = hex::decode(hex) {
        if let Ok(s) = String::from_utf8(v) {
            return s;
        }
    }
    "".to_string()
}

#[cfg(target_os = "macos")]
fn check_win_menu(app: &AppHandle, id: &MenuId) {
    if let Some(menu) = app.menu() {
        if let Some(MenuItemKind::Submenu(ref win_menu)) = menu.get(tauri::menu::WINDOW_SUBMENU_ID)
        {
            if let Ok(items) = win_menu.items() {
                for item in items {
                    if let MenuItemKind::Check(item) = item {
                        item.set_checked(item.id() == id).unwrap();
                    }
                }
            }
        }
    }
}

#[cfg(target_os = "macos")]
fn add_win_menu(app: &AppHandle, label: &str) {
    let title = if label == "" {
        "MD Viewer".to_string()
    } else {
        url_to_path(label)
    };
    let label = STANDARD.encode(label);
    if let Some(win) = app.get_webview_window(&label) {
        let _ = win.set_focus();
        let id = format!("window-{}", &label);
        let id = MenuId::new(id);
        check_win_menu(app, &id);
        return;
    }
    let win =
        tauri::WebviewWindowBuilder::new(app, &label, tauri::WebviewUrl::App("index.html".into()))
            .inner_size(800.0, 600.0)
            .title(&title)
            .build()
            .unwrap();
    if let Some(menu) = app.menu() {
        if let Some(MenuItemKind::Submenu(ref win_menu)) = menu.get(tauri::menu::WINDOW_SUBMENU_ID)
        {
            let id = format!("window-{}", &label);
            let menu = CheckMenuItemBuilder::with_id(&id, &title)
                .build(app)
                .unwrap();
            win_menu.append(&menu).unwrap();
            check_win_menu(app, menu.id());

            win.on_window_event(move |e| match e {
                tauri::WindowEvent::Destroyed => {
                    remove_win_menu(menu.app_handle(), menu.id());
                }
                tauri::WindowEvent::Focused(focused) => {
                    if *focused {
                        check_win_menu(menu.app_handle(), menu.id());
                    }
                }
                _ => {}
            });
        }
    }
}

#[cfg(any(target_os = "windows", target_os = "linux"))]
fn add_win(app: &AppHandle, label: &str) {
    let title = if label == "" {
        "MD Viewer".to_string()
    } else {
        url_to_path(label)
    };
    let label = hex::encode(label);
    tauri::WebviewWindowBuilder::new(app, &label, tauri::WebviewUrl::App("index.html".into()))
        .inner_size(800.0, 600.0)
        .title(&title)
        .build()
        .unwrap();
}

#[cfg(target_os = "macos")]
fn remove_win_menu(app: &AppHandle, id: &MenuId) {
    if let Some(menu) = app.menu() {
        if let Some(MenuItemKind::Submenu(ref win_menu)) = menu.get(tauri::menu::WINDOW_SUBMENU_ID)
        {
            if let Ok(items) = win_menu.items() {
                for item in items {
                    if let MenuItemKind::Check(item) = item {
                        if item.id() == id {
                            win_menu.remove(&item).unwrap();
                        }
                    }
                }
            }
        }
    }
}

fn main() {
    let app = tauri::Builder::default();
    #[cfg(target_os = "macos")]
    let app = app.plugin(tauri_plugin_deep_link::init());
    let app = app
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            md_to_html,
            url_to_path,
            join,
            decode_hex
        ])
        .setup(|app| {
            #[cfg(any(target_os = "windows", target_os = "linux"))]
            {
                // NOTICE: `args` may include URL protocol (`your-app-protocol://`) or arguments (`--`) if app supports them.
                let mut urls = Vec::new();
                for arg in std::env::args().skip(1) {
                    if let Ok(url) = url::Url::from_file_path(&arg) {
                        urls.push(url.to_string());
                    }
                }

                if let Some(url) = urls.get(0) {
                    add_win(app.handle(), url);
                } else {
                    add_win(app.handle(), "");
                }
            }

            #[cfg(target_os = "macos")]
            {
                if let Some(menu) = app.menu() {
                    if let Some(MenuItemKind::Submenu(win_menu)) =
                        menu.get(tauri::menu::WINDOW_SUBMENU_ID)
                    {
                        let separator = PredefinedMenuItem::separator(app)?;
                        win_menu.insert(&separator, 4)?;
                    }
                }

                let deep_link = app.deep_link();
                if let Ok(Some(urls)) = deep_link.get_current() {
                    for url in urls {
                        let url = url.to_string();
                        add_win_menu(app.handle(), &url);
                    }
                } else {
                    add_win_menu(app.handle(), "");
                }

                app.on_menu_event(move |app, event| {
                    let id = event.id();
                    if id.0.starts_with("window-") {
                        check_win_menu(app, id);
                        let label = &id.0[7..];
                        if let Some(win) = app.get_webview_window(label) {
                            let _ = win.set_focus();
                        }
                    }
                });
            }
            
            Ok(())
        });

    #[cfg(target_os = "macos")]
    app.build(tauri::generate_context!())
        .expect("error while running tauri application")
        .run(|app, event| {
            #[cfg(target_os = "macos")]
            if let tauri::RunEvent::Opened { urls } = event {
                for url in urls {
                    let url = url.to_string();
                    add_win_menu(app, &url);
                }
            }
        });

    #[cfg(any(target_os = "windows", target_os = "linux"))]
    app.run(tauri::generate_context!())
        .expect("error while running tauri application");
}
