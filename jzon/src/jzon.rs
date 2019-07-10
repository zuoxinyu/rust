#![allow(non_snake_case)]
#![allow(dead_code)]
use std::string::String;
use std::vec::Vec;
use std::collections::HashMap;
use std::char;
use std::mem;

#[derive(Debug)]
pub enum Jzon {
    Object(HashMap<String, Jzon>),
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
    Expect(String),
}

struct State<T> {
    value: T,
    consumed: usize,
}


impl ParseErr {
    pub fn new() -> ParseErr {
        ParseErr::ParseError(String::from("parse error"))
    }

    pub fn expect(str: &str) -> ParseErr {
        ParseErr::ParseError(String::from(str))
    }
}

type ParseResult<T> = Result<State<T>, ParseErr>;

impl Jzon {
    pub fn parse(bytes: &[u8]) -> ParseResult<Jzon> {
        match bytes[0] as char {
            ' ' | '\t' | '\r' | '\n' => Jzon::parse(&bytes[1..]),
            '-' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => Jzon::parseNumber(bytes),
            't' => Jzon::parseTrue(bytes),
            'f' => Jzon::parseFalse(bytes),
            'n' => Jzon::parseNull(bytes),
            '"' => Jzon::parseString(bytes),
            '{' => Jzon::parseObject(bytes),
            '[' => Jzon::parseArray(bytes),
            _ => Err(ParseErr::expect("invalid character")),
        }
    }

    fn parseObject(bytes: &[u8]) -> ParseResult<Jzon> {
        let mut state = State{
            value: Jzon::Object(HashMap::new()),
            consumed: 0,
        };

        let Jzon::Object(ref mut map) = state.value;
        loop {
            match Jzon::parsePair(&bytes[(1+state.consumed)..]) {
                Ok(State{value: (key, value), consumed: n}) => {
                    map[&key] = value;
                    state.consumed += n;
                },
                Err(e) => return Err(e)
            }

            match bytes[1+state.consumed] as char {
                ',' => continue,
                '}' => break,
                ' ' | '\t' | '\n' | '\r' => continue,
                _    => return Err(ParseErr::expect(",}"))
            }
        }

        Ok(state)
    }

    fn parsePair(bytes: &[u8]) -> ParseResult<(String, Jzon)> {
        if bytes[0] as char == '"' {
            match Jzon::parseStringLiteral(bytes) {
                Ok(State{value: k, consumed: nk}) => {
                    if bytes[nk] as char != ':' {
                        return Err(ParseErr::expect(":"))
                    }

                    if let Ok(State{value, consumed: nv}) = Jzon::parse(&bytes[nk+1..]) {
                        return Ok(State{value: (k, value), consumed: nk+1+nv});
                    } else {
                        return Err(ParseErr::expect("value"));
                    }
                },
                Err(e) => return Err(e),
            }
        } else {
            Err(ParseErr::expect("\""))
        }
    }

    fn parseArray(bytes: &[u8]) -> ParseResult<Jzon> {
        let mut state = State{
            value: Jzon::Array(Vec::new()),
            consumed: 0,
        };

        let Jzon::Array(ref mut vec) = state.value;
        loop {

            match Jzon::parse(&bytes[(1+state.consumed)..]) {
                Ok(State{value: v, consumed: n}) => {
                    vec.push(v);
                    state.consumed += n;
                },
                Err(e) => return Err(e)
            }

            match bytes[1+state.consumed] as char {
                ',' => continue,
                ']' => break,
                ' ' | '\t' | '\n' | '\r' => continue,
                _    => return Err(ParseErr::expect(",}"))
            }
        }

        Ok(state)
    }

    fn parseTrue(bytes: &[u8]) -> ParseResult<Jzon> {
        unimplemented!()
    }

    fn parseFalse(bytes: &[u8]) -> ParseResult<Jzon> {
        Err(ParseErr::new())
    }

    fn parseNull(bytes: &[u8]) -> ParseResult<Jzon> {
        Err(ParseErr::new())
    }

    fn parseNumber(bytes: &[u8]) -> ParseResult<Jzon> {
        Err(ParseErr::new())
    }

    fn parseString(bytes: &[u8]) -> ParseResult<Jzon> {
        Err(ParseErr::new())
    }

    fn parseStringLiteral(bytes: &[u8]) -> ParseResult<String> {
        let mut s = String::new();
        let remain_bytes = &bytes[1..];
        loop {
            s.push(match remain_bytes[0] as char {
                '\\' => {
                    let State{value, consumed} = Jzon::parseEscapedChar(&bytes[1..])?;
                    value
                },
                ch => ch,
            });
        }
    }

    fn parseEscapedChar(bytes: &[u8]) -> ParseResult<char> {
        let ch = match bytes[1] as char {
            'b' => 8 as char,
            't' => '\t',
            'n' => '\n',
            'r' => '\r',
            '"' => '\"',
            '/' => '/',
            '\\' => '\\',
            'u' => return Jzon::parseUnicodePoint(&bytes[1..]),
            _ => return Err(ParseErr::new()),
        };

        Ok(State{value: ch, consumed: 2})
    }

    fn parseUnicodePoint(bytes: &[u8]) -> ParseResult<char> {
        fn invalid(cp: u32) -> bool { 0xDC00 <= cp && cp <= 0xDFFF || cp == 0 }
        let uc = Jzon::parseHex4(&bytes[0..4]);
        if invalid(uc) {
            return Err(ParseErr::expect("invalid codepoint"));
        }

        if 0xD800 <= uc && uc <= 0xDBFF {
            if !(bytes[4] == b'\\' && bytes[5] == b'u') {
                return Err(ParseErr::expect("need succeed codepoint"));
            }

            let uc2 = Jzon::parseHex4(&bytes[6..10]);
            if invalid(uc2) {
                return Err(ParseErr::expect("invalid codepoint"));
            }

            uc = 0x10000 + (((uc&0x3FF) << 10 | uc2) & 0x3FF);
        }

        let len = match uc {
            uc if uc < 0x80 => 1,
            uc if uc < 0x800 => 2,
            uc if uc < 0x10000 => 3,
            _ => 4
        };
        let mut parsed : [u8;4] = [0,0,0,0];
        let firstByteMarkMap = [0x00, 0x00, 0xC0, 0xE0, 0xF0];
        for i in 1..len {
            parsed[i] = ((uc | 0x80) & 0xBF) as u8; uc >>= 6;
        }
        parsed[0] = (uc | firstByteMarkMap[len]) as u8;

        let mut realParsed : [u8;4] = [0,0,0,0];
        for (i, c) in parsed.iter().enumerate() {
            if (*c) != 0u8 {
                realParsed[i] = *c;
            }
        }

        let mut u : u32 = 0;
        unsafe {
            u = mem::transmute::<[u8;4], u32>(realParsed);
        }

        if let Some(uchar) = char::from_u32(u) {
            Ok(State{ value: uchar, consumed: realParsed.len()})
        } else {
            Err(ParseErr::expect("unicode point"))
        }

    }

    fn parseHex4(bytes: &[u8]) -> u32 {
        //let hex = bytes[0..4].iter().enumerate().fold(0, |init, (i, ch)| { ((*ch) as char).to_digit(16).map(|res| init + res * (0x1000u32 >> (i * 4))) });
        0
    }
}

