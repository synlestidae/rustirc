use mio::*;//{EventLoop, Handler, EventSet};
use mio::unix::PipeReader; 
use std::os::unix::io::FromRawFd;

const NETWORK: Token = Token(0);
const KEYBOARD: Token = Token(1);

pub fn main() {
    let mut stdin_pipe;
    unsafe {
        stdin_pipe = PipeReader::from_raw_fd(0);
    }

    let mut event_loop = EventLoop::new().unwrap();
    event_loop.run(&mut EverythingHandler).unwrap();
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum EventToken {
    Network,
    Keyboard
}

struct EverythingHandler;
impl Handler for EverythingHandler {
    type Message = ();
    type Timeout = ();
    fn ready(&mut self, _loop: &mut EventLoop<EverythingHandler>, token: Token, _: EventSet) {
        match token {
            NETWORK => {
            },
            KEYBOARD => {
            },
            _ => {}
        }
    }
}
