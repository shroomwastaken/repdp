#![allow(unused)]

use std::any::Any;
use std::fmt::{write, Display, Formatter};
use crate::utils::Vec3;
use crate::parseable::Parseable;
use macros::AutoParse;
use crate::{dumper, get_fields};
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

impl Display for CmdInfo {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "\t\tFlags: {:>10}\n", self.flags)?;
		write!(f, "\t\t{:<20} {:>10}\n", "View Origin:", self.view_origin)?;
		write!(f, "\t\t{:<20} {:>10}\n", "View Angles:", self.view_angles)?;
		write!(f, "\t\t{:<20} {:>10}\n", "Local View Angles:", self.local_view_angles)?;
		write!(f, "\t\t{:<20} {:>10}\n", "View Origin 2:", self.view_origin2)?;
		write!(f, "\t\t{:<20} {:>10}\n", "View Angles 2:", self.view_angles2)?;
		write!(f, "\t\t{:<20} {:>10}", "Local View Angles 2:", self.local_view_angles2)
	}
}

/// Literally the packet packet
#[derive(Debug)]
pub struct PPacket {
	pub cmd_info: CmdInfo,
	pub in_sequence: i32,
	pub out_sequence: i32,
	pub size: i32,
	pub messages: Vec<NetSvcMessage>,
}

impl Display for PPacket {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "\tCmdInfo:\n{}\n", self.cmd_info)?;
		write!(f, "\t{:<20} {}\n", "In Sequence:", self.in_sequence)?;
		write!(f, "\t{:<20} {}\n", "Out Sequence:", self.out_sequence)?;
		write!(f, "\t{:<20} {}\n", "Data Size (bytes):", self.size)?;
		write!(f, "\tMessages:\n")?;
		for message in &self.messages {
			write!(f, "\t\tMessage: {:#?}\n", message)?;
		}
		Ok(())
	}
}

#[derive(Debug, AutoParse)]
pub struct ConsoleCmd {
	pub data_size: i32,
	pub data: String,
}

impl Display for ConsoleCmd {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "\t{:<20} {}\n", "Data Size:", self.data_size)?;
		write!(f, "\t{:<20} {}\n", "Data:", self.data)
	}
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

impl Display for UserCmdInfo {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "\t\tCommand Number: {}\n", self.command_number.unwrap_or(0))?;
		write!(f, "\t\tTick Count: {}\n", self.tick_count.unwrap_or(0))?;
		write!(f, "\t\tView Angles: ")?;
		for view_angle in &self.view_angles.as_vec() {
			write!(f, "{:>10} ", view_angle.unwrap_or(0f32))?;
		}
		write!(f, "\n\t\tForward Move: {}\n", self.forward_move.unwrap_or(0f32))?;
		write!(f, "\t\tSide Move: {}\n", self.side_move.unwrap_or(0f32))?;
		write!(f, "\t\tUp Move: {}\n", self.up_move.unwrap_or(0f32))?;
		write!(f, "\t\tButtons: {}\n", self.buttons.unwrap_or(0))?;
		write!(f, "\t\tImpulse: {}\n", self.impulse.unwrap_or(0))?;
		write!(f, "\t\tWeapon Select: {}\n", self.weapon_select.unwrap_or(0))?;
		write!(f, "\t\tWeapon Subtype: {}\n", self.weapon_subtype.unwrap_or(0))?;
		write!(f, "\t\tMouse DX: {}\n", self.mouse_dx.unwrap_or(0))?;
		write!(f, "\t\tMouse DY: {}\n", self.mouse_dy.unwrap_or(0))
	}
}

#[derive(Debug)]
pub struct UserCmd {
	pub cmd: i32,
	pub size: i32,
	pub info: UserCmdInfo,
}

impl Display for UserCmd {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "\tCmd: {}\n", self.cmd)?;
		write!(f, "\tSize: {}\n", self.size)?;
		write!(f, "\tUserCmdInfo:\n{}", self.info)
	}
}

#[derive(Debug)]
pub struct DataTables {
	pub size: i32,
	pub data: Vec<u8>,
}

impl Display for DataTables {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "\tSize: {}\n", self.size)?;
		write!(f, "\tData:")?;
		for i in &self.data {
			write!(f, "{:<10}", i)?;
		}
		write!(f, "\n")
	}
}

#[derive(Debug)]
pub struct StringTables {
	pub size: i32,
	pub data: Vec<u8>,
}

impl Display for StringTables {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		write!(f, "Size: {}\n", self.size)?;
		write!(f, "\tData:")?;
		for i in &self.data {
			write!(f, "{:<10}", i)?;
		}
		write!(f, "\n")
	}
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

impl Display for Packet {
	fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
		match self {
			Packet::SignOn(t, p) => {
				write!(f, "[{}] SIGNON ({})\n{}", t, consts::SIGNON, p)
			}
			Packet::Packet(t, p) => {
				write!(f, "[{}] PACKET ({})\n{}", t, consts::PPACKET, p)
			}
			Packet::SyncTick(t) => {
				write!(f, "[{}] SYNC TICK ({})\n", t, consts::SYNCTICK)
			}
			Packet::ConsoleCmd(t, p) => {
				write!(f, "[{}] CONSOLE COMMAND ({})\n{}", t, consts::CONSOLECMD, p)
			}
			Packet::UserCmd(t, p) => {
				write!(f, "[{}] USER COMMAND ({})\n{}", t, consts::USERCMD, p)
			}
			Packet::DataTables(t, p) => {
				write!(f, "[{}] DATA TABLES ({})\n{}", t, consts::DATATABLES, p)
			}
			Packet::Stop(t) => {
				write!(f, "[{}] STOP ({})\n", t, consts::STOP)
			}
			Packet::StringTables(t, p) => {
				write!(f, "[{}] STRING TABLES ({})\n{}", t, consts::STRINGTABLES, p)
			}
		}
	}
}