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

    #[token(" ", handle_indent())]
    DEDENT,

    #[token(".")]
    DOT,

    #[token("")]
    ENDFILE,

 	#[regex(r"[\n]")]
    ENDLINE,

    #[token("...")]
    ELLIPSIS,

    #[regex(r"-?[0-9]+\.[0-9]*")]
    FLOAT,

    #[token("from")]
    FROM,

    #[regex("[A-Za-z_][A-Za-z0-9_]*")]
    ID,

    #[token("import")]
    IMPORT,

    #[token(" ", handle_indent())]
    INDENT,

    #[token("")]
    INVALID,

    #[regex(r"(-|0[xX])?[0-9]+")]
    INT,

    #[token("<")]
    LANGLE,

    #[token("{")]
    LBRACE,

    #[token("[")]
    LBRACKET,

    #[token("(")]
    LPAREN,

    #[regex(r"[-+*/|%&]|[-+*/|%&]=|=")]
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

    #[regex(r"(\.|[^\])*")]
    STRING,

	// Logos requires one token variant to handle errors,
    // it can be named anything you wish.
    #[error]
    ERROR,
}

