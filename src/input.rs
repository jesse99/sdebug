//! Use linenoise (aka readline) to allow the user to enter in and edit
//! a command line.
use *;
use crest::prelude::*;
use log::*;
use parse::*;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::env;
use std::u64;
use state::*;
use time::*;
//use std::str::FromStr;

type Handler = fn (&Config, &Endpoint, &Vec<String>) -> ();

type Commands = Vec<(Vec<String>, Handler, &'static str)>;

/// Use linenoise to read in lines typed by the user and process them.
pub fn read_lines(config: Config, endpoint: Endpoint)
{
	let hpath = get_history_path();
	let commands = init_commands();
	
	// TODO: instead of () install a completion fn
	let mut editor = Editor::<()>::new();
	if let Err(err) = editor.load_history(&hpath) {
		println!("Error loading history: {}", err);
	}
	loop {
		let prompt = get_prompt(&config, &endpoint);
		let readline = editor.readline(&prompt);
		match readline {
			Ok(line) => {
				match line.as_ref() {
					"quit" | "exit" | "q" => {	// if more hard-coded commands are added then also update print_help
						break;
					},
					"help" | "?" => {
						print_help(&commands);
					},
					_ => {
						editor.add_history_entry(&line);	// even if there is an error, it's nice to have it in the history so that the user can easily repair it
						match tokenize(&line) {
							Ok(args) => process_line(&config, &endpoint, &args, &commands),
							Err(s) => println!("{}", s),
						}
					}
				}
			},
			Err(ReadlineError::Eof) => {	// this is control-D
				break
			},
//			Err(ReadlineError::Interrupted) => {	// this is control-C
//				break
//			},
			Err(err) => {
				println!("Error: {:?}", err);
				break
			}
		}
	}
	editor.save_history(&hpath).unwrap();
}

fn process_line(config: &Config, endpoint: &Endpoint, args: &Vec<String>, commands: &Commands)
{
	if args.len() > 0 {
		if let Some(handler) = find_handler(&args, commands) {
			handler(config, endpoint, &args);
			
		} else {
			// We failed to find a command that matched what the user typed
			// so check for some common error cases.
			let mut matches = find_longer_commands(&args, commands);	// user typed: get
			if matches.is_empty() {
				matches = find_shorter_commands(&args, commands);		// user typed: get log all hmm
			}
			if matches.is_empty() && args.len() > 1 {
				if let Some((_, prefix)) = args.split_last() {			// user typed: get log oops
					matches = find_longer_commands(&prefix.to_vec(), commands);
				}
			}
			if matches.len() > 0 {
				println!("Did you mean:"); 	// TODO: use red?
				for m in matches.iter() {
					println!("   {}", m);
				}
			} else {
				print!("Failed to find a matching command.\n");
			}
		}
	}
}

// Return a handler if args exactly matches a command.
fn find_handler(args: &Vec<String>, commands: &Commands) -> Option<Handler>
{
	let handlers: Vec<Handler> = commands.iter()
		.filter(|command| args_matches(args, &command.0) && no_extra_args(args, &command.0))
		.map(|command| command.1)
		.collect();
	if handlers.len() == 1 {
		Some(handlers[0])
	} else {
		None
	}
}

// Return the commands that args is a prefix of, e.g. "get" matches "get logs all", "get state", etc.
fn find_longer_commands(args: &Vec<String>, commands: &Commands) -> Vec<String>
{
	commands.iter()
		.filter(|command| args_matches(args, &command.0) && args.len() < command.0.len())
		.map(|command| command.0.join(" "))
		.collect()
}

// Return the commands that are a prefix of args, e.g. "get log all hmm" matches "get log all".
fn find_shorter_commands(args: &Vec<String>, commands: &Commands) -> Vec<String>
{
	commands.iter()
		.filter(|command| command_matches(args, &command.0) && args.len() > command.0.len())
		.map(|command| command.0.join(" "))
		.collect()
}

fn args_matches(args: &Vec<String>, command: &Vec<String>) -> bool
{
	args.iter()
		.enumerate()
		.all(|(i, _)| arg_matches(args, command, i))
}

fn no_extra_args(args: &Vec<String>, command: &Vec<String>) -> bool
{
	args.len() == command.len() || (args.len()+1 == command.len() && command.last().unwrap().starts_with("["))
}

fn command_matches(args: &Vec<String>, command: &Vec<String>) -> bool
{
	command.iter()
		.enumerate()
		.all(|(i, _)| arg_matches(args, command, i))
}

fn arg_matches(args: &Vec<String>, command: &Vec<String>, index: usize) -> bool
{
	if index < args.len() && index < command.len() {
		match command[index].as_ref() {
			"<time>" => parse_time(&args[index]).is_some(),
			"[level]" => parse_level(&args[index]).is_some(),
			"<number>" => parse_number(&args[index]).is_some(),
			"<path>" => parse_path(&args[index]).is_some(),
			"<value>" => true,
			_ => args[index] == command[index]
		}
	} else if index == args.len() && args.len() == command.len() + 1 {
		// Special case for optional last command and missing arg.
		command.last().unwrap().starts_with("[")
	} else {
		false
	}
}

// These are structured as verb target [options].
fn init_commands() -> Commands
{
	vec!(
		(cmd("get log [level]"),                 get_log_all,    "print the logs for all components"),
		(cmd("get log <path> [level]"),          get_log_path,   "   for components matching path"),
		(cmd("get log <number> [level]"),        get_log_n,      "   last N lines"),
		(cmd("get log <path> <number> [level]"), get_log_path_n, "   N lines for path"),
		(cmd("get state"),                       get_state,      "print the store for the current time"),
		(cmd("get state <path>"),                get_state_path, "   for components matching path"),
		(cmd("set state <path> <value>"),        set_state_path, "set state for the component (the path should include the state name)"),
		(cmd("set time <time>"),                 set_time_secs,  "advance sim time")	// TODO: support rollback, maybe by undoing Effects
	)
}

fn cmd(text: &str) -> Vec<String>
{
	text.split_whitespace().map(|s| s.to_string()).collect()
}

fn print_help(commands: &Commands)
{
	println!("The commands are:");
	let biggest = commands.iter().max_by(|x, y| x.0.join(" ").len().cmp(&y.0.join(" ").len())).unwrap();
	let max_len = biggest.0.join(" ").len();
	for command in commands {
		println!("   {:<width$} {}", command.0.join(" "), command.2, width = max_len+2);
	}
	println!("");
	println!("   exit | quit | q | ctrl-D          exit sdebug");
	println!("   help | ?                          print this message");
	
	let units: Vec<&str> = UNITS.iter().map(|u| u.0).collect();
	println!("Arguments in <angle brackets> are required. Arguments in [square brackets] are optional.");
	println!("");
	println!("Level must be error, warning, info, debug, or excessive. It defaults to info.");
	println!("Numbers are non-negative integer values.");
	println!("Paths are component paths, e.g. bob.heart.right-ventricle. Paths may be globbed.");
	println!("Times are floating point numbers with a {} suffix. If suffix is missing s is used.", units.join(", "));
	println!("Values are ints, floats (decimal point is required), or strings.");
	println!("Strings are quoted with ', \", or `. Escapes are not currently supported.");
}

// get log [level]
fn get_log_all(config: &Config, endpoint: &Endpoint, args: &Vec<String>)
{
	let level = if args.len() > 2 {&args[2]} else {"info"};
	print_log(config, endpoint, "*", u64::MAX, &level);
}

// get log <path> [level]
fn get_log_path(config: &Config, endpoint: &Endpoint, args: &Vec<String>)
{
	let path = &args[2];
	let level = if args.len() > 3 {&args[3]} else {"info"};
	print_log(config, endpoint, path, u64::MAX, &level);
}

// get log <number> [level]
fn get_log_n(config: &Config, endpoint: &Endpoint, args: &Vec<String>)
{
	let limit = parse_number(&args[2]).unwrap();
	let level = if args.len() > 3 {&args[3]} else {"info"};
	print_log(config, endpoint, "*", limit, &level);
}

// get log <path> <number> [level]
fn get_log_path_n(config: &Config, endpoint: &Endpoint, args: &Vec<String>)
{
	let path = &args[2];
	let limit = parse_number(&args[3]).unwrap();
	let level = if args.len() > 4 {&args[4]} else {"info"};
	print_log(config, endpoint, path, limit, &level);
}

// get state
fn get_state(_: &Config, endpoint: &Endpoint, _: &Vec<String>)
{
	print_state(endpoint, "*");
}

// get state <path>
fn get_state_path(_: &Config, endpoint: &Endpoint, args: &Vec<String>)
{
	let path = &args[2];
	print_state(endpoint, path);
}

// set state <path> <value>
fn set_state_path(_: &Config, endpoint: &Endpoint, args: &Vec<String>)
{
	let path = &args[2];
	let value = &args[3];
	if parse_int(value).is_some() {
		set_state(endpoint, path, "int", value);
	} else if parse_float(value).is_some() {
		set_state(endpoint, path, "float", value);
	} else {
		set_state(endpoint, path, "string", value);
	}
}

// set time secs
fn set_time_secs(_: &Config, endpoint: &Endpoint, args: &Vec<String>)
{
	let time = parse_time(&args[2]).unwrap();
	set_time(endpoint, time);
}

fn get_history_path() -> String
{
	let file_name = ".sdebug-history.txt";
	match env::home_dir() {
		Some(path) => path.join(file_name).to_str().unwrap().to_string(),
		None => file_name.to_string()
	}
}

fn get_prompt(config: &Config, endpoint: &Endpoint) -> String
{
	let time = get_time(endpoint);
	let prompt = format!("{:.1$}", time, config.precision);

	if config.colorize {
		let begin_color = "\x1b[34;1m";	// bright blue, see https://en.wikipedia.org/wiki/ANSI_escape_code#Colors
		let end_color = "\x1b[0m";
		format!("{}{}> {}", begin_color, prompt, end_color)
	} else {
		format!("{}> ", prompt)
	}
}
