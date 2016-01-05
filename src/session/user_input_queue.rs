use std::io;
use std::sync::mpsc::{Sender};
use session::message_queue::{AppAction};

use session::message::{Message, Prefix, Command};
use session::log::{log};

use std::io::Write;
use view::out::prompt;

pub struct InputQueue {
	sender : Sender<AppAction>
}

fn parse_input_line(line : &String) -> Option<AppAction> {
	match parse_command(line) {
		Ok(action) => Some(action),
		Err(_) => None
	}
}

fn parse_command(line_string : &String) -> Result<AppAction, ()>  {
	let mut line : Vec<char> = line_string.chars().collect();

	let mut index = 0;

	if (index <= line.len() && line[index] == '/') {
		index += 1;
	}else if (index <= line.len() && line[index] != '/') {
		//parse a message!
		return Ok(AppAction::UserInput(Message {
			prefix : None,
			command : Command::LetterCommand {
				command : "PRIVMSG".to_string()
			},
			parameters : vec![line_string.trim().to_string()]
		}));
	}
	else{
		log(&format!("String is empty or does not begin with slash"));
		return Err(());
	}

	let mut command = String::new();

	while (line[index].is_alphabetic() && line[index].is_lowercase()) {
		command.push(line[index]);
		index += 1;
	}

	if (command.len() == 0) {
		return Err(());
	}

	eat_char(&mut line, ' ', &mut index);

	if (command == "join".to_string()) {
		let mut channel = String::new();

		while (index < line.len()) {
			channel.push(line[index]);
			index += 1;
		}
		
		let message = Message {
			prefix : None,
			command : Command::LetterCommand {command : command},
			parameters : vec![channel]
		};

		let action = AppAction::UserInput(message);

		return Ok(action);
	}else{
		log(&format!("Unknown command {}", command));
		return Err(());
	}


}

fn eat_char(chars : &Vec<char>, c : char, expectedIndex: &mut usize) -> Result<char, (String, usize)> {
	if chars[expectedIndex.clone()] == c {
		*expectedIndex = (*expectedIndex + 1);
		return Ok(c);
	}else{
		return Err((format!("Expected '{}', got '{}' ", c, chars[*expectedIndex]), *expectedIndex));
	}
}

impl InputQueue {
	pub fn new(sender : Sender<AppAction>) -> InputQueue {
		return InputQueue {sender : sender};
	}

	pub fn run(self : &mut Self) {
		loop {
			let mut input = String::new();
			let mut stdin_obj = io::stdin();
			let mut stdout_obj = io::stdout();
			stdout_obj.flush();

			let input = prompt();
			let message = parse_input_line(&input);

			if (message.is_some()) {
				self.sender.send(message.unwrap());
			}else{
				println!("Invalid command sequence {}", input);
			}
		}
	}
}