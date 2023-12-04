use crate::types::Solution;
use std::cmp::max;

fn part1(input: &str) -> String {
	let mut ans = 0;

	'outer: for (i, line) in input.lines().enumerate() {
		for cube in line.split_once(":").unwrap().1.split([',', ';']) {
			let data: Vec<&str> = cube.split(" ").collect();

			let max = match data[2] {
				"red" => 12,
				"green" => 13,
				"blue" => 14,
				_ => unreachable!(),
			};

			if data[1].parse::<i32>().unwrap() > max {
				continue 'outer;
			}
		}

		ans += (i + 1) as u32;
	}

	ans.to_string()
}

fn part2(input: &str) -> String {
	let mut ans = 0;

	for line in input.lines() {
		let mut fewest = [0, 0, 0];

		for cube in line.split_once(":").unwrap().1.split([',', ';']) {
			let data: Vec<&str> = cube.split(" ").collect();

			let index = match data[2] {
				"red" => 0,
				"green" => 1,
				"blue" => 2,
				_ => unreachable!(),
			};

			fewest[index] = max(fewest[index], data[1].parse().unwrap());
		}

		ans += fewest[0] * fewest[1] * fewest[2];
	}

	ans.to_string()
}

pub const SOLUTION: Solution = Solution {
	part1: Some(&part1),
	part2: Some(&part2),
};
