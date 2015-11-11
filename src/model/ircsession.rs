use model::smallobjects::{User, MessageLine, ChannelName};

pub struct IrcSession {
	pub me : User,
	active_channels : Vec<IrcChannel>,
	active_channel : Option<String>
}

pub struct IrcChannel {
	users : Vec<User>,
	messages : Vec<MessageLine>,
	channelName : ChannelName
}

impl IrcSession {
	pub fn new(my_username : &String) -> IrcSession {
		IrcSession {
			active_channels : Vec::new(),
			active_channel : None,
			me : User { nick : my_username.clone()
			}
		}
	}

	pub fn clear_users(self : &mut Self, channel_name : &ChannelName) {
		match (self._get_channel_index(&channel_name)) {
			Some(i) => self.active_channels[i].users = Vec::new(),
			None => {},
		}
	}

	pub fn add_users(self : &mut Self, channel_name : &ChannelName, 
		users : &Vec<User>) {
			match (self._get_channel_index(channel_name)) {
				Some(i) => {
					let mut channel_users = &mut self.active_channels[i].users;
					channel_users.append(&mut users.clone());
				},
				None => {}	
		}
	}

	pub fn set_active_channel(self : &mut Self, channel : &String) {
		self.active_channel = Some(channel.clone());
		println!("Joined {}", channel);
	}

	pub fn handle_message(self : &mut Self, target : &String, message_text : &String) {
		println!("{}: {}", target, message_text);
		/*match self._get_channel_index(target) {
			Some(i) => {
				self.active_channels[i].messages.push(MessageLine {
					sender : target.clone(),
					body : message_text.clone()
				})
			},
			None => {}
		}*/
 	}

	fn _get_channel_index(self : &mut Self, channel_name : &ChannelName) -> Option<usize> {
		let mut index = 0; 
		let mut index_of_channel : Option<usize> = None;

		for channel in self.active_channels.iter() {
			if (&channel.channelName == channel_name) {
				return Some(index);
			}
			index += 1;
		}

		return None;
	}

	fn _render(self : &mut Self) {

	}
}

impl IrcChannel {
	pub fn new(name : String) -> IrcChannel {
		return IrcChannel {
			users : Vec::new(),
			messages : Vec::new(),
			channelName : name
		}
	}

	pub fn add_message(line : MessageLine) {
		panic!("Not implemented");
	}

	pub fn add_user(user : User) {
		panic!("Not implemented");
	}

	pub fn add_users(channel : ChannelName, users : Vec<User>) {
		panic!("Not implemented");
	}

	pub fn remove_user(user : User) {
		panic!("Not implemented");
	}

	pub fn remove_users(user : Vec<User>) {

	}
}