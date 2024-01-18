"use strict";

// find <th>, make it be click-able
function makeSortable(table) {
    var headers=table.getElementsByTagName("th");
    for(var i=0;i<headers.length;i++){
        (function(n){
            var flag=false;
            headers[n].onclick=function(){
                // sort rows by (table, n)
                var tbody=table.tBodies[0];// first ele, <tbody>
                var rows=tbody.getElementsByTagName("tr");//all rows of tbody
                rows=Array.prototype.slice.call(rows, 1);// snapshot of all row

                // sort by <td>[n]
                rows.sort(function(row1,row2){
                    var cell1=row1.getElementsByTagName("td")[n];
                    var cell2=row2.getElementsByTagName("td")[n];
                    var val1= cell1.textContent||cell1.innerText;
                    var val2= cell2.textContent||cell2.innerText;
                    if(val1 < val2){
                        return -1;
                    }else if(val1 > val2){
                        return 1;
                    }else{
                        return 0;
                    }
                });
                if(flag){
                    rows.reverse();
                }
                //add to end of tbody
                //not need delete
                //other things will pop to top if it is not <tr>
                for(var i=0; i<rows.length; i++){
                    tbody.appendChild(rows[i]);
                }
                flag=!flag;
            }
        }(i));
    }
}

window.onload = function(){
    var table = document.getElementById("tstTable");
    makeSortable(table);

    var table2 = document.getElementById("tstTable2");
    if (table2) {
        makeSortable(table2);
    }

    var table3 = document.getElementById("tstTable3");
    makeSortable(table);
        if (table3) {
        makeSortable(table3);
    }
}
