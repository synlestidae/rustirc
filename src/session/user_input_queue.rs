use std::io;
use std::sync::mpsc::{Sender};
use session::message_queue::{AppAction};

use session::message::{Message, Prefix, Command};
use session::log::{log};

pub struct InputQueue {
	sender : Sender<AppAction>
}

fn parse_input_line(line : &String) -> Option<AppAction> {
	match parse_command(line) {
		Ok(action) => Some(action),
		Err(_) => None
	}
}

// /join
// Type /join #channelname -- to join a channel of your choice
// Example: /join #bossmom

fn parse_command(line_string : &String) -> Result<AppAction, ()>  {
	log(&format!("Parsing {}", line_string));

	let mut line : Vec<char> = line_string.chars().collect();

	let mut index = 0;

	if (index <= line.len() && line[index] == '/') {
		index += 1;
	}else{
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

	log(&format!("Command is {}", command));

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

		let action = AppAction::Transmit(message);


		log("Finished parsing");
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
			match stdin_obj.read_line(&mut input) {
			    Ok(n) => {
			        let message = parse_input_line(&input);
			        if (message.is_some()) {
			        	self.sender.send(message.unwrap());

			        }else{
			        	println!("Invalid command sequence {}", input);
			        }
			    }
			    Err(error) => {},
			}
		}
	}
}