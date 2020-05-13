// Copyright 2020-2020 the age-rs authors. See copying.md for legal info.

// Parser for the nyan file format

#![warn(
	missing_debug_implementations,
	missing_docs,
	rust_2018_idioms,
	rust_2018_compatibility
)]
#![warn(clippy::all)]

extern crate pest;

#[macro_use]
extern crate pest_derive;

use pest::Parser;

enum NyanValue<'a> {
	Object(Vec<(&'a str, NyanValue<'a>)>),
	Array(Vec<NyanValue<'a>>),
	String(&'a str),
	Number(f64),
	Boolean(bool),
	Null,
}

#[derive(Parser)]
#[grammar = "json.pest"]
struct JSONParser;

#[cfg(test)]
mod tests {

	#[test]
	fn test_parse() {
		assert_eq!(2, 2);
	}
}
