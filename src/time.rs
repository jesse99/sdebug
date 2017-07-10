//! REST commands related to time.
use crest::prelude::*;
use helpers::*;

/// Returns the current simulation time in seconds.
pub fn get_time(endpoint: &Endpoint) -> f64
{
	get_rest::<f64>(endpoint, &["time"])
}

/// Returns the number of significant decimal places in simulation
/// times, e.g. if the sim is tracking time to the ms then this will
/// return 3.
pub fn get_time_precision(endpoint: &Endpoint) -> usize
{
	get_rest::<usize>(endpoint, &["time", "precision"])
}

/// Advance simulation time to the specified number of seconds.
pub fn set_time(endpoint: &Endpoint, secs: f64)
{
	assert!(secs > 0.0);
	let arg = format!("{:.12}", secs);
	let result = post_rest(endpoint, &["time", &arg]);
	if result != "ok" {
		println!("{}", result);
	}
}

