use std::ops::AddAssign;

#[derive(Clone)]
enum Offset {
	Neg(usize),
	Pos(usize),
}

impl AddAssign<Offset> for usize {
	fn add_assign(&mut self, other: Offset) {
		*self = match other {
			Offset::Neg(a) => *self - a,
			Offset::Pos(a) => *self + a,
		};
	}
}

impl AddAssign<Offset> for isize {
	fn add_assign(&mut self, other: Offset) {
		*self = match other {
			Offset::Neg(a) => *self - (a as isize),
			Offset::Pos(a) => *self + (a as isize),
		};
	}
}

enum Instruction {
	Nop,
	Acc,
	Jmp,
}

type InstrOffset = (Instruction, Offset, bool);

fn parse_instruction(full_instruction: &String) -> InstrOffset {
	let instruction_splitted: Vec<&str> = full_instruction.split(' ').collect();
	let offset_i: isize = instruction_splitted[1].parse().unwrap();

	let offset = if offset_i < 0 {
		Offset::Neg(offset_i.abs() as usize)
	} else {
		Offset::Pos(offset_i.abs() as usize)
	};

	let instr = match instruction_splitted[0] {
		"jmp" => Instruction::Jmp,
		"acc" => Instruction::Acc,
		_ => Instruction::Nop,
	};

	(instr, offset, false)
}

pub fn run(input: Vec<String>) {
	let mut instructions: Vec<InstrOffset> = input
		.iter()
		.map(parse_instruction)
		.collect();

	let mut pc = 0;
	let mut acc: isize = 0;
	let mut trace: Vec<(usize, isize)> = Vec::new();

	while pc < instructions.len() {
		if instructions[pc].2 == true {
			pc = trace[trace.len() - 1].0;

			match instructions[pc].0 {
				Instruction::Nop => {
					pc += instructions[pc].1.clone();
				},
				_ => {
					pc += 1;
				},
			}

			acc = trace[trace.len() - 1].1;
			trace.pop();

			// Answer: 1089
			println!("[BT] l. {}, acc={}", pc, acc);
			continue;
		}

		instructions[pc].2 = true;

		match instructions[pc].0 {
			Instruction::Jmp => {
				trace.push((pc, acc));
				println!("{} jmp {}", pc, acc);

				pc += instructions[pc].1.clone();
				continue;
			},
			Instruction::Acc => {
				acc += instructions[pc].1.clone();
				println!("{} acc {}", pc, acc);
			},
			_ => {
				println!("nop");
			}
		}

		pc += 1;
	}

	println!("acc={}", acc);
}