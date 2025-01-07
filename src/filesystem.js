const { convertFileSrc, invoke } = window.__TAURI__.core;
const { readTextFile } = window.__TAURI__.fs;
import { render_markdown } from "./main.js";
function handle_file_select(filename) {
  if (filename.endsWith("md")) {
    readTextFile(convertFileSrc(filename)).then(
      (ret) => {
        var tag_id = document.getElementById('markdown_input');
        tag_id.innerHTML = "<pre>".concat("", ret).concat("", "</pre>");
        render_markdown();
      }
    );
  }
}

function dropdown(id) {
  var dropdown_element = document.getElementById(id);
  var dropdown_children = dropdown_element.children;
  console.log(dropdown_element.getAttribute("expanded"));
  if (dropdown_element.getAttribute("expanded") == "false") {
    dropdown_element.setAttribute("expanded", "true");
  }
  else {
    dropdown_element.setAttribute("expanded", "false");
  }
  for (var i = 0; i < dropdown_children.length; i++) {
    var child = dropdown_children[i];
    console.log(child.id);
    if (child.className === "filetree-node") {
      if (document.getElementById(id).getAttribute("expanded") == "true") {
        document.getElementById(child.id).style.visibility = "visible";
        document.getElementById(child.id).style.height = "auto";
      } else {
        document.getElementById(child.id).style.visibility = "hidden";
        document.getElementById(child.id).style.height = 0;
      }
    }
  }

}
window.onload = function() {
  invoke("dir_tree_html", { basepath: "~/Documents/Knowledgebase", filter: ["*"] }).then(
    (ret) => {
      var tag_id = document.getElementById('filetree');
      tag_id.innerHTML = ret;
    }
  )
}
let filetree = document.getElementById('filetree');
// Options for the observer (which mutations to observe)
const config = { attributes: true, childList: true, subtree: true };

// Callback function to execute when mutations are observed
const callback = (mutationList, observer) => {
  var anchors = document.getElementsByClassName("filetree-directory-button");
  for (var i = 0; i < anchors.length; i++) {
    var anchor = anchors[i];
    anchor.onclick = function() {
      dropdown(this.parentElement.id);
    };
  };
  var anchors = document.getElementsByClassName("filetree-file-button");
  for (var i = 0; i < anchors.length; i++) {
    var anchor = anchors[i];
    anchor.onclick = function() {
      handle_file_select(this.parentElement.id);
    };
  };

};

// Create an observer instance linked to the callback function
const observer = new MutationObserver(callback);

// Start observing the target node for configured mutations
observer.observe(filetree, config);

