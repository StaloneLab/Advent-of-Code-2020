pub fn run(input: Vec<String>) {
	// Convert strings to usizes
	let input_mapped = input
		.iter()
		.map(|ls| ls.parse::<isize>().unwrap())
		.into_iter();

	input_mapped
		.clone()
		.for_each(|l1| {
			input_mapped
				.clone()
				.for_each(|l2| {
					let s = 2020 - l1 - l2;

					if input_mapped.clone().find(|&x| x == s).is_some() {
						println!("ok: {}x{}x{}={}", l1, l2, s, l1*l2*s);
					}
				})
		});
}