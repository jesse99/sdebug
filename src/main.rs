//! Command line score debugger allowing you to inspect state and move
//! time both forward and backward.
#[macro_use]
extern crate clap;

#[macro_use]
extern crate serde_derive;

extern crate crest;
extern crate rustyline;
extern crate serde;

mod helpers;
mod input;
mod log;
mod parse;
mod time;

use clap::{App, ArgMatches};
use crest::prelude::*;
use helpers::*;
use input::*;
use std::fmt::Display;
use std::str::FromStr;
use std::usize;
use time::*;

pub struct Config
{
	server: String,
	port: i32,
	precision: usize,
	colorize: bool,
}

impl Config
{
	fn new() -> Config
	{
		// These are the defaults: all of them can be overriden using command line options.
		Config {
			server: "127.0.0.1".to_string(),
			port: 9000,
			precision: usize::MAX,
			colorize: true,
		}
	}
}

// Min and max are inclusive.
fn match_num<T>(matches: &ArgMatches, name: &str, min: T, max: T) -> T
		where T: Copy + Display + FromStr + PartialOrd
{
	match value_t!(matches.value_of(name), T) {
		Ok(value) if value < min => fatal_err(&format!("--{} should be greater than {}", name, min)),
		Ok(value) if value > max => fatal_err(&format!("--{} should be less than {}", name, max)),
		Ok(value) => value,
		_ => fatal_err(&format!("--{} should be a number", name)),
	}
}

fn parse_options() -> Config
{
	let mut config = Config::new();
	
	// see https://docs.rs/clap/2.24.2/clap/struct.Arg.html#method.from_usage for syntax
	let usage = format!(
		"--no-colors 'Don't color code output'
		--port=[NUM] 'Port the score simulation is bound to [{default_port}]'
		--precision=[NUM] 'Time decimal places [score's precision]'
		--server=[ADDRESS] 'Address the score simulation is bound to [{default_server}]'",
		default_server = config.server,
		default_port = config.port);
	
	let matches = App::new("sdebug")
		.version("0.1.0")
		.author("Jesse Jones <jesse9jones@gmail.com>")
		.about("score debugger.")
		.args_from_usage(&usage)
	.get_matches();
		
	if matches.is_present("no-colors") {
		config.colorize = false;
	}
	if matches.is_present("server") {
		config.server = matches.value_of("server").unwrap().to_string();
	}
	if matches.is_present("port") {
		config.port = match_num(&matches, "port", 1, 65535) as i32;
	}
	if matches.is_present("precision") {
		config.precision = match_num(&matches, "precision", 0, 32) as usize;
	}
		
	config
}

fn startup(mut config: Config, endpoint: Endpoint)
{
	if config.precision == usize::MAX {
		config.precision = get_time_precision(&endpoint);
	} else {
		get_time(&endpoint);	// make sure that we're connected to score
	}
	read_lines(config, endpoint);
}

fn main()
{
	let config = parse_options();
	let url = format!("http://{}:{}/", config.server, config.port);
	match Endpoint::new(&url) {
		Ok(endpoint) => startup(config, endpoint),
		Err(err) => println!("error connecting to {}: {}", url, err),
	}
}
