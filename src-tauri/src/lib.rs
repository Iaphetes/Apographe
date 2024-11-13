use std::str::FromStr;

use comrak::{format_html, parse_document, Arena, Options};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn parse_markdown(document: &str) -> String {
    let arena = Arena::new();

    // Parse the document into a root `AstNode`
    let root = parse_document(&arena, document, &Options::default());

    // Iterate over all the descendants of root.
    // for node in root.descendants() {
    //     if let NodeValue::Text(ref mut text) = node.data.borrow_mut().value {
    //         // If the node is a text node, perform the string replacement.
    //         *text = text.replace(orig_string, replacement);
    //     }
    // }
    let mut html = vec![];
    format_html(root, &Options::default(), &mut html).unwrap();
    // println!("{:?}", String::from_utf8(html.clone()));
    String::from_utf8(html).unwrap()
    // String::from_str("lololo").unwrap()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![parse_markdown])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
