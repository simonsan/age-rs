// Copyright 2020-2020 the age-rs authors. See copying.md for legal info.

// lexer.rs
// Lexical Analyzer stage
use logos;
use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
enum Token {
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

    // TODO
    // #[token("")]
    // INVALID,

    // #[token(" ", handle_indent())]
    // INDENT,

    // #[token(" ", handle_indent())]
    // DEDENT,

    // #[token("")]
    // ENDFILE,

    // #[regex(b"\"(\.|[ˆ\"])*\"")]
    // STRING,
}

fn track_brackets() {} 

fn handle_indent() {}


#[test]
fn lexer() {
    let mut lex = Token::lexer(
    	concat!(
            "GameEntity.types = {Building, BuildingMisc}",
            "Sound.play_delay = 0.0f",
            // "Sound.sounds = o{"../shared/sounds/creation_sound_337.opus"}",
            "ResourceCost.amount = {WoodAmount, GoldAmount}",
            // "Animation.sprite = "./graphics/idle_archery_range.sprite"",
        ),
);
}