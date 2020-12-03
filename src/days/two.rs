enum QADState {
	FirstNumber(u8),
	SecondNumber(u8),
	ConstraintChar,
	Password,
}

fn qad_parser(input: &str) -> (u8, u8, char, String) {
	let mut state = QADState::FirstNumber(0);
	let mut result = (0u8, 0u8, '0', String::new());

	for c in input.chars() {
		match state {
			QADState::FirstNumber(t) => {
				if c == '-' {
					result.0 = t;
					state = QADState::SecondNumber(0u8);
					continue;
				}

				let nn = c.to_digit(10).unwrap() as u8;
				state = QADState::FirstNumber(t * 10 + nn);
			},
			QADState::SecondNumber(t) => {
				if c == ' ' {
					result.1 = t;
					state = QADState::ConstraintChar;
					continue;
				}

				let nn = c.to_digit(10).unwrap() as u8;
				state = QADState::SecondNumber(t * 10 + nn);
			},
			QADState::ConstraintChar => {
				result.2 = c;
				state = QADState::Password;
			},
			_ => {
				if c.is_ascii_alphabetic() { result.3.push(c); }
			}
		}
	}

	result
}

pub fn run_first(input: Vec<String>) {
	let mut count = 0;

	input
		.iter()
		.for_each(|l| {
			let (a, b, c, d) = qad_parser(l);
			let nb_occ = d.split(c).collect::<Vec<&str>>().len() - 1;

			if nb_occ >= a.into() && nb_occ <= b.into() {
				println!("ok");
				count += 1;
			} else {
				println!("ko");
			}
		});
		
	println!("{}", count);
}

pub fn run(input: Vec<String>) {
	let mut count = 0;

	input
		.iter()
		.for_each(|l| {
			let (a, b, c, d) = qad_parser(l);
			let mut chars = d.chars();

			println!("{} {} {}", a, b, d);
			let first_char = chars.nth(a as usize - 1).unwrap();
			let second_char = chars.nth((b - a) as usize - 1).unwrap();

			if (first_char == c) ^ (second_char == c) {
				println!("ok");
				count += 1;
			} else {
				println!("ko");
			}
		});
		
	println!("{}", count);
}