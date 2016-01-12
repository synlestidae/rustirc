use message::Message;

#[derive(Debug)]
pub enum AppAction {
	Terminate,
	Transmit(Message),
	UserInput(Message),
	NetworkInput(Message)
}