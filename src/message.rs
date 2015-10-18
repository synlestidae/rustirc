#[derive(Debug, Eq, PartialEq)]
pub struct Message {
	pub prefix : Option<Prefix>,
	pub command : Command,
	pub parameters : Vec<String>
}

#[derive(Debug, Eq, PartialEq)]
pub enum QueueControlMessage {
	TERMINATE
}

impl Message {
	pub fn to_message_bytes(self : &Self) -> Vec<u8> {
		let prefix = self.prefix_string();
		let command = self.command_string();
		let parameters = self.params_string();
		let formatted_message = format!("{} {} {}\r\n", prefix, command, parameters);

		print!("{}", formatted_message);

		return formatted_message.into_bytes();
	}

	fn prefix_string(self:&Self) -> String {
		let mut string = String::new();
		match self.prefix {
			Some(Prefix::ServerNamePrefix{ref name}) => string = format!(":{} ", name),
			_ => panic!("Not implemented")
		}
		return string;
	}

	fn command_string(self : &Self) -> String {
		match self.command {
			Command::LetterCommand {ref command} => command.clone(),
			Command::DigitCommand {ref command} => panic!("Not ready for digit commands yet")
		}
	}

	fn params_string(self : &Self) -> String {
		return (&(self.parameters.clone())).connect(" ");
	}
}

#[derive(Debug, Eq, PartialEq)]
pub enum Prefix {
	ServerNamePrefix {name : String},
	NickNamePrefix {name : String, parts : Option<(String, String)> }
}

#[derive(Debug, Eq, PartialEq)]
pub enum Command {
	LetterCommand {command : String},
	DigitCommand {command : String}
}

pub fn user_message(servername : String, user : String, realname : String) -> Message {
	Message {
		prefix : Some(Prefix::ServerNamePrefix{
			name : servername
		}),
		command : Command::LetterCommand {
			command : String::from("USER")
		},
		parameters : vec![user, String::from("0"), String::from("*"), realname]
	}
}

pub fn nick_message(servername : String, nick : String) -> Message {
	Message {
		prefix : Some(Prefix::ServerNamePrefix{
			name : servername
		}),
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