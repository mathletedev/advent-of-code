pub struct Solution<'a> {
	pub part1: Option<&'a dyn Fn(&str) -> String>,
	pub part2: Option<&'a dyn Fn(&str) -> String>,
}
