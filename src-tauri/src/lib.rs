use std::str::FromStr;

use comrak::{
    format_html, nodes::NodeValue, parse_document, Arena, ExtensionOptions, Options, RenderOptions,
    RenderOptionsBuilder,
};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn parse_markdown(document: &str) -> String {
    println!("{:?}", document.bytes());
    let arena = Arena::new();

    // Parse the document into a root `AstNode`
    let mut options = Options::default();
    // options.render.hardbreaks = true;
    let root = parse_document(&arena, document, &options);

    // Iterate over all the descendants of root.
    for node in root.descendants() {
        if let NodeValue::SoftBreak = node.data.borrow_mut().value {
            println!("linebreak {:?}", node);
        }
    }
    let mut html = vec![];
    format_html(root, &options, &mut html).unwrap();
    println!("{:?}", String::from_utf8(html.clone()));
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
