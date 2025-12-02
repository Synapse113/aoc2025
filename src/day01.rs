use std::fs;

pub fn main() {
	let input_text = fs::read_to_string("./inputs/day01.txt")
		.unwrap()
		.replace("L", "-")
		.replace("R", "");

	let count = input_text.lines().fold((50, 0), |mut acc, line| {
		let clicks: i32 = line.parse().unwrap();
		let sum = (acc.0 + clicks).rem_euclid(100);
		let range = 1..=clicks.abs();
		let passes = range
			.map(|i| (acc.0 + i * clicks.signum()).rem_euclid(100))
			.filter(|&i| i == 0)
			.count();

		acc.1 += passes;

		(sum, acc.1)
	});

	println!("{}", count.1);
}
