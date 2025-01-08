use comrak::{format_html, nodes::NodeValue, parse_document, Arena, Options};
use std::{
    env,
    
    path::{absolute, Path},
};
#[tauri::command]
pub fn parse_markdown(document: &str, pathtemplate: &str, basepath: &str) -> String {
    let rendered_markdown: String;
    let path = "/foo/bar.txt";
    println!("{:?}", shellexpand::full(path));
    let arena = Arena::new();

    // Parse the document into a root `AstNode`
    let mut options = Options::default();
    options.render.unsafe_ = true;
    // options.render.hardbreaks = true;
    let root = parse_document(&arena, document, &options);

    // Iterate over all the descendants of root.

    if let Ok(resolved_basepath) = shellexpand::full(&basepath) {
        println!(
            "{:?}",
            env::set_current_dir(&resolved_basepath.into_owned())
        );
    }

    for node in root.descendants() {
        if let NodeValue::Image(ref mut image_node) = node.data.borrow_mut().value {
            let image_path = Path::new(&image_node.url).to_path_buf();

            let absolute_image_path_res = absolute(image_node.url.clone());

            let absolute_image_path = absolute_image_path_res.unwrap_or(image_path.clone());
            let absolute_image_path_str = absolute_image_path.as_path().to_string_lossy();
            if let Ok(resolved_path) = shellexpand::full(&absolute_image_path_str) {
                image_node.url = pathtemplate.replace("FILEPATH", &resolved_path);
            }
            println!("{}", image_node.url);
        }
    }
    let mut html = vec![];
    format_html(root, &options, &mut html).unwrap();
    // println!("{}", String::from_utf8(html.clone()).unwrap());
    rendered_markdown = String::from_utf8(html).unwrap();
    return rendered_markdown.to_owned();
    // String::from_str("lololo").unwrap()
}
