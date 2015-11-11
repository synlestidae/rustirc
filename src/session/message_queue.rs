use std::net::TcpStream;
use std::sync::mpsc::{Sender, Receiver};
use session::message::{Message, QueueControlMessage};
use session::log::{log};
use std::io::{BufWriter};
use std::io::Write;
use std::io;

pub enum AppAction {
	Terminate,
	Transmit(Message)
}

pub struct WritingQueue {
	pub receiver : Receiver<AppAction>,
	pub stream : BufWriter<TcpStream>,
}


impl WritingQueue {
	pub fn run(self :&mut Self) {
		loop {
			match self.receiver.recv() {
				Ok(AppAction::Transmit(ref message)) => {
					println!("Transmitting command: {}", message.to_string());
					self.stream.write_all(&message.to_message_bytes()); 
					self.stream.flush();
					()
				},
				Ok(AppAction::Terminate) => {return;},
				Err(_) => {} //ignore errors
			}
		}
	}
}