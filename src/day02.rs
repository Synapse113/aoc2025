use std::fs;

fn is_invalid(s: &str) -> bool {
	let halflen = s.len() / 2;

	for i in (0..halflen).rev() {
		let sub = &s[0..=i];

		if s.len() % sub.len() != 0 {
			continue;
		}

		let sub2 = sub.repeat(s.len() / sub.len());

		if sub2 == s {
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

		for n in start..=end {
			let s = n.to_string();

			if is_invalid(&s) {
				invalid_sum += n;
			}
		}
	}

	println!("sum of invalid ids: {}", invalid_sum);
}
