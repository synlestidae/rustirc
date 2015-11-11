use std::fmt;

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
	pub fn to_string(self : &Self) -> String {
		let prefix = self.prefix_string();
		let command = self.command_string();
		let parameters = self.params_string();


		if prefix.len() == 0 {
			let formatted_message = format!("{} {}\r\n", command, parameters);

			print!("{}", formatted_message);

			return formatted_message;
		}else{
			let formatted_message = format!("{} {} {}\r\n", prefix, command, parameters);

			print!("{}", formatted_message);

			return formatted_message;
		}
	} 

	pub fn to_message_bytes(self : &Self) -> Vec<u8> {
		return self.to_string().into_bytes();
	}

	fn prefix_string(self:&Self) -> String {
		let mut string = String::new();
		match self.prefix {
			Some(Prefix::ServerNamePrefix{ref name}) => string = format!(":{} ", name),
			_ => return String::new()
		}
		return string;
	}

	fn command_string(self : &Self) -> String {
		match self.command {
			Command::LetterCommand {ref command} => command.clone().to_uppercase(),
			Command::DigitCommand {ref command} => panic!("Not ready for digit commands yet")
		}
	}

	fn params_string(self : &Self) -> String {
		if self.parameters.len() > 0 {
			let mut out_string = String::new();
			for (i, p) in self.parameters.iter().enumerate() {
				if (i != 0) {
					out_string.push(' ');
				}

				if (i + 1 == self.parameters.len() && p.contains(" ")) {
					out_string.push_str(&format!(":{}", p));
				}else{
					out_string.push_str(&format!("{}", p));
				}
			}

			return out_string;
		}else{
			return String::new();
		}
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

/*impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    	match self {
    		ref Command::LetterCommand {command : command} => write!(f, "{}", command),
    		ref Command::DigitCommand {command : command} => write!(f, "{}", command)
    	}
    }
}*/

pub fn user_message(servername : String, user : String, realname : String) -> Message {
	Message {
		//prefix : Some(Prefix::ServerNamePrefix{
		//	name : servername
		//}),
		prefix : None,
		command : Command::LetterCommand {
			command : String::from("USER")
		},
		parameters : vec![user, String::from("*"), String::from("8"), realname]
	}
}

pub fn nick_message(servername : String, nick : String) -> Message {
	Message {
		//prefix : Some(Prefix::ServerNamePrefix{
		//	name : servername
		//}),
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