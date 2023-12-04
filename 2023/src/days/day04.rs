use crate::types::Solution;
use std::collections::HashSet;

fn part1(input: &str) -> String {
	let mut ans = 0;

	for line in input.lines() {
		let parsed = line.split_once(": ").unwrap().1;
		let card = parsed.split_once(" | ").unwrap();

		let mut winning: HashSet<i32> = HashSet::new();
		for c in card.0.split(" ").filter(|c| !c.is_empty()) {
			winning.insert(c.parse().unwrap());
		}

		let mut count = 0;
		for c in card.1.split(" ").filter(|c| !c.is_empty()) {
			if winning.contains(&c.parse().unwrap()) {
				count += 1;
			}
		}

		ans += if count == 0 { 0 } else { 1 << (count - 1) };
	}

	ans.to_string()
}

fn part2(input: &str) -> String {
	let mut ans = 0;
	let mut dp = [1; 1000];

	for (i, line) in input.lines().rev().enumerate() {
		let parsed = line.split_once(": ").unwrap().1;
		let card = parsed.split_once(" | ").unwrap();

		let mut winning: HashSet<i32> = HashSet::new();
		for c in card.0.split(" ").filter(|c| !c.is_empty()) {
			winning.insert(c.parse().unwrap());
		}

		let mut count = 0;
		for c in card.1.split(" ").filter(|c| !c.is_empty()) {
			if winning.contains(&c.parse().unwrap()) {
				count += 1;
			}
		}

		for j in (i - count)..i {
			dp[i] += dp[j];
		}

		ans += dp[i];
	}

	ans.to_string()
}

pub const SOLUTION: Solution = Solution {
	part1: Some(&part1),
	part2: Some(&part2),
};
