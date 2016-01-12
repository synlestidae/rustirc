use std::io;
use std::io::{BufRead, Write};

pub fn prompt(prompt_text : &str) -> String {
	let stdin = io::stdin();
	let mut stdout = io::stdout();

	print!("{}", prompt_text);
	stdout.flush();

	let mut line = String::new();

	stdin.lock().read_line(&mut line);

	line = line.trim().to_string();
	return line;
}