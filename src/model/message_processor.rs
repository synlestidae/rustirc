use model::smallobjects::User;
use model::ircsession::{IrcSession, IrcChannel};

use session::message::{Message, Command, Prefix};

pub struct MessageProcessor {
	session : IrcSession,
	temp_name_list : Vec<User>
}

impl MessageProcessor {

	pub fn new(session : IrcSession) -> MessageProcessor {
		MessageProcessor {
			temp_name_list : Vec::new(),
			session : session
		}
	}

	pub fn process_message(self : &mut Self, message : &mut Message) -> bool {
		match message.command {
			Command::LetterCommand {
				command : ref command_string
			} => {
				//do something with letter command
				let command_str = command_string.to_lowercase();
				if (command_str == "privmsg") {
					//display a message
				}
				//or just do nothing

			},
			Command::DigitCommand {command : ref numeric}=> {
				match numeric.as_ref() {
					"353" => {
						//parse list of names
						self.process_names_list(&mut message.parameters);
					},
					"376" => {
						//do something with values...
						println!("Current users: {:?}", self.temp_name_list);
						self.temp_name_list = Vec::new();
					},
					"401" => println!("No such username"),
					"403" => println!("Server name does not exist"),
					"404" => println!("That channel does not exist"),
					"405" => println!("You have joined too many channels"),
					_ => println!("Couldn't work out the poopy numeric command from server: '{}'", numeric)
				}
			} 
		}
		return false;
	}

	fn process_names_list(self : &mut Self, names : &mut Vec<String>) {
		if (names.len() < 2) {
			return;
		}

		for nick in names[2].split(" ") {
			self.temp_name_list.push(User {
				nick : nick.to_string()
			});
		}
	}

}