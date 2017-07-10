//! Misc useful functions.
use crest::error::Result;
use crest::prelude::*;
use serde;
use std::io;
use std::io::Write;
use std::process;

/// Writes message to stderr and exits.
pub fn fatal_err(message: &str) -> !
{
	let _ = writeln!(&mut io::stderr(), "{}", message);
	process::exit(1);
}

/// Deserializes the json from a REST GET. Path should be something
/// like &["log", "all"].
pub fn get_rest<T>(endpoint: &Endpoint, path: &[&str]) -> T
	where T: for <'de> serde::Deserialize<'de>
{
	match try_get_rest::<T>(endpoint, path) {
		Ok(result) => result,
		Err(err) => fatal_err(&format!("{:?}", err)),
	}
}

/// Returns the result which is usually "ok".
pub fn post_rest(endpoint: &Endpoint, path: &[&str]) -> String
{
	match try_post_rest(endpoint, path) {
		Ok(result) => result,
		Err(err) => fatal_err(&format!("{:?}", err)),
	}
}

fn try_get_rest<T>(endpoint: &Endpoint, path: &[&str]) -> Result<T>
	where T: for <'de> serde::Deserialize<'de>
{
	let request = try!(endpoint.get(path));
	let response = try!(request.send());
	let data = try!(response.into::<T>());
	Ok(data)
}

fn try_post_rest(endpoint: &Endpoint, path: &[&str]) -> Result<String>
{
	let request = try!(endpoint.post(path));
//	println!("request: {:?}", request);
	let response = try!(request.send());
//	println!("response: {:?}", response);
	let data = try!(response.into::<String>());
//	println!("data: {:?}", data);
	Ok(data)
}
