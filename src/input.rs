//! Use linenoise (aka readline) to allow the user to enter in and edit
//! a command line.
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::env;

type Handler = fn (&Vec<&str>) -> ();

type Commands = Vec<(Vec<&'static str>, Handler)>;

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
					// TODO: need to support help and ?
					_ => {
						editor.add_history_entry(&line);	// even if there is an error, it's nice to have it in the history so that the user can easily repair it
						process_line(&line, &commands);
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

fn process_line(line: &str, commands: &Commands)
{
	let args: Vec<&str> = line.split_whitespace().collect();	// this is safe because score doesn't allow whitespace in component names
	if args.len() > 0 {
		if let Some(handler) = find_handler(&args, commands) {
			handler(&args);
			
		} else {
			// We failed to find a command that matched what the user typed
			// so check for some common error cases.
			let mut matches = find_longer_commands(&args, commands);
			if matches.is_empty() {
				matches = find_shorter_commands(&args, commands);
			}
			if matches.len() > 0 {
				println!("Did you mean:"); 	// TODO: use red?
				for m in matches.iter() {
					println!("   {}", m);
				}
			} else {
				print!("failed to find a matching command\n");
			}
		}
	}
}

// Return a handler if args exactly matches a command.
fn find_handler(args: &Vec<&str>, commands: &Commands) -> Option<Handler>
{
	let handlers: Vec<Handler> = commands.iter()
		.filter(|command| args_matches(args, &command.0) && args.len() == command.0.len())
		.map(|command| command.1)
		.collect();
	if handlers.len() == 1 {
		Some(handlers[0])
	} else {
		None
	}
}

// Return the commands that args is a prefix of, e.g. "get" matches "get logs all", "get state", etc.
fn find_longer_commands(args: &Vec<&str>, commands: &Commands) -> Vec<String>
{
	commands.iter()
		.filter(|command| args_matches(args, &command.0) && args.len() < command.0.len())
		.map(|command| command.0.join(" "))
		.collect()
}

// Return the commands that are a prefix of args, e.g. "get log all hmm" matches "get log all".
fn find_shorter_commands(args: &Vec<&str>, commands: &Commands) -> Vec<String>
{
	commands.iter()
		.filter(|command| command_matches(args, &command.0) && args.len() > command.0.len())
		.map(|command| command.0.join(" "))
		.collect()
}

fn args_matches(args: &Vec<&str>, command: &Vec<&str>) -> bool
{
	args.iter()
		.enumerate()
		.all(|(i, _)| arg_matches(args, command, i))
}

fn command_matches(args: &Vec<&str>, command: &Vec<&str>) -> bool
{
	command.iter()
		.enumerate()
		.all(|(i, _)| arg_matches(args, command, i))
}

fn arg_matches(args: &Vec<&str>, command: &Vec<&str>, index: usize) -> bool
{
	if index < args.len() && index < command.len() {
		args[index] == command[index]	// TODO: handle stuff like <secs>
	} else {
		false
	}
}

fn init_commands() -> Commands
{
	vec!(
		(vec!("get",    "log",  "all"),    get_log_all),
		(vec!("goober", "log",  "all"),    get_log_all),
		(vec!("set",    "time", "<secs>"), set_time_secs)
	)
}

// get log all
fn get_log_all(args: &Vec<&str>)
{
	println!("get: {:?}", args);
}

// set time secs
fn set_time_secs(args: &Vec<&str>)
{
	println!("set: {:?}", args);
}

fn get_history_path() -> String
{
	let file_name = ".sdebug-history.txt";
	match env::home_dir() {
		Some(path) => path.join(file_name).to_str().unwrap().to_string(),
		None => file_name.to_string()
	}
}