use crate::types::Solution;

fn part1(input: &str) -> String {
	let mut iter = input.lines();

	let times: Vec<i32> = iter
		.next()
		.unwrap()
		.split_once(":")
		.unwrap()
		.1
		.split(" ")
		.filter(|c| !c.is_empty())
		.map(|c| c.parse().unwrap())
		.collect();

	let dists: Vec<i32> = iter
		.next()
		.unwrap()
		.split_once(":")
		.unwrap()
		.1
		.split(" ")
		.filter(|c| !c.is_empty())
		.map(|c| c.parse().unwrap())
		.collect();

	let mut ans = 1;
	for (i, time) in times.into_iter().enumerate() {
		let mut count = 0;
		for x in 1..time {
			if x * (time - x) > dists[i] {
				count += 1;
			}
		}

		ans *= count;
	}

	ans.to_string()
}

fn part2(input: &str) -> String {
	let mut iter = input.lines();

	let time: u64 = iter
		.next()
		.unwrap()
		.split_once(":")
		.unwrap()
		.1
		.chars()
		.filter(|c| !c.is_whitespace())
		.collect::<String>()
		.parse()
		.unwrap();

	let dist: u64 = iter
		.next()
		.unwrap()
		.split_once(":")
		.unwrap()
		.1
		.chars()
		.filter(|c| !c.is_whitespace())
		.collect::<String>()
		.parse()
		.unwrap();

	let mut low: u64 = 0;
	let mut high: u64 = time;

	while low < high {
		let mid = (low + high) / 2;

		if mid * (time - mid) >= dist {
			high = mid;
		} else {
			low = mid + 1;
		}
	}

	(time - 2 * low + 1).to_string()
}

pub const SOLUTION: Solution = Solution {
	part1: Some(&part1),
	part2: Some(&part2),
};
