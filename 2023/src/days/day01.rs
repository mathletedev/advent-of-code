use crate::types::Solution;

fn part1(input: &str) -> String {
	let mut ans = 0;

	for line in input.lines() {
		for c in line.chars() {
			if c.is_digit(10) {
				ans += c.to_digit(10).unwrap() * 10;
				break;
			}
		}

		for c in line.chars().rev() {
			if c.is_digit(10) {
				ans += c.to_digit(10).unwrap();
				break;
			}
		}
	}

	return ans.to_string();
}

fn part2(input: &str) -> String {
	let mut ans = 0;

	let digits = [
		"one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
	];

	for line in input.lines() {
		let mut first1 = (usize::MAX, 0);
		let mut last1 = first1;

		for (i, c) in line.chars().enumerate() {
			if c.is_digit(10) {
				first1 = (i, c.to_digit(10).unwrap() as usize);
				break;
			}
		}

		for (i, c) in line.chars().rev().enumerate() {
			if c.is_digit(10) {
				last1 = (i, c.to_digit(10).unwrap() as usize);
				break;
			}
		}

		let mut first2 = (usize::MAX, 0);
		let mut last2 = first2;

		for (i, digit) in digits.into_iter().enumerate() {
			match line.find(digit) {
				Some(x) => {
					if x < first2.0 {
						first2 = (x, i + 1);
					}
				}
				None => {}
			}

			match line
				.chars()
				.rev()
				.collect::<String>()
				.find(&digit.chars().rev().collect::<String>())
			{
				Some(x) => {
					if x < last2.0 {
						last2 = (x, i + 1)
					}
				}
				None => {}
			}
		}

		let first = if first1.0 < first2.0 {
			first1.1
		} else {
			first2.1
		};

		let last = if last1.0 < last2.0 { last1.1 } else { last2.1 };

		ans += first * 10 + last;
	}

	return ans.to_string();
}

pub const SOLUTION: Solution = Solution {
	part1: Some(&part1),
	part2: Some(&part2),
};
