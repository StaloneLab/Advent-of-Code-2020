pub fn run(input: Vec<String>) {
	let mut input_ordered = input
		.iter()
		.map(|x| x.parse::<usize>().unwrap())
		.collect::<Vec<usize>>();

	input_ordered.push(0);
	input_ordered.sort();

	let mut diff_one = 0;
	let mut diff_three = 1;

	for ns in input_ordered.windows(2) {
		let n1 = ns[0];
		let n2 = ns[1];

		if (n2 - n1) == 1 {
			diff_one += 1;
		} else if (n2 - n1) == 3 {
			diff_three += 1;
		}
	}

	println!("Part 1: {} 1's * {} 3's = {}", diff_one, diff_three, diff_one * diff_three);

	let mut input_ordered_weight: Vec<(usize, usize)> = input_ordered
		.iter()
		.map(|x| (*x, 0usize))
		.collect();

	// Attribute weight to the three first items
	input_ordered_weight[0].1 = 1;
	input_ordered_weight[1].1 = 1;

	if input_ordered_weight[2].0 - input_ordered_weight[0].0 <= 3 {
		input_ordered_weight[2].1 = 2;
	} else {
		input_ordered_weight[2].1 = 1;
	}

	for ni in 0..input_ordered_weight.len()-3 {
		let target = input_ordered_weight[ni + 3].0;
		let mut weight = 0;

		for i in 0..3 {
			let (n, w) = input_ordered_weight[ni + i];

			if (target - n) <= 3 {
				weight += w;
			}
		}

		input_ordered_weight[ni + 3].1 = weight;
	}

	let result = input_ordered_weight[input_ordered_weight.len() - 1].1;

	println!("Part 2: {}", result);
}