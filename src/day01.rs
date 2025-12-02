use std::fs;

pub fn main() {
	let input_text = fs::read_to_string("./inputs/day01.txt")
		.unwrap()
		.replace("L", "-")
		.replace("R", "+");

	let count = input_text.lines().fold((50, 0), |mut acc, line| {
		let clicks: i32 = line.parse().unwrap();
		let sum: i32 = (acc.0 + clicks).rem_euclid(100);

		if sum == 0 {
			acc.1 += 1;
		}

		(sum, acc.1)
	});

	println!("{}", count.1);
}
