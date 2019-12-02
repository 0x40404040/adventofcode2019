use common::part;
use day_01::{required_fuel, run, total_fuel};

fn main() {
	if let Err(e) = run(part::one_or_two(required_fuel, total_fuel)) {
		eprintln!("Error: {}", e);
		std::process::exit(1);
	}
}
