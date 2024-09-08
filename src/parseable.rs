use crate::reader::BitReader;
use crate::if_exists;

// the parseable trait and its implementation for basic types

pub trait Parseable {
	fn parse(r: &mut BitReader) -> anyhow::Result<Self> where Self: Sized;
	fn parse_amount(r: &mut BitReader, amount: usize) -> anyhow::Result<Self> where Self: Sized;
}

impl Parseable for i32 {
	fn parse(r: &mut BitReader) -> anyhow::Result<i32> { return r.read_int(32); }
	fn parse_amount(r: &mut BitReader, amount: usize) -> anyhow::Result<i32> { return r.read_int(amount); }
}

impl Parseable for Option<i32> {
	fn parse(r: &mut BitReader) -> anyhow::Result<Option<i32>> { return if_exists!(r, r.read_int(32)?); }
	fn parse_amount(r: &mut BitReader, amount: usize) -> anyhow::Result<Option<i32>> { return if_exists!(r, r.read_int(amount)?); }
}

impl Parseable for f32 {
	fn parse(r: &mut BitReader) -> anyhow::Result<f32> { return r.read_float(32); }
	fn parse_amount(_: &mut BitReader, _: usize) -> anyhow::Result<f32> { panic!("NO"); }
}

impl Parseable for Option<f32> {
	fn parse(r: &mut BitReader) -> anyhow::Result<Option<f32>> { return if_exists!(r, r.read_float(32)?); }
	fn parse_amount(_: &mut BitReader, _: usize) -> anyhow::Result<Self> where Self: Sized { panic!("NO"); }
}

impl Parseable for i16 {
	fn parse(r: &mut BitReader) -> anyhow::Result<i16> { return r.read_short(16); }
	fn parse_amount(r: &mut BitReader, amount: usize) -> anyhow::Result<i16> { return r.read_short(amount); }
}

impl Parseable for Option<i16> {
	fn parse(r: &mut BitReader) -> anyhow::Result<Option<i16>> { return if_exists!(r, r.read_short(16)?); }
	fn parse_amount(r: &mut BitReader, amount: usize) -> anyhow::Result<Option<i16>> { return if_exists!(r, r.read_short(amount)?); }
}

impl Parseable for u8 {
	fn parse(r: &mut BitReader) -> anyhow::Result<u8> { return r.read_byte(8); }
	fn parse_amount(r: &mut BitReader, amount: usize) -> anyhow::Result<u8> { return r.read_byte(amount); }
}

impl Parseable for Option<u8> {
	fn parse(r: &mut BitReader) -> anyhow::Result<Option<u8>> { return if_exists!(r, r.read_byte(8)?); }
	fn parse_amount(r: &mut BitReader, amount: usize) -> anyhow::Result<Option<u8>> { return if_exists!(r, r.read_byte(amount)?); }
}

impl Parseable for u64 {
	fn parse(r: &mut BitReader) -> anyhow::Result<u64> { return r.read_uint_64(); }
	fn parse_amount(r: &mut BitReader, amount: usize) -> anyhow::Result<u64> { return r.read_uint_64(); }
}

impl Parseable for String {
	fn parse(r: &mut BitReader) -> anyhow::Result<String> { return r.read_ascii_string_nulled(); }
	fn parse_amount(r: &mut BitReader, amount: usize) -> anyhow::Result<String> { return r.read_ascii_string(amount); }
}

impl Parseable for bool {
	fn parse(r: &mut BitReader) -> anyhow::Result<bool> { return r.read_bool(); }
	fn parse_amount(_: &mut BitReader, _: usize) -> anyhow::Result<bool> { panic!("NOOOO") }
}
