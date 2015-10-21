use model::smallobjects::{User, MessageLine, ChannelName};

struct IrcSession {
	active_channels : Vec<IrcChannel>
}

struct IrcChannel {
	users : Vec<User>,
	messages : Vec<MessageLine>,
	channelName : ChannelName
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

	pub fn new_user(user : User) {
		panic!("Not implemented");
	}

	pub fn new_users(users : Vec<User>) {
		panic!("Not implemented");
	}
}