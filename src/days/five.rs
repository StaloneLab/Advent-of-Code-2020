fn parse_pass(uid: &String) -> (u8, u8) {
	let mut row_str: String = uid[..7].to_string();
	row_str = row_str.replace("F", "0");
	row_str = row_str.replace("B", "1");
	let row = u8::from_str_radix(&row_str, 2).unwrap();

	let mut col_str: String = uid[7..].to_string();
	col_str = col_str.replace("L", "0");
	col_str = col_str.replace("R", "1");
	let col = u8::from_str_radix(&col_str, 2).unwrap();

	(row, col)
}

pub fn run(input: Vec<String>) {
	let mut result: Vec<usize> = input
		.iter()
		.map(|s| parse_pass(s))
		.map(|(r, c)| 8 * r as usize + c as usize)
		.collect();

	result.sort();

	for (seat_n, seat_id) in result.iter().enumerate() {
		if(seat_n > 0) {
			let prev = result[seat_n - 1];

			if *seat_id != prev + 1 {
				println!("seat {}", seat_id - 1);
			}
		}
	}
}