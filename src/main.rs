mod day01;
mod day02;
mod day03;
mod day04;
use std::env;

fn main() {
	let day = env::args().nth(1).expect("Please specify a day to run");

	match day.as_str() {
		"day01" => day01::main(),
		"day02" => day02::main(),
		"day03" => day03::main(),
		"day04" => day04::main(),
		_ => panic!("solution for that day does not exist"),
	}
}
