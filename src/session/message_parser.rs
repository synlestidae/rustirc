#![feature(advanced_slice_patterns, slice_patterns)]
use std::char;
use std::str;
use session::message::{Message, Prefix, Command};
use session::log::{log};


pub fn parse_message(message_str : &String) -> Result<Message, (String, usize)>  {
	let chars : Vec<char> = message_str.chars().collect();

	let mut index : usize = 0;
	let mut prefix = None;

	if (chars.len() > 0 && chars[0] == ':') {
		prefix = Some(try!(parse_prefix(&chars, &mut index)));
	}

	let command = try!(parse_command(&chars, &mut index));

	let parameters = try!(parse_parameters(&chars, &mut index));

	eat_char(&chars, '\r', &mut index);
	eat_char(&chars, '\n', &mut index);

	return Ok(Message {
		prefix : prefix, 
		command : command, 
		parameters : parameters
	});
}

fn parse_prefix(message_str : &Vec<char>, index : &mut usize) -> Result<Prefix, (String, usize)> {
	//for now assume parsing a hostname
	eat_char(message_str, ':', index);
	let hostname = try!(parse_host(message_str, index));
	let result = eat_char(message_str, ' ', index);
	return Ok(Prefix::ServerNamePrefix {
		name : hostname
	});
}

fn parse_host(message_str : &Vec<char>, index : &mut usize) -> Result<String, (String, usize)> {
	let gh_index = *index;

	while message_str[*index] != ' ' {
		*index += 1;
	}

	if *index == gh_index {
		return Err((String::from("Expected hostname, got whitespace"), *index));
	}

	let mut out_host = String::new();
	for i in ((gh_index) .. (*index)).into_iter() {
		out_host.push(message_str[i].clone());
	}
	return Ok(out_host);
}

fn parse_command(message_str : &Vec<char>, index : &mut usize) -> Result<Command, (String, usize)> {
	let mut command_str = String::new();
	if message_str[*index].is_digit(10) {
		while *index < message_str.len() - 2 && (message_str[*index]).is_digit(10) {
			command_str.push(message_str[*index]);
			*index = *index + 1;
		}
		return Ok(Command::DigitCommand {
			command : command_str
		});
	}else if message_str[*index].is_alphabetic() {
		while *index < message_str.len() - 2 && message_str[*index].is_alphabetic(){
			command_str.push(message_str[*index]);
			*index = *index + 1;
		}
		return Ok(Command::LetterCommand {
			command : command_str
		});
	}else{
		return Err((String::from("Character was not digit or alphabetic character"), *index));
	}
}

fn parse_parameters(message_str : &Vec<char>, index : &mut usize) -> Result<Vec<String>, (String, usize)> {
		let mut param_list = Vec::new();
		match parse_parameters_rec(message_str, index, &mut param_list) {
			Ok(vec) => {
				let result = (*vec).clone(); //cheating
				return Ok(result);
			},
			Err(e) => return Err(e) 
		}
}

fn parse_parameters_rec<'a>(message_str : &Vec<char>, 
	index : &mut usize, parsedParams : &'a mut Vec<String>) -> Result<&'a Vec<String>, (String, usize)> {
	if *index >= message_str.len() - 2 {
			return Ok(parsedParams);
	}
	eat_char(message_str, ' ', index);
	if (message_str[*index] == ':') {
		eat_char(message_str, ':', index);
		return parse_trailing_parameters_rec(message_str, index, parsedParams);
	}
	let middle = try!(parse_middle(message_str, index));
	parsedParams.push(middle);
	if *index < message_str.len() - 2 {
		return parse_parameters_rec(message_str, index, parsedParams);
	}
	return Ok(parsedParams);
}

fn parse_trailing_parameters_rec<'a>(message_str : &Vec<char>, 
	index : &mut usize, parsedParams : &'a mut Vec<String>) -> Result<&'a Vec<String>, (String, usize)> {
	let mut param = String::new();
	while *index < message_str.len() - 2 && (is_valid_middle_char(message_str[*index]) || message_str[*index] == ' ' || message_str[*index] == ':') {
		param.push(message_str[*index]);
		*index += 1;
	}
	parsedParams.push(param);
	Ok(parsedParams)
}

fn parse_middle(message_str : &Vec<char>, index : &mut usize) -> Result<String, (String, usize)> {
	let mut middle = String::new();
	while *index < message_str.len() - 2 && is_valid_middle_char(message_str[*index]) {
		middle.push(message_str[*index]);
		*index += 1;
	}
	if middle.len() == 0 {
		return Err(("middle is empty".to_string(), *index));
	}
	return Ok(middle);
}

fn is_valid_middle_char(c : char) -> bool {
	return  c != '\r' && c != '\n' && c != ' ' && c != ':';
}

fn eat_char(chars : &Vec<char>, c : char, expectedIndex: &mut usize) -> Result<char, (String, usize)> {
	if chars[expectedIndex.clone()] == c {
		*expectedIndex = (*expectedIndex + 1);
		return Ok(c);
	}else{
		return Err((format!("Expected '{}', got '{}' ", c, chars[*expectedIndex]), *expectedIndex));
	}
}

mod message_parser_tests {
	use super::{parse_message};
	use session::message::{Message, Prefix, Command};

	#[test]
	fn test_nickmessage_parses_1() {
		let messageStr = String::from(":irc.freenode.net NICK nico676\r\n");
		let host = String::from("irc.freenode.net");
		let command = String::from("NICK");
		let message = parse_message(messageStr).unwrap();
		assert_eq!(Command::LetterCommand {command : command}, message.command);
		assert_eq!(Prefix::ServerNamePrefix {name : host.clone()}, message.prefix.unwrap());
		assert_eq!("nico676", &message.parameters[0]);
	}

	#[test]
	fn test_nickmessage_parses_2() {
		let message = String::from(":someserver.net NICK nico676\r\n");
		let host = String::from("someserver.net");
		let message = parse_message(message).unwrap();
		assert_eq!(Command::LetterCommand {command : "NICK".to_string()}, message.command);
		assert_eq!(Prefix::ServerNamePrefix {name : host}, message.prefix.unwrap());
		assert_eq!("nico676", &message.parameters[0]);
	}


	#[test]
	fn test_nickmessage_parses_3() {
		let message = String::from(":localhost NICK ll\r\n");
		let host = String::from("localhost");
		let message = parse_message(message).unwrap();
		assert_eq!(Prefix::ServerNamePrefix {name : host}, message.prefix.unwrap());
		assert_eq!(Command::LetterCommand {command : "NICK".to_string()}, message.command);	
		assert_eq!("ll", &message.parameters[0]);
	}

	#[test]
	fn test_usermessage_parses_1() {
		let message = String::from("USER someguy 0 * :Some Guy\r\n");
		let message = parse_message(message).unwrap();
		assert!(message.prefix.is_none());
		assert_eq!(Command::LetterCommand{command : "USER".to_string()}, message.command);
	}

	#[test]
	fn test_usermessage_parses_2() {
		let message = String::from("USER jono3 0 * :John Third\r\n");
		let message = parse_message(message).unwrap();
		assert!(message.prefix.is_none());
		assert_eq!(Command::LetterCommand{command : "USER".to_string()}, message.command);
		assert_eq!(message.parameters, vec!["jono3".to_string(), "0".to_string(), "*".to_string(), "John Third".to_string()]);
	}

	#[test]
	fn test_usermessage_parses_3() {
		let message = String::from("USER gary_l 0 * :Gary Larson\r\n");
		let message = parse_message(message).unwrap();
		assert!(message.prefix.is_none());
		assert_eq!(Command::LetterCommand{command : "USER".to_string()}, message.command);
	}
}