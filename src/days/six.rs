use std::collections::HashSet;

pub fn run(input: Vec<String>) {
	let mut sum = 0;
	let mut first_member = true;
	let mut current_set = HashSet::new();

	for l in input.iter() {
		if !l.is_empty() {
			if first_member {
				l.chars().for_each(|c| { current_set.insert(c); });
			} else {
				for c in current_set.clone() {
					if !l.contains(c) {
						current_set.remove(&c);
					}
				}
			}

			first_member = false;
		} else {
			sum += current_set.len();
			first_member = true;
			current_set = HashSet::new();
		}
	}

	sum += current_set.len();

	println!("Count: {}", sum);
}