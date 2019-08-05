extern crate jzon;
use jzon::jzon::*;
use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let mut args = env::args();
    if args.len() < 2 {
        return print_usage();
    }

    let mut text = args.nth(1).unwrap();
    if text == "-f" {
        match args.next() {
            Some(file_name) => text = fs::read_to_string(file_name)?,
            None => return print_usage(),
        }
    }

    match Jzon::parse(&text.into_bytes()) {
        Ok(jz) => println!("{}", jz),
        Err(e) => println!("{:?}", e),
    }

    Ok(())
}

fn print_usage() -> io::Result<()> {
    let exe = env::current_exe()?;
    println!("usage: {:?} <text> | -f <file> ", exe.file_stem().unwrap());
    Ok(())
}
