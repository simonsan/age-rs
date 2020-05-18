// Copyright 2020-2020 the age-rs authors. See copying.md for legal info.

// parser.rs
// Parsing stage

#[macro_use]
use pest_derive;
use pest;

enum NyanValue<'a> {
	Object(Vec<(&'a str, NyanValue<'a>)>),
	Array(Vec<NyanValue<'a>>),
	String(&'a str),
	Number(f64),
	Boolean(bool),
	Null,
}

#[derive(Parser)]
#[grammar = "nyan.pest"]
struct NyanParser;
