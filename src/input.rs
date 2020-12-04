use std::fs::File;
use std::io::{ BufRead, BufReader };

pub fn from_list(day: &str) -> Vec<String> {
	let filename = format!("input/{}.txt", day);

	let file = File::open(&filename).expect(
		format!("Unable to open input file {}", filename).as_str(),
	);

	BufReader::new(file)
		.lines()
		.map(|l| l.unwrap())
		//.filter(|l| !l.is_empty())
		.collect()
}