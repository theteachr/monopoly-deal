use std::{
	fmt::Display,
	io::{stdin, stdout, Write},
};

/// Prints `prompt` and returns the trimmed input coming from stdin.
pub fn input<T: Display>(prompt: T) -> String {
	let mut input = String::new();

	print!("{}", prompt);
	stdout().flush().expect("Couldn't flush :<");

	stdin()
		.read_line(&mut input)
		.expect("Couldn't read from `stdin` :<");

	return input.trim().to_string();
}

/// Prints the items in the `iterator` along with their index.
pub fn print_indexed<'a, T: 'a + Display>(iterator: impl Iterator<Item = &'a T>) {
	for (i, item) in iterator.enumerate() {
		println!("{}: {}", i, item);
	}
}

/// Parses the user input into a valid index.
///
/// # Arguments
///
/// * `prompt` - text shown to the user
/// * `size` - the number of items in the `iterator`
pub fn read_index<T: Display>(prompt: T, size: usize) -> usize {
	// Only one right input: 0, if there's only one element in the iterator.
	if size == 1 {
		return 0;
	}
	
	// Keep asking the user for a number until they enter a valid index.
	loop {
		// Check if the entered number can be parsed into a `u8`.
		match input(&prompt).parse::<usize>() {
			// Break if the entered number is a valid index (i. e. a positive value less than the size of the container).
			Ok(n) if (n) < size => break n,

			// Otherwise, display the message and loop.
			_ => println!(
				"Invalid number, entered value should be between 0..={}.",
				size - 1
			),
		}
	}
}

/// - Prints the enumerated iterator
/// - `prompt`s the user for input, until input is a valid index
/// - Returns the index
pub fn print_read_index<'a, T: Display, U: 'a + Display>(
	prompt: T,
	iterator: impl Iterator<Item = &'a U>,
	size: usize,
) -> usize {
	print_indexed(iterator);
	read_index(prompt, size)
}
