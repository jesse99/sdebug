"use strict";

/* global makeRequest, format:true */

var SDEBUG = {}
SDEBUG.precision = 6;
SDEBUG.last_logged_time = -1.0;

window.onload = function()
{
	var widget = document.getElementsByName("run-until")[0];
	widget.addEventListener("click", () => {
		var input = document.getElementById("run-time");
		run_until("/time/" + input.value);
	});

	/* TODO: When we support log filtering we'll need to re-submit the call if the change was filtered out. */
	widget = document.getElementsByName("run-until-changed")[0];
	widget.addEventListener("click", () => {run_until("/run/until/log-changed");});

	widget = document.getElementById("run-time");
	widget.addEventListener("keyup", (event) => {
		event.preventDefault();
		if (event.keyCode == 13) {
			var button = document.getElementsByName("run-until")[0];
			button.click();
		}
	});

	get_precision();
	sync_ui();
};

function sync_ui()
{
	refresh_exited();
	refresh_header();
	refresh_sub_header();
	refresh_table();
}

function run_until(endpoint)
{
	makeRequest("POST", endpoint)
		.then(() => {			
			sync_ui();
		})
		.catch((err) => {
			console.error(err);
		});
}

function get_precision()
{
	makeRequest("GET", "/time/precision")
		.then((data) => {			
			SDEBUG.precision = data;
		})
		.catch((err) => {
			console.error(err);
		});
}

function refresh_exited()
{
	makeRequest("GET", "/exited")
		.then((data) => {			
			var widget = document.getElementsByName("run-until")[0];
			if (data) {
				widget.appendClass("is-static"); 

				widget = document.getElementById("run-time");
				widget.setAttribute("disabled", "disabled");

				widget = document.getElementsByName("run-until-changed")[0];
				widget.appendClass("is-static"); 
			} else {
				widget.removeClass("is-static"); 

				widget = document.getElementById("run-time");
				widget.removeAttribute("disabled");

				widget = document.getElementsByName("run-until-changed")[0];
				widget.removeClass("is-static"); 
			}
		})
		.catch((err) => {
			console.error(err);
		});
}

function refresh_header()
{
	makeRequest("GET", "/time")
		.then((data) => {			
			const mesg = format("Simulator @ {0}s", data.toFixed(SDEBUG.precision));
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

function refresh_sub_header()
{
	makeRequest("GET", "/state/*.display-title")
		.then((data) => {			
			var header = document.getElementById("sub-header");
			header.innerHTML = data[0][1];
		})
		.catch((err) => {
			var header = document.getElementById("sub-header");
			if (err.status == 400) {
				header.innerHTML = "";
			} else {
				console.error(err);
				header.innerHTML = "?";
			}
		});
}

function refresh_table()
{
	function append_row(time, path, level, message)
	{
		var body = document.getElementById("table-body");
		var row = body.insertRow(-1);
		row.appendClass(level); 
		
		var cell = row.insertCell(-1);
		cell.appendChild(document.createTextNode(time.toFixed(SDEBUG.precision)));
		cell.appendClass("leftmost"); 

		row.insertCell(-1).appendChild(document.createTextNode(level));
		row.insertCell(-1).appendChild(document.createTextNode(path));

		cell = row.insertCell(-1);
		cell.appendChild(document.createTextNode(message));
		cell.appendClass("rightmost"); 
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
