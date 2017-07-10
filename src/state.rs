//! REST commands related to the simulation store.
use crest::prelude::*;
use helpers::*;

pub fn print_state(endpoint: &Endpoint, path: &str)
{
	let lines = get_state(endpoint, path);
	
	let mut component = "";
	for line in lines.iter() {
		let path = line.split(" = ").next().unwrap();
		let (stem, _) = path.split_at(path.rfind('.').unwrap());
		if stem != component {
			if !component.is_empty() {
				println!("");
			}
			component = stem;
		}
		println!("{}", line);
	}
}

pub fn set_state(endpoint: &Endpoint, path: &str, kind: &str, value: &str)
{
	post_rest(endpoint, &["state", kind, path, value]);
}

fn get_state(endpoint: &Endpoint, path: &str) -> Vec<String>
{
	get_rest::<Vec<String>>(endpoint, &["state", path])
}
