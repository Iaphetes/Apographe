function toggle_visibility(id) {
    if (document.getElementById(id).style.visibility == "hidden") {
        document.getElementById(id).style.visibility = "visible";
        var style = window.getComputedStyle(document.body);
        document.getElementById(id).style.width = style.getPropertyValue("--sidebar-width");
    } else {
        document.getElementById(id).style.visibility = "hidden";
        document.getElementById(id).style.width = 0;
    }
}
function handleShortcut(event) {
    var markdown_editor = document.getElementById("markdown_input");

    if (document.activeElement === markdown_editor) {
        if (event.ctrlKey) {
            if (event.key === "l") {
                event.preventDefault();
            }
        }
    }
}
document.addEventListener("keydown", handleShortcut);
document.getElementById("hide-sidebar").onclick = function () {
    toggle_visibility("sidebar");
};
