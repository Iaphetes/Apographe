use comrak::{format_html, nodes::NodeValue, parse_document, Arena, Options};
use html_tag::HtmlTag;
use shellexpand;
use std::{
    collections::BTreeMap,
    env,
    ffi::{OsStr, OsString},
    fs,
    path::{absolute, Path},
};
use tauri::{ipc::IpcResponse, Manager};
use tauri_plugin_fs::FsExt;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
fn parse_markdown(document: &str, pathtemplate: &str, basepath: &str) -> String {
    let mut rendered_markdown: String = String::new();
    let path = "/foo/bar.txt";
    println!("{:?}", shellexpand::full(path));
    // let webview = app.get_webview_window("main").unwrap();
    let arena = Arena::new();

    // Parse the document into a root `AstNode`
    let mut options = Options::default();
    options.render.unsafe_ = true;
    // options.render.hardbreaks = true;
    let root = parse_document(&arena, document, &options);

    // Iterate over all the descendants of root.
    env::set_current_dir(basepath);
    for node in root.descendants() {
        if let NodeValue::Image(ref mut image_node) = node.data.borrow_mut().value {
            let image_path = Path::new(&image_node.url).to_path_buf();
            let absolute_image_path = absolute(image_node.url.clone());
            let absolute_image_path = 
.unwrap_or(image_path)
                .as_path()
                .to_string_lossy()
            if let Ok(resolved_path) = shellexpand::full(&absolute_image_path) {
                image_node.url = pathtemplate.replace("FILEPATH", &resolved_path);
            }
        }
    }
    let mut html = vec![];
    format_html(root, &options, &mut html).unwrap();
    println!("{}", String::from_utf8(html.clone()).unwrap());
    rendered_markdown = String::from_utf8(html).unwrap();
    return rendered_markdown.to_owned();
    // String::from_str("lololo").unwrap()
}
// <div class="filetree-node" id="folder3">
//     <button class="filetree-element">
//         <img class="topbar_icon" src="images/dropdown.svg" />folder 3
//     </button>
// </div>

fn add_dir_tree_node(path: &Path, filter: &Vec<String>) -> HtmlTag {
    let mut html = HtmlTag::new("div")
        .with_class("filetree-node")
        .with_id(&format!(
            "{}",
            Path::new(path)
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
        ))
        .with_attribute("expanded", "false")
        .with_child(
            HtmlTag::new("button")
                .with_class("filetree-directory-button")
                .with_child(
                    HtmlTag::new("img")
                        .with_class("filetree-icon")
                        .with_attribute("src", "images/directory.svg"),
                )
                .with_child(
                    HtmlTag::new("a").with_body(
                        &Path::new(path)
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy(),
                    ),
                ),
        );
    if let Ok(entries) = fs::read_dir(path) {
        for dir_entry_res in entries {
            println!("{:?}", dir_entry_res);
            if let Ok(dir_entry) = dir_entry_res {
                if let Ok(metadata) = fs::metadata(dir_entry.path()) {
                    if metadata.is_file() {
                        let mut file_div = HtmlTag::new("div")
                            .with_class("filetree-node")
                            .with_id(&format!("{}", dir_entry.path().to_string_lossy()))
                            .with_attribute("style", "visibility: hidden; height: 0px;");
                        let mut file_button =
                            HtmlTag::new("button").with_class("filetree-file-button");
                        match dir_entry
                            .path()
                            .extension()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string()
                            .as_str()
                        {
                            "md" => file_button.add_child(
                                HtmlTag::new("img")
                                    .with_class("filetree-icon")
                                    .with_attribute("src", "images/markdown.svg"),
                            ),

                            _ => file_button.add_child(
                                HtmlTag::new("img")
                                    .with_class("filetree-icon")
                                    .with_attribute("src", "images/file.svg"),
                            ),
                        };

                        file_button.add_child(
                            HtmlTag::new("a").with_body(&dir_entry.file_name().to_string_lossy()),
                        );
                        file_div.add_child(file_button);

                        html.add_child(file_div)
                    } else if metadata.is_dir() {
                        html.add_child(
                            add_dir_tree_node(&dir_entry.path(), &filter)
                                .with_attribute("style", "visibility: hidden; height: 0px;"), // .with_style("visibility", "hidden")
                                                                                              // .with_style("height", "0px"),
                        );
                    }
                }
            }
        }
    }
    return html;
}
#[tauri::command]
fn dir_tree_html(basepath: &str, filter: Vec<String>) -> String {
    match shellexpand::full(basepath) {
        Ok(path) => add_dir_tree_node(&Path::new(&path.into_owned()), &filter).to_html(),

        Err(_) => String::new(),
    }
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
        .invoke_handler(tauri::generate_handler![dir_tree_html, parse_markdown])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
