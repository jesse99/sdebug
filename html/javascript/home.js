"use strict";

/* global makeRequest, format:true */

var SDEBUG = {}
SDEBUG.precision = 6;
SDEBUG.exited = false;

window.onload = function()
{
	var run_until = document.getElementsByName("run-until")[0];
	run_until.addEventListener("click", on_run_until);

	set_precision();
	refresh_header();
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
