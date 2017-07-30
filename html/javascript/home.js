"use strict";

/* global makeRequest, format:true */

var SDEBUG = {}
SDEBUG.precision = 6;
SDEBUG.last_logged_time = -1.0;
SDEBUG.exited = false;

window.onload = function()
{
	var run_until = document.getElementsByName("run-until")[0];
	run_until.addEventListener("click", on_run_until);

	set_precision();
	refresh_header();
	refresh_table();
};

function on_run_until()
{
	var input = document.getElementById("run-time");
	makeRequest("POST", "/time/" + input.value)
		.then((data) => {			
			if (data === "exited") {
				SDEBUG.exited = true;
			}
			refresh_header();
			refresh_table();
		})
		.catch((err) => {
			console.error(err);
		});
}

function set_precision()
{
	makeRequest("GET", "/time/precision")
		.then((data) => {			
			SDEBUG.precision = data;
		})
		.catch((err) => {
			console.error(err);
		});
}

function refresh_header()
{
	makeRequest("GET", "/time")
		.then((data) => {			
			var mesg = format("Simulator @ {0}s", data.toFixed(SDEBUG.precision));
			if (SDEBUG.exited) {
				mesg += " (exited)";
			}
			var header = document.getElementById("header");
			header.innerHTML = mesg;

			var input = document.getElementById("run-time");
			input.value = data;
		})
		.catch((err) => {
			console.error(err);

			var header = document.getElementById("header");
			header.innerHTML = "Simulator @ ?s";
		});
}

function refresh_table()
{
	function append_row(time, path, level, message)
	{
		var body = document.getElementById("table-body");
		var row = body.insertRow(-1);
		row.setAttribute("class", level); 
		
		var cell = row.insertCell(-1);
		cell.appendChild(document.createTextNode(time.toFixed(SDEBUG.precision)));
		cell.setAttribute("class", "leftmost"); 

		row.insertCell(-1).appendChild(document.createTextNode(level));
		row.insertCell(-1).appendChild(document.createTextNode(path));

		cell = row.insertCell(-1);
		cell.appendChild(document.createTextNode(message));
		cell.setAttribute("class", "rightmost"); 
	}

	makeRequest("GET", "/log/after/" + SDEBUG.last_logged_time)
		.then((data) => {	
			for (var row of data) {		
				append_row(row.time, row.path, row.level, row.message);
				SDEBUG.last_logged_time = row.time;
			}
		})
		.catch((err) => {
			console.error(err);
			append_row(SDEBUG.last_logged_time, "?", "Error", "AJAX failed");
		});
}
