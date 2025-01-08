use html_tag::HtmlTag;
use shellexpand;
use std::{
    fs::{self, DirEntry},
    path::Path,
};
#[tauri::command]
pub fn dir_tree_html(basepath: &str, filter: Vec<String>) -> String {
    match shellexpand::full(basepath) {
        Ok(path) => add_dir_tree_node(&Path::new(&path.into_owned()), &filter).to_html(),

        Err(_) => String::new(),
    }
}
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
                        html.add_child(div_from_dir_entry(&dir_entry))
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

fn div_from_dir_entry(dir_entry: &DirEntry) -> HtmlTag {
    let mut file_div = HtmlTag::new("div")
        .with_class("filetree-node")
        .with_id(&format!("{}", dir_entry.path().to_string_lossy()))
        .with_attribute("style", "visibility: hidden; height: 0px;");
    let mut file_button = HtmlTag::new("button").with_class("filetree-file-button");
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

    file_button.add_child(HtmlTag::new("a").with_body(&dir_entry.file_name().to_string_lossy()));
    file_div.add_child(file_button);
    return file_div;
}
