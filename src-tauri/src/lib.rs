use comrak::{format_html, nodes::NodeValue, parse_document, Arena, Options};
use tauri::Manager;
use tauri_plugin_fs::FsExt;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn parse_markdown(document: &str) -> String {
    let mut rendered_markdown: String = String::new();
    tauri::Builder::default().setup(move |app| {
        let webview = app.get_webview_window("main").unwrap();
        let arena = Arena::new();

        // Parse the document into a root `AstNode`
        let mut options = Options::default();
        options.render.unsafe_ = true;
        // options.render.hardbreaks = true;
        let root = parse_document(&arena, document, &options);

        // Iterate over all the descendants of root.
        for node in root.descendants() {
            if let NodeValue::Image(ref mut image_node) = node.data.borrow_mut().value {
                // image_node.url = format!("file://{}", image_node.url);
                println!("{:?}", webview.eval(&image_node.url));
            }
        }
        let mut html = vec![];
        format_html(root, &options, &mut html).unwrap();
        println!("{}", String::from_utf8(html.clone()).unwrap());
        rendered_markdown = String::from_utf8(html).unwrap();
        Ok(())
    });
    return "".to_owned();
    // String::from_str("lololo").unwrap()
}

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
        .invoke_handler(tauri::generate_handler![greet])
        .invoke_handler(tauri::generate_handler![parse_markdown])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
