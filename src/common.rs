use std::{
	fmt::Display,
	io::{stdin, stdout, Write},
};

/// Prints `prompt` and returns the trimmed input coming from stdin.
pub fn input(prompt: &str) -> String {
	let mut input = String::new();

	print!("{}", prompt);
	stdout().flush().expect("Couldn't flush :<");

	stdin()
		.read_line(&mut input)
		.expect("Couldn't read from `stdin` :<");

	return input.trim().to_string();
}

/// Prints the items in the `iterator` along with their index and returns the index
/// chosen by the user.
///
/// # Arguments
///
/// * `prompt` - text shown to the user
/// * `iterator` - a container of items that the user needs to choose from
/// * `size` - the number of items in the `iterator`
pub fn read_index<'a, T: 'a + Display>(
	prompt: &str,
	iterator: impl Iterator<Item = &'a T>,
	size: usize,
) -> usize {
	// print the indexed items
	for (i, item) in iterator.enumerate() {
		println!("{}: {}", i, item);
	}

	// keep asking the user for a number until they enter a valid index
	loop {
		// check if the entered number can be parsed into a `u8`
		match input(prompt).parse::<u8>() {
			// break if the entered number is a valid index (i. e. a positive value less than the size of the container)
			Ok(n) if (n as usize) < size => break n.into(),

			// otherwise, display the message and loop
			_ => println!(
				"Invalid number, entered value should be between 0..={}.",
				size
			),
		}
	}
}
