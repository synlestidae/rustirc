use std::io;
use std::io::{Write, Read};

pub fn redo_prompt() {
	let mut stdout = io::stdout();
	print!("> ");
	stdout.flush();
}

pub fn prompt() -> String {
	let mut stdout_obj = io::stdout();
	let mut stdin_obj = io::stdin();
	let mut input = String::new();
	print!("> ");
	stdout_obj.flush();

	match stdin_obj.read_line(&mut input) {
		Ok(n) => return input,
		Err(_) => panic!("Failed to read input from stdin")
	}
}

pub fn print_str(line_in: &str) {
	let mut line = line_in.trim();
	println!("\r{}",line);
	redo_prompt();
}

pub fn overwrite_print_str(line_in: &str) {
	print!("\r");
	print_str(line_in);
}

