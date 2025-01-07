import { showGlobalSearch, escape } from "./ui.js";

document
  .addEventListener("keydown",
    function(event) {
      if (event.ctrlKey && event.key === "k") {
        event.preventDefault();
        showGlobalSearch();

      }
      if (event.key == "Escape"){
        event.preventDefault();
        escape();
      }
    });


