use std::env::args;
use std::net::TcpStream;
use std::io::{Read, Write};
use std::io::{BufReader, BufWriter};
use std::io::{BufRead};
use std::thread;
use std::sync::mpsc::channel;
use std::io;

mod message;
mod message_parser;
mod message_queue;

fn main() {
	let host = "irc.freenode.net";
	let port = 6667;

	let connection_string = (host,port)	;

	println!("Welcome! Connecting to...");

	let mut stream_connect = TcpStream::connect(connection_string);

	let nick = String::from("nico676");

	match stream_connect {
		Err(_) => println!("Failed to connect. Goodbye."),
		Ok(mut tcpstream) => { println!("Connected!"); begin_chatting(nick, &mut tcpstream); }
	}
}

trait IrcMessage {
	fn to_message_bytes(self : &Self) -> Vec<u8>;
}

struct NickMessage {
	nickname : String
}

struct UserMessage {
	username : String,
	real_name : String
}

impl NickMessage {
	pub fn new(nickname : String) -> NickMessage {
		return NickMessage {nickname : nickname};
	}
}

impl IrcMessage for NickMessage {
	fn to_message_bytes(self : &Self) -> Vec<u8> {
		let mut out = format!("NICK {}\r\n", self.nickname);
		return out.as_bytes().to_owned(); 
	}
}

impl IrcMessage for UserMessage {
	fn to_message_bytes(self : &Self) -> Vec<u8> {
		let mut out = format!("USER {} 0 * :{}\r\n", self.username, self.real_name);
		return out.as_bytes().to_owned();
	}
}

fn begin_chatting(nickname : String, stream : &mut TcpStream) {
	let servername = String::from("irc.freenode.net");
	let user = String::from("ha4542");

	let nick_message = message::nick_message(servername.clone(), nickname);
	let user_message = message::user_message(servername.clone(), user, String::from("Harry Potter"));

	let (tx, rx) = channel::<Result<message::Message, message::QueueControlMessage>>();

	let mut reader = BufReader::new(stream.try_clone().unwrap());
	let mut writer = BufWriter::new(stream);

	let mut message_from_server = String::new();
	let mut queue = message_queue::RecvMessageQueue::new(rx);

	thread::spawn(move ||
		while (true) {
			reader.read_line(&mut message_from_server);
			//Parse the message
			//Give it to the queue
			println!("{}", message_from_server);
		});

	writer.write_all(&(nick_message.to_message_bytes()));
	writer.write_all(&(user_message.to_message_bytes()));

	//Just continue in the main thread
	let mut input = io::stdin();
	let mut line = String::new();
	input.read_line(&mut line);

	let join_message = message::join_channel_message(servername, line);
	writer.write_all(&(join_message.to_message_bytes()));

	while true {
		line = String::new();
		input.read_line(&mut line);
		println!("You wrote: {}", line.trim());
	}
}