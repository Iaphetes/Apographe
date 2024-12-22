const { convertFileSrc, invoke } = window.__TAURI__.core;
// const {  invoke } = window.__TAURI__.core;
const { homeDir, join } = window.__TAURI__.path;
const { readFile } = window.__TAURI__.fs;

const appDataDirPath = await homeDir();
const filePath = await join(appDataDirPath, 'Pictures/wallpaper.png');

const assetUrl = convertFileSrc(filePath);

let text = "";
// window.addEventListener("DOMCharacterDataModified", () => {
  // var tag_id = document.getElementById('rendered_markdown');
  // tag_id.innerHTML = "<p>HI</p>"
// window.addEventListener("DOMContentLoaded", () => {
let textarea = document.getElementById('markdown_input');
textarea.addEventListener('input', ()=> {
    text = textarea.innerText;
    var tag_id = document.getElementById('rendered_markdown');
    tag_id.innerHTML = "<p>HI</p>"
  invoke("parse_markdown", { document: text }).then(
    (ret)=>{    
      var tag_id = document.getElementById('rendered_markdown');
      tag_id.innerHTML = "<pre>".concat("", ret).concat("", "</pre>");
   // tag_id.innerHTML = assetUrl.concat(" ", ' \n <img src="'.concat("", assetUrl).concat("", '" alt="Girl in a jacket" width="500" height="600">'))
    }
  );

// });
});
document.getElementById("hide-sidebar").onclick = function() {toggle_visibility("sidebar");};
function toggle_visibility(id) {
    if (document.getElementById(id).style.visibility == "hidden"){
        document.getElementById(id).style.visibility = "visible";
        var style = window.getComputedStyle(document.body);
        document.getElementById(id).style.width = style.getPropertyValue("--sidebar-width");
    }else{
        
        document.getElementById(id).style.visibility = "hidden";
        document.getElementById(id).style.width = 0;
    }
    
}
function handleShortcut(event) {
    var markdown_editor = document.getElementById('markdown_input');

    if (document.activeElement === markdown_editor){
        if (event.ctrlKey) {
            if (event.key === "l"){            
                event.preventDefault();
            }
        }
    }
}
document.addEventListener("keydown", handleShortcut);
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
