"use strict";

/* global SVG, makeRequest, format:true */

var SDEBUG = {}
SDEBUG.precision = 6;
SDEBUG.last_logged_time = -1.0;
SDEBUG.draw = null;

SDEBUG.tab_name = "log";
SDEBUG.get_current_state = get_current_log;			// () => promise
SDEBUG.current_state_has_changed = log_has_changed;	// (current_state) => bool
SDEBUG.apply_current_state = apply_log;				// (current_state) => void
SDEBUG.old_state = {};

window.onload = function()
{
	init_map();

	var widget = document.getElementsByName("run-until")[0];
	widget.addEventListener("click", () => {
		var input = document.getElementById("run-time");
		run_until(input.value);
	});

	widget = document.getElementsByName("run-until-changed")[0];
	widget.addEventListener("click", () => {run_until_changed();});

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
	widget.addEventListener("click", () => {SDEBUG.old_state = []; update_tab_view(null);});

	deselect_tabs("map", "state", "components");
	initialize_log();
	get_precision();
	sync_ui(null);
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
	if (name == "components") {
		console.error("need to set callbacks");
		view.style.display = "inline";

	} else if (name == "log") {
		SDEBUG.get_current_state = get_current_log;
		SDEBUG.current_state_has_changed = log_has_changed;
		SDEBUG.apply_current_state = apply_log;
		initialize_log();
		view.style.display = "table";

	} else if (name == "map") {
		SDEBUG.get_current_state = get_current_map;
		SDEBUG.current_state_has_changed = state_has_changed;
		SDEBUG.apply_current_state = apply_map;
		initialize_map();
		view.style.display = "block";

	} else if (name == "state") {
		SDEBUG.get_current_state = get_current_state;
		SDEBUG.current_state_has_changed = state_has_changed;
		SDEBUG.apply_current_state = apply_state;
		initialize_state();
		view = document.getElementById("show-display-view");
		view.style.display = "block";

	} else {
		console.error("bad name: " + name)
	}

	update_tab_view(null);
}

function sync_ui(on_changed)
{
	refresh_exited();
	refresh_header();
	refresh_sub_header();
	return update_tab_view(on_changed);
}

function run_until(time)
{
	makeRequest("POST", "/run/until/" + time)
		.then(() => {			
			sync_ui(null);
		})
		.catch((err) => {
			console.error(err);
		});
}

function run_until_changed()
{
	makeRequest("POST", "/run/once")
		.then((data) => {			
			sync_ui((changed) => {
				if (data != "exited" && !changed)
					run_until_changed();
			});
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

function update_tab_view(on_changed)
{
	SDEBUG.get_current_state()
		.then((new_state) => {			
			const changed = SDEBUG.current_state_has_changed(new_state);
			if (changed) {
				SDEBUG.apply_current_state(new_state);
				SDEBUG.old_state = new_state;
			}
			if (on_changed)
				on_changed(changed);
		})
		.catch((err) => {
			console.error(err);
			if (on_changed)
				on_changed(true);		// true so that we don't keep repeating something that isn't working
		});	
}

// ---- Log -----------------------------------------------------------------------------
// For logs old and new state will be [time, path, level, message].
function initialize_log()
{
	SDEBUG.old_state = [];
	SDEBUG.last_logged_time = -1.0;
}

function get_current_log()
{
	if (SDEBUG.last_logged_time < 0.0)
		return makeRequest("GET", "/log");
	else
		return makeRequest("GET", "/log/after/" + SDEBUG.last_logged_time);
}

function log_has_changed(new_state)
{
	return new_state.length > 0;
}

function apply_log(new_state)
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

	var last = SDEBUG.last_logged_time;
	for (var row of new_state) {
		if (row.time > SDEBUG.last_logged_time) {	
			append_row(row.time, row.path, row.level, row.message);
			last = row.time;
		}
	}
	SDEBUG.last_logged_time = last;
}

// ---- State ---------------------------------------------------------------------------
// For state old and new state will be [(path, value, kind)] where kind is "int", "float", or "string"
// and the rows are sorted by path.
function initialize_state()
{
	SDEBUG.old_state = [];
}

function get_current_state()
{
	return makeRequest("GET", "/state/**");
}

function state_has_changed(new_state)
{
	if (new_state.length != SDEBUG.old_state.length)
		return true;

	for (var i = 0; i < new_state.length; i++) {
		if (new_state[i][0] !== SDEBUG.old_state[i][0])
			return true;
		else if (new_state[i][1] !== SDEBUG.old_state[i][1])
			return true;
	}

	return false;
}

function apply_state(new_state)
{
	function append_row(path, value, klass, kind)
	{
		var body = document.getElementById("state-body");
		var row = body.insertRow(-1);

		var parts = path.split(".");
		var key = parts.pop();
		var component = parts.join(".");
		
		var cell = row.insertCell(-1);
		cell.appendChild(document.createTextNode(component));
		cell.appendClass("leftmost"); 
		if (klass === "removed")
			cell.appendClass("removed"); 

		cell = row.insertCell(-1);
		cell.appendChild(document.createTextNode(key));

		cell = row.insertCell(-1);
		cell.appendChild(document.createTextNode(value));
		cell.appendClass("rightmost"); 

		if (klass !== "")
			cell.appendClass(klass); 
		if (klass !== "removed")
			cell.addEventListener("click", () => {edit_value(path, kind, value);});
	}

	var body = document.getElementById("state-body");
	while (body.rows.length > 0) {
		body.deleteRow(-1);
	}

	var row = [];
	var old_values = {};	// create some temporary maps so that we can do lookups
	for (row of SDEBUG.old_state) {
		old_values[row[0]] = row[1];
	}

	var path = "";
	var new_values = {};
	for (row of new_state) {
		path = row[0];
		var value = row[1];
		var kind = row[2];
		if (show_state(path)) {
			var klass = "";
			if (!(path in old_values))
				klass = "added";
			else if (old_values[path] !== value)
				klass = "changed";
			append_row(path, value, klass, kind);
		}
		new_values[path] = value;
	}

	for (path in old_values)
	{
		if (!(path in new_values)) {
			if (show_state(path))
				append_row(path, old_values[path], "removed");
		}
	}
}

function edit_value(path, kind, old)
{
	var value = prompt("New value: ", old);
	if (value !== null) {
		makeRequest("POST", format("/state/{0}/{1}/{2}", kind, path, value))
			.then(() => {			
				update_tab_view(null);
			})
			.catch((err) => {
				alert(format("POST failed: {0}", err));
			});
	}
}

function show_state(path)
{
	var checkbox = document.getElementById("show-display-state");
	return checkbox.checked || !path.includes(".display-");
}

// ---- Map -----------------------------------------------------------------------------
function init_map()
{
	makeRequest("GET", "/state/*.display-size-x")
		.then((data) => {			
			const width = data[0][1];
			makeRequest("GET", "/state/*.display-size-y")
				.then((data2) => {			
					const height = data2[0][1];
					SDEBUG.draw = SVG("map-view").size("100%", 1000);
					SDEBUG.draw.viewbox(0, 0, 1.1*width, 1.1*height);	// we add a bit extra so components at the edges are still visible
				});
		})
		.catch((err) => {
			// TODO: disable the map tab?
			console.error("init_map failed: " + err)
		});
}

// For map old and new state will be [(path, value, kind)] where kind is "int", "float", or "string"
// and the rows are sorted by path.
function initialize_map()
{
	SDEBUG.old_state = [];
	SDEBUG.draw.clear();
}

function get_current_map()
{
	return makeRequest("GET", "/state/*.display-*");
}

function apply_map(new_state)
{
	function draw_component(state, component_name)
	{
		// TODO: If there is a display-icon value then use that instead of display-name.
		const x = state[component_name + ".display-location-x"];
		const y = state[component_name + ".display-location-y"];

		if (x && y && SDEBUG.draw) {
			var name = state[component_name + ".display-name"];
			if (name) {
				var details = state[component_name + ".display-details"];
				var color = state[component_name + ".display-color"];
				if (!color)
					color = "black";

				SDEBUG.draw.text((add) => {
					add.move(x, y).tspan(name).fill(color).font({"anchor": "middle", "size": 4})
					.tspan(details).move(x, y).dy(1).fill(color).font({"anchor": "middle", "size": 1});
				});
			}
		}
	}

	SDEBUG.draw.clear();

	var names = new Set();
	var state = {};
	var name = "";
	for (var row of new_state) {
		var path = row[0];
		var value = row[1];
		var parts = path.split(".");
		name = parts[0];
		names.add(name);
		state[path] = value;
	}

	for (name of names)
	{
		draw_component(state, name)
	}
}
