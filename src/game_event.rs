/*
the game gives us a SvcGameEventList message where it gives descriptors for
game events that appear later in SvcGameEvent messages.
each descriptor provides an id, a name and a key_definitions hashmap that pairs (key name) to (key type)
therefore, when we read an SvcGameEvent we
1) get its id
2) look it up in the GameEventList
3) understand that this event will contain key_values with the described types
4) read the keys
*/

use std::collections::HashMap;

use crate::parsing::DEMO_INFO;
use crate::reader::BitReader;
use crate::parseable::Parseable;

#[derive(Debug)]
pub enum GameEventKeyType {
	Boolean(bool),
	Float(f32),
	Int16(i16),
	Int32(i32),
	Int8(u8),
	String(String),
	UInt64(u64),
}

#[derive(Debug, Clone)]
pub struct GameEventDescriptor {
	event_id: i16,
	name: String,
	key_definitions: HashMap<String, u8>
}

impl GameEventDescriptor {
	pub fn parse(r: &mut BitReader) -> anyhow::Result<GameEventDescriptor> {
		let mut res: GameEventDescriptor = GameEventDescriptor {
			event_id: i16::parse_amount(r, 9)?,
			name: String::parse(r)?,
			key_definitions: HashMap::new(),
		};
		let mut val_type: u8 = u8::parse_amount(r, 3)?;
		while val_type != 0 {
			res.key_definitions.insert(String::parse(r)?, val_type);
			val_type = u8::parse_amount(r, 3)?;
		}
		return Ok(res);
	}
}

#[derive(Debug)]
pub struct GameEvent {
	key_values: HashMap<String, GameEventKeyType>
}

impl GameEvent {
	pub fn parse(r: &mut BitReader) -> anyhow::Result<GameEvent> {
		let game_event_list: &Vec<GameEventDescriptor> = &DEMO_INFO.lock().unwrap().game_event_list;
		let mut res: GameEvent = GameEvent {
			key_values: HashMap::new(),
		};

		let event_id: i16 = i16::parse_amount(r, 9)?;
		// game event list is hopefully populated by now
		let descriptor: &GameEventDescriptor = &game_event_list[event_id as usize];
		for (name, value_type) in &descriptor.key_definitions {
			res.key_values.insert(
				name.to_string(), match value_type {
					1 => { GameEventKeyType::String(String::parse(r)?) }
					2 => { GameEventKeyType::Float(f32::parse(r)?) }
					3 => { GameEventKeyType::Int32(i32::parse(r)?) }
					4 => { GameEventKeyType::Int16(i16::parse(r)?) }
					5 => { GameEventKeyType::Int8(u8::parse(r)?) }
					6 => { GameEventKeyType::Boolean(bool::parse(r)?) }
					7 => { GameEventKeyType::UInt64(u64::parse(r)?) }
					_ => { panic!("huh!") }
				}
			);
		}

		return Ok(res);
	}
}
