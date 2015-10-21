pub struct User {
	nick : String
}

pub struct MessageLine {
	sender : User,
	body : String
}

pub type ChannelName = String;