"use strict";

/* global makeRequest, format:true */

var SDEBUG = {}
SDEBUG.precision = 6;
SDEBUG.last_logged_time = -1.0;
SDEBUG.tab_name = "log";
SDEBUG.old_state = {};

window.onload = function()
{
	var widget = document.getElementsByName("run-until")[0];
	widget.addEventListener("click", () => {
		var input = document.getElementById("run-time");
		run_until("/time/" + input.value);
	});

	/* TODO: When we support log filtering we'll need to re-submit the call if the change was filtered out. */
	widget = document.getElementsByName("run-until-changed")[0];
	widget.addEventListener("click", () => {run_until("/run/until/" + SDEBUG.tab_name + "-changed");});

	widget = document.getElementById("run-time");
	widget.addEventListener("keyup", (event) => {
		event.preventDefault();
		if (event.keyCode == 13) {
			var button = document.getElementsByName("run-until")[0];
			button.click();
		}
	});

	widget = document.getElementById("map-tab");
	widget.addEventListener("click", () => {deselect_tabs("log", "state", "components"); select_tab("map");});

	widget = document.getElementById("log-tab");
	widget.addEventListener("click", () => {deselect_tabs("map", "state", "components"); select_tab("log");});

	widget = document.getElementById("state-tab");
	widget.addEventListener("click", () => {deselect_tabs("map", "log", "components"); select_tab("state");});

	widget = document.getElementById("components-tab");
	widget.addEventListener("click", () => {deselect_tabs("map", "log", "state"); select_tab("components");});

	widget = document.getElementById("show-display-state");
	widget.addEventListener("click", () => {SDEBUG.old_state = {}; refresh_states();});

	deselect_tabs("map", "state", "components");
	get_precision();
	sync_ui();
};

function deselect_tabs(...names)
{
	for (var name of names) {
		var tab = document.getElementById(name + "-tab");
		tab.removeClass("is-active")

		var view = document.getElementById(name + "-view");
		view.style.display = "none";

		if (name == "state") {
			view = document.getElementById("show-display-view");
			view.style.display = "none";
		}
	}
}

function select_tab(name)
{
	SDEBUG.tab_name = name;

	var tab = document.getElementById(name + "-tab");
	tab.appendClass("is-active")

	var view = document.getElementById(name + "-view");
	if (name === "log" || name === "state") {
		view.style.display = "table";
	} else {
		view.style.display = "inline";
	}

	if (name == "state") {
		view = document.getElementById("show-display-view");
		view.style.display = "block";
	}
}

function sync_ui()
{
	refresh_exited();
	refresh_header();
	refresh_sub_header();

	refresh_logs();
	refresh_states();
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

function refresh_logs()
{
	function append_row(time, path, level, message)
	{
		var body = document.getElementById("log-body");
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

function refresh_states()
{
	function append_row(path, value, klass)
	{
		var body = document.getElementById("state-body");
		var row = body.insertRow(-1);
		
		var cell = row.insertCell(-1);
		cell.appendChild(document.createTextNode(path));
		cell.appendClass("leftmost"); 
		if (klass === "removed") {
			cell.appendClass("removed"); 
		}

		cell = row.insertCell(-1);
		cell.appendChild(document.createTextNode(value));
		cell.appendClass("rightmost"); 

		if (klass !== "") {
			cell.appendClass(klass); 
		}
	}

	makeRequest("GET", "/state/*")
		.then((state) => {	
			var body = document.getElementById("state-body");
			while (body.rows.length > 0) {
				body.deleteRow(-1);
			}

			var new_state = {};
			var updated = false;
			var path = "";
			for (var row of state) {
				path = row[0];
				var value = row[1];
				if (show_state(path)) {
					var klass = "";
					if (!(path in SDEBUG.old_state))
						klass = "added";
					else if (SDEBUG.old_state[path] !== value)
						klass = "changed";
					append_row(path, value, klass);
					new_state[path] = value;

					if (klass !== "")
						updated = true;
				}
			}

			for (path in SDEBUG.old_state)
			{
				if (!(path in new_state)) {
					append_row(path, SDEBUG.old_state[path], "removed");
					updated = true;
				}
			}

			SDEBUG.old_state = new_state;

			if (!updated && SDEBUG.tab_name == "state")
				run_until("/run/until/state-changed");
		})
		.catch((err) => {
			console.error(err);
			append_row("?", "AJAX failed");
		});
}

function show_state(path)
{
	var checkbox = document.getElementById("show-display-state");
	return checkbox.checked || !path.includes(".display-");
}