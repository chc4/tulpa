#![allow(unused_imports)]
#![feature(crate_in_paths)]
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::BufReader;

pub mod parsing;

#[macro_use]
extern crate lalrpop_util;
lalrpop_mod!(pub parser);

extern crate luther;
#[macro_use]
extern crate luther_derive;
use luther::spanned::StrExt;

use crate::parsing::{Ast, Token};

fn main() -> io::Result<()> {
    use luther::Lexer;

    let mut f = File::open("example.tul")?;
    let mut prog = String::new();
    f.read_to_string(&mut prog)?;

    println!("{}", prog);

    let spanned = prog.spanned_chars();
    let tokens = Token::lexer(spanned).map_span(|s| s.into_inner());
    //thing(Box::new(tokens));

    //println!("{:?}", tokens.map(|entry| entry.map(|(_,tok,_)| tok) ).collect::<Vec<_>>());

    let parser = parser::TopParser::new();
    let out = parser.parse(tokens).unwrap();
    println!("{:?}", out);
    Ok(())
}
