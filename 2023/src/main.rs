use std::{
	fs::read_to_string,
	io::{stdin, stdout, Error, Write},
};

pub mod days;
pub mod types;

fn main() -> Result<(), Error> {
	let mut data = String::new();
	print!("Enter day: ");
	stdout().flush()?;
	stdin().read_line(&mut data)?;

	let day: u8 = data.trim().parse().unwrap();

	data = String::new();
	print!("Enter part: ");
	stdout().flush()?;
	stdin().read_line(&mut data)?;

	let part: u8 = data.trim().parse().unwrap();

	let input = read_to_string(format!("./input/day{:02}-{}.txt", day, part))?;

	let solution = match day {
		1 => days::day01::SOLUTION,
		_ => panic!(),
	};

	let ans = (if part == 1 {
		solution.part1
	} else {
		solution.part2
	})
	.unwrap()(&input);

	println!("{ans}");

	Ok(())
}