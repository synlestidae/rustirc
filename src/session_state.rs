pub struct Session {
	pub nick : String,
	pub name : String,
	pub state : SessionState
}

impl Session {
	pub fn new(nick : &str, name : &str) -> Session {
		Session {
			nick : nick.to_string(),
			name : name.to_string(),
			state : SessionState::Connected_LoginReady
		}
	}
}

pub enum SessionState { 
	Nothing,
	Connected_LoginReady,
	Connected_LoginWaiting,
	Connected_LoginFailed,
	Connected_LoggedIn,
	InRoom(String)
}