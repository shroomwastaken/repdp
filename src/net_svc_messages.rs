/*
net/svc messages are part of the ppacket
they can contain all sorts of data (see structs in this file)
they are usually NOT byte-aligned
*/

use macros::AutoParse;
use crate::{
	demo::Game,
	parseable::Parseable,
	parsing::DEMO_INFO,
	game_event::{GameEventDescriptor, GameEvent},
	reader::BitReader,
	utils::log2_of_x_plus_one
};

// -------------helper types begin----------------

type ConVar = (String, String);

#[derive(Debug, AutoParse)]
pub struct ServerClass {
	class_id: i32,
	class_name: String,
	data_table_name: String
}

// -------------helper types end------------------

#[derive(Debug)]
pub enum NetSvcMessage {
	NetNop,
	NetDisconnect(NetDisconnect),
	NetFile(NetFile),
	NetTick(NetTick),
	NetStringCmd(NetStringCmd),
	NetSetConVar(NetSetConVar),
	NetSignonState(NetSignonState),
	SvcServerInfo(SvcServerInfo),
	SvcSendTable(SvcSendTable),
	SvcClassInfo(SvcClassInfo),
	SvcSetPause(SvcSetPause),
	SvcCreateStringTable(SvcCreateStringTable),
	SvcUpdateStringTable(SvcUpdateStringTable),
	SvcVoiceInit(SvcVoiceInit),
	SvcVoiceData(SvcVoiceData),
	SvcPrint(SvcPrint),
	SvcSounds(SvcSounds),
	SvcSetView(SvcSetView),
	SvcFixAngle(SvcFixAngle),
	SvcCrosshairAngle(SvcCrosshairAngle),
	SvcBspDecal(SvcBspDecal),
	SvcSplitScreen(SvcSplitScreen),
	SvcUserMessage(SvcUserMessage),
	SvcEntityMessage(SvcEntityMessage),
	SvcGameEvent(SvcGameEvent),
	SvcPacketEntities(SvcPacketEntities),
	SvcTempEntities(SvcTempEntities),
	SvcPrefetch(SvcPrefetch),
	SvcMenu(SvcMenu),
	SvcGameEventList(SvcGameEventList),
	SvcGetCvarValue(SvcGetCvarValue),
	SvcCmdKeyValues(SvcCmdKeyValues),
}

pub fn parse_net_svc_messages(r: &mut BitReader, size: usize) -> anyhow::Result<Vec<NetSvcMessage>> {
	let type_size: usize = DEMO_INFO.lock().unwrap().net_svc_message_bits;
	let mut messages: Vec<NetSvcMessage> = vec![];
	let start_index: i32 = r.current as i32;
	while ((start_index + size as i32) - r.current as i32) > 6 {
		let msg_type: u8 = u8::parse_amount(r, type_size)?;
		let message: NetSvcMessage = NetSvcMessage::parse(r, msg_type)?;
		messages.push(message);
	}
	return Ok(messages);
}

impl NetSvcMessage {
	// ugly
	pub fn parse(r: &mut BitReader, msg_type: u8) -> anyhow::Result<NetSvcMessage> {
		return Ok(match msg_type {
			0 => {NetSvcMessage::NetNop}
			1 => {NetSvcMessage::NetDisconnect(NetDisconnect::parse(r)?)}
			2 => {NetSvcMessage::NetFile(NetFile::parse(r)?)}
			3 => {NetSvcMessage::NetTick(NetTick::parse(r)?)}
			4 => {NetSvcMessage::NetStringCmd(NetStringCmd::parse(r)?)}
			5 => {NetSvcMessage::NetSetConVar(NetSetConVar::parse(r)?)}
			6 => {NetSvcMessage::NetSignonState(NetSignonState::parse(r)?)}
			7 => {NetSvcMessage::SvcPrint(SvcPrint::parse(r)?)}
			8 => {NetSvcMessage::SvcServerInfo(SvcServerInfo::parse(r)?)}
			9 => {NetSvcMessage::SvcSendTable(SvcSendTable::parse(r)?)}
			10 => {NetSvcMessage::SvcClassInfo(SvcClassInfo::parse(r)?)}
			11 => {NetSvcMessage::SvcSetPause(SvcSetPause::parse(r)?)}
			12 => {NetSvcMessage::SvcCreateStringTable(SvcCreateStringTable::parse(r)?)}
			13 => {NetSvcMessage::SvcUpdateStringTable(SvcUpdateStringTable::parse(r)?)}
			14 => {NetSvcMessage::SvcVoiceInit(SvcVoiceInit::parse(r)?)}
			15 => {NetSvcMessage::SvcVoiceData(SvcVoiceData::parse(r)?)}
			17 => {NetSvcMessage::SvcSounds(SvcSounds::parse(r)?)}
			18 => {NetSvcMessage::SvcSetView(SvcSetView::parse(r)?)}
			19 => {NetSvcMessage::SvcFixAngle(SvcFixAngle::parse(r)?)}
			20 => {NetSvcMessage::SvcCrosshairAngle(SvcCrosshairAngle::parse(r)?)}
			21 => {NetSvcMessage::SvcBspDecal(SvcBspDecal::parse(r)?)}
			22 => {NetSvcMessage::SvcSplitScreen(SvcSplitScreen::parse(r)?)}
			23 => {NetSvcMessage::SvcUserMessage(SvcUserMessage::parse(r)?)}
			24 => {NetSvcMessage::SvcEntityMessage(SvcEntityMessage::parse(r)?)}
			25 => {NetSvcMessage::SvcGameEvent(SvcGameEvent::parse(r)?)}
			26 => {NetSvcMessage::SvcPacketEntities(SvcPacketEntities::parse(r)?)}
			27 => {NetSvcMessage::SvcTempEntities(SvcTempEntities::parse(r)?)}
			28 => {NetSvcMessage::SvcPrefetch(SvcPrefetch::parse(r)?)}
			29 => {NetSvcMessage::SvcMenu(SvcMenu::parse(r)?)}
			30 => {NetSvcMessage::SvcGameEventList(SvcGameEventList::parse(r)?)}
			31 => {NetSvcMessage::SvcGetCvarValue(SvcGetCvarValue::parse(r)?)}
			32 => {NetSvcMessage::SvcCmdKeyValues(SvcCmdKeyValues::parse(r)?)}
			_ => { panic!("NO") }
		});
	}
}

#[derive(Debug, AutoParse)]
pub struct NetDisconnect { data: String, }

#[derive(Debug, AutoParse)]
pub struct NetFile {
	transfer_id: i32,
	file_name: String,
	file_requested: bool,
}

#[derive(Debug, AutoParse)]
// the last two fields are scaled up by 10^5,
// we don't care about this for now
pub struct NetTick {
	tick: i32,
	host_frame_time: i16,
	host_frame_time_std_deviation: i16,
}

#[derive(Debug, AutoParse)]
pub struct NetStringCmd { cmd: String, }

#[derive(Debug)]
pub struct NetSetConVar {
	length: u8,
	convars: Vec<ConVar>
}

impl NetSetConVar {
	pub fn parse(r: &mut BitReader) -> anyhow::Result<NetSetConVar> {
		let mut res: NetSetConVar = NetSetConVar {
			length: u8::parse(r)?,
			convars: vec![],
		};

		for _ in 0..res.length {
			res.convars.push((
				r.read_ascii_string_nulled()?,
				r.read_ascii_string_nulled()?
			))
		}

		return Ok(res);
	}
}

#[derive(Debug, AutoParse)]
pub struct NetSignonState {
	signon_state: u8,
	spawn_count: i32,
	// extra demo protocol 4 shit is here but we dont care (yet)
}

#[derive(Debug)]
pub struct SvcServerInfo {
	pub protocol: i16,
	pub server_count: i32,
	pub is_hltv: bool,
	pub is_dedicated: bool,
	pub client_crc: i32,
	pub max_classes: i16,
	pub tick_interval: f32,
	pub map_crc: Option<i32>, // its either one or the other so theyre both an option
	pub map_md5: Option<Vec<u8>>,
	pub player_slot: u8,
	pub max_clients: u8,
	pub platform: char,
	pub game_dir: String,
	pub map_name: String,
	pub sky_name: String,
	pub host_name: String,
	pub has_replay: Option<bool>, // only exists past network protocol 16
}

impl SvcServerInfo {
	pub fn parse(r: &mut BitReader) -> anyhow::Result<SvcServerInfo> {
		// i knooooow its uglyyy but theres nothing i can really do about it :(
		let is_steampipe: bool = DEMO_INFO.lock().unwrap().game == Game::PORTAL_STEAMPIPE;
		let res: SvcServerInfo = SvcServerInfo {
			protocol: i16::parse(r)?,
			server_count: i32::parse(r)?,
			is_hltv: bool::parse(r)?,
			is_dedicated: bool::parse(r)?,
			client_crc: i32::parse(r)?,
			max_classes:  i16::parse(r)?,
			tick_interval: f32::parse(r)?,
			map_crc: if !is_steampipe { Some(i32::parse(r)?) } else { None },
			map_md5: if is_steampipe { Some(r.read_bytes(16)?) } else { None },
			player_slot: u8::parse(r)?,
			max_clients: u8::parse(r)?,
			platform: u8::parse(r)? as char,
			game_dir: String::parse(r)?,
			map_name: String::parse(r)?,
			sky_name: String::parse(r)?,
			host_name: String::parse(r)?,
			has_replay: if is_steampipe { Some(r.read_bool()?) } else { None }
		};
		return Ok(res);
	}
}

#[derive(Debug, AutoParse)]
pub struct SvcSendTable {
	needs_decoder: bool,
	length: u8,
	#[size(length)]
	props: i32
}

#[derive(Debug)]
pub struct SvcClassInfo {
	length: i16,
	create_on_client: bool,
	server_classes: Vec<ServerClass>
}

impl SvcClassInfo {
	pub fn parse(r: &mut BitReader) -> anyhow::Result<SvcClassInfo> {
		let mut res: SvcClassInfo = SvcClassInfo {
			length: i16::parse(r)?,
			create_on_client: bool::parse(r)?,
			server_classes: vec![],
		};

		if !res.create_on_client {
			for _ in 0..res.length {
				res.server_classes.push(ServerClass::parse(r)?);
			}
		}

		return Ok(res);
	}
}

#[derive(Debug, AutoParse)]
pub struct SvcSetPause { paused: bool }

#[derive(Debug)]
pub struct SvcCreateStringTable {
	pub name: String,
	pub max_entries: i16,
	pub num_entries: i32,
	pub length: i32,
	pub user_data_fixed_size: bool,
	pub user_data_size: Option<i16>,
	pub user_data_size_bits: Option<u8>,
	pub flags: u8, // TODO: flags
	// pub string_data: ???
}

impl SvcCreateStringTable {
	pub fn parse(r: &mut BitReader) -> anyhow::Result<SvcCreateStringTable> {
		let net_protocol: i32 = DEMO_INFO.lock().unwrap().net_protocol;
		let demo_protocol: i32 = DEMO_INFO.lock().unwrap().demo_protocol;
		let name: String = String::parse(r)?;
		let max_entries: i16 = i16::parse(r)?;
		let mut res: SvcCreateStringTable = SvcCreateStringTable {
			name,
			max_entries: max_entries.clone(),
			num_entries: i32::parse_amount(r, log2_of_x_plus_one(max_entries as usize))?,
			length: if net_protocol == 24 { r.read_var_int32()? } else { i32::parse_amount(r, 20)? },
			user_data_fixed_size: bool::parse(r)?,
			user_data_size: None,
			user_data_size_bits: None,
			flags: 0,
		};

		if res.user_data_fixed_size {
			res.user_data_size = Some(i16::parse_amount(r, 12)?);
			res.user_data_size_bits = Some(u8::parse_amount(r, 4)?)
		}

		if net_protocol >= 15 {
			res.flags = r.read_byte(if demo_protocol == 4 { 2 } else { 1 })?;
		}

		// TODO for way in the future: read string data
		r.skip(res.length as usize)?;

		return Ok(res);
	}
}

#[derive(Debug)]
pub struct SvcUpdateStringTable {
	table_id: u8,
	num_changed_entries: i32,
	length: i32,
	// data: ???
}

impl SvcUpdateStringTable {
	pub fn parse(r: &mut BitReader) -> anyhow::Result<SvcUpdateStringTable> {
		let res: SvcUpdateStringTable = SvcUpdateStringTable {
			table_id: u8::parse_amount(r, 5)?,
			num_changed_entries: if let Some(num) = Option::<i32>::parse_amount(r, 16)? { num } else { 1 },
			length: i32::parse_amount(r, 20)?
		};
		r.skip(res.length as usize)?;
		return Ok(res);
	}
}

#[derive(Debug)]
pub struct SvcVoiceInit {
	codec: String,
	quality: u8,
	sample_rate: Option<i32>
}

impl SvcVoiceInit {
	pub fn parse(r: &mut BitReader) -> anyhow::Result<SvcVoiceInit> {
		let mut res: SvcVoiceInit = SvcVoiceInit {
			codec: String::parse(r)?,
			quality: u8::parse(r)?,
			sample_rate: None,
		};
		if res.quality == 255 {
			res.sample_rate = Some(i32::parse(r)?)
		}
		return Ok(res);
	}
}

#[derive(Debug)]
pub struct SvcVoiceData {
	client: u8,
	proximity: u8,
	length: i16,
	audible: Vec<bool>,
	// data: ???
}

impl SvcVoiceData {
	pub fn parse(r: &mut BitReader) -> anyhow::Result<SvcVoiceData> {
		let res: SvcVoiceData = SvcVoiceData {
			client: u8::parse(r)?,
			proximity: u8::parse(r)?,
			length: i16::parse(r)?,
			audible: vec![bool::parse(r)?]
		};
		r.skip(res.length as usize)?;
		return Ok(res);
	}
}

#[derive(Debug, AutoParse)]
pub struct SvcPrint { message: String }

#[derive(Debug)]
pub struct SvcSounds {
	reliable_sounds: bool,
	num_sounds: u8,
	length: i16,
	// data: ???
}

impl SvcSounds {
	pub fn parse(r: &mut BitReader) -> anyhow::Result<SvcSounds> {
		let reliable_sounds: bool = bool::parse(r)?;
		let res: SvcSounds = SvcSounds {
			reliable_sounds: reliable_sounds.clone(),
			num_sounds: if reliable_sounds { 1 } else { u8::parse(r)? },
			length: if reliable_sounds { i16::parse_amount(r, 8)? } else { i16::parse(r)? },
		};
		r.skip(res.length as usize)?;
		return Ok(res);
	}
}

#[derive(Debug, AutoParse)]
pub struct SvcSetView {
	#[size(11)]
	ent_index: i16
}

#[derive(Debug)]
pub struct SvcFixAngle {
	pub relative: bool,
	pub angle: Vec<f32>,
}

impl SvcFixAngle {
	pub fn parse(r: &mut BitReader) -> anyhow::Result<SvcFixAngle> {
		let mut res: SvcFixAngle = SvcFixAngle {
			relative: bool::parse(r)?,
			angle: vec![],
		};

		for _ in 0..3 {
			res.angle.push(i16::parse(r)? as f32 * (360f32 / (1 << 16) as f32));
		}

		return Ok(res);
	}
}

#[derive(Debug)]
pub struct SvcCrosshairAngle {
	pub angle: Vec<f32>,
}

impl SvcCrosshairAngle {
	pub fn parse(r: &mut BitReader) -> anyhow::Result<SvcCrosshairAngle> {
		let mut res: SvcCrosshairAngle = SvcCrosshairAngle {
			angle: vec![],
		};

		for _ in 0..3 {
			res.angle.push(i16::parse(r)? as f32 * (360f32 / (1 << 16) as f32));
		}

		return Ok(res);
	}
}

#[derive(Debug)]
pub struct SvcBspDecal {
	pub pos: Vec<Option<f32>>,
	pub decal_texture_index: i16,
	pub entity_index: Option<i16>,
	pub model_index: Option<i16>,
	pub low_priority: bool,
}

impl SvcBspDecal {
	pub fn parse(r: &mut BitReader) -> anyhow::Result<SvcBspDecal> {
		let mut res: SvcBspDecal = SvcBspDecal {
			pos: r.read_vector_coords()?,
			decal_texture_index: i16::parse_amount(r, 9)?,
			entity_index: None,
			model_index: None,
			low_priority: false
		};

		if bool::parse(r)? {
			res.entity_index = Some(i16::parse_amount(r, 11)?);
			res.model_index = Some(i16::parse_amount(r, 11)?);
		}
		res.low_priority = bool::parse(r)?;

		return Ok(res);
	}
}

// p2 thing that i'll have because why not
#[derive(Debug)]
pub struct SvcSplitScreen {
	type_: bool,
	length: i16,
	// data: ???
}

impl SvcSplitScreen {
	pub fn parse(r: &mut BitReader) -> anyhow::Result<SvcSplitScreen> {
		let res: SvcSplitScreen = SvcSplitScreen {
			type_: bool::parse(r)?, length: i16::parse_amount(r, 11)?,
		};
		r.skip(res.length as usize)?;
		return Ok(res);
	}
}

// ...and then they put the 40 variations INTO one of the other 40 variations!!!
#[derive(Debug)]
pub struct SvcUserMessage {
	type_: u8,
	length: i16,
	// data: UserMessage, TODO: do this shit
}

impl SvcUserMessage {
	pub fn parse(r: &mut BitReader) -> anyhow::Result<SvcUserMessage> {
		let res: SvcUserMessage = SvcUserMessage {
			type_: u8::parse(r)?,
			length: i16::parse_amount(r, 11)?,
		};
		r.skip(res.length as usize)?;
		return Ok(res);
	}
}

#[derive(Debug)]
pub struct SvcEntityMessage {
	pub entity_index: i16,
	pub class_id: i16,
	pub length: i16,
	// data: ???
}

impl SvcEntityMessage {
	pub fn parse(r: &mut BitReader) -> anyhow::Result<Self> {
		let res: SvcEntityMessage = SvcEntityMessage {
			entity_index: i16::parse_amount(r, 11)?,
			class_id: i16::parse_amount(r, 9)?,
			length: i16::parse_amount(r, 11)?,
		};
		r.skip(res.length as usize)?;
		return Ok(res);
	}
}

#[derive(Debug, AutoParse)]
pub struct SvcGameEvent {
	length: i16,
	data: GameEvent
}

#[derive(Debug)]
pub struct SvcPacketEntities {
	pub max_entries: i16,
	pub is_delta: bool,
	pub delta_from: Option<i32>,
	pub base_line: bool,
	pub updated_entries: i16,
	pub length: i32,
	pub update_baseline: bool,
	// pub data: ???
}

impl SvcPacketEntities {
	pub fn parse(r: &mut BitReader) -> anyhow::Result<SvcPacketEntities> {
		let max_entries: i16 = i16::parse_amount(r, 11)?;
		let is_delta: bool = bool::parse(r)?;
		let mut delta_from: Option<i32> = None;
		if is_delta {
			delta_from = Some(i32::parse(r)?);
		}
		let res: SvcPacketEntities = SvcPacketEntities {
			max_entries,
			is_delta,
			delta_from,
			base_line: bool::parse(r)?,
			updated_entries: i16::parse_amount(r, 11)?,
			length: i32::parse_amount(r, 20)?,
			update_baseline: bool::parse(r)?,
		};
		r.skip(res.length as usize)?;
		return Ok(res);
	}
}

#[derive(Debug)]
pub struct SvcTempEntities {
	num_entries: u8,
	length: i32,
	// data: ???
}

impl SvcTempEntities {
	pub fn parse(r: &mut BitReader) -> anyhow::Result<SvcTempEntities> {
		let res: SvcTempEntities = SvcTempEntities {
			num_entries: u8::parse(r)?,
			length: if DEMO_INFO.lock().unwrap().net_protocol == 24 { r.read_var_int32()? } else { i32::parse_amount(r, 17)? }
		};
		r.skip(res.length as usize)?;
		return Ok(res);
	}
}

#[derive(Debug)]
pub struct SvcPrefetch {
	sound_index: i16,
	// sound_name: String, get this and store it here when you finish stringtables
}

impl SvcPrefetch {
	pub fn parse(r: &mut BitReader) -> anyhow::Result<SvcPrefetch> {
		let res: SvcPrefetch = SvcPrefetch {
			sound_index: i16::parse_amount(
				r, if DEMO_INFO.lock().unwrap().net_protocol == 24 { 14 } else { 13 }
			)?,
		};
		return Ok(res);
	}
}

// shouldn't be autoparse since theres data after the length but thats literally never caused an issue in iipdp
// there i dont even do r.skip(length) and everything still works fine
#[derive(Debug, AutoParse)]
pub struct SvcMenu {
	menu_type: i16,
	length: i32,
	// data: ???
}

#[derive(Debug)]
pub struct SvcGameEventList {
	events: i16,
	length: i32,
	// data will be stored in the public static GAME_EVENT_LIST as well
	descriptor_list: Vec<GameEventDescriptor>
}

impl SvcGameEventList {
	pub fn parse(r: &mut BitReader) -> anyhow::Result<SvcGameEventList> {
		let mut res: SvcGameEventList = SvcGameEventList {
			events: i16::parse_amount(r, 9)?,
			length: i32::parse_amount(r, 20)?,
			descriptor_list: vec![],
		};
		for _ in 0..res.events {
			res.descriptor_list.push(GameEventDescriptor::parse(r)?);
		}
		DEMO_INFO.lock().unwrap().game_event_list = res.descriptor_list.clone();
		return Ok(res);
	}
}

#[derive(Debug, AutoParse)]
pub struct SvcGetCvarValue {
	#[size(32)]
	cookie: String, // idk why this field is the way that it is
	cvar_name: String,
}

#[derive(Debug)]
pub struct SvcCmdKeyValues {
	length: i32,
	// data: ???
}

impl SvcCmdKeyValues {
	pub fn parse(r: &mut BitReader) -> anyhow::Result<SvcCmdKeyValues> {
		let res: SvcCmdKeyValues = SvcCmdKeyValues {
			length: i32::parse(r)?,
		};
		r.skip(res.length as usize)?;
		return Ok(res);
	}
}
