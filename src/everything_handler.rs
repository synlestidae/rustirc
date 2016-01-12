use mio::*;
use mio::tcp::TcpStream;

use std::sync::mpsc;
use std::thread;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write, Read};
use std::net::{ToSocketAddrs};
use std::env;
use std::path::PathBuf; 
use std::net::{SocketAddr};
use std::str::FromStr;

use session::parse_input_line;

const NETWORK: Token = Token(0);
const KEYBOARD: Token = Token(1);

pub struct EverythingHandler(pub mpsc::Receiver<String>, pub Box<BufReader<TcpStream>>);

impl EverythingHandler {
    fn handle_action(&mut self, token : Token) {
        let action = match token {
            NETWORK => {
                let mut line = String::new();
                (*self.1).read_line(&mut line);
                None
            },
            KEYBOARD => {
                let line = self.0.try_recv();
                if line.is_ok() {
                    Some(parse_input_line(&line.unwrap()))
                }
                else {
                    None
                }
            },
            _ => None
        };
    }
}

impl Handler for EverythingHandler {
    type Message = Token;
    type Timeout = ();

    fn ready(&mut self, _loop: &mut EventLoop<EverythingHandler>, token: Token, _: EventSet) {
        self.handle_action(token)
    }

    fn notify(&mut self, _loop: &mut EventLoop<EverythingHandler>, token: Token) {
        self.handle_action(token);
    }
}