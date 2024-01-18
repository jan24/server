"use strict";

window.onload = function() {
    var input = document.getElementById('enter_sn');
    input.focus();
    }


let tbody = document.getElementById("tstTable").firstElementChild;
var i;
for (i=1; i< tbody.childElementCount; i++) {
    var tr = tbody.children[i];
    var tst_res = tr.getElementsByClassName("tst_res")[0];
    var tst_fail = tr.getElementsByClassName("tst_fail")[0];
    var tst_area = tr.getElementsByClassName("tst_area")[0];
    if (tst_res.textContent == 'F') {
        tst_res.style.color = 'red';
        tst_fail.style.color = 'red';
        tst_area.style.color = 'red';
    } else if (tst_res.textContent == 'P') {
        tst_res.style.color = 'green';
    }
    else if (tst_res.textContent == 'S') {
        tr.style.color = '#bbb';
    }
}



//let show = document.getElementById("show");
//
//let enter_sn = document.getElementById("enter_sn");
//enter_sn.addEventListener("keyup", function(event) {
//    if (event.keyCode === 13) {
//        var sn = enter_sn.value;
//        alert(`you enter ${sn}`);
//        get_sn_data('all', `${sn}`);
//    }
//});
//
//let querybutton = document.getElementById("queryButton");
//function queryClick() {
//    var sn = enter_sn.value;
//    alert(`you enter ${sn}`);
//    get_sn_data('all', `${sn}`);
//}
//querybutton.onclick = queryClick;
//
//
//function success(text) {
//    var j = JSON.parse(text);
//
//    show.value = JSON.stringify(j, null, '  ');
//    var table = document.getElementById("recordth");
//    var i;
//    var tr = `<tr>
//            <th>test time(UTC)</th>
//            <th>sn</th>
//            <th>pid</th>
//            <th>pn</th>
//            <th>area</th>
//            <th>res</th>
//            <th>machine</th>
//            <th>container</th>
//            <th>fail item</th>
//            <th>fail msg</th>
//        </tr>`;
//    var _tr = '';
//    for (i=0; i < j.length; i++) {
//        _tr =`<tr>
//                <td style="width: 240px">${j[i][0]}</td>   <!-- time -->
//                <td   style="width: 120">${j[i][1]}</td>     <!-- sn -->
//                <td style="width: 140px">${j[i][2]}</td>   <!-- pid -->
//                <td style="width: 150px">${j[i][3]}</td>   <!-- pn -->
//                <td  style="width: 70px">${j[i][4]}</td>    <!-- area -->
//                <td  style="width: 15px; text-align:center">${j[i][5]}</td>  <!-- result -->
//                <td style="width: 110px">${j[i][6]}</td>   <!-- machine -->
//                <td style="width: 150px">${j[i][7]}</td>   <!-- cell -->
//                <td style="width: 180px">${j[i][8]}</td>   <!-- fail -->
//                <td style="width: 120px">${j[i][9]}</td>   <!-- fail detail -->
//            </tr>`;
//        tr = tr + _tr;
//    }
//    show.value = tr;
//    table.innerHTML = tr;
//
//}
//
//function fail(code) { }
//
//function get_sn_data(source, sn) {
//    var request = new XMLHttpRequest();
//    request.onreadystatechange = function () {
//        if (request.readyState === 4) {
//            if (request.status === 200) {
//                return success(request.responseText);
//            } else {
//                return fail(request.status);
//            }
//        }
//    }
//    request.open('GET', `/json/sn_data/?source=${source}&sn=${sn}`);
//    request.send();
//}
