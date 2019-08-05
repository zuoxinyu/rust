extern crate jzon;
use jzon::jzon::*;
use std::env;
use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let mut args = env::args();
    if args.len() < 2 {
        let exe = env::current_exe()?;
        println!(
            "usage: {} <text> | -f <file> ",
            exe.file_stem().unwrap().to_str().unwrap()
        );
        return Ok(());
    }

    let mut text = args.nth(1).unwrap();
    if text == "-f" {
        if let Some(file_name) = args.nth(2) {
            text = fs::read_to_string(file_name)?;
        }
    }

    match Jzon::parse(text.as_bytes()) {
        Ok(jz) => println!("{}", jz),
        Err(e) => println!("{:?}", e),
    }

    Ok(())
}
