mod input;
mod days;

use input::from_list;
use days::{ one, two, three, four, five, six, seven, eight };

fn main() {
	let args: Vec<String> = std::env::args().collect();
	let day: u8 = args[1].parse::<u8>().expect("Invalid day number");

	match day {
		1 => one::run(from_list("one")),
		2 => two::run(from_list("two")),
		3 => three::run(from_list("three")),
		4 => four::run(from_list("four")),
		5 => five::run(from_list("five")),
		6 => six::run(from_list("six")),
		7 => seven::run(from_list("seven")),
		8 => eight::run(from_list("eight")),
		_ => eprintln!("Nothing for this day"),
	};
}