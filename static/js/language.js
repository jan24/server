"use strict";

let nav_dropdown = document.getElementById('language-nav');
let before_lang = nav_dropdown.getAttribute("data-current-lang");  // before onchange
nav_dropdown.value = before_lang;

function langChange() {
    let lang = this.value;  // after onchange
    if (lang == "en-US") {
        lang = "";
    } else {
        lang = `/${lang}`;
    }
    let path = `/${window.location.href.split('/').slice(3).join('/')}`;
    if (path.startsWith("/en-US/") || before_lang != "en-US") {
        path = `/${window.location.href.split('/').slice(4).join('/')}`;
    }

    window.location = `${lang}${path}`;
}

nav_dropdown.onchange = langChange;
