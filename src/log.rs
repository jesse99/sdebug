//! REST commands related to logging.
use *;
use crest::prelude::*;
use helpers::*;

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
	if config.colorize {
		let begin_color = match line.level.as_ref() {
			"Error" => "\x1b[31;1m",			// bright red
			"Warning" => "\x1b[31m",			// red
			"Info" => "\x1b[30;1m",				// bold black
			"Debug" => "",
			"Excessive" => "\x1b[1;38;5;244m",	// light grey
			_ => {assert!(false); "xxx"},
		};
		let end_color = "\x1b[0m";
		print!("{}{}  {}  {}{}\n", begin_color, time, line.path, line.message, end_color);
	} else {
		print!("{}  {:5.5} {}  {}\n", time, line.level, line.path, line.message);
	}
}

