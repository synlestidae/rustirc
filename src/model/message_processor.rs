use model::smallobjects::User;
use model::ircsession::{IrcSession, IrcChannel};
use std::collections::HashMap;
use std::sync::mpsc::Sender;
use session::message_queue::{AppAction};

use session::message::{Message, Command, Prefix};

pub struct MessageProcessor {
	session : IrcSession,
	channels_users : HashMap<String, Vec<User>>,
	sender : Sender<AppAction>
}

impl MessageProcessor {

	pub fn new(sender : Sender<AppAction>, session : IrcSession) -> MessageProcessor {
		MessageProcessor {
			channels_users : HashMap::new(),
			session : session,
			sender : sender
		}
	}

	pub fn process_message(self : &mut Self, message_in : &Message) -> bool {
		let mut message = message_in.clone();
		//println!("Message: {:?}", message);
		match message.command {
			Command::LetterCommand {
				command : ref command_string
			} => {
				//do something with letter command
				let command_str = command_string.to_lowercase();
				if (command_str == "privmsg") {
					self.process_private_message(message);
				}else if (command_str == "join") {
					self.join_channel(&message.parameters);
				}
			},
			Command::DigitCommand {command : ref numeric}=> {
				match numeric.as_ref() {
					"353" => {
						//parse list of names
						self.process_names_list(&mut message.parameters.clone());
					},
					"366" => {
						self.flush_names();
						//self.temp_name_list = Vec::new();
					},
					"401" => println!("No such username"),
					"403" => println!("Server name does not exist"),
					"404" => println!("That channel does not exist"),
					"405" => println!("You have joined too many channels"),
					_ => println!("Couldn't work out command from server: '{}'", numeric)
				}
			} 
		}
		return false;
	}

	fn process_names_list(self : &mut Self, names : &mut Vec<String>) {
		if (names.len() < 3) {
			return;
		}

		let channel = &names[2];
		let mut nicks = Vec::new();

		for nick in names[3].split(" ") {
			nicks.push(User {
				nick : nick.to_string()
			});
		}

		match self.channels_users.get_mut(channel) {
			None => {
				//do nothing
			}
			Some(existingList) => {
				existingList.append(&mut nicks);
				return;
			}
		}

		self.channels_users.insert(channel.clone(), nicks);
	}

	fn flush_names(self : &mut Self) {
		for (channel, users) in self.channels_users.iter() {
			self.session.clear_users(channel);
			self.session.add_users(channel, &users);
		}
	}

	fn join_channel(self : &mut Self, parameters : &Vec<String>) {
		if parameters.len() < 1 {
			return;
		}

		let chan = &parameters[0];

		self.session.set_active_channel(chan);

		self.sender.send(AppAction::Transmit(Message {
			command : Command::LetterCommand{command : "NAMES".to_string()},
			parameters : vec![chan.clone()],
			prefix : None
		}));
	}

	fn process_private_message(self : &mut Self, message : &Message) {
		if message.parameters.len() < 2 {
			return;
		}

		println!("Params: {:?}", message);

		let params = &message.parameters;

		match message.prefix {
			Some(Prefix::ServerNamePrefix{name : prefix}) => {
				let mut name = String::new();
				let mut index : usize = 0;
				while (prefix[index] != '!') {
					name.push(prefix[index]);
				} 
				self.session.handle_message(&name, &params[params.len() - 1]);

			},
			_ => {}
		}
	}

}