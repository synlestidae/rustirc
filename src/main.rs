use std::env::args;
use std::net::TcpStream;
use std::io::{Read, Write};
use std::io::BufReader;
use std::io::BufRead;

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
		let mut out = format!("NICK {}\n", self.nickname);
		return out.as_bytes().to_owned(); 
	}
}

impl IrcMessage for UserMessage {
	fn to_message_bytes(self : &Self) -> Vec<u8> {
		let mut out = format!("USER {} 0 * :{}\n", self.username, self.real_name);
		return out.as_bytes().to_owned();
	}
}

fn begin_chatting(nickname : String, stream : &mut TcpStream) {
	let nick_message = (NickMessage::new(nickname.clone())).to_message_bytes();
	let user_message = UserMessage{username : nickname, real_name : String::from("Suzy Q")}.to_message_bytes();

	let mut message_from_server = String::new();
	stream.write_all(&nick_message);
	stream.write_all(&user_message);

	let mut reader = BufReader::new(stream);
	while (true) {
		reader.read_line(&mut message_from_server);
		println!("{}", message_from_server);
	}
}