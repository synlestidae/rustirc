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

fn main() {
	let host = "irc.freenode.net";
	let port = 6667;

	let connection_string = (host,port)	;

	println!("Welcome friend! Connecting to...");

	let mut stream_connect = TcpStream::connect(connection_string);

	let nick = String::from("p3nny54");

	match stream_connect {
		Err(_) => println!("Failed to connect. Goodbye."),
		Ok(mut tcpstream) => { println!("Connected!"); begin_chatting(nick, &mut tcpstream); }
	}
}

fn begin_chatting(nickname : String, stream : &mut TcpStream) {
	let servername = String::from("irc.freenode.net");
	let user = nickname.clone();

	let nick_message = nick_message(servername.clone(), nickname);
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
	
	let mut session = 

	thread::spawn(move || {
		loop {
			let mut line = String::new();

			socket_reader.read_line(&mut line);
			
			let message = parse_message(&line).unwrap();

			log(&format!("{:?}", message));

			match message.command {
				Command::LetterCommand {
					command : command
				} => {
				if command.to_lowercase() == "ping" {
					log(&format!("Ponging..."));
					action_tx.send(AppAction::Transmit(
						Message {
							prefix : message.prefix,
							command : Command::LetterCommand {
								command : "PONG".to_string()
							},
							parameters : Vec::new()
						}));	
				}else if command.to_lowercase() == "names" {

					}
				},
				Command::DigitCommand {command : numeric}=> {
					match numeric.as_ref() {
						"401" => println!("No such username"),
						"403" => println!("Server name does not exist"),
						"404" => println!("That channel does not exist"),
						"405" => println!("You have joined too many channels"),
						_ => println!("Couldn't work out the numeric command from server: {}", numeric)
					}
				} 
			}
		}
	});

	writingQueue.run();
}