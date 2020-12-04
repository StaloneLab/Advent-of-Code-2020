static required_keywords: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

pub fn run(input: Vec<String>) {
	let mut count = 0;
	let mut found_keywords: Vec<&str> = Vec::new();

	for (i, line) in input.iter().enumerate() {
		if line.is_empty() {
			if found_keywords.len() == 7 {
				println!("line {} valid", i);
				count += 1;
			} else {
				println!("line {} invalid", i);
			}

			found_keywords = Vec::new();
		}

		for split in line.split(' ') {
			let result: Vec<&str> = split.split(':').collect();

			if required_keywords.contains(&result[0]) & !found_keywords.contains(&result[0]) {
				found_keywords.push(result[0]);
			}
		}
	}

	if found_keywords.len() == 7 {
		println!("last line valid");
		count += 1;
	}

	println!("done: {}", count);
}