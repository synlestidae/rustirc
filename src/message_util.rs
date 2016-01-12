use message::{Message, Command, Prefix};
use app_action::AppAction;

pub fn user_message(servername : String, user : String, realname : String) -> Message {
	Message {
		prefix : None,
		command : Command::LetterCommand {
			command : String::from("USER")
		},
		parameters : vec![user, String::from("*"), String::from("8"), realname]
	}
}

pub fn nick_message(servername : String, nick : String) -> Message {
	Message {
		prefix : None,
		command : Command::LetterCommand {
			command : "NICK".to_string()
		},
		parameters : vec![nick]
	}
}

pub fn join_channel_message(servername: String, channel_name : String) -> Message {
	Message {
		prefix : Some(Prefix::ServerNamePrefix {
			name : servername
		}),
		command : Command::LetterCommand {
			command : "JOIN".to_string()
		},
		parameters : vec![channel_name]
	}
}

pub fn parse_input_line(line : &str) -> Option<AppAction> {
	match parse_command(line) {
		Ok(action) => Some(action),
		Err(_) => None
	}
}

fn parse_command(line_string : &str) -> Result<AppAction, ()>  {
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