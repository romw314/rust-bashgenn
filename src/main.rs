use clap::Parser;
use clap::crate_version;
use clap::crate_authors;
use std::fs::File;
use std::path::PathBuf;
use std::collections::HashMap;
use std::io::BufRead;
use std::io::Write;
use std::time::Duration;
use linereader::LineReader;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
	/// Specify the output file
	#[arg(short, long, value_name = "FILE")]
	output: Option<String>,

	/// File to build
	file: PathBuf,

	/// Interpret the script instead of compiling it to Bash
	#[arg(short, long)]
	interpret: bool
}

struct Command<'a> {
	/// The command name (e.g. ECHO)
	name: &'a str,

	/// The first argument of the command
	arg1: &'a str,

	/// The second argument of the command
	arg2: &'a str
}

struct Runtime<'a, TInput> {
	/// The runtime variables of the script
	vars: &'a mut HashMap<String, String>,

	/// A `linereader::LineReader` to read the script from
	input_reader: &'a mut LineReader<TInput>,

	/// A `std::io::Stdin` to read the script input from
	stdin: &'a std::io::Stdin
}

impl Command<'_> {
	fn new<'a>(line: &'a String) -> Command<'a> {
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

fn _wout(line: String) {
	println!("HRO | {}", line);
}

macro_rules! wout {
	( $( $x:tt )* ) => {
		_wout(format!($($x)*));
	}
}

macro_rules! wc_while {
	( $condition:expr, $runtime:expr, $buffer:expr ) => {
		{
			let mut vec = Vec::new();
			while let Some(line) = $runtime.input_reader.next_line() {
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
				vec.iter().for_each(|line| run_command(Command::new(line), $runtime, $buffer));
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

fn run_command<TInput>(cmd: Command, runtime: &mut Runtime<TInput>, buffer: &mut String) where TInput: std::io::Read {
	match cmd.name {
		"READ" => {
			let mut input = String::new();
			runtime.stdin.lock().read_line(&mut input).unwrap();
			setvar(runtime.vars, cmd.arg1, input.trim_end().to_string());
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
			wc_while!(true, runtime, buffer);
		}
		"STRGET" => {
			wc_while!(getvar(runtime.vars, cmd.arg1).len() > 0, runtime, buffer);
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

fn proc_file_line<'a, TInput>(interpret: bool, line: &'a String, runtime: &mut Runtime<TInput>, buffer: &mut String) where TInput: std::io::Read {
	let cmd: Command<'a> = Command::new(&line);
	if interpret {
		run_command(cmd, runtime, buffer);
	}
	else {
		match cmd.name {
			"READ" => {
				wout!("read RUNTIME_{}", cmd.arg1);
			}
			"ECHO" => {
				wout!("echo \"$RUNTIME_{}\"", cmd.arg1);
			}
			"COPY" => {
				wout!("RUNTIME_{}=\"$RUNTIME_{}", cmd.arg1, cmd.arg2);
			}
			"" => {}
			"STDIN" | "STRGET" | "FIRST" | "STOREFIRST" | "LAST" | "STORELAST" | "NONL" | "STOP" | "KILL" | "DONE" | "REPEAT" | "CHINC" | "CHDEC" | "STORECHINC" | "STORECHDEC" | "STRRANGE" | "STRRANGELESS" | "STRCAT" => {
				unimplemented!();
			}
			_ => {
				panic!("Unknown command (maybe you want to interpret with -i)");
			}
		}
	}
}

fn main() {
	let cli = Cli::parse();

	println!("RBGN v{} created by {}", crate_version!(), crate_authors!(" and "));

	let file = File::open(cli.file).unwrap();
	let mut reader = LineReader::new(file);
	let mut vars: HashMap<String, String> = HashMap::new();
	let mut stdin = std::io::stdin();
	let mut buffer = String::new();
	let mut runtime = Runtime { vars: &mut vars, input_reader: &mut reader, stdin: &mut stdin };

	while let Some(line) = runtime.input_reader.next_line() {
		proc_file_line(cli.interpret, &String::from(String::from_utf8(line.unwrap().to_vec()).unwrap().trim()), &mut runtime, &mut buffer);
	}
}
