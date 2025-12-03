use std::fs;

fn is_invalid(num: &str) -> bool {
	let halflen = num.len() / 2;

	for i in (0..halflen).rev() {
		let sub = &num[0..=i];

		if num.len() % sub.len() != 0 {
			continue;
		}

		let repeated_substring = sub.repeat(num.len() / sub.len());

		if repeated_substring == num {
			return true;
		}
	}

	false
}

pub fn main() {
	let input_text = fs::read_to_string("./inputs/day02.txt").unwrap();
	let id_ranges = input_text.trim().split(",");
	let mut invalid_sum = 0;

	for range in id_ranges {
		let r: Vec<&str> = range.split("-").collect();
		let start: i64 = r[0].parse().unwrap();
		let end: i64 = r[1].parse().unwrap();

		for num in start..=end {
			let s = num.to_string();

			if is_invalid(&s) {
				invalid_sum += num;
			}
		}
	}

	println!("sum of invalid ids: {}", invalid_sum);
}
