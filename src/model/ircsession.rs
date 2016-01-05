use model::smallobjects::{User, MessageLine, ChannelName};

use std::io;
use std::io::Write;

use view::out::{print_str};

pub struct IrcSession {
	pub me : User,
	active_channels : Vec<IrcChannel>,
	active_channel : Option<String>
}

pub struct IrcChannel {
	previous_users : Vec<User>,
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

	pub fn clear_users(self : &mut Self, channel_name : &ChannelName) -> Vec<User> {
		let mut new_users = Vec::new();
		if let Some(i) = self._get_channel_index(&channel_name) {
			for user in (&self.active_channels[i]).users {
				if !(&self.active_channels[i].previous_users).contains(&user) {
					new_users.push(user.clone());
				}
			}
			self.active_channels[i].users = Vec::new();
		}
		return new_users;
	}

	pub fn add_users(self : &mut Self, channel_name : &ChannelName, 
		users : &Vec<User>) {
		if let Some(i) = self._get_channel_index(channel_name) {
			if self.active_channels[i].previous_users.len() == 0 {
				self.active_channels[i].previous_users = self.active_channels[i].users.clone();
			}
			let mut channel_users = &mut self.active_channels[i].users;
			channel_users.append(&mut users.clone());
		}
	}

	pub fn set_active_channel(self : &mut Self, channel : &String) {
		self.active_channel = Some(channel.clone());
		print_str(&format!("Joined {}", channel))
	}

	pub fn get_active_channel(self : &Self) -> Option<String> {
		return self.active_channel.clone();
	}

	pub fn handle_message(self : &mut Self, target : &String, message_text : &String) {
		print_str(&format!("\r{}: {}", target, message_text.trim()));
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
			previous_users : Vec::new(),
			users : Vec::new(),
			messages : Vec::new(),
			channelName : name
		}
	}
}