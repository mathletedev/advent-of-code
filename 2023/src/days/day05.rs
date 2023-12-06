use crate::types::Solution;
use std::cmp::min;

fn part1(input: &str) -> String {
	let maps: Vec<&str> = input.split("\n\n").collect();

	let mut curr: Vec<i64> = maps[0]
		.split_once(": ")
		.unwrap()
		.1
		.split(" ")
		.map(|c| c.parse().unwrap())
		.collect();

	for i in 1..maps.len() {
		for x in &mut curr {
			for line in maps[i]
				.split_once("\n")
				.unwrap()
				.1
				.split("\n")
				.filter(|c| !c.is_empty())
			{
				let range: Vec<i64> = line
					.split(" ")
					.filter(|c| !c.is_empty())
					.map(|c| c.parse().unwrap())
					.collect();

				if *x >= range[1] && *x < range[1] + range[2] {
					*x += range[0] - range[1];
					break;
				}
			}
		}
	}

	let mut ans: i64 = i64::MAX;
	for x in curr {
		ans = min(ans, x);
	}

	ans.to_string()
}

pub const SOLUTION: Solution = Solution {
	part1: Some(&part1),
	part2: None,
};
