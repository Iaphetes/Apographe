use html_tag::HtmlTag;
use std::{
    fs::{self, DirEntry},
    path::Path,
};

#[tauri::command]
pub fn search_files(basepath: &str, searchstring: &str, filter: Vec<String>) -> Vec<String> {
    match shellexpand::full(basepath) {
        Ok(path) => _search_files(&Path::new(&path.into_owned()), searchstring, &filter),

        Err(_) => Vec::new(),
    }
}

fn search_result_div_from_dir_entry(dir_entry: &DirEntry) -> HtmlTag {
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
pub fn _search_files(path: &Path, searchstring: &str, filter: &Vec<String>) -> Vec<String> {
    let mut search_results: Vec<String> = Vec::new();
    if let Ok(entries) = fs::read_dir(path) {
        for dir_entry_res in entries {
            if let Ok(dir_entry) = dir_entry_res {
                if let Ok(metadata) = fs::metadata(dir_entry.path()) {
                    if metadata.is_file() {
                        if filter.contains(
                            &dir_entry
                                .path()
                                .extension()
                                .unwrap_or_default()
                                .to_string_lossy()
                                .to_string()
                        ) {
                            let div =search_result_div_from_dir_entry(&dir_entry).to_html();
                            println!("{:?}", div);
                            search_results
                                .push(div);
                        }
                    } else if metadata.is_dir() {
                        search_results.append(&mut _search_files(
                            &dir_entry.path(),
                            searchstring,
                            &filter,
                        ));
                    }
                }
            }
        }
    }
    println!("{:?}", search_results);
    return search_results;
}
