"use strict";

/* global makeRequest, format:true */

var SDEBUG = {}
SDEBUG.precision = 6;
SDEBUG.exited = false;

window.onload = function()
{
	var run_until = document.getElementsByName("run-until")[0];
	run_until.addEventListener("click", on_run_until);

	start_set_precision();
	start_refresh_header();
};

function on_run_until()
{
	var input = document.getElementById("run-time");
	
	var request = new XMLHttpRequest();
	request.addEventListener("load", finish_run_until);
	request.addEventListener("error", on_ajax_error);
	request.open("POST", "/time/" + input.value);
	request.send();
}

function start_set_precision()
{
	var request = new XMLHttpRequest();
	request.addEventListener("load", finish_set_precision);
	request.addEventListener("error", on_ajax_error);
	request.open("GET", "/time/precision");
	request.send();
}

function start_refresh_header()
{
	makeRequest("GET", "/time")
		.then((data) => {finish_refresh_header(data);})
		.catch((err) => {console.log("fetch failed with ", err);});


//	var request = new XMLHttpRequest();
//	request.addEventListener("load", finish_refresh_header);
//	request.addEventListener("error", on_ajax_error);
//	request.open("GET", "/time");
//	request.send();
}

function finish_run_until(progress)
{
	var json = JSON.parse(this.responseText);
	if (json == "exited") {
		SDEBUG.exited = true;
	}
	start_refresh_header();
}

function finish_set_precision(progress)
{
	SDEBUG.precision = JSON.parse(this.responseText);
}

function finish_refresh_header(data)
{
	var header = document.getElementById("header");
	
	var mesg = format("Simulator @ {0}s", data.toFixed(SDEBUG.precision));
	if (SDEBUG.exited) {
		mesg += " (exited)";
	}
	header.innerHTML = mesg;

	var input = document.getElementById("run-time");
	input.value = data;
}

function on_ajax_error(progress)
{
	if (window.console) {
		console.log("ajax failed with ", this.statusText);
	}
}
