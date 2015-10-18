#![feature(advanced_slice_patterns, slice_patterns)]
use std::char;
use std::str;
use message::{Message, Prefix, Command};

fn parse_message(message_str : String) -> Result<Message, (String, usize)>  {
	let chars : Vec<char> = message_str.chars().collect();

	let mut index : usize = 0;
	let mut prefix = None;

	if (chars.len() > 0 && chars[0] == ':') {
		prefix = Some(try!(parse_prefix(&chars, &mut index)));
	}else{
		println!("THIS IS CHAR {}", chars[0]);
	}

	println!("Parsing command...");
	let command = try!(parse_command(&chars, &mut index));

	println!("Parsing params...");
	let parameters = try!(parse_parameters(&chars, &mut index));

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
	if (message_str[*index]).is_digit(10) {
		while *index < message_str.len() && (message_str[*index]).is_digit(10) {
			command_str.push(message_str[*index]);
			*index = *index + 1;
		}
		return Ok(Command::DigitCommand {
			command : command_str
		});
	}else if message_str[*index].is_alphabetic() {
		while *index < message_str.len() && message_str[*index].is_alphabetic(){
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
	return Ok(Vec::new());
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
	use message::{Message, Prefix, Command};

	#[test]
	fn smoke_test_1() {
		let message = String::from(":irc.freenode.net NICK nico676\r\n");
		let host = String::from(":irc.freenode.net");
		let message = parse_message(message);
		assert_eq!(Prefix::ServerNamePrefix {name : host}, message.unwrap().prefix.unwrap())	;
	}
}

//:irc.freenode.net NICK nico676