use std::fs::read_to_string;
use std::io;

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

struct Computer {
	rom: Vec<usize>,
	memory: Vec<usize>,
	ip: usize,
}

impl From<Vec<usize>> for Computer {
	fn from(v: Vec<usize>) -> Self {
		Computer {
			rom: v.clone(),
			memory: v,
			ip: 0,
		}
	}
}

impl Computer {
	// convenient function for testing purpose only
	#[allow(unused)]
	fn run_memory(ram: Vec<usize>) -> Self {
		let mut computer = Computer::from(ram);
		computer.run();
		computer
	}

	//resets the the memory and instruction pointer
	pub fn reset(&mut self) {
		self.ip = 0;
		self.memory = self.rom.clone();
	}

	// runs the rom code
	pub fn run(&mut self) {
		loop {
			let opcode = self.memory[self.ip];
			if opcode == 99 {
				break;
			}

			let src1 = self.memory[self.ip + 1];
			let src2 = self.memory[self.ip + 2];
			let dst = self.memory[self.ip + 3];

			let par1 = self.memory[src1];
			let par2 = self.memory[src2];

			match opcode {
				1 => self.memory[dst] = par1 + par2,
				2 => self.memory[dst] = par1 * par2,
				e => panic!("Error: {} no valid opcode", e),
			}

			self.ip += 4;
		}
	}

	// get a value in memory
	pub fn get(&self, pos: usize) -> usize {
		self.memory[pos]
	}

	// set a value in memory
	pub fn set(&mut self, pos: usize, val: usize) {
		self.memory[pos] = val;
	}

	// testing purpose only
	#[allow(unused)]
	pub fn mem(&self) -> &[usize] {
		&self.memory
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_computer_run() {
		assert_eq!(
			Computer::run_memory(vec![1, 0, 0, 0, 99]).mem(),
			&[2, 0, 0, 0, 99]
		);
		assert_eq!(
			Computer::run_memory(vec![2, 3, 0, 3, 99]).mem(),
			&[2, 3, 0, 6, 99]
		);
		assert_eq!(
			Computer::run_memory(vec![2, 4, 4, 5, 99, 0]).mem(),
			&[2, 4, 4, 5, 99, 9801]
		);
		assert_eq!(
			Computer::run_memory(vec![1, 1, 1, 4, 99, 5, 6, 0, 99]).mem(),
			&[30, 1, 1, 4, 2, 5, 6, 0, 99]
		);
	}
}
