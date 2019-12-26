use std::fs::read_to_string;
use std::io;

use common::intcode::Computer;
use common::part::one_or_two;
use common::reexport::itertools::Itertools;

fn main() {
	if let Err(e) = run() {
		eprintln!("Error {}", e);
		std::process::exit(1);
	}
}

fn run() -> io::Result<()> {
	let ints: Vec<usize> = read_to_string("./input.txt")?
		.trim()
		.split(',')
		.map(|s| s.parse::<usize>().unwrap())
		.collect();

	let mut computer = Computer::from(ints);

	if one_or_two(true, false) {
		computer.set(1, 12);
		computer.set(2, 2);
		computer.run();
		println!("Result: {}", computer.get(0));
	} else {
		let magic_number = 19_690_720;
		let result = (0..=99).permutations(2).find(|p| {
			let (noun, verb) = (p[0], p[1]);
			computer.reset();
			computer.set(1, noun);
			computer.set(2, verb);
			computer.run();
			computer.get(0) == magic_number
		});

		match result {
			Some(v) => println!("Result: {}", 100 * v[0] + v[1]),
			None => println!("No numbers found."),
		}
	}

	Ok(())
}
