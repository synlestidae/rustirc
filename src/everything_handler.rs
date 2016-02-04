use mio::*;
use mio::tcp::TcpStream;

use std::sync::mpsc;
use std::thread;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Write, Read};
use std::iter;
use std::net::{ToSocketAddrs};
use std::env;
use std::path::PathBuf; 
use std::net::{SocketAddr};
use std::str;
use std::str::{FromStr};

use message_util::*;
use session_state::{Session, SessionState};

use std::mem;
use std::iter::IntoIterator;

const NETWORK: Token = Token(0);
const KEYBOARD: Token = Token(1);

pub struct EverythingHandler {
    pub recv : mpsc::Receiver<String>, 
    pub stream : TcpStream, 
    lines_in : Vec<String>, 
    lines_out : Vec<String>,
    session : Session,
    buf : Vec<u8>,
    line_in_progress : Vec<u8>
}

impl EverythingHandler {
    pub fn new(recv : mpsc::Receiver<String>, stream : TcpStream, session : Session) 
        -> EverythingHandler {
        let mut buf = Vec::with_capacity(256);
            for _ in 0..256 {
                buf.push(0);
        };
        EverythingHandler {
            recv : recv,
            stream : stream,
            lines_in : Vec::new(),
            lines_out : Vec::new(),
            session : session,
            buf : buf,
            line_in_progress : Vec::new()
        }
    }

    fn handle_action(&mut self, token : Token, event_set : EventSet) {
        let action = match token {
            NETWORK => {
                if event_set.is_readable() {
                    let mut buf = &mut self.buf;
                    match self.stream.try_read(buf) {
                        Ok(option) => {
                            if let Some(bytes_read) = option {
                                self.line_in_progress.extend(buf[0..bytes_read].iter());
                            }

                            let message_bits = process_str(&self.line_in_progress);
                        },
                        Err(err) => {}   
                    }
                }
                else if event_set.is_writable() {
                    if let SessionState::Connected_LoginReady = self.session.state {
                        let nick_result = self.stream.write(&nick_message(&self.session.nick).to_message_bytes_rn());
                        let name_result = self.stream.write(&user_message(&self.session.name, 
                            &self.session.name)
                            .to_message_bytes_rn());
                        self.stream.flush();
                        if nick_result.is_ok() && name_result.is_ok() {
                            self.session.state = SessionState::Connected_LoginWaiting;
                        }
                    }
                }
                else if event_set.is_hup() {
                    //TODO Need to recconnect or exit here
                }
                else if event_set.is_error() {
                    //TODO need to handle the error here.
                }
            },
            KEYBOARD => {
                let line = self.recv.try_recv();
                if line.is_ok() {
                    let action = Some(parse_input_line(&line.unwrap()));
                }
                else {
                    
                }
            },
            _ => {}
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

fn process_str(data_in : &Vec<u8>) -> (Vec<String>, Vec<u8>) {
    let mut state : bool = false; 
    let mut out_str = Vec::new();
    let mut buf = Vec::new();
    for b in data_in {
        match (b, state) {
            (&10, false) => {
                state = true;
                let mut buf_for_string = Vec::new();
                mem::swap(&mut buf_for_string, &mut buf);
                out_str.push(String::from_utf8(buf_for_string).unwrap());
            }
            (&13, true) => {

                state = false;
            }
            (&b, _) => {
                state = false;
                buf.push(b);
            }  
        }
    }
    (out_str, buf)
}