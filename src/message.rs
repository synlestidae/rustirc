use std::fmt;

#[derive(Debug, Eq, PartialEq)]
pub struct Message {
	pub prefix : Option<Prefix>,
	pub command : Command,
	pub parameters : Vec<String>
}

impl Message {
	pub fn to_string(self : &Self) -> String {
		let prefix = self.prefix_string();
		let command = self.command_string();
		let parameters = self.params_string();


		if prefix.len() == 0 {
			let formatted_message = format!("{} {}\r\n", command, parameters);

			return formatted_message;
		}
		else {
			let formatted_message = format!("{} {} {}\r\n", prefix, command, parameters);

			return formatted_message;
		}
	} 

	pub fn to_message_bytes(self : &Self) -> Vec<u8> {
		return self.to_string().into_bytes();
	}

	pub fn to_message_bytes_rn(self : &Self) -> Vec<u8> {
		return format!("{}\r\n", self.to_string()).into_bytes();
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

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Prefix {
	ServerNamePrefix {name : String},
	NickNamePrefix {name : String, parts : Option<(String, String)> }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum Command {
	LetterCommand {command : String},
	DigitCommand {command : String}
}