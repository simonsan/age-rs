// Copyright 2020-2020 the age-rs authors. See copying.md for legal info.

// Entrypoint for libnyan-rs

#![warn(
	missing_debug_implementations,
	missing_docs,
	rust_2018_idioms,
	rust_2018_compatibility
)]
#![warn(clippy::all)]

// `error_chain!` can recurse deeply
#![recursion_limit = "1024"]

#[macro_use]
extern crate error_chain;

// Errors module for error-chain
mod errors {
    // Create the Error, ErrorKind, ResultExt, and Result types
    error_chain! {}
}



pub mod cgs;
pub mod lex;
pub mod parse;

use errors::*;


fn main() {
    if let Err(ref e) = run() {
        println!("error: {}", e);

        for e in e.iter().skip(1) {
            println!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            println!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}


fn run() -> Result<()> {
    use std::fs::File;

    // This operation will fail
    File::open("contacts")
        .chain_err(|| "unable to open contacts file")?;

    Ok(())
}


#[cfg(test)]
mod tests {
	#[test]
	fn it_works() {
		assert_eq!(2 + 2, 4);
	}
}
