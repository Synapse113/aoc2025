use std::fs;

fn get_substrings(s: &str) -> bool {
	let halflen = (s.len() as f64 / 2.0).floor() as usize;

	for i in (0..halflen).rev() {
		let sub = &s[0..=i];

		// need to be able to divide evenly
		if s.len() % sub.len() != 0 {
			continue;
		}

		for j in 0..s.len() / sub.len() {
			let index = j * sub.len();
			let sub2 = &s[index..index + sub.len()];

			if sub2 != sub {
				break;
			}

			if index + sub.len() == s.len() {
				return true;
			}
		}
	}

	false
}

pub fn main() {
	let input_text = fs::read_to_string("./inputs/day02.txt").unwrap();
	let id_ranges = input_text.trim().split(",");
	let mut invalid = 0;

	for range in id_ranges {
		let r: Vec<&str> = range.split("-").collect();
		let start: i64 = r[0].parse().unwrap();
		let end: i64 = r[1].parse().unwrap();

		for n in start..=end {
			let s = n.to_string();

			if get_substrings(&s) {
				invalid += n;
			}
		}
	}

	println!("sum of invalid ids: {}", invalid);
}
