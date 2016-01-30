use mio::*;
use mio::tcp::TcpStream;

use std::io::Write;
use std::net::ToSocketAddrs;
use std::net::SocketAddr;
use std::str::FromStr;

const NETWORK: Token = Token(0);

pub struct EverythingHandler {
    pub stream: TcpStream,
    has_requested: bool,
}

impl EverythingHandler {
    pub fn new(stream: TcpStream) -> EverythingHandler {
        EverythingHandler {
            stream: stream,
            has_requested: false,
        }
    }

    fn handle_action(&mut self, token: Token, event_set: EventSet) {
        let action = match token {
            NETWORK => {
                if event_set.is_readable() {
                    let mut buf = Vec::with_capacity(256);
                    match self.stream.try_read(&mut buf) {
                        Ok(option) => {
                            if let Some(bytes_read) = option {
                                println!("Got {:?} bytes", bytes_read);
                            } else {
                                println!("Hmmm... nothing in the buffer: {:?}", buf)
                            }
                        }
                        Err(err) => println!("Some error: {:?}", err),   
                    }
                } else if event_set.is_writable() && !self.has_requested {
                    match self.stream.write(&format!("HTTP 1.1 GET /index.html").into_bytes()) {
                        Ok(_) => {
                            println!("Writing success");
                            self.has_requested = true;
                        }
                        _ => println!("Writing fail"), 
                    }
                } else if event_set.is_hup() {
                    println!("They hung up")
                } else if event_set.is_error() {
                    println!("An error occurred")
                }
            }
            _ => {}
        };
    }
}

impl Handler for EverythingHandler {
    type Message = Token;
    type Timeout = ();

    fn ready(&mut self,
             _loop: &mut EventLoop<EverythingHandler>,
             token: Token,
             event_set: EventSet) {
        self.handle_action(token, event_set)
    }
}

pub fn main() {
    let mut event_loop = EventLoop::new().unwrap();
    let stream = TcpStream::connect(&"127.0.0.1:80"
                                         .to_socket_addrs()
                                         .unwrap()
                                         .nth(0)
                                         .unwrap())
                     .unwrap();

    let mut handler = EverythingHandler::new(stream.try_clone().unwrap());
    event_loop.register(&stream,
                        NETWORK,
                        EventSet::all(),
                        PollOpt::level() | PollOpt::oneshot())
              .unwrap();

    event_loop.run(&mut handler).unwrap();
}
