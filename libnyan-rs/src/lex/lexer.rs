// Copyright 2020-2020 the age-rs authors. See copying.md for legal info.

// lexer.rs
// Lexical Analyzer stage
use logos::{
	self,
	Logos,
};

#[derive(Logos, Debug, Clone, Copy, PartialEq)]
enum Token {
	#[token("#", logos::skip)]
	COMMENT,

	#[token("as")]
	AS,

	#[token("@")]
	AT,

	#[token(":")]
	COLON,

	#[token(",")]
	COMMA,

	#[token(".")]
	DOT,

	#[regex(r"[\n]")]
	ENDLINE,

	#[token("...")]
	ELLIPSIS,

	#[regex(r"-?[0-9]+\.[0-9]*", |lex| lex.slice().parse())]
	Float(f64),

	#[token("from")]
	FROM,

	#[regex("[A-Za-z_][A-Za-z0-9_]*")]
	ID,

	#[token("import")]
	IMPORT,

	#[regex("(-|0[xX])?[0-9]+", |lex| lex.slice().parse())]
	Integer(i64),

	#[token("<")]
	LANGLE,

	#[token("{")]
	LBRACE,

	#[token("[")]
	LBRACKET,

	#[token("(")]
	LPAREN,

	#[regex("[-+*/|%&]|[-+*/|%&]=|=")]
	OPERATOR,

	#[token("pass")]
	PASS,

	#[token(">")]
	RANGLE,

	#[token("}")]
	RBRACE,

	#[token("]")]
	RBRACKET,

	#[token(")")]
	RPAREN,

	// Logos requires one token variant to handle errors,
	// it can be named anything you wish.
	#[error]
	ERROR,
	/* TODO
	 * #[token("")]
	 * INVALID, */

	/* #[token(" ")]
	 * INDENT, */

	/* #[token(" ")]
	 * DEDENT, */

	/* #[token("")]
	 * ENDFILE, */

	/* #[regex(b"\"(\.|[ˆ\"])*\"")]
	 * STRING, */
}

fn track_brackets() {
	unimplemented!()
}

fn handle_indent() {
	unimplemented!()
}

#[test]
fn lexer() {
	let mut lex = Token::lexer(concat!(
		"# NYAN FILE",
		"0.0f",
		"GameEntity.types = {Building, BuildingMisc}",
		// "Sound.sounds = o{"../shared/sounds/creation_sound_337.opus"}",
		"ResourceCost.amount = {WoodAmount, GoldAmount}",
		// "Animation.sprite = "./graphics/idle_archery_range.sprite"",
	));

	assert_eq!(lex.next(), Some(Token::COMMENT));
	assert_eq!(lex.next(), Some(Token::Float(0.0)));
	// assert_eq!(lex.slice(),"
	// abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789".
	// as_bytes());

	// assert_eq!(lex.next(), Some(Token::OtherAsciiPrintable));
	// assert_eq!(lex.slice(), "!\"#$%&'()*+,-./:;<=>?@[\\]^_`{|}~".as_bytes());

	assert_eq!(lex.next(), None);
}
