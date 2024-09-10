use std::io::Write;
use std::any::{Any, TypeId};
use crate::demo::Demo;
use crate::utils::Vec3;

// check if type of value is a primitive
// not used but might later
fn is_primitive(value: &dyn Any) -> bool {
    return vec![
		TypeId::of::<String>(),
		TypeId::of::<u8>(),
		TypeId::of::<i32>(),
		TypeId::of::<u64>(),
		TypeId::of::<i16>(),
		TypeId::of::<char>(),
	].contains(&value.type_id());
}

// doesnt work for types with types inside of them
// will improve in following commits
#[macro_export]
macro_rules! gen_dump_impl {
	($struct: ident) => {
		impl $struct {
			pub fn dump(&self) -> anyhow::Result<Vec<(String, String)>> {
				use std::fmt::Write;
				let mut buf: String = "".to_string();
				buf.write_fmt(format_args!("{self:#?}"))?;
				let interm: Vec<String> = buf.split("\n")
					.map(|s| s.trim().to_string())
					.collect();

				let res: Vec<(String, String)> = interm[1..interm.len() - 2].into_iter()
					.map(|s| {
						let mut it = s.split(": ");
						(it.next().unwrap().to_string(), it.next().unwrap().to_string())
					})
					.collect();

				return Ok(res);
			}
		}
	};
}

pub struct Dumper<'a> {
	pub demo: &'a Demo,
	pub tabs: u8,
	pub out: &'a mut dyn Write
}

impl<'a> Dumper<'a> {
	pub fn dump_header(&mut self) -> anyhow::Result<()> {
		for (mut k, v) in self.demo.header.dump()? {
			k.push(':');
			self.out.write_fmt(format_args!("{k:<20}{v}\n"))?;
		}

		return Ok(());
	}
}