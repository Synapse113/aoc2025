use std::fs;

pub fn main() {
	let input_text = fs::read_to_string("./inputs/day05.txt").unwrap();
	let (ranges, ingredients) = input_text.trim().split_once("\n\n").unwrap();
	let real_ranges: Vec<_> = ranges
		.lines()
		.map(|l| {
			let (start, end) = l.split_once('-').unwrap();

			start.parse::<i64>().unwrap()..end.parse::<i64>().unwrap()
		})
		.collect();

	let mut num_fresh = 0;

	for i in ingredients.lines() {
		for r in real_ranges.clone() {
			if r.contains(&i.parse().unwrap()) {
				num_fresh += 1;
				break;
			}
		}
	}

	println!("{}", num_fresh);
}
