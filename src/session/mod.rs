use message::{Message};

pub struct IrcSessionView;
impl IrcSessionView {
	pub fn new(room : &str) {
		println!("You have joined {}", room);
	}

	pub fn display_message(from : &str, saidWhat : &str) {
		println!("{}: {}", from, saidWhat);
	}
}