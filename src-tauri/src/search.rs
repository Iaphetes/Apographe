
pub fn search_files(searchstring: &str, filter: Vec<&str>) -> HtmlTag{

    if let Ok(entries) = fs::read_dir(path) {
        for dir_entry_res in entries {
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
}
