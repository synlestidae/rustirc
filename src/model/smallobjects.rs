#[derive(Debug, Clone)]
pub struct User {
	pub nick : String
}

#[derive(Debug, Clone)]
pub struct MessageLine {
	pub sender : User,
	pub body : String
}

pub type ChannelName = String;