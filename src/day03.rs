use std::fs;

pub fn main() {
	let input_text = fs::read_to_string("./inputs/day03.txt").unwrap();

	let sum = input_text.lines().fold(0, |acc, bank| {
		let batteries: Vec<_> = bank.chars().map(|c| c.to_digit(10).unwrap()).collect();
		let mut left = 0;
		let mut right = 0;
		let mut stop_pos = 0;

		for (i, b) in batteries[0..batteries.len() - 1].iter().enumerate() {
			if *b > left {
				left = *b;
				stop_pos = i;
			}
		}

		for b in batteries[stop_pos + 1..batteries.len()].iter() {
			if *b > right {
				right = *b;
			}
		}

		let num: i32 = format!("{}{}", left, right).parse().unwrap();

		acc + num
	});

	println!("sum {}", sum);
}
