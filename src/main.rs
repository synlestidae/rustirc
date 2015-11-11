use std::env::args;
use std::net::TcpStream;
use std::io::{Read, Write};
use std::io::{BufReader, BufWriter};
use std::io::{BufRead};
use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::io;

mod session;
mod model;

use session::message::{Message, Prefix, Command, QueueControlMessage, nick_message, user_message, join_channel_message};
use session::message_queue::{AppAction, WritingQueue};
use session::user_input_queue::{InputQueue};
use session::message_parser::{parse_message};
use session::log::{log};

use model::ircsession::{IrcSession, IrcChannel};
use model::message_processor::{MessageProcessor};

fn prompt(prompt_text : &str) -> String {
	let stdin = io::stdin();
	let mut stdout = io::stdout();

	print!("{}", prompt_text);
	stdout.flush();

	let mut line = String::new();

	stdin.lock().read_line(&mut line);

	line = line.trim().to_string();
	return line;
}

fn main() {
	let host = "irc.freenode.net";
	let port = 6667;

	let connection_string = (host,port)	;

	println!("Welcome friend! Connecting to {} on port {}...", host, port);

	let mut stream_connect = TcpStream::connect(connection_string);

	let mut nick = prompt("Please enter your nick: ").to_string();

	match stream_connect {
		Err(_) => println!("Failed to connect. Goodbye."),
		Ok(mut tcpstream) => { println!("Connected!"); begin_chatting(nick, &mut tcpstream); }
	}
}

fn begin_chatting(nickname : String, stream : &mut TcpStream) {
	let servername = String::from("irc.freenode.net");
	let user = nickname.clone();

	let nick_message = nick_message(servername.clone(), nickname.clone());
	let user_message = user_message(servername.clone(), user, String::from("Harry Potter"));

	let (action_tx, action_rx) = channel::<AppAction>();

	let mut socket_reader = BufReader::new(stream.try_clone().unwrap());
	let mut socket_writer = BufWriter::new(stream.try_clone().unwrap());

	let mut writingQueue = WritingQueue {
		receiver : action_rx,
		stream : socket_writer
	};

	let mut action_tx_clone = action_tx.clone();

	thread::spawn(|| {
		InputQueue::new(action_tx_clone).run();
	});

	action_tx.send(AppAction::Transmit(nick_message));
	action_tx.send(AppAction::Transmit(user_message));
	
	let mut session_processor = MessageProcessor::new(action_tx, IrcSession::new(&nickname));

	thread::spawn(move || {
		loop {
			let mut line = String::new();

			socket_reader.read_line(&mut line);
			
			let message = parse_message(&line).unwrap();

			session_processor.process_message(&message);
		}
	});

	writingQueue.run();
}