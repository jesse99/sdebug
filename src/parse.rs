//! Various parsing related helpers.
use std::str::FromStr;

/// The units we allow with durations. (These are public so that help can
/// print them).
pub const UNITS: &'static [(&'static str, f64)] = &[
	("ns", 0.000_000_001),
	("us", 0.000_001),
	("ms", 0.001),
	("s", 1.0),
	("m", 60.0),
	("h", 60.0*60.0),
	("d", 24.0*60.0*60.0)];

/// Split words on whitespace and quote characters, e.g. "set state    'hey there'"
/// becomes ["set", "state", "hey there"]. Quote characters are ', ", and `. Note
/// that paths cannot have whitespace so it's OK to do this sort of splitting.
pub fn tokenize(text: &str) -> Result<Vec<String>, String>
{
	let mut words = Vec::new();

	let mut quote = None;
	let mut word = String::new();
	for ch in text.chars() {
		if let Some(q) = quote {
			if ch == q {
				words.push(word.clone());	// empty is OK
				word.clear();
				quote = None;
			} else {
				word.push(ch);
			}
		} else {
			if ch.is_whitespace() {
				if !word.is_empty() {
					words.push(word.clone());
					word.clear();
				}
			} else if ch == '"' || ch == '\'' || ch == '`' {
				quote = Some(ch);
			} else {
				word.push(ch);
			}
		}
	}
	
	// The input module can't distinguish between stuff like "hello" and hello
	// but that just means that users can sometimes omit quotes which isn't
	// so bad and does mirror what shells do.
	//
	// Although users can also type stuff like "get 'log' all" which is a bit
	// more obnoxious.
	if quote.is_some() {
		Err(format!("Un-terminated {}", quote.unwrap()))
	} else {
		if !word.is_empty() {
			words.push(word);
		}
		Ok(words)
	}
}

/// Converts a string like "10.5ms" to a number in seconds.
/// Returns None if the string isn't a duration or has extra characters.
pub fn parse_duration(arg: &str) -> Option<f64>
{
	if let Some((prefix, scale_by)) = parse_time_suffix(arg) {
		if let Ok(x) = f64::from_str(prefix) {
			Some(x*scale_by)
		} else {
			None
		}
	} else {
		None
	}
}

/// Converts a string like "105" to a number. Returns None if the string
/// isn't a number or has extra characters.
pub fn parse_number(arg: &str) -> Option<u64>
{
	if let Ok(x) = u64::from_str(arg) {
		Some(x)
	} else {
		None
	}
}

/// Returns the path if it actually looks like a path. Returns None for things
/// like not starting with a letter.
pub fn parse_path(arg: &str) -> Option<&str>
{
	// Paths start with a letter and have no whitespace, control, or quote characters.
	if valid_component_name(arg) && parse_level(arg).is_none() {
		Some(arg)
	} else {
		None
	}
}

/// Returns the arg if it is a log level (e.g. "info"). Returns None otherwise.
pub fn parse_level(arg: &str) -> Option<&str>
{
	match arg {
		"error" | "warning" | "info" | "debug" | "excessive" => Some(arg),
		_ => None,
	}
}

// i.e. does the path start with a legit component name
fn valid_component_name(path: &str) -> bool
{
	path.chars().nth(0).unwrap_or('1').is_alphabetic()
}

// Returns prefix and the suffix scale factor.
fn parse_time_suffix(arg: &str) -> Option<(&str, f64)>
{
	if let Some(result) = UNITS.iter().find(|u| arg.ends_with(u.0)) {
		let prefix = arg.split_at(arg.len() - result.0.len()).0;
		Some((prefix, result.1))
	} else {
		None
	}
}
