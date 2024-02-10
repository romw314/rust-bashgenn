//! To run a script, you can use [linereader](https://docs.rs/linereader).
//!
//! Here is an example of running one command:
//!
//! ```
//! # extern crate anyhow;
//! use linereader::LineReader;
//! use std::{io, collections::HashMap};
//! use rbgn::{LineReaderWrapper, Command, Runtime, InputReader, run_command};
//! # fn main() -> anyhow::Result<()> {
//! let mut reader = LineReaderWrapper::new("STATIC_STR_VAR my_var my_value".as_bytes());
//! let mut stdin = LineReader::new(io::stdin());
//! let mut vars: HashMap<String, String> = HashMap::new();
//! let mut runtime = Runtime { vars: &mut vars, stdin: &mut stdin };
//! let line = &String::from(std::str::from_utf8(reader.next_line().unwrap()?).unwrap());
//! let cmd = Command::new(line);
//! run_command(cmd, &mut runtime, &mut reader);
//! assert_eq!(vars.get(&String::from("my_var")).unwrap(), "my_value");
//! # Ok(())
//! # }
//! ```
//!
//! Here is a more complex example:
//!
//! ```no_run
//! # extern crate anyhow;
//! use linereader::LineReader;
//! use std::{io, fs::File, collections::HashMap};
//! use rbgn::{LineReaderWrapper, Command, Runtime, InputReader, run_command};
//! # fn main() -> anyhow::Result<()> {
//! let file = File::open("script.bgn")?;
//! let mut reader = LineReaderWrapper::new(file);
//! let mut stdin = LineReader::new(io::stdin());
//! let mut vars: HashMap<String, String> = HashMap::new();
//! let mut runtime = Runtime { vars: &mut vars, stdin: &mut stdin };
//! while let Some(line) = reader.next_line() {
//!     let line = &String::from(std::str::from_utf8(line?).unwrap());
//!     let cmd = Command::new(line);
//!     run_command(cmd, &mut runtime, &mut reader);
//! }
//! # Ok(())
//! # }
//! ```

use std::time::Duration;
use std::io::Write;
use std::collections::HashMap;
use linereader::LineReader;

pub trait InputReader {
	fn next_line(&mut self) -> Option<Result<&[u8], std::io::Error>>;
}

/// Wraps `linereader::LineReader` to implement the `InputReader` trait
pub struct LineReaderWrapper<R: std::io::Read> {
	reader: LineReader<R>
}

impl<R: std::io::Read> InputReader for LineReaderWrapper<R> {
	fn next_line(&mut self) -> Option<Result<&[u8], std::io::Error>> {
		self.reader.next_line()
	}
}

impl<R: std::io::Read> LineReaderWrapper<R> {
	pub fn new(input: R) -> LineReaderWrapper<R> {
		return LineReaderWrapper { reader: LineReader::new(input) };
	}
}

pub struct Runtime<'a, TInput: std::io::Read> {
	/// The runtime variables of the script
	pub vars: &'a mut HashMap<String, String>,

	/// A `LineReaderWrapper` to read the script input from
	pub stdin: &'a mut LineReader<TInput>
}

pub struct Command<'a> {
	/// The command name (e.g. ECHO)
	pub name: &'a str,

	/// The first argument of the command
	pub arg1: &'a str,

	/// The second argument of the command
	pub arg2: &'a str
}

impl Command<'_> {
	/// Creates a new command from a line
	pub fn new<'a>(line: &'a String) -> Command<'a> {
		let trimmed_line = line.trim();
		if trimmed_line.starts_with("-") {
			return Command { name: "", arg1: "", arg2: "" };
		}
		let mut parts = trimmed_line.splitn(3, ' ');
		let name = parts.next().unwrap_or("");
		let arg1 = parts.next().unwrap_or("");
		let arg2 = parts.next().unwrap_or("");
		return Command { name, arg1, arg2 };
	}
}

macro_rules! wc_while {
	( $condition:expr, $runtime:expr, $reader:expr ) => {
		{
			let mut vec = Vec::new();
			while let Some(line) = $reader.next_line() {
				let line_str = String::from(std::str::from_utf8(line.unwrap()).unwrap().trim());
				if line_str == "DONE" {
					break;
				}
				else {
					vec.push(line_str);
				}
			}
			#[allow(while_true)]
			while $condition {
				vec.iter().for_each(|line| run_command(Command::new(line), $runtime, $reader));
			}
		}
	}
}

fn setvar(vars: &mut HashMap<String, String>, name: &str, value: String) {
	*vars.entry(String::from(name)).or_insert(String::new()) = value;
}

fn getvar(vars: &mut HashMap<String, String>, name: &str) -> String {
	return vars.get(&String::from(name)).unwrap_or(&String::new()).to_string();
}

pub fn run_command<'a, TInput: std::io::Read>(cmd: Command, runtime: &mut Runtime<TInput>, reader: &mut dyn InputReader) {
	match cmd.name {
		"READ" => {
			setvar(runtime.vars, cmd.arg1, std::str::from_utf8(runtime.stdin.next_line().unwrap().unwrap()).unwrap().trim_end().to_string());
		}
		"ECHO" => {
			println!("{}", getvar(runtime.vars, cmd.arg1));
		}
		"__RBGN_NONL" => {
			print!("{}", getvar(runtime.vars, cmd.arg1));
		}
		"__RBGN_FLUSH" => {
			std::io::stdout().flush().unwrap();
		}
		"WAIT" | "SLEEP" | "SLEEP_SECS" => {
			std::thread::sleep(Duration::from_secs(cmd.arg1.parse::<u64>().unwrap()));
		}
		"NONL" => {
			print!("{}", getvar(runtime.vars, cmd.arg1));
			std::io::stdout().flush().unwrap();
		}
		"FIRST" => {
			let s = getvar(runtime.vars, cmd.arg1);
			println!("{}", s.chars().nth(0).unwrap());
			setvar(runtime.vars, cmd.arg1, String::from(&s[1..s.len()]));
		}
		"STOREFIRST" => {
			let s = getvar(runtime.vars, cmd.arg1);
			setvar(runtime.vars, cmd.arg2, s.chars().nth(0).unwrap().to_string());
			setvar(runtime.vars, cmd.arg1, String::from(&s[1..s.len()]));
		}
		"LAST" => {
			let s = getvar(runtime.vars, cmd.arg1);
			println!("{}", s.chars().nth(s.len() - 1).unwrap());
			setvar(runtime.vars, cmd.arg1, String::from(&s[..s.len() - 1]));
		}
		"STORELAST" => {
			let s = getvar(runtime.vars, cmd.arg1);
			setvar(runtime.vars, cmd.arg2, s.chars().nth(s.len() - 1).unwrap().to_string());
			setvar(runtime.vars, cmd.arg1, String::from(&s[..s.len() - 1]));
		}
		"FOREVER" => {
			wc_while!(true, runtime, reader);
		}
		"STRGET" => {
			wc_while!(getvar(runtime.vars, cmd.arg1).len() > 0, runtime, reader);
		}
		"STATIC_STR_VAR" => {
			setvar(runtime.vars, cmd.arg1, String::from(cmd.arg2));
		}
		"STATIC_STR_SPACE" => {
			setvar(runtime.vars, cmd.arg1, String::from(" "));
		}
		"CONST_SET" => {
			setvar(runtime.vars, format!("_RBGN_INTERNAL_CONST_{}", cmd.arg1).as_str(), String::from(cmd.arg2));
		}
		"CONST_WRITE" => {
			let const_value = getvar(runtime.vars, format!("_RBGN_INTERNAL_CONST_{}", cmd.arg1).as_str());
			setvar(runtime.vars, cmd.arg2, const_value);
		}
		"_OPT" => {}
		"" => {}
		_ => {
			panic!("Unknown command '{}'", cmd.name);
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_cmd() {
		let line = &String::from(" \t\nOUR_COMMAND 1 2\n\t ");
		let cmd = Command::new(line);
		assert_eq!(cmd.name, "OUR_COMMAND");
		assert_eq!(cmd.arg1, "1");
		assert_eq!(cmd.arg2, "2");
	}

	#[test]
	fn test_vars() {
		let mut reader = LineReaderWrapper::new("STATIC_STR_VAR our_variable our_content".as_bytes());
		let mut stdin = LineReader::new("".as_bytes());
		let mut vars: HashMap<String, String> = HashMap::new();
		let mut runtime = Runtime { vars: &mut vars, stdin: &mut stdin };
		let line = &String::from(std::str::from_utf8(reader.next_line().unwrap().unwrap()).unwrap());
		let cmd = Command::new(line);
		run_command(cmd, &mut runtime, &mut reader);
		assert_eq!(vars.get(&String::from("our_variable")).unwrap(), "our_content");
	}
}
