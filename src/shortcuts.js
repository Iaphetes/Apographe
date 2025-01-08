import { showFileSearch, escape } from "./ui.js";

document
  .addEventListener("keydown",
    function(event) {
      if (event.ctrlKey && event.key === "k") {
        event.preventDefault();
        showFileSearch();

      }
      if (event.key == "Escape"){
        event.preventDefault();
        escape();
      }
    });


