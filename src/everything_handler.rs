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

use message_util::parse_input_line;

const NETWORK: Token = Token(0);
const KEYBOARD: Token = Token(1);

pub struct EverythingHandler(pub mpsc::Receiver<String>, pub Box<TcpStream>);

impl EverythingHandler {
    fn handle_action(&mut self, token : Token) {
        let mut buf = Vec::with_capacity(250);
        let action = match token {
            NETWORK => {
                match (*self.1).try_read(&mut buf) {
                    Ok(_) => {
                        if (buf.len() != 0) {
                            println!("Got: {:?}", buf);
                        }
                    },
                    Err(err) => println!("Some error: {:?}", err)   
                }
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