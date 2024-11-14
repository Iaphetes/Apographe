const { convertFileSrc, invoke } = window.__TAURI__.core;
const { homeDir, join } = window.__TAURI__.path;
const { readFile } = window.__TAURI__.fs;

const appDataDirPath = await homeDir();
const filePath = await join(appDataDirPath, 'Pictures/wallpaper.png');

const assetUrl = convertFileSrc(filePath);

let text = "";
window.addEventListener("DOMCharacterDataModified", () => {
// window.addEventListener("DOMContentLoaded", () => {
  let textarea = document.getElementById('markdown_input');
  textarea.addEventListener('input', ()=> {
    text = textarea.innerText;
    invoke("parse_markdown", { document: text }).then(
      (ret)=>{    
        var tag_id = document.getElementById('rendered_markdown');
        // tag_id.innerHTML = "<pre>".concat("", ret).concat("", "</pre>");
        tag_id.innerText = assetUrl.concat("", ' \n <img src="'.concat("", assetUrl).concat("", '" alt="Girl in a jacket" width="500" height="600">'))
      }
    );

  });
});
