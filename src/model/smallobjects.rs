#[derive(Debug)]
pub struct User {
	pub nick : String
}

pub struct MessageLine {
	pub sender : User,
	pub body : String
}

pub type ChannelName = String;