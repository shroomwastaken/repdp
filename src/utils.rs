use std::any::Any;
use std::fmt::{Debug, Display};

use crate::parseable::Parseable;
use crate::reader::BitReader;

// miscellaneous smaller structs and useful functions

pub struct Vec3<T> { pub x: T, pub y: T, pub z: T }

impl<T> Vec3<T> {
	pub fn as_vec(&self) -> Vec<&T> {
		vec![&self.x, &self.y, &self.z]
	}
}

impl<T: Parseable> Parseable for Vec3<T> {
	fn parse(r: &mut BitReader) -> anyhow::Result<Vec3<T>> {
		return Ok(Vec3 {
			x: T::parse(r)?,
			y: T::parse(r)?,
			z: T::parse(r)?,
		});
	}

	fn parse_amount(r: &mut BitReader, amount: usize) -> anyhow::Result<Vec3<T>> {
		return Ok(Vec3 {
			x: T::parse_amount(r, amount)?,
			y: T::parse_amount(r, amount)?,
			z: T::parse_amount(r, amount)?,
		});
	}
}

impl<T: Display> Display for Vec3<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:>10} {:>10} {:>10}", self.x, self.y, self.z)
	}
}

impl<T: Debug> Debug for Vec3<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		write!(f, "{:>10?} {:>10?} {:>10?}", self.x, self.y, self.z)
	}
}

// this just comes up a lot
// thanks untitledparser for neat implementation
pub fn log2_of_x_plus_one(x: usize) -> usize {
	let mut j: usize = 31;
	while (x & (1 << j)) == 0 { j -= 1 }
	return j + 1;
}