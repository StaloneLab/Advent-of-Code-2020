pub fn _run(input: Vec<String>, slope_x: usize, slope_y: usize) -> usize {
	let mut map = [[false; 256]; 512];
	let mut max_line = 0;
	let mut max_col = 0;

	// Preprocess input
	input
		.iter()
		.enumerate()
		.for_each(|(i, l)| {
			if i > max_line { max_line = i; }

			l
				.chars()
				.enumerate()
				.for_each(|(j, c)| {
					if j > max_col { max_col = j; }

					if c == '#' {
						map[i][j] = true;
					}
				})
		});

	let mut trees = 0;

	for i in 0..=(max_line+1)/slope_x {
		let col = (i * slope_y) % (max_col + 1);
		let li = i * slope_x;

		if map[li][col] {
			trees += 1;
		}
	}

	println!("{} {} {}", max_line, max_col, trees);

	return trees;
}

pub fn run(input: Vec<String>) {
	let a = _run(input.clone(), 1, 1);
	let b = _run(input.clone(), 1, 3);
	let c = _run(input.clone(), 1, 5);
	let d = _run(input.clone(), 1, 7);
	let e = _run(input.clone(), 2, 1);

	println!("Final: {}", a*b*c*d*e);
}