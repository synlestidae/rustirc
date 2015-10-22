use std::sync::mpsc::{Sender, Receiver};
use session::message::{Message, QueueControlMessage};
use std::io::{BufWriter};
use std::net::tcp::TcpStream;
use std::io::Write;

pub struct RecvMessageQueue {
	recv : Receiver<Result<Message, QueueControlMessage>>
}

impl RecvMessageQueue {
	pub fn new(recv : Receiver<Result<Message, QueueControlMessage>>) -> RecvMessageQueue {
		RecvMessageQueue {
			recv : recv
		}
	}

	pub fn run(self : &mut Self, output : &mut BufWriter<&mut TcpStream>) {
		loop {
			match self.recv.recv() {
				Err(_) => return, //Errors mean we terminate
				Ok(Err(QueueControlMessage::TERMINATE)) => return,
				Ok(Ok(ref message)) => {
					self.deal_with_message(&message);
				}
			}
		}
	}

	pub fn deal_with_message(self : &Self, message : &Message) {
		println!("Got some kind of message");
	}
}

pub struct WritingQueue {
	receiver : Receiver<Option<String>>,
	stream : BufWriter<TcpStream>
}


impl WritingQueue {
	pub fn run(self :&mut Self) {
		loop {
			match self.receiver.recv() {
				Ok(None) => break,
				Ok(Some(string)) => {self.stream.write_all(&string.into_bytes()); ()},
				Err(_) => {} //ignore errors
			}
		}
	}
}