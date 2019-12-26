use common::part::one_or_two;

const START: usize = 372_304;
const END: usize = 847_060;

fn main() {
	run()
}

fn run() {
	let valid_pw_count = (START..END)
		.filter(|x| valid_password(*x, one_or_two(false, true)))
		.count();

	println!("Result = {}", valid_pw_count);
}

#[derive(PartialEq)]
enum PasswordState {
	SingleDigit(usize),
	DoubleDigit(usize),
	ManyDigit(usize),
	Increment(usize),
	Invalid,
}

impl PasswordState {
	fn next_state(self, digit: usize, strict: bool) -> Self {
		use PasswordState::*;
		match self {
			SingleDigit(pdigit) if (pdigit < digit) => SingleDigit(digit),
			SingleDigit(pdigit) if (pdigit == digit) => DoubleDigit(digit),
			DoubleDigit(pdigit) if (!strict && pdigit <= digit) => DoubleDigit(digit),

			DoubleDigit(pdigit) if (pdigit == digit) => ManyDigit(digit),
			DoubleDigit(pdigit) if (pdigit < digit) => Increment(digit),
			Increment(pdigit) if (pdigit <= digit) => Increment(digit),
			ManyDigit(pdigit) if (pdigit == digit) => ManyDigit(digit),
			ManyDigit(pdigit) if (pdigit < digit) => SingleDigit(digit),
			_ => Invalid,
		}
	}

	fn is_valid(&self) -> bool {
		match *self {
			PasswordState::DoubleDigit(_) => true,
			PasswordState::Increment(_) => true,
			_ => false,
		}
	}
}

fn valid_password(p: usize, strict: bool) -> bool {
	let digits = num_to_digits(p);
	if digits.len() != 6 {
		return false;
	}
	let password: PasswordState = digits.iter().fold(PasswordState::SingleDigit(0), |pws, x| {
		pws.next_state(*x, strict)
	});

	password.is_valid()
}

// Breaks a number into single digits
fn num_to_digits(mut n: usize) -> Vec<usize> {
	let mut digits = Vec::new();
	while n > 0 {
		digits.insert(0, n % 10);
		n /= 10;
	}
	digits
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_num_to_digits() {
		assert_eq!(num_to_digits(123), vec![1, 2, 3]);
		assert_eq!(num_to_digits(55555), vec![5; 5]);
	}

	#[test]
	fn test_valid_password_not_strict() {
		assert!(valid_password(111111, false));
		assert!(valid_password(122345, false));
		assert!(valid_password(111123, false));
		assert!(valid_password(133679, false));
		assert_eq!(valid_password(223450, false), false);
		assert_eq!(valid_password(123456, false), false);
	}

	#[test]
	fn test_valid_password_strict() {
		assert!(valid_password(111122, true));
		assert!(valid_password(122345, true));
		assert!(valid_password(112233, true));
		assert!(valid_password(377888, true));
		assert_eq!(valid_password(111111, true), false);
		assert_eq!(valid_password(111115, true), false);
		assert_eq!(valid_password(223450, true), false);
		assert_eq!(valid_password(123456, true), false);
		assert_eq!(valid_password(111222, true), false);
		assert_eq!(valid_password(123444, true), false);
	}
}
