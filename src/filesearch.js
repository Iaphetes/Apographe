const { convertFileSrc, invoke } = window.__TAURI__.core;

var search_input = document.getElementById("file-search-dialog-input");
search_input.addEventListener('input', () => {
  search_files();
});


function search_files(){
  var text = search_input.innerText;
  invoke("search_files", { searchstring: text, basepath: "$HOME/Documents/Knowledgebase", filter: ["md"]}).then(
    (ret) => {
      var tag_id = document.getElementById('rendered_markdown');
      var result_div =  "";
      ret.array.forEach(element => {
        result_div += element;
      });
      tag_id.innerHTML = "<pre>".concat("", result_div).concat("", "</pre>");
      // tag_id.innerHTML = assetUrl.concat(" ", ' \n <img src="'.concat("", assetUrl).concat("", '" alt="Girl in a jacket" width="500" height="600">'))
    }
  );
}
