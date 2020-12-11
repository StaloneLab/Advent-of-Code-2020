pub fn run(input: Vec<String>) {
	let numbers: Vec<usize> = input
		.iter()
		.map(|x| x.parse::<usize>().unwrap())
		.collect();

	let non_sum = numbers
		.iter()
		.enumerate()
		.skip(25)
		.find_map(|(i, n)| {
			for i1 in (i - 25)..i {
				let n1 = numbers[i1];

				for i2 in (i - 25)..i1 {
					let n2 = numbers[i2];

					if (n1 + n2) == *n {
						return None;
					}
				}
			}

			Some((i, n))
		});

	let (non_sum_idx, non_sum_num) = non_sum.unwrap();

	println!("Part 1: {} at index {}", non_sum_num, non_sum_idx);

	let nice_sum = numbers
		.iter()
		.enumerate()
		.take(non_sum_idx - 2)
		.find_map(|(i, n)| {
			for chain_len in 2..(non_sum_idx - i) {
				let mut sum: usize = *n;
				let mut max = 0;
				let mut min = 9e10 as usize;

				for offset in 1..chain_len {
					if numbers[i + offset] > max {
						max = numbers[i + offset];
					}

					if numbers[i + offset] < min {
						min = numbers[i + offset];
					}

					sum += numbers[i + offset];
				}

				if sum == *non_sum_num {
					return Some(min + max);
				}
			}

			None
		});

	println!("Part 2: {}", nice_sum.unwrap());
}