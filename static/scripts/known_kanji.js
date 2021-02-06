function kanji_grid() {
    // Reset the grid
    $("#kanji").empty();
    $("#remove").hide();

    // Show the number of kanji added
    let known_kanji = localStorage.getItem("known_kanji") || "";
    $("#num_known").text(known_kanji.length);

    // Fill the kanji grid
    for (let i = 0; i < known_kanji.length; i++) {
        $("#kanji").append(`<div class="selectable">${known_kanji[i]}</div>`);
    }

    const ds = new DragSelect({
        area: document.getElementById("kanji"),
        selectables: document.getElementsByClassName("selectable"),
        draggability: false,
        immediateDrag: false,
        dragKeys: { "up": [], "right": [], "down": [], "left": [] },
        selectedClass: "selected",
    });

    ds.subscribe("callback", ({ items, _ }) => {
        if (items.length) {
            $("#remove").show();
        } else {
            $("#remove").hide();
        }
    });
}

$(document).ready(kanji_grid);

// Add kanji
$("form").submit(e => {
    e.preventDefault();
    let known_kanji = new Set(localStorage.getItem("known_kanji"));
    // Regex to identify kanji
    let re = /[\u3400-\u4DB5\u4E00-\u9FCB\uF900-\uFA6A]/ug;
    for (let kanji of $("#new_kanji").val().matchAll(re)) {
        known_kanji.add(kanji[0]);
    }
    // Save updated kanji list to localStorage
    localStorage.setItem("known_kanji", [...known_kanji].join(""));
    // Reset the input field
    $("#new_kanji").val("");
    // Update kanji grid
    kanji_grid();
});

// Remove kanji
$("#remove").click(() => {
    // TODO confirmation screen
    let known_kanji = new Set(localStorage.getItem("known_kanji"));
    $("#kanji div.selected").each(function () {
        known_kanji.delete($(this).text());
    });
    // Save updated kanji list to localStorage
    localStorage.setItem("known_kanji", [...known_kanji].join(""));
    // Update kanji grid
    kanji_grid();
});
