use std::fmt::{self, Debug, Display, Formatter, Write};
use macros::AutoParse;
use crate::parseable::Parseable;
use crate::game_event::GameEventDescriptor;
use crate::packet::Packet;

#[derive(Debug)]
pub struct Demo {
	pub header: Header,
	pub packets: Vec<Packet>
}

impl Display for Demo {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "{}\n", self.header)?;
		for packet in &self.packets {
			write!(f, "{}\n", packet)?;
		}
		write!(f, "\n")
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

impl Display for Header {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "Signature: {}\n", self.signature)?;
		write!(f, "Demo Protocol: {}\n", self.demo_protocol)?;
		write!(f, "Network Protocol: {}\n", self.network_protocol)?;
		write!(f, "Server Name: {}\n", self.server_name)?;
		write!(f, "Client Name: {}\n", self.client_name)?;
		write!(f, "Map Name: {}\n", self.map_name)?;
		write!(f, "Game Directory: {}\n", self.game_directory)?;
		write!(f, "Playback Time: {}\n", self.playback_time)?;
		write!(f, "Playback ticks: {}\n", self.playback_ticks)?;
		write!(f, "Playback frames: {}\n", self.playback_frames)?;
		write!(f, "Sign On Length: {}\n", self.sign_on_length)
	}
}

/// Extra stuff to help us parse and be more clear about things
#[allow(non_camel_case_types)]
#[derive(Debug, PartialEq)]
pub enum Game {
	PORTAL_5135,
	PORTAL_3420,
	PORTAL_STEAMPIPE,
}

/// Struct to hold all pre-made values that differ based on protocol version
#[derive(Debug)]
pub struct DemoInfo {
	pub net_svc_message_bits: usize,
	pub net_protocol: i32,
	pub demo_protocol: i32,
	pub game: Game,
	pub game_event_list: Vec<GameEventDescriptor>,
}

impl Display for DemoInfo {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		write!(f, "Net Svc Message Bits: {}\n", self.net_svc_message_bits)?;
		write!(f, "Net Protocol: {}\n", self.net_protocol)?;
		write!(f, "Demo Protocol: {}\n", self.demo_protocol)?;
		write!(f, "Game: {:?}\n", self.game)?;
		write!(f, "Game Event List: {}\n", self.net_svc_message_bits)?;
		for game_event in &self.game_event_list {
			write!(f, "Game Event:\n{}", game_event)?;
		}
		write!(f, "\n")
	}
}