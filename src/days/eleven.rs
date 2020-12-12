#[derive(Clone)]
enum SeatState {
	Floor,
	Empty,
	Occupied,
}

impl SeatState {
	pub fn is_empty(&self) -> bool {
		if let SeatState::Occupied = self {
			false
		} else {
			true
		}
	}

	pub fn is_floor(&self) -> bool {
		if let SeatState::Floor = self {
			true
		} else {
			false
		}
	}
}

type Seats = Vec<Vec<SeatState>>;

fn run_pass(seats: &mut Seats) -> bool {
	// Make the system evolve
	let before_state = seats.clone();
	let mut changed = false;

	for (rid, row) in seats.iter_mut().enumerate() {
		for (sid, seat) in row.iter_mut().enumerate() {
			let mut all_empty = true;
			let mut occupied_count = 0;

			for offset_rid in 0..=2 {
				for offset_sid in 0..=2 {
					if	rid + offset_rid >= 1 && sid + offset_sid >= 1 &&
						rid + offset_rid <= before_state.len() &&
						sid + offset_sid <= before_state[0].len() &&
						(offset_rid != 1 || offset_sid != 1) {
						let test_seat = &before_state[rid + offset_rid - 1][sid + offset_sid - 1];

						if !test_seat.is_empty() {
							occupied_count += 1;
							all_empty = false;
						}
					}
				}
			}

			if !seat.is_floor() {
				if all_empty && seat.is_empty() {
					*seat = SeatState::Occupied;
					changed = true;
				}

				if occupied_count >= 4 && !seat.is_empty() {
					*seat = SeatState::Empty;
					changed = true;
				}
			}
		}
	}

	changed
}

fn display_seats(seats: &Seats) {
	for row in seats {
		for seat in row {
			match seat {
				SeatState::Occupied => print!("#"),
				SeatState::Empty => print!("L"),
				SeatState::Floor => print!("."),
			}
		}

		println!("");
	}
}

pub fn run(input: Vec<String>) {
	// Parse initial seats disposition
	let mut seats: Seats = input
		.iter()
		.map(|l| {
			l
				.chars()
				.map(|c| {
					match c {
						'L' => SeatState::Empty,
						'#' => SeatState::Occupied,
						'.' => SeatState::Floor,
						_ => panic!("Invalid seat state"),
					}
				})
				.collect()
		})
		.collect();

	let mut i = 0;

	println!("========== Pass {} ==========", i);
	display_seats(&seats);

	while run_pass(&mut seats) {
		i += 1;
		println!("========== Pass {} ==========", i);
		display_seats(&seats);
	}

	let mut occupied = 0;

	for row in seats {
		for seat in row {
			if !seat.is_empty() { occupied += 1; }
		}
	}

	println!("Part 1: {}", occupied);
}