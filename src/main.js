const { invoke } = window.__TAURI__.core;
const {readFile  } = window.__TAURI__.fs;

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsgEl.textContent = await invoke("greet", { name: "harald" });
}

let text = "";
window.addEventListener("DOMCharacterDataModified", () => {
// window.addEventListener("DOMContentLoaded", () => {
  let textarea = document.getElementById('markdown_input');
  textarea.addEventListener('input', ()=> {
    text = textarea.innerText;
    invoke("parse_markdown", { document: text }).then(
      (ret)=>{    
        var tag_id = document.getElementById('rendered_markdown');
        tag_id.innerHTML = "<pre>".concat("", ret).concat("", "</pre>");
        // tag_id.innerHTML =  ret;
      }
    );

  });
});
