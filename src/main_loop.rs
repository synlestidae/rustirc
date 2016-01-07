use mio::*;//{EventLoop, Handler, EventSet};

use std::sync::mpsc;
use std::thread;
use std::io;
use std::io::{BufWriter, BufRead, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::env;
use std::path::PathBuf;

use session::parse_input_line;
use model::{MessageProcessor, IrcSession};

const NETWORK: Token = Token(0);
const KEYBOARD: Token = Token(1);

pub fn main() {
    let nick = prompt("Please enter a nick: ");
    let name = prompt("Please enter your name: ");

    let mut args : Vec<_> = env::args().collect();
    if args.len() == 3 {
        let host = &args[1];
        let port = &args[2];
        if let Ok(port_int) = port.parse::<u16>() {
            return start_session(host, port_int, &nick, &name);    
        }
        else {
            println!("Port must be valid 16-bit integer")
        }
    }
    else {
        println!("Usage: {} HOST PORT", env::current_exe().unwrap_or(PathBuf::from("this_program")).display()); 
    }
} 

fn start_session(host_name : &str, port_number : u16, nick : &str, name : &str) {
    let (keyboard_tx, keyboard_rx): (mpsc::Sender<String>, mpsc::Receiver<String>) 
        = mpsc::channel();
    
    let mut event_loop = EventLoop::new().unwrap();
    let mut handler = EverythingHandler(keyboard_rx);
    let handler_channel = event_loop.channel();

    println!("Connecting to {} on port {}", host_name, port_number);

    let mut connection_result = TcpStream::connect((host_name, port_number));
    let connection = match connection_result {
        Ok(connection_) => connection_,
        Err(error) => {
            println!("Failed to connect: {}", error);
            return;
        }
    };

    thread::spawn(move || {
        loop {
            let stdin = io::stdin();
            let mut line = String::new();
            stdin.lock().read_line(&mut line);
            //have to send the text content first
            //since event queue doesn't block
            keyboard_tx.send(line);
            handler_channel.send(KEYBOARD);
        }
    });

    let session = IrcSession::new(nick);
    let mut processor = MessageProcessor::new(session, BufWriter::new(connection.try_clone().unwrap()));

    event_loop.run(&mut handler).unwrap();
}

struct EverythingHandler(mpsc::Receiver<String>);
impl EverythingHandler {
    fn handle_action(&mut self, token : Token) {
        let action = match token {
            NETWORK => {
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