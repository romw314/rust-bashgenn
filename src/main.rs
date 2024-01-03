use clap::Parser;
use clap::crate_version;
use clap::crate_authors;
use std::fs::File;
use std::path::PathBuf;
use std::collections::HashMap;
use linereader::LineReader;
use rbgn::Runtime;
use rbgn::Command;
use rbgn::run_command;
use rbgn::LineReaderWrapper;
use rbgn::InputReader;

#[derive(Parser)]
#[command(author = crate_authors!(" and "), version, about, long_about = None)]
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

fn _wout(line: String) {
	println!("HRO | {}", line);
}

macro_rules! wout {
	( $( $x:tt )* ) => {
		_wout(format!($($x)*));
	}
}

fn proc_file_line<'a, TInput: std::io::Read, TStdin: std::io::Read>(interpret: bool, line: &'a String, runtime: &mut Runtime<TStdin>, reader: &'a mut LineReaderWrapper<TInput>) {
	let cmd: Command<'a> = Command::new(&line);
	if interpret {
		run_command(cmd, runtime, reader);
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
	let mut reader = LineReaderWrapper::new(file);
	let mut vars: HashMap<String, String> = HashMap::new();
	let mut stdin = std::io::stdin();
	let mut runtime = Runtime { vars: &mut vars, stdin: &mut LineReader::new(&mut stdin) };

	while let Some(line) = reader.next_line() {
		proc_file_line(cli.interpret, &String::from(String::from_utf8(line.unwrap().to_vec()).unwrap().trim()), &mut runtime, &mut reader);
	}
}
