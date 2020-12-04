use std::collections::HashMap;
use std::sync::Arc;

static required_keywords: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];

fn interval_checker_generator(start: usize, end: usize) -> Box<Fn(&str) -> bool> {
	Box::new(move |s: &str| {
		let sn = s.parse::<usize>();

		if sn.is_err() {
			return false;
		}

		let n = sn.unwrap();

		n >= start && n <= end
	})
}

pub fn run(input: Vec<String>) {
	let height_cm_checker = interval_checker_generator(150, 193);
	let height_in_checker = interval_checker_generator(59, 76);

	let keywords_mapping: HashMap<&str, Box<Fn(&str) -> bool>> = {
		let mut hm = HashMap::new();
	
		hm.insert("byr", interval_checker_generator(1920, 2002));
		hm.insert("iyr", interval_checker_generator(2010, 2020));
		hm.insert("eyr", interval_checker_generator(2020, 2030));
		hm.insert("hgt", Box::new(|s: &str| {
			if(!s.ends_with("cm") && !s.ends_with("in")) { return false; }

			if s.ends_with("cm") {
				return height_cm_checker(&s[..s.len() - 2]);
			} else if s.ends_with("in") {
				return height_in_checker(&s[..s.len() - 2]);
			} else {
				return false;
			}
		}));
		hm.insert("hcl", Box::new(|s: &str| {
			if(!s.starts_with('#')) { return false; }

			usize::from_str_radix(&s[1..], 16).is_ok()
		}));
		hm.insert("ecl", Box::new(|s: &str| {
			["amb", "blu", "brn", "gry", "grn", "hzl", "oth"].contains(&s)
		}));
		hm.insert("pid", Box::new(|s: &str| {
			let mut valid = s.len() == 9;

			for c in s.chars() {
				if !c.is_ascii_digit() {
					valid = false;
				}
			}

			return valid
		}));
	
		hm
	};

	let mut count = 0;
	let mut found_keywords: Vec<&str> = Vec::new();

	for (i, line) in input.iter().enumerate() {
		if line.is_empty() {
			if found_keywords.len() == 7 {
				count += 1;
			}

			found_keywords = Vec::new();
		} else {
			for split in line.split(' ') {
				let result: Vec<&str> = split.split(':').collect();
				let key = result[0];
				let value = result[1];
				let valid: bool = {
					if key == "cid" {
						true	
					} else {
						keywords_mapping[key](value)
					}
				};

				println!("{} {}", split, valid);

				if required_keywords.contains(&result[0]) && !found_keywords.contains(&result[0]) && valid {
					found_keywords.push(result[0]);
				}
			}
		}
	}

	if found_keywords.len() == 7 { count += 1; }

	println!("valid: {}", count);
}