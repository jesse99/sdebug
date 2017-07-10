//! REST commands related to the simulation store.
use crest::prelude::*;
use helpers::*;

pub fn print_state(endpoint: &Endpoint, path: &str)
{
	let lines = get_state(endpoint, path);
	for line in lines.iter() {
		println!("{}", line);
	}
}

fn get_state(endpoint: &Endpoint, path: &str) -> Vec<String>
{
	get_rest::<Vec<String>>(endpoint, &["state", path])
}
