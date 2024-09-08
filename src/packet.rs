#![allow(unused)]

use crate::utils::Vec3;
use crate::parseable::Parseable;
use macros::AutoParse;
use crate::net_svc_messages::NetSvcMessage;

#[derive(Debug, AutoParse)]
pub struct CmdInfo {
	pub flags: i32, // TODO: cmd info flags
	pub view_origin: Vec3::<f32>,
	pub view_angles: Vec3::<f32>,
	pub local_view_angles: Vec3::<f32>,
	pub view_origin2: Vec3::<f32>,
	pub view_angles2: Vec3::<f32>,
	pub local_view_angles2: Vec3::<f32>,
}

// literally the packet packet
#[derive(Debug)]
pub struct PPacket {
	pub cmd_info: CmdInfo,
	pub in_sequence: i32,
	pub out_sequence: i32,
	pub size: i32,
	pub messages: Vec<NetSvcMessage>,
}

#[derive(Debug, AutoParse)]
pub struct ConsoleCmd {
	pub data_size: i32,
	pub data: String,
}

#[derive(Debug)]
pub struct UserCmdInfo {
	pub command_number: Option<i32>,
	pub tick_count: Option<i32>,
	pub view_angles: Vec3<Option<f32>>,
	pub forward_move: Option<f32>,
	pub side_move: Option<f32>,
	pub up_move: Option<f32>,
	pub buttons: Option<i32>, // TODO: button flags
	pub impulse: Option<i32>,
	pub weapon_select: Option<i32>,
	pub weapon_subtype: Option<i32>,
	pub mouse_dx: Option<i32>,
	pub mouse_dy: Option<i32>,
}

#[derive(Debug)]
pub struct UserCmd {
	pub cmd: i32,
	pub size: i32,
	pub info: UserCmdInfo,
}

#[derive(Debug)]
pub struct DataTables {
	pub size: i32,
	pub data: Vec<u8>,
}

#[derive(Debug)]
pub struct StringTables {
	pub size: i32,
	pub data: Vec<u8>,
}

pub mod consts {
	// packet type numbers as represented in the file
	pub const SIGNON: u8 = 1;
	pub const PPACKET: u8 = 2;
	pub const SYNCTICK: u8 = 3;
	pub const CONSOLECMD: u8 = 4;
	pub const USERCMD: u8 = 5;
	pub const DATATABLES: u8 = 6;
	pub const STOP: u8 = 7;
	pub const STRINGTABLES: u8 = 8;
}

#[derive(Debug)]
pub enum Packet {
	SignOn(i32, PPacket),
	Packet(i32, PPacket),
	SyncTick(i32), // contains no data
	ConsoleCmd(i32, ConsoleCmd),
	UserCmd(i32, UserCmd),
	DataTables(i32, DataTables),
	Stop(i32), // contains no data
	StringTables(i32, StringTables),
}