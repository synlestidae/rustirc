use std::sync::mpsc::{Sender, Receiver};
use message::{Message, QueueControlMessage};

pub struct RecvMessageQueue {
	recv : Receiver<Result<Message, QueueControlMessage>>
}

//pub struct SendMessageQueue {
//	sender : Sender<Result<Message, QueueControlMessage>>
//}

impl RecvMessageQueue {
	pub fn new(recv : Receiver<Result<Message, QueueControlMessage>>) -> RecvMessageQueue {
		RecvMessageQueue {
			recv : recv
		}
	}

	pub fn run(self : &mut Self) {
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