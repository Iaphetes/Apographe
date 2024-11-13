const { invoke } = window.__TAURI__.core;

let greetInputEl;
let greetMsgEl;

async function greet() {
  // Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
  greetMsgEl.textContent = await invoke("greet", { name: "harald" });
}

let text = "";
window.addEventListener("DOMContentLoaded", () => {
  let textarea = document.getElementById('markdown_input');
  textarea.addEventListener('input', ()=> {
    text = textarea.value;
    invoke("parse_markdown", { document: text }).then(
      (ret)=>{    
        var tag_id = document.getElementById('rendered_markdown');
        tag_id.innerHTML = ret;
      }
    );

  });
});
var x = document.createElement("INPUT");
x.setAttribute("type", "text");
