//! Use linenoise (aka readline) to allow the user to enter in and edit
//! a command line.
use parse::*;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::env;
//use std::str::FromStr;

type Handler = fn (&Vec<String>) -> ();

type Commands = Vec<(Vec<&'static str>, Handler, &'static str)>;

/// Use linenoise to read in lines typed by the user and process them.
pub fn read_lines()
{
	let hpath = get_history_path();
	let commands = init_commands();
	
	// TODO: instead of () install a completion fn
	let mut editor = Editor::<()>::new();
	if let Err(err) = editor.load_history(&hpath) {
		println!("Error loading history: {}", err);
	}
	loop {
		let readline = editor.readline("> ");
		match readline {
			Ok(line) => {
				match line.as_ref() {
					"quit" | "exit" => {
						break;
					},
					"help" | "?" => {
						print_help(&commands);
					},
					_ => {
						editor.add_history_entry(&line);	// even if there is an error, it's nice to have it in the history so that the user can easily repair it
						match tokenize(&line) {
							Ok(args) => process_line(&args, &commands),
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

fn process_line(args: &Vec<String>, commands: &Commands)
{
	if args.len() > 0 {
		if let Some(handler) = find_handler(&args, commands) {
			handler(&args);
			
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
		.filter(|command| args_matches(args, &command.0) && (args.len() == command.0.len() || *command.0.last().unwrap() == "<value>"))
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

fn args_matches(args: &Vec<String>, command: &Vec<&str>) -> bool
{
	args.iter()
		.enumerate()
		.all(|(i, _)| arg_matches(args, command, i))
}

fn command_matches(args: &Vec<String>, command: &Vec<&str>) -> bool
{
	command.iter()
		.enumerate()
		.all(|(i, _)| arg_matches(args, command, i))
}

fn arg_matches(args: &Vec<String>, command: &Vec<&str>, index: usize) -> bool
{
	if index < args.len() && index < command.len() {
		match command[index] {
			"<duration>" => parse_duration(&args[index]).is_some(),
			"<number>" => parse_number(&args[index]).is_some(),
			"<path>" => parse_path(&args[index]).is_some(),
			"<value>" => true,	// TODO: handle strings
			_ => args[index] == command[index]
		}
	} else {
		false
	}
}

// These are structured as verb target [options].
fn init_commands() -> Commands
{
	vec!(
		(vec!("get", "log"),                        get_log,        "print the entire log"),
		(vec!("get", "log", "<path>"),              get_log_path,   "print the entire log for components"),
		(vec!("get", "log", "<number>"),            get_log_n,      "print the last N log lines"),
		(vec!("get", "log", "<path>", "<number>"),  get_log_path_n, "print the last N log lines for components"),
		(vec!("get", "state"),                      get_state,      "print the store for the current time"),
		(vec!("get", "state", "<path>"),            get_state_path, "print the store for the current time and components"),
		(vec!("set", "state", "<path>", "<value>"), set_state,      "set state for the component (the path should include the state name)"),
		(vec!("set", "time", "<duration>"),         set_time_secs,  "advance or rollback sim time")
	)
}

fn print_help(commands: &Commands)
{
	println!("The commands are:");
	let biggest = commands.iter().max_by(|x, y| x.0.join(" ").len().cmp(&y.0.join(" ").len())).unwrap();
	let max_len = biggest.0.join(" ").len();
	for command in commands {
		println!("   {:<width$} {}", command.0.join(" "), command.2, width = max_len+2);
	}
	
	let units: Vec<&str> = UNITS.iter().map(|u| u.0).collect();
	println!("Arguments in <angle brackets> are required. Arguments in [square brackets] are optional.");	// TODO: do we actually use the [optional] syntax?
	println!("");
	println!("Durations are floating point numbers with a {} suffix.", units.join(", "));
	println!("Numbers are non-negative integer values.");
	println!("Paths are component paths, e.g. bob.heart.right-ventricle. Paths may be globbed.");
	println!("Values are ints, floats (decimal point is required), or strings.");
	println!("Strings are quoted with ', \", or `. Escapes are not currently supported.");
}

// get log
fn get_log(_: &Vec<String>)
{
	println!("all log lines");
}

// get log <path>
fn get_log_path(args: &Vec<String>)
{
	println!("all log lines for {}", args[2]);
}

// get log <number>
fn get_log_n(args: &Vec<String>)
{
	let n = parse_number(&args[2]).unwrap();
	println!("{} log lines", n);
}

// get log <path> <number>
fn get_log_path_n(args: &Vec<String>)
{
	let n = parse_number(&args[3]).unwrap();
	println!("{} log lines for {}", n, args[2]);
}

// get state
fn get_state(_: &Vec<String>)
{
	println!("all state for the current time");
}

// get state <path>
fn get_state_path(args: &Vec<String>)
{
	println!("all state for {}", args[2]);
}

// set state <path> <value>
fn set_state(args: &Vec<String>)
{
	let value = args.split_at(2).1;
	println!("{} = {}", args[2], value.join(" "));
}

// set time secs
fn set_time_secs(args: &Vec<String>)
{
	let duration = parse_duration(&args[2]).unwrap();
	println!("set time {:.6}", duration);
}

fn get_history_path() -> String
{
	let file_name = ".sdebug-history.txt";
	match env::home_dir() {
		Some(path) => path.join(file_name).to_str().unwrap().to_string(),
		None => file_name.to_string()
	}
}