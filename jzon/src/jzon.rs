use std::string::String;
#[allow(dead_code)]
use std::vec::Vec;

#[derive(Debug)]
pub enum Jzon {
    Object(Vec<(String, Jzon)>),
    Array(Vec<Jzon>),
    String(String),
    Integer(i64),
    Double(f64),
    Bool(bool),
    Null,
}

#[derive(Debug)]
pub enum ParseErr {
    ParseError(String),
}

impl ParseErr {
    pub fn new() -> ParseErr {
        ParseErr::ParseError(String::from("parse error"))
    }
}

impl Jzon {
    pub fn parse(bytes: &[u8]) -> Result<Jzon, ParseErr> {
        match bytes[0] as char {
            ' ' | '\t' | '\r' | '\n' => Jzon::parse(&bytes[1..]),
            '-' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => {
                Jzon::parseNumber(bytes)
            }
            't' => Jzon::parseTrue(bytes),
            'f' => Jzon::parseFalse(bytes),
            'n' => Jzon::parseNull(bytes),
            '"' => Jzon::parseString(bytes),
            '{' => Jzon::parseObject(bytes),
            '[' => Jzon::parseArray(bytes),
            _ => Result::Err(ParseErr::new()),
        }
    }

    #[allow(non_snake_case)]
    fn parseObject(bytes: &[u8]) -> Result<Jzon, ParseErr> {
        let mut pairs: Vec<(String, Jzon)> = Vec::new();
        loop {
            let key = Jzon::parseString(&bytes[1..]).expect("expect string");
            let key_string = match key {
                Jzon::String(key_str) => key_str,
                _ => String::new(),
            };
        }
    }

    #[allow(non_snake_case)]
    fn parseArray(bytes: &[u8]) -> Result<Jzon, ParseErr> {
        Result::Err(ParseErr::new())
    }

    #[allow(non_snake_case)]
    fn parseTrue(bytes: &[u8]) -> Result<Jzon, ParseErr> {
        Result::Err(ParseErr::new())
    }

    #[allow(non_snake_case)]
    fn parseFalse(bytes: &[u8]) -> Result<Jzon, ParseErr> {
        Result::Err(ParseErr::new())
    }

    #[allow(non_snake_case)]
    fn parseNull(bytes: &[u8]) -> Result<Jzon, ParseErr> {
        Result::Err(ParseErr::new())
    }

    #[allow(non_snake_case)]
    fn parseNumber(bytes: &[u8]) -> Result<Jzon, ParseErr> {
        Result::Err(ParseErr::new())
    }

    #[allow(non_snake_case)]
    fn parseString(bytes: &[u8]) -> Result<Jzon, ParseErr> {
        Result::Err(ParseErr::new())
    }

    #[allow(non_snake_case)]
    fn parseStringLiteral(bytes: &[u8]) -> Result<String, ParseErr> {
        let mut s = String::new();
        let remain_bytes = &bytes[1..];
        loop {
            match remain_bytes[0] as char {
                '\\' => {
                    let _ = match remain_bytes[1] as char {
                        'b' => s.push(08 as char),
                        't' => s.push('\t'),
                        'n' => s.push('\n'),
                        'r' => s.push('\r'),
                        '"' => s.push('\"'),
                        '/' => s.push('/'),
                        '\\' => s.push('\\'),
                        _ => return Result::Err(ParseErr::new()),
                    };
                }
                _ => {}
            }
        }
    }
}
