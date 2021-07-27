"use strict";

import { TokiSamaSearch } from "../pkg/index.js"

const query_string = window.location.search;
const url_params = new URLSearchParams(query_string);
var query = url_params.get('q');
var toki_sama;
var textfield = document.getElementById("entry");
var resultsfield = document.getElementById("results");


Promise.all(
    [
        fetch("/pu.csv").then(x => x.text()),
        fetch("/nimi_pu.txt").then(x => x.text()),
        fetch("/compounds.txt").then(x => x.text()),
        fetch("/generated_day2.tsv").then(x => x.text()),
    ]
)
.then(([pu, nimi_pu, compounds, model]) => {
    toki_sama = new TokiSamaSearch(pu, nimi_pu, compounds, model);
    console.log("Finished search init!");

    textfield.removeAttribute("disabled");
    textfield.setAttribute("placeholder", "teacher");
    textfield.focus();

    textfield.addEventListener('input', e => {
        let res = render(toki_sama.search(e.target.value));
    })
})


function render(results) {
    console.log(results);
    resultsfield.innerHTML = results;
}