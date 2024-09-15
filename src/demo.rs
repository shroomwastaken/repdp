use std::collections::HashMap;
use std::fmt::{self, Debug, Display, Formatter, Write};
use macros::AutoParse;
use crate::parseable::Parseable;
use crate::game_event::GameEventDescriptor;
use crate::imp_dump_display;
use crate::packet::Packet;

#[derive(Debug)]
pub struct Demo {
	pub header: Header,
	pub packets: Vec<Packet>
}

impl Display for Demo {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}\n{}\n", self.info, self.header)?;
		for packet in self.packets {
			write!(f, "{}", packet)?;
		}
		Ok(())
	}
}

/// All major structs are located here
#[derive(Debug, Clone, AutoParse)]
pub struct Header {
	#[size(64)]
	signature: String,
	pub demo_protocol: i32,
	pub network_protocol: i32,
	#[size(2080)]
	server_name: String,
	#[size(2080)]
	client_name: String,
	#[size(2080)]
	map_name: String,
	#[size(2080)]
	game_directory: String,
	playback_time: f32,
	playback_ticks: i32,
	playback_frames: i32,
	sign_on_length: i32,
}

imp_dump_display!(Header);

/// Extra stuff to help us parse and be more clear about things
#[allow(non_camel_case_types)]
#[derive(PartialEq)]
pub enum Game {
	PORTAL_5135,
	PORTAL_3420,
	PORTAL_STEAMPIPE,
}

/// Struct to hold all pre-made values that differ based on protocol version
pub struct DemoInfo {
	pub net_svc_message_bits: usize,
	pub net_protocol: i32,
	pub demo_protocol: i32,
	pub game: Game,
	pub game_event_list: Vec<GameEventDescriptor>,
}