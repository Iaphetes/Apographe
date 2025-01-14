mod file_tree;
mod markdown_parser;
mod search;
use std::env;
use tauri_plugin_fs::FsExt;

use search::search_files;
use file_tree::dir_tree_html;
use markdown_parser::parse_markdown;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_shell::init())
        .setup(|app| {
            // allowed the given directory
            let scope = app.fs_scope();
            scope.allow_directory(tauri::path::BaseDirectory::Home.variable(), true);
            app.set_theme(Some(tauri::Theme::Dark));

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![dir_tree_html, parse_markdown, search_files])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
