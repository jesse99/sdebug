"use strict";

var SDEBUG = {}
SDEBUG.precision = 6;

window.onload = function()
{
	set_precision();
	refresh_header();
};

function set_precision()
{
	var request = new XMLHttpRequest();
	request.addEventListener("load", on_set_precision);
	request.addEventListener("error", on_ajax_error);
	request.open("GET", "/time/precision");
	request.send();
}

function refresh_header()
{
	var request = new XMLHttpRequest();
	request.addEventListener("load", on_refresh_header);
	request.addEventListener("error", on_ajax_error);
	request.open("GET", "/time");
	request.send();
}

function on_set_precision(progress)
{
	SDEBUG.precision = JSON.parse(this.responseText);
}

function on_refresh_header(progress)
{
	var json = JSON.parse(this.responseText);
	var header = document.getElementById("header");
	header.innerHTML = "Simulator @ " + json.toFixed(SDEBUG.precision) + "s";
	console.log("time = ", json);
}

function on_ajax_error(progress)
{
	if (window.console) {
		console.log("ajax failed with ", this.statusText);
	}
}
