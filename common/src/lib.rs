pub mod intcode;

/// Module to help with the 2 part distinction in advent of code  
pub mod part {
    /// returns one or two depending on the FIRST command line arugment
    /// exit(1) when no or wrong CLI argument is found
    /// CLI -- 1 returns one
    /// CLI -- 2 returns two
    pub fn one_or_two<T>(one: T, two: T) -> T {
        let arg1 = std::env::args().nth(1).unwrap_or_default();
        match arg1.as_ref() {
            "1" => one,
            "2" => two,
            _ => {
                eprintln!("First CLI argument has to be 1 or 2");
                std::process::exit(1);
            }
        }
    }

    /// returns true if part one is given as command line argument
    pub fn is_part_one() -> bool {
        one_or_two(true, false)
    }
}

pub mod reexport {
    pub use itertools;
}
