use std::io::{stdin, stdout, Write};

/// Prints `prompt` and returns the trimmed* input coming from stdin.
/// * removes '\n' captured when enter or return is pressed
pub(crate) fn input(prompt: &str) -> String {
	let mut input = String::new();

	print!("{}", prompt);
	stdout().flush().expect("Couldn't flush :<");

	stdin()
		.read_line(&mut input)
		.expect("Couldn't read from `stdin` :<");

	return input.trim().to_string();
}
