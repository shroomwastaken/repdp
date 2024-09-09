use lazy_static::lazy_static;
use std::sync::{Arc, Mutex};
use crate::error::ParserError;
use crate::parseable::Parseable;
use crate::demo::{Demo, Game, DemoInfo, Header};
use crate::reader::BitReader;
use crate::packet::*;
use crate::packet::consts::*;
use crate::utils::Vec3;
use crate::net_svc_messages::parse_net_svc_messages;

// to check while parsing
// this WILL be populated by the point where they need to be checked
// i hate having to do .lock().unwrap() every time but that's seemingly what i have to do
lazy_static! {
	pub static ref DEMO_INFO: Arc<Mutex<DemoInfo>> = Arc::new(
		Mutex::new(
			DemoInfo {
				net_svc_message_bits: 0,
				net_protocol: 0,
				demo_protocol: 0,
				game: Game::PORTAL_5135,
				game_event_list: vec![],
			}
		)
	);
}

pub fn parse_demo(r: &mut BitReader) -> anyhow::Result<Demo> {
	let header: Header = Header::parse(r)?;
	if header.demo_protocol != 3 {
		return Err(ParserError::UnsupportedDemo(format!(
			"demo protocol {} not supported!", header.demo_protocol
		)).into());
	}

	let demo_info: DemoInfo = DemoInfo {
		net_svc_message_bits: match header.network_protocol {
			14 => { 5 },
			_ => { 6 },
		},
		net_protocol: header.network_protocol,
		demo_protocol: header.demo_protocol,
		game: match header.network_protocol {
			14 => { Game::PORTAL_3420 }
			15 => { Game::PORTAL_5135 }
			24 => { Game::PORTAL_STEAMPIPE }
			_ => { return Err(ParserError::ParserError("unsupported network protocol".to_string()).into()) }
		},
		game_event_list: vec![],
	};

	*DEMO_INFO.lock().unwrap() = demo_info;

	let demo: Demo = Demo {
		header,
		packets: parse_packets(r)?,
	};

	return Ok(demo);
}

pub fn parse_packets(r: &mut BitReader) -> anyhow::Result<Vec<Packet>> {
	let mut res: Vec<Packet> = vec![];

	loop {
		let p_type: u8 = u8::parse(r)?;
		let packet: Packet = match p_type {
			SIGNON => { Packet::SignOn(i32::parse(r)?, PPacket::parse(r)?) },
			PPACKET => { Packet::Packet(i32::parse(r)?, PPacket::parse(r)?) },
			SYNCTICK => { Packet::SyncTick(i32::parse(r)?) },
			CONSOLECMD => { Packet::ConsoleCmd(i32::parse(r)?, ConsoleCmd::parse(r)?) },
			USERCMD => { Packet::UserCmd(i32::parse(r)?, UserCmd::parse(r)?) },
			DATATABLES => { Packet::DataTables(i32::parse(r)?, DataTables::parse(r)?) },
			STOP => { Packet::Stop(i32::parse_amount(r, 24)?) },
			STRINGTABLES => { Packet::StringTables(i32::parse(r)?, StringTables::parse(r)?) },
			_ => { return Err(ParserError::ParserError(format!(
				"trying to read nonexistent packet type {}", p_type
			)).into()) },
		};

		res.push(packet);

		if p_type == STOP { break; }
	}

	return Ok(res);
}

impl PPacket {
	pub fn parse(r: &mut BitReader) -> anyhow::Result<PPacket> {
		let mut res: PPacket = PPacket {
			cmd_info: CmdInfo::parse(r)?,
			in_sequence: i32::parse(r)?,
			out_sequence: i32::parse(r)?,
			size: i32::parse(r)?,
			messages: vec![],
		};
		res.messages = parse_net_svc_messages(&mut r.split_and_skip(res.size as usize * 8)?, res.size as usize)?;
		return Ok(res);
	}
}

impl UserCmdInfo {
	pub fn parse(r: &mut BitReader) -> anyhow::Result<UserCmdInfo> {
		let mut res: UserCmdInfo = UserCmdInfo {
			command_number: Option::<i32>::parse(r)?,
			tick_count: Option::<i32>::parse(r)?,
			view_angles: Vec3::<Option<f32>>::parse(r)?,
			forward_move: Option::<f32>::parse(r)?,
			side_move: Option::<f32>::parse(r)?,
			up_move: Option::<f32>::parse(r)?,
			buttons: Option::<i32>::parse(r)?,
			impulse: Option::<i32>::parse_amount(r, 8)?,
			weapon_select: Option::<i32>::parse_amount(r, 11)?,
			weapon_subtype: None,
			mouse_dx: None,
			mouse_dy: None,
		};

		if res.weapon_select.is_some() { res.weapon_subtype = Option::<i32>::parse_amount(r, 6)? }

		res.mouse_dx = r.read_sint_if_exists(16)?;
		res.mouse_dy = r.read_sint_if_exists(16)?;

		return Ok(res);
	}
}

impl UserCmd {
	pub fn parse(r: &mut BitReader) -> anyhow::Result<UserCmd> {
		let cmd: i32 = i32::parse(r)?;
		let size: i32 = i32::parse(r)?;
		let res: UserCmd = UserCmd {
			cmd,
			size: size.clone(),
			info: UserCmdInfo::parse(&mut r.split_and_skip(size as usize * 8)?)?,
		};

		return Ok(res);
	}
}

impl DataTables {
	pub fn parse(r: &mut BitReader) -> anyhow::Result<DataTables> {
		let res: DataTables = DataTables {
			size: i32::parse(r)?, data: vec![],
		};
		r.skip((res.size * 8) as usize)?;
		return Ok(res);
	}
}

impl StringTables {
	pub fn parse(r: &mut BitReader) -> anyhow::Result<StringTables> {
		let res: StringTables = StringTables {
			size: i32::parse(r)?, data: vec![],
		};
		r.skip((res.size * 8) as usize)?;
		return Ok(res);
	}
}