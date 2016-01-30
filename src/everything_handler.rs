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

use message_util::*;
use session_state::{Session, SessionState};

const NETWORK: Token = Token(0);
const KEYBOARD: Token = Token(1);

pub struct EverythingHandler {
    pub recv : mpsc::Receiver<String>, 
    pub stream : TcpStream, 
    lines_in : Vec<String>, 
    lines_out : Vec<String>,
    session : Session
}

impl EverythingHandler {
    pub fn new(recv : mpsc::Receiver<String>, stream : TcpStream, session : Session) 
        -> EverythingHandler {
        EverythingHandler {
            recv : recv,
            stream : stream,
            lines_in : Vec::new(),
            lines_out : Vec::new(),
            session : session
        }
    }

    fn handle_action(&mut self, token : Token, event_set : EventSet) {
        let action = match token {
            NETWORK => {
                if event_set.is_readable() {
                    let mut buf = Vec::with_capacity(256);
                    for _ in 0..256 {
                        buf.push(0);
                    }

                    match self.stream.try_read(&mut buf) {
                        Ok(option) => {
                            if let Some(bytes_read) = option {
                                println!("Got: {:?}", bytes_read);
                            }
                            else {
                                println!("Hmmm... nothing in the buffer: {:?}", buf)
                            }
                        },
                        Err(err) => println!("Some error: {:?}", err)   
                    }
                }
                else if event_set.is_writable() {
                    println!("Gonna try to log write");
                    if let SessionState::Connected_LoginReady = self.session.state {
                        println!("Gonna try to log in");
                        let nick_result = self.stream.write(&nick_message(&self.session.nick).to_message_bytes_rn());
                        let name_result = self.stream.write(&user_message(&self.session.name, 
                            &self.session.name)
                            .to_message_bytes_rn());
                        self.stream.flush();
                        if nick_result.is_ok() && name_result.is_ok() {
                            self.session.state = SessionState::Connected_LoginWaiting;
                        }
                        println!("{:?} {:?}", nick_result, name_result);
                    }
                }
                else if event_set.is_hup() {
                    println!("Holy shit they hung up")
                }
                else if event_set.is_error() {
                    println!("Holy shit an error occurred")
                }
                None
            },
            KEYBOARD => {
                let line = self.recv.try_recv();
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

    fn ready(&mut self, _loop: &mut EventLoop<EverythingHandler>, token: Token, event_set : EventSet) {
        self.handle_action(token, event_set)
    }

    fn notify(&mut self, _loop: &mut EventLoop<EverythingHandler>, token: Token) {
        self.handle_action(token, EventSet::readable());
    }
}