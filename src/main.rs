use std::env::args;
use std::net::TcpStream;
use std::io::{Read, Write};
use std::io::{BufReader, BufWriter};
use std::io::{BufRead};
use std::thread;
use std::sync::mpsc::channel;
use std::sync::mpsc::Sender;
use std::io;

mod main_loop;
mod everything_handler;
mod util;
mod message;
mod message_util;
mod app_action;

extern crate mio;
extern crate log;

fn main() {
	main_loop::main();
}