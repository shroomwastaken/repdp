use core::fmt;
use std::error::Error;

#[derive(Debug)]
pub enum ParserError {
	ArgumentError(String),
	InvalidFile,
	ReaderOOBError(String),
	ParserError(String),
	UnsupportedDemo(String)
}

impl fmt::Display for ParserError {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			ParserError::ReaderOOBError(s) => {
				Ok(f.write_fmt(format_args!("reader out of bounds error: {}", s))?)
			},
			ParserError::InvalidFile => {
				Ok(f.write_str("file has invalid signature!")?)
			},
			ParserError::ArgumentError(s) => {
				Ok(f.write_fmt(format_args!("invalid arguments provided: {s}"))?)
			},
			ParserError::ParserError(s) => {
				Ok(f.write_fmt(format_args!("parser error: {s}"))?)
			},
			ParserError::UnsupportedDemo(s) => {
				Ok(f.write_fmt(format_args!("unsupported demo: {s}"))?)
			},
		}
	}
}

impl Error for ParserError {
	fn source(&self) -> Option<&(dyn Error + 'static)> {
		match self {
			ParserError::ReaderOOBError(_) => None,
			ParserError::InvalidFile => None,
			ParserError::ArgumentError(_) => None,
			ParserError::ParserError(_) => None,
			ParserError::UnsupportedDemo(_) => None,
		}
	}
}