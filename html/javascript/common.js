// Misc utility functions.
"use strict";

/* exported makeRequest */

// Replaces {0} with argument 0, {1} with argument 1, etc.
// Argument index can be appended with ":j" to print the argument as json.
function format(pattern, ...args) // TODO: replace this with template literals (once vs code's javascript extension doesn't freak out)
{
	return pattern.replace(/{(\d+?)(:j)?}/g,
		(match, number, json) =>
		{
			if (typeof args[number] === "undefined")
				return "undefined";
			else if (json)
				return JSON.stringify(args[number]);
			else
				return args[number];
		}
	);
}

// Append a new class attribute onto the element.
Element.prototype.appendClass = function(name)
{
	var old = this.getAttribute("class");
	if (!old) {
		old = "";
	}
	var parts = old.split(/\s+/);
	var index = parts.indexOf(name);
	if (index < 0) {
		parts.push(name);
		this.setAttribute("class", parts.join(" ")); 
	}
}

// Remove an existing class attribute from the element.
Element.prototype.removeClass = function(name)
{
	var old = this.getAttribute("class");
	if (!old) {
		old = "";
	}
	var parts = old.split(/\s+/);
	var index = parts.indexOf(name);
	if (index >= 0) {
		parts.splice(index, 1);
	}
	this.setAttribute("class", parts.join(" ")); 
}

// Make an AJAX call and either call resolved with json from the response text or reject with an Error.
function makeRequest (method, url) {
	return new Promise((resolved, reject) => {
		var xhr = new XMLHttpRequest();
		xhr.open(method, url);
		xhr.onload = () => {
			if (xhr.status >= 200 && xhr.status <= 299) {
				try {
					const data = JSON.parse(xhr.responseText);
					resolved(data);
				} catch (exception) {
					reject(new Error(format("{0} {1} result ({2}) didn't parse as json", method, url, xhr.responseText)));
				}
			} else {
				reject(new Error(format("{0} {1} failed with {2} ({3})", method, url, xhr.statusText, xhr.status)));
			}
		};
		xhr.onerror = () => {
			reject(new Error(format("{0} {1} failed with {2} ({3})", method, url, xhr.statusText, xhr.status)));
		};
		xhr.send();
	});
}
