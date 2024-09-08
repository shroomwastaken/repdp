use macros::AutoParse;
use crate::parseable::Parseable;

use crate::packet::Packet;

// all major structs located here

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


#[derive(Debug)]
pub struct Demo {
	pub header: Header,
	pub packets: Vec<Packet>
}

// extra stuff to help us parse aand be more clear about things

#[allow(non_camel_case_types)]
#[derive(PartialEq)]
pub enum Game {
	PORTAL_5135,
	PORTAL_3420,
	PORTAL_STEAMPIPE,
}