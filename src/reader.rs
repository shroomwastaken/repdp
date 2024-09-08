// taken straight from the original iipdp, this is probably its best part

// the struct and the read_bits, fetch and skip functions are from https://github.com/evanlin96069/sdp-c/blob/master/src/bits.c
// huge thanks to evanlin96069 for giving advice and making readable c code

// when calling these functions amount is measured in BITS!!!

use crate::error::ParserError;

#[macro_export]
macro_rules! if_exists { ($r:expr, $expr:expr) => { if $r.read_bool()? { Ok(Some($expr)) } else { Ok(None) } }; }

#[derive(Debug, Clone)]
pub struct BitReader<'a> {
	pub bits: &'a Vec<u8>,
	pub offset: u8,
	pub bit_size: usize,
	pub byte_size: usize,
	pub fetch: u64,
	pub current: usize,
}

impl<'a> BitReader<'a> {
	pub fn new(bits: &'a Vec<u8>) -> anyhow::Result<BitReader<'a>> {
		let mut new_reader: BitReader = BitReader {
			bits: bits,
			offset: 0,
			bit_size: 0,
			byte_size: 0,
			fetch: 0,
			current: 0,
		};
		new_reader.bits = bits;
		new_reader.bit_size = new_reader.bits.len() * 8;
		new_reader.byte_size = new_reader.bit_size / 8;
		new_reader.fetch()?;

		return Ok(new_reader);
	}

	pub fn fetch(&mut self) -> anyhow::Result<()> {
		let block: usize = if self.current / 8 + 8 > self.byte_size { self.byte_size - 8 } else { self.current / 8 };
		self.fetch = u64::from_le_bytes(self.bits[block..block + 8].try_into()?);
		self.offset = (self.current - (block * 8)) as u8;
		Ok(())
	}

	// reads bits
	pub fn read_bits(&mut self, amount: usize) -> anyhow::Result<u64> {
		if self.current + amount > self.bit_size {
			return Err(ParserError::ReaderOOBError(
				format!("reader overflow! {} + {} > {}", self.current, amount, self.bit_size)
			).into());
		}
		let mut new_amount: usize = amount.to_owned(); // this is so that i can modify the amount value
		let mut res: u64 = 0;
		let remain: usize = (64 - self.offset).into(); // bits remaining in the current fetch
		let mut shift: usize = 0; // uhhhhh


		if remain == 0 { // if we've gone through the entire fetch
			self.fetch()?;
		} else if new_amount > remain { // if we havent gone trough the entire fetch yet but we need to get the next one
			res |= (self.fetch >> self.offset) & ((1u64 << remain) - 1);
			self.skip(remain)?; // add remain bits to the current bit index, get new fetch (see skip function)
			new_amount -= remain; // how many bits are left to read
			shift = remain; // what position to put those bits in
		}

		/*
			EXAMPLE:

			fetch = 0b0111000000010100110100000000000000000000010001101001000011000000
			offset = 6
			amount = 8
			remain = 58
			shift: 0

			fetch >> offset                             = 0b0000000111000000010100110100000000000000000000010001101001000011
			(1u64 << amount) - 1                        = 0b0000000000000000000000000000000000000000000000000000000011111111
			(fetch >> offset) & ((1u64 << amount) - 1)  = 0b0000000000000000000000000000000000000000000000000000000001000011

			then we right shift it by "shift" to put the bits into their correct place in the number if we fetched above
		*/

		res |= ((self.fetch >> self.offset) & ((1u64 << new_amount) - 1)) << shift;

		self.current += new_amount;
		self.offset += new_amount as u8;
		return Ok(res);
	}

	pub fn skip(&mut self, amount: usize) -> anyhow::Result<()> {
		self.current += amount;
		if self.current > self.bit_size {
			panic!("overflow while skipping");
		}
		self.fetch()?;
		return Ok(());
	}

	// reads character by character until the character is \0
	pub fn read_ascii_string_nulled(&mut self) -> anyhow::Result<String> {
		let mut char_vec: Vec<u8> = Vec::new();
		let mut cur_char: u64 = self.read_bits(8)?;
		while cur_char != 0 {
			char_vec.push(cur_char as u8);
			cur_char = self.read_bits(8)?;
		}

		return Ok(String::from_utf8(char_vec)?.trim_end_matches("\0").to_string());
	}

	// read ascii string that has a determined length
	pub fn read_ascii_string(&mut self, amount: usize) -> anyhow::Result<String> {
		let mut char_vec: Vec<u8> = Vec::new();
		for _ in 0..amount / 8 {
			let cur_char: u64 = self.read_bits(8)?;
			char_vec.push(cur_char as u8);
		}

		return Ok(String::from_utf8(char_vec)?.trim_end_matches("\0").to_string());
	}

	// clones the bitreader and skips amount bits in the parent one
	pub fn split_and_skip(&mut self, amount: usize) -> anyhow::Result<BitReader> {
		let new_reader: BitReader = self.clone();

		self.skip(amount)?;

		return Ok(new_reader);
	}

	pub fn read_byte(&mut self, amount: usize) -> anyhow::Result<u8> {
		return Ok(self.read_bits(amount)? as u8);
	}

	pub fn read_uint(&mut self, amount: usize) -> anyhow::Result<u32> {
		return Ok(self.read_bits(amount)? as u32);
	}

	pub fn read_int(&mut self, amount: usize) -> anyhow::Result<i32> {
		return Ok(i32::from_le_bytes((self.read_bits(amount)? as u32).to_le_bytes()));
	}

	// shortcut
	pub fn read_short(&mut self, amount: usize) -> anyhow::Result<i16> { return Ok(self.read_int(amount)? as i16); }

	// i dont remember the difference between this and the previous method
	// and why this needs to exist
	pub fn read_sint(&mut self, amount: usize) -> anyhow::Result<i32> {
		let mut res: i32 = self.read_bits(amount)? as i32;
		if (res & (1 << (amount - 1))) != 0 {
			res |= i32::MAX << amount;
		}
		return Ok(res);
	}

	// i dislike steampipe
	// thanks jukspa :)
	pub fn read_var_int32(&mut self) -> anyhow::Result<i32> {
		let mut res: i32 = 0;

		for i in 0..5 {
			let b: i32 = self.read_uint(8)? as i32;
			res |= (b & 0x7F) << (7 * i);
			if (b & 0x80) == 0 { break; }
		}
		return Ok(res);
	}

	pub fn read_uint_64(&mut self) -> anyhow::Result<u64> { return self.read_bits(64); }

	pub fn read_float(&mut self, amount: usize) -> anyhow::Result<f32> {
		return Ok((f32::from_le_bytes((self.read_bits(amount)? as u32).to_le_bytes()) * 1000.0).round() / 1000.0);
	}

	// also used once
	pub fn read_bytes(&mut self, amount: usize) -> anyhow::Result<Vec<u8>> {
		let mut res: Vec<u8> = Vec::new();
		for _ in 0..amount {
			let cur_byte: u8 = self.read_bits(8)? as u8;
			res.push(cur_byte);
		}

		return Ok(res);
	}

	pub fn read_bool(&mut self) -> anyhow::Result<bool> { return Ok(self.read_bits(1)? == 1); }

	pub fn read_sint_if_exists(&mut self, amount: usize) -> anyhow::Result<Option<i32>> { return if_exists!(self, self.read_sint(amount)?) }

	// these next two funcitons i just copy-pasted from dem.nekz.me
	// no clue how they work but they do indeed work :)
	pub fn read_vector_coord(&mut self) -> anyhow::Result<f32> {
		let coord_integer_bits: usize = 14;
		let coord_fractional_bits: usize = 5;
		let coord_denominator: u8 = 1u8 << coord_fractional_bits;
		let coord_resolution: f32 = 1f32 / (coord_denominator as f32);

		let mut value: f32 = 0f32;
		let integer: bool = self.read_bool()?;
		let fraction: bool = self.read_bool()?;

		if integer || fraction {
			let sign: bool = self.read_bool()?;

			if integer { value += self.read_int(coord_integer_bits)? as f32; }
			if fraction { value += self.read_float(coord_fractional_bits)? * coord_resolution; }
			if sign { value = -value; }
		}

		return Ok(value);
	}

	// calls the previous function for x, y, and z coords
	pub fn read_vector_coords(&mut self) -> anyhow::Result<Vec<Option<f32>>> {
		let (x, y, z) = (self.read_bool()?, self.read_bool()?, self.read_bool()?);

		let mut coords_vec: Vec<Option<f32>> = Vec::new();

		if x { coords_vec.push(Some(self.read_vector_coord()?)) } else { coords_vec.push(None) }
		if y { coords_vec.push(Some(self.read_vector_coord()?)) } else { coords_vec.push(None) }
		if z { coords_vec.push(Some(self.read_vector_coord()?)) } else { coords_vec.push(None) }

		return Ok(coords_vec);
	}
}
