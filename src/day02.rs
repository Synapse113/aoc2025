use std::fs;

pub fn main() {
	let input_text = fs::read_to_string("./inputs/day02.txt").unwrap();
	let id_ranges = input_text.trim().split(",");
	let mut invalid = 0;

	for range in id_ranges {
		let r: Vec<&str> = range.split("-").collect();
		let start: i64 = r[0].parse().unwrap();
		let end: i64 = r[1].parse().unwrap();
		let numeric_range = start..=end;

		for n in numeric_range {
			let s = n.to_string();

			// if the number is an odd number of digits then skip
			if s.len() % 2 != 0 {
				continue;
			}

			// compare halves
			let halves = s.split_at(s.len() / 2);

			if halves.0 == halves.1 {
				invalid += n;
			}
		}
	}

	println!("sum of invalid ids: {}", invalid);
}
