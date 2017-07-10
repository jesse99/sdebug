//! REST commands related to logging.
use *;
use crest::prelude::*;
use helpers::*;
//use ::serde::{Deserialize, Serialize};
//use serde::de::{Deserialize, Deserializer};

pub fn print_log(config: &Config, endpoint: &Endpoint, path: &str, limit: u64, level: &str)
{
	let lines = get_log(endpoint, path, limit, level);
	for line in lines.iter() {
		print_line(config, line);
	}
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LogLine
{
	time: f64,
	path: String,
	level: String,
	message: String,
}

// Returns simulation log lines.
fn get_log(endpoint: &Endpoint, path: &str, limit: u64, level: &str) -> Vec<LogLine>
{
	let limit = format!("{}", limit);
	get_rest::<Vec<LogLine>>(endpoint, &["log", path, &limit, level])
}

fn print_line(config: &Config, line: &LogLine)
{
	let time = format!("{:.1$}", line.time, config.precision);
	print!("{}  {:.5} {}  {}\n", time, line.level, line.path, line.message);
}

