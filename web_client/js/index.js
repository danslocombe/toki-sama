"use strict";

import { TokiSamaSearch } from "../pkg/index.js"

const query_string = window.location.search;
const url_params = new URLSearchParams(query_string);
var query = url_params.get('q');
var toki_sama;
var textfield = document.getElementById("entry");
var resultsfield = document.getElementById("results");
var explanation = document.getElementById("explanation");


Promise.all(
    [
        fetch("pu.csv").then(x => x.text()),
        fetch("nimi_pu.txt").then(x => x.text()),
        fetch("compounds.txt").then(x => x.text()),
        fetch("generated_day2.tsv").then(x => x.text()),
    ]
)
.then(([pu, nimi_pu, compounds, model]) => {
    toki_sama = new TokiSamaSearch(pu, nimi_pu, compounds, model);
    console.log("Finished search init!");

    textfield.removeAttribute("disabled");
    textfield.setAttribute("placeholder", "teacher");
    textfield.focus();

    textfield.addEventListener('input', e => {
        const prefix = e.target.value;
        resultsfield.innerHTML = "";

        if (prefix.length > 0) {
            render(prefix, toki_sama.search(prefix));
            explanation.hidden = true;
        }
        else {
            textfield.setAttribute("placeholder", "");
            explanation.hidden = false;
        }
    })
})


function render(prefix, results_string) {
    const results = JSON.parse(results_string)

    if (results.length == 0) {
        return;
    }


    var card = document.createElement("ul");
    card.setAttribute("class", "card");

    // Create key
    let key_elem = document.createElement("li");
    key_elem.setAttribute("class", "card-item");

    let english_elem = document.createElement("span");
    english_elem.setAttribute("class", "item-english");
    english_elem.innerHTML = "English";

    let toki_elem = document.createElement("span");
    toki_elem.setAttribute("class", "item-toki-pona");
    toki_elem.innerHTML = "toki pona";

    key_elem.appendChild(english_elem);
    key_elem.appendChild(toki_elem);

    card.appendChild(key_elem);

    // Start rendering results

    for (let result of results) {
        let title = document.createElement("li");
        title.setAttribute("class", "card-item");

        let english_elem = document.createElement("span");
        english_elem.setAttribute("class", "item-english");

		let title_english = document.createElement("h3");
		title_english.setAttribute("class", "title");
		title_english.innerHTML = highlight_completion(prefix, result.english_search);
        english_elem.appendChild(title_english);

        let toki_elem = document.createElement("span");
        toki_elem.setAttribute("class", "item-english");

		let title_toki = document.createElement("h3");
		title_toki.setAttribute("class", "title");
		title_toki.innerHTML = result.original_translation_string;
        toki_elem.appendChild(title_toki);

        //let source = document.createElement("p")
		//source.setAttribute("class", "title");
		//source.innerHTML = " (" + result.source + ")";
        //toki_elem.appendChild(source);

        title.appendChild(english_elem);
        title.appendChild(toki_elem);

        card.appendChild(title);

        for (let similar of result.similar) {
            let similar_elem = document.createElement("li");
            similar_elem.setAttribute("class", "card-item");

            let english_elem = document.createElement("span");
            english_elem.setAttribute("class", "item-english");
            english_elem.innerHTML = similar.english;

            let toki_elem = document.createElement("span");
            toki_elem.setAttribute("class", "item-toki-pona");
            toki_elem.innerHTML = similar.toki_pona_string;

            similar_elem.appendChild(english_elem);
            similar_elem.appendChild(toki_elem);

            card.appendChild(similar_elem);
        }
    }

    resultsfield.appendChild(card);
}

function highlight_completion(prefix, full) {
    let res = prefix;
    const completion =full.substring(prefix.length);
    res += "<b>";
    res += completion;
    res += "</b>";

    return res;
}