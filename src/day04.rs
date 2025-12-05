use std::fs;

type CharGrid = Vec<Vec<char>>;

fn get_in_bounds(grid: &CharGrid, i: i32, j: i32) -> i32 {
	if let (Ok(i), Ok(j)) = (usize::try_from(i), usize::try_from(j)) {
		if i < grid.len() && j < grid[0].len() && grid[i][j] == '@' {
			return 1;
		}
	}

	0
}

fn count_neighbors(grid: &CharGrid, i: i32, j: i32) -> i32 {
	let mut count = 0;

	count += get_in_bounds(grid, i - 1, j - 1); // top left
	count += get_in_bounds(grid, i - 1, j); // top mid
	count += get_in_bounds(grid, i - 1, j + 1); // top right
	count += get_in_bounds(grid, i, j - 1); // mid left
	count += get_in_bounds(grid, i, j + 1); // mid right
	count += get_in_bounds(grid, i + 1, j - 1); // bottom left
	count += get_in_bounds(grid, i + 1, j); // bottom mid
	count += get_in_bounds(grid, i + 1, j + 1); // bottom right

	count
}

pub fn main() {
	let mut count = 0;
	let board_text = fs::read_to_string("./inputs/day04.txt").unwrap();
	let mut grid: CharGrid = board_text.lines().map(|l| l.chars().collect()).collect();
	let mut new_grid = grid.clone();
	let mut removed = true;

	while removed {
		removed = false;

		for i in 0..grid.len() {
			for j in 0..grid[0].len() {
				if grid[i][j] == '.' {
					continue;
				}

				let neighbor_count = count_neighbors(&grid, i as i32, j as i32);

				if neighbor_count < 4 {
					new_grid[i][j] = '.';
					count += 1;
					removed = true;
				}
			}
		}
		grid = new_grid.clone();
	}

	println!("{}", count);
}
