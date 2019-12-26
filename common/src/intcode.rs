pub struct Computer {
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
	fn mem(&self) -> &[usize] {
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
