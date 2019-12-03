use std::fs::read_to_string;
use std::io;

// runs the challenge
// takes a function to calculate the fuel from the input mass
pub fn run(f: fn(u32) -> u32) -> io::Result<()> {
	let result: u32 = read_to_string("./input.txt")?
		.lines()
		.map(|l| f(l.parse::<u32>().unwrap()))
		.sum();
	println!("Result: {}", result);
	Ok(())
}

// returns the fuel required to launch a given module based on its mass.
pub fn required_fuel(mass: u32) -> u32 {
	// check to avoid overflow
	if mass < 6 {
		return 0;
	}
	mass / 3 - 2
}

// fuel itself requires fuel just like a module - take its mass.
// returns the total fuel required to launch a given module
#[allow(unused)]
pub fn total_fuel_recursive(mass: u32) -> u32 {
	if mass < 1 {
		return 0;
	}
	let req_fuel = required_fuel(mass);
	req_fuel + total_fuel_recursive(req_fuel)
}

#[allow(unused)]
pub fn total_fuel_iter(mass: u32) -> u32 {
	std::iter::successors(Some(mass), |m| (m / 3).checked_sub(2))
		.skip(1)
		.sum()
}

pub fn total_fuel(mass: u32) -> u32 {
	let mut fuel_sum = 0;
	let mut fuel = required_fuel(mass);
	while fuel > 0 {
		fuel_sum += fuel;
		fuel = required_fuel(fuel);
	}
	fuel_sum
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_required_fuel() {
		assert_eq!(required_fuel(12), 2);
		assert_eq!(required_fuel(14), 2);
		assert_eq!(required_fuel(1969), 654);
		assert_eq!(required_fuel(100756), 33583);
	}

	#[test]
	fn test_total_fuel_recursive() {
		assert_eq!(total_fuel_recursive(12), 2);
		assert_eq!(total_fuel_recursive(14), 2);
		assert_eq!(total_fuel_recursive(1969), 966);
		assert_eq!(total_fuel_recursive(100756), 50346);
	}

	#[test]
	fn test_total_fuel_iter() {
		assert_eq!(total_fuel_iter(12), 2);
		assert_eq!(total_fuel_iter(14), 2);
		assert_eq!(total_fuel_iter(1969), 966);
		assert_eq!(total_fuel_iter(100756), 50346);
	}

	#[test]
	fn test_total_fuel() {
		assert_eq!(total_fuel(12), 2);
		assert_eq!(total_fuel(14), 2);
		assert_eq!(total_fuel(1969), 966);
		assert_eq!(total_fuel(100756), 50346);
	}
}
