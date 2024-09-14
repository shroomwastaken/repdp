use std::io::Write;
use std::any::{Any, TypeId};
use crate::demo::Demo;

// check if type of value is a primitive
// not used but might later
fn is_primitive(value: &dyn Any) -> bool {
    vec![
		TypeId::of::<String>(),
		TypeId::of::<u8>(),
		TypeId::of::<i32>(),
		TypeId::of::<u64>(),
		TypeId::of::<i16>(),
		TypeId::of::<char>(),
	].contains(&value.type_id())
}

pub struct Dumper<'a> {
	pub demo: &'a Demo,
	pub tabs: u8,
	pub out: &'a mut dyn Write
}

impl<'a> Dumper<'a> {
	pub fn dump_header(&mut self) -> std::io::Result<()> {
		self.out.write_fmt(format_args!("{}", self.demo.header))
	}
}