pub mod message;
pub mod message_queue;
pub mod message_parser;

use session::message::{Message};

use model::smallobjects::{User, MessageLine, ChannelName};
use model::ircsession::{IrcSession};

pub struct IrcSessionView {
	session : IrcSession,
	current_channel : ChannelName
}

impl IrcSessionView {
	
}