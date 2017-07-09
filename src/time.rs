//! REST commands related to time.
use crest::error::Result;
use crest::prelude::*;
use serde;
use std::process;

// TODO: comment
pub fn get_time_label(endpoint: &Endpoint) -> String
{
	match get_rest::<String>(endpoint, &["time", "label"]) {
		Ok(result) => result,
		Err(err) => {println!("{}", err); process::exit(1);}
	}
}

pub fn get_rest<T>(endpoint: &Endpoint, path: &[&str]) -> Result<T>
	where T: for <'de> serde::Deserialize<'de>
{
	let request = try!(endpoint.get(path));
	let response = try!(request.send());
	let data = try!(response.into::<T>());
	Ok(data)
}
