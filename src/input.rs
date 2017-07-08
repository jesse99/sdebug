//! Use linenoise (aka readline) to allow the user to enter in and edit
//! a command line.
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::env;

pub fn read_lines()
{
	let hpath = get_history_path();
	
	// TODO: instead of () install a completion fn
	let mut editor = Editor::<()>::new();
	if let Err(err) = editor.load_history(&hpath) {
		println!("Error loading history: {}", err);
	}
	loop {
		let readline = editor.readline("> ");
		match readline {
			Ok(line) => {
				editor.add_history_entry(&line);
				process_line(&line);
			},
//			Err(ReadlineError::Interrupted) => {
//				println!("CTRL-C");
//				break
//			},
			Err(ReadlineError::Eof) => {
				break
			},
			Err(err) => {
				println!("Error: {:?}", err);
				break
			}
		}
	}
	editor.save_history(&hpath).unwrap();
}

fn process_line(line: &str)
{
	println!("Line: {}", line);
}

fn get_history_path() -> String
{
	let file_name = ".sdebug-history.txt";
	match env::home_dir() {
		Some(path) => path.join(file_name).to_str().unwrap().to_string(),
		None => file_name.to_string()
	}
}