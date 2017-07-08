//! Use linenoise (aka readline) to allow the user to enter in and edit
//! a command line.
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::env;

type Handler = fn (&Vec<&str>) -> ();

type Commands = Vec<(&'static str, Handler)>;

pub fn read_lines()
{
	let hpath = get_history_path();
	let verb_handlers: Commands = vec!(
		("get", process_get),
		("set", process_set));
	
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
						editor.add_history_entry(&line);
						process_line(&line, &verb_handlers);
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

fn process_line(line: &str, handlers: &Commands)
{
	let mut args: Vec<&str> = line.split_whitespace().collect();	// this is safe because score doesn't allow whitespace in component names
	if args.len() > 0 {
		let matches = find_matching_handlers(args[0], handlers);
		match matches.len() {
			0 => print!("expected {}\n", handler_names(handlers).join(" or ")),	// TODO: use red?
			1 => matches[0].1(&args.split_off(1)),
			_ => print!("{} matches {}\n", args[0], handler_names(&matches).join(" and ")),
		}
	}
}

fn handler_names(handlers: &Commands) -> Vec<&'static str>
{
	handlers.iter().map(|h| h.0).collect()
}

fn find_matching_handlers(arg: &str, handlers: &Commands) -> Commands
{
	handlers.iter()
		.filter(|h| h.0.starts_with(arg))
		.map(|h| *h)
		.collect()
}

// get log all
fn process_get(args: &Vec<&str>)
{
	println!("get: {:?}", args);
}

// set time secs
fn process_set(args: &Vec<&str>)
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