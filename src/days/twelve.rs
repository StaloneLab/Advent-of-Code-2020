fn rotate_ninety(pos: (&mut isize, &mut isize), count: usize) {
	for _ in 0..count {
		let old_east = *pos.1;
		*pos.1 = *pos.0;
		*pos.0 = -old_east;
	}
}

pub fn run(input: Vec<String>) {
	let mut angle = 0;
	let mut north: isize = 0;
	let mut east: isize = 0;

	for line in &input {
		let command = &line[0..1];
		let value = &line[1..].parse::<isize>().unwrap();

		match command {
			"N" => north += value,
			"S" => north -= value,
			"E" => east += value,
			"W" => east -= value,
			"F" => {
				match angle {
					0 => east += value,
					90 | -270 => north -= value,
					180 | -180 => east -= value,
					270 | -90 => north += value,
					_ => panic!("Invalid angle: {}", angle),
				}
			},
			"R" => angle = (angle + value) % 360,
			"L" => angle = (angle as isize - value) % 360,
			_ => panic!("Invalid command"),
		}
	}

	println!("Part 1: {}", north.abs() + east.abs());

	let mut waypoint_north: isize = 1;
	let mut waypoint_east: isize = 10;

	let mut ship_north: isize = 0;
	let mut ship_east: isize = 0;

	for line in &input {
		let command = &line[0..1];
		let value = &line[1..].parse::<isize>().unwrap();

		match command {
			"N" => waypoint_north += value,
			"S" => waypoint_north -= value,
			"E" => waypoint_east += value,
			"W" => waypoint_east -= value,
			"F" => {
				ship_north += value * waypoint_north;
				ship_east += value * waypoint_east;
			},
			"R" => {
				match value % 360 {
					0 => {},
					90 | -270 => rotate_ninety((&mut waypoint_north, &mut waypoint_east), 1),
					180 | -180 => rotate_ninety((&mut waypoint_north, &mut waypoint_east), 2),
					270 | -90 => rotate_ninety((&mut waypoint_north, &mut waypoint_east), 3),
					_ => panic!("Invalid angle: {}", angle),
				}
			},
			"L" => {
				match value % 360 {
					0 => {},
					90 | -270 => rotate_ninety((&mut waypoint_north, &mut waypoint_east), 3),
					180 | -180 => rotate_ninety((&mut waypoint_north, &mut waypoint_east), 2),
					270 | -90 => rotate_ninety((&mut waypoint_north, &mut waypoint_east), 1),
					_ => panic!("Invalid angle: {}", angle),
				}
			},
			_ => panic!("Invalid command"),
		}
	}

	println!("Part 2: {}", ship_north.abs() + ship_east.abs());
}