mod input;
mod days;

use input::from_list;
use days::{ one, two, three };

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let day: u8 = args[1].parse::<u8>().expect("Invalid day number");

	match day {
		1 => one::run(from_list("one")),
		2 => two::run(from_list("two")),
		3 => three::run(from_list("three")),
		_ => eprintln!("Nothing for this day"),
	};
}