use std::env::args;
use std::net::TcpStream;
use std::io::{Read, Write};
use std::io::{BufReader, BufWriter};
use std::io::{BufRead};
use std::thread;
use std::sync::mpsc::channel;
use std::io;

mod session;
mod model;

use session::message::{Message, QueueControlMessage, nick_message, user_message, join_channel_message};
use session::message_queue::{RecvMessageQueue, WritingQueue};
use session::message_parser::{parse_message};

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

fn begin_chatting(nickname : String, stream : &mut TcpStream) {
	let servername = String::from("irc.freenode.net");
	let user = String::from("ha4542");

	let nick_message = nick_message(servername.clone(), nickname);
	let user_message = user_message(servername.clone(), user, String::from("Harry Potter"));

	let (tx, rx) = channel::<Result<Message, QueueControlMessage>>();
	let (string_tx, string_rx) = channel::<Option<String>>();

	let mut reader = BufReader::new(stream.try_clone().unwrap());
	let mut writer = BufWriter::new(stream);
	let mut queue = RecvMessageQueue::new(rx);

	let mut writingQueue = WritingQueue {
		receiver : string_rx,
		stream : writer
	};

	thread::spawn(move || {
		writingQueue.run();
	});

	thread::spawn(move ||
		loop {
			let mut message_from_server = String::new();
			reader.read_line(&mut message_from_server);
			match parse_message(message_from_server) {
				Ok(message) => { tx.send(Ok(message)); ()},
				Err(_) => ()
			}
		});



	//writer.write_all(&(nick_message.to_message_bytes()));
	//writer.write_all(&(user_message.to_message_bytes()));

	//Just continue in the main thread
	let mut input = io::stdin();
	let mut line = String::new();
	input.read_line(&mut line);

	let join_message = join_channel_message(servername, line);
	writer.write_all(&(join_message.to_message_bytes()));

	loop {
		line = String::new();
		input.read_line(&mut line);
		println!("You wrote: {}", line.trim());
	}
}