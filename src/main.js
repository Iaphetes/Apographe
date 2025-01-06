const { convertFileSrc, invoke } = window.__TAURI__.core;
const { homeDir, join } = window.__TAURI__.path;
const { readFile } = window.__TAURI__.fs;

const appDataDirPath = await homeDir();
const filePath = await join(appDataDirPath, 'Pictures/wallpaper.png');

const assetUrl = convertFileSrc(filePath);

let text = "";
let placeholder_path = "FILEPATH";
let path_template = convertFileSrc(placeholder_path);

let textarea = document.getElementById('markdown_input');


export function render_markdown() {
  text = textarea.innerText;
  invoke("parse_markdown", { document: text, pathtemplate: path_template, basepath: "$HOME/Documents/Knowledgebase" }).then(
    (ret) => {
      var tag_id = document.getElementById('rendered_markdown');
      tag_id.innerHTML = "<pre>".concat("", ret).concat("", "</pre>");
      // tag_id.innerHTML = assetUrl.concat(" ", ' \n <img src="'.concat("", assetUrl).concat("", '" alt="Girl in a jacket" width="500" height="600">'))
    }
  );
}
textarea.addEventListener('input', () => {
  render_markdown();


});
const dialog = document.querySelector("dialog");
function showSearch() {
  dialog.show();
}

document
  .addEventListener("keydown",
    function(event) {
      if (event.key === "a") {
        event.preventDefault();
        showSearch();
      }
    });
// Random tree
// const N = 300;
// const gData = {
//   nodes: [...Array(N).keys()].map(i => ({ id: i })),
//   links: [...Array(N).keys()]
//     .filter(id => id)
//     .map(id => ({
//       source: id,
//       target: Math.round(Math.random() * (id-1))
//     }))
// };
// const Graph = new ForceGraph(
//   (document.getElementById('graph'))
//     .linkDirectionalParticles(2)
//     .graphData(gData);
