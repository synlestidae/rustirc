use std::net::TcpStream;
use std::sync::mpsc::{Sender, Receiver};
use session::message::{Message, QueueControlMessage};
use session::log::{log};
use std::io::{BufWriter};
use std::io::Write;
use std::io;

pub enum AppAction {
	Terminate,
	Transmit(Message),
	UserInput(Message),
	NetworkInput(Message)
}