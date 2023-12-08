use crate::types::Solution;

fn value(c: char) -> i32 {
	return match c {
		'2' => 0,
		'3' => 1,
		'4' => 2,
		'5' => 3,
		'6' => 4,
		'7' => 5,
		'8' => 6,
		'9' => 7,
		'T' => 8,
		'J' => 9,
		'Q' => 10,
		'K' => 11,
		'A' => 12,
		_ => unreachable!(),
	};
}

fn qual(hand: &str) -> i32 {
	let mut freq = vec![0; 13];

	for c in hand.chars() {
		freq[value(c) as usize] += 1;
	}

	let mut kinds = vec![0; 6];

	for x in freq {
		kinds[x] += 1;
	}

	let mut res = 0;

	if kinds[5] == 1 {
		res = 6;
	} else if kinds[4] == 1 {
		res = 5;
	} else if kinds[3] == 1 && kinds[2] == 1 {
		res = 4;
	} else if kinds[3] == 1 {
		res = 3
	} else if kinds[2] == 2 {
		res = 2;
	} else if kinds[2] == 1 {
		res = 1;
	}

	res *= 100000000;

	for (i, x) in hand.chars().rev().enumerate() {
		res += value(x) * 13_i32.pow(i as u32);
	}

	res
}

fn part1(input: &str) -> String {
	let mut turns: Vec<(&str, i32)> = input
		.lines()
		.map(|line| {
			let split = line.split_once(" ").unwrap();

			(split.0, split.1.parse().unwrap())
		})
		.collect();

	turns.sort_by(|a, b| qual(a.0).cmp(&qual(b.0)));

	let mut ans = 0;

	for (i, turn) in turns.into_iter().enumerate() {
		ans += turn.1 * (i + 1) as i32;
	}

	ans.to_string()
}

pub const SOLUTION: Solution = Solution {
	part1: Some(&part1),
	part2: None,
};
