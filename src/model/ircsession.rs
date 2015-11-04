use model::smallobjects::{User, MessageLine, ChannelName};

pub struct IrcSession {
	pub me : User,
	active_channels : Vec<IrcChannel>
}

pub struct IrcChannel {
	users : Vec<User>,
	messages : Vec<MessageLine>,
	channelName : ChannelName
}

impl IrcSession {
	pub fn new(my_username : String) -> IrcSession {
		IrcSession {
			active_channels : Vec::new(),
			me : User { nick : my_username }
		}
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

	pub fn add_users(users : Vec<User>) {
		panic!("Not implemented");
	}

	pub fn remove_user(user : User) {
		panic!("Not implemented");
	}

	pub fn remove_users(user : Vec<User>) {

	}
}