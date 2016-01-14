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

use everything_handler::EverythingHandler;

use message_util::{nick_message, user_message};

const NETWORK: Token = Token(0);
const KEYBOARD: Token = Token(1);

pub fn main() {
    let mut args : Vec<_> = env::args().collect();
    if args.len() == 3 || args.len() == 2{
        let host = &args[1];
        let port = match args.get(2) {None => format!("6667"), Some(n) => n.to_string()};  
        if let Ok(port_int) = port.parse::<u16>() {
            return start_session(&host, port_int, 
                        &prompt("Please enter a nick: "), 
                        &prompt("Please enter your name: "));         
        }
        else {
            println!("Port must be valid 16-bit integer")
        }
    }
    else {
        println!("Usage: {} HOST PORT", env::current_exe().unwrap_or(PathBuf::from("this_program")).display())
    }
} 

fn start_session(host : &str, port_number : u16, nick : &str, name : &str) {
    let (keyboard_tx, keyboard_rx): (mpsc::Sender<String>, mpsc::Receiver<String>) 
        = mpsc::channel();

    println!("Connecting on port {}", port_number);

    let connection_string : &str = &format!("{}:{}", host, port_number);

    let mut connection_result = TcpStream::connect(&connection_string.to_socket_addrs().unwrap().nth(0).unwrap());
    let mut connection = match connection_result {
        Ok(connection_) => connection_,
        Err(error) => {
            println!("Failed to connect: {}", error);
            return;
        }
    };

    println!("Setting up your session...");

    let mut event_loop = EventLoop::new().unwrap();
    let handler_channel = event_loop.channel();

    let mut handler = EverythingHandler(keyboard_rx, 
        Box::new(connection.try_clone().unwrap()));

    thread::spawn(move || {
        loop {
            let stdin = io::stdin();
            let mut line = String::new();
            stdin.lock().read_line(&mut line);
            keyboard_tx.send(line);
            handler_channel.send(KEYBOARD);
        }
    });

    event_loop.register(&connection, NETWORK, EventSet::readable(),
                    PollOpt::level() | PollOpt::oneshot()).unwrap();

    connection.write(&nick_message(nick).to_message_bytes_rn());
    connection.write(&user_message(name, name).to_message_bytes_rn());
    connection.flush();

    println!("Connected!");

    event_loop.run(&mut handler).unwrap();
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