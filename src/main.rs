extern crate macros;

mod reader; // bit reader struct
mod error; // custom errors
mod demo; // major structs representing the file
mod parsing; // parsing logic
mod utils; // various useful things
mod parseable; // the Parseable trait
mod packet; // packet types and related types
mod net_svc_messages; // net/svc message structs
mod game_event; // for SvcGameEventList/SvcGameEvent
mod dumper;
mod print;
// struct that controls dumping the demo's human-readable contents

use std::time::Instant;

fn main() {
	if let Err(err) = run() {
		println!("{}", err.to_string())
	}
}

fn run() -> anyhow::Result<()> {
	let args: Vec<String> = std::env::args().collect();
	if args.len() != 2 {
		return Err(error::ParserError::ArgumentError("expected one arg: the file name".to_string()).into());
	}

	let mut reader: reader::BitReader;
	let vec: Vec<u8> = match std::fs::read(args[1].clone()) {
		Ok(vec) => { vec }
		Err(err) => { return Err(err.into()) }
	};
	reader = reader::BitReader::new(&vec)?;

	let start_time: Instant = Instant::now();
	let demo: demo::Demo = parsing::parse_demo(&mut reader)?;

	let mut dumper: dumper::Dumper = dumper::Dumper {
		demo: &demo,
		tabs: 0,
		out: &mut std::io::stdout().lock(),
	};
	dumper.dump_header()?;
	println!("\nTook {:?} to parse", Instant::now().duration_since(start_time));

	Ok(())
}
