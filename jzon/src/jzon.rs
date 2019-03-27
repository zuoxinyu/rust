#![allow(non_snake_case)]
#![allow(dead_code)]
use std::string::String;
use std::vec::Vec;
use std::mem;
use std::collections::HashMap;

trait HKT {
    type TypeA;
}

trait Rebind<T> {
    type TypeB;
}

trait Functor: HKT {
    fn fmap<Y, F:Fn(Self::TypeA) -> Y>(self, f: F) -> Self::TypeB where Self: Rebind<Y>;
}

trait Monad: Functor {
    // a -> M a
    fn unit(value: Self::TypeA) -> Self;
    // M a -> (a -> M b) -> M b
    fn bind<U, F: Fn(Self::TypeA) -> Self::TypeB>(self, f: F) -> Self::TypeB where Self: Rebind<U>;
    //fn join<U>(self) -> Self::TypeA where Self::TypeA: Equals<Self::TypeB>, Self: Rebind<U> + Sized;
}

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

trait JSON<T> {
    fn into_json(self) -> Vec<u8>;
    fn from_json(json: &[u8]) -> Result<T, ParseErr>;
}

struct Position {
    line: usize,
    column: usize,
}

struct State<T> {
    value: T,
    consumed: usize,
    pos: Position,
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
            ' ' | '\t' | '\r' | '\n'
            => Jzon::parse(&bytes[1..]),
            '-' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0'
            => Jzon::parseNumber(bytes),
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
            pos: Position{line: 0, column: 0}
        };

        let Jzon::Object(ref mut map) = state.value;
        loop {
            match Jzon::parsePair(&bytes[(1+state.consumed)..]) {
                Ok(State{value: (key, value), consumed: n, pos: pos}) => {
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
        match bytes[0] as char {
            '"' => match Jzon::parseStringLiteral(bytes) {
                    Ok(State{value: k, consumed: nk, pos: pos}) => {
                        if let Ok(State{value, consumed: nv, pos: pos}) = Jzon::parse(&bytes[nk..]) {
                            return Ok(State{value: (k, value), consumed: nk+nv, pos});
                        } else {
                            return Err(ParseErr::expect("value"));
                        }
                    },
                    Err(e) => return Err(e),
                }
            _ => return Err(ParseErr::expect("\""))
        }
    }

    fn parseArray(bytes: &[u8]) -> ParseResult<Jzon> {
        unimplemented!()
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

    #[allow(non_snake_case)]
    fn parseStringLiteral(bytes: &[u8]) -> ParseResult<String> {
        let mut s = String::new();
        let remain_bytes = &bytes[1..];
        loop {
            s.push(match remain_bytes[0] as char {
                '\\' => {
                    let State{value, consumed, pos} = Jzon::parseEscapedChar(&bytes[1..])?;
                    value
                },
                ch => ch,
            });
        }
    }

    #[allow(non_snake_case)]
    fn parseEscapedChar(bytes: &[u8]) -> ParseResult<char> {
        Ok(match bytes[1] as char {
            'b' => 08 as char,
            't' => '\t',
            'n' => '\n',
            'r' => '\r',
            '"' => '\"',
            '/' => '/',
            '\\' => '\\',
            'u' => Jzon::parseUnicodePoint(&bytes[1..])?,
            _ => return Err(ParseErr::new()),
        })
    }

    #[allow(non_snake_case)]
    fn parseUnicodePoint(bytes: &[u8]) -> ParseResult<char> {
        fn invalid(cp: u32) -> bool { 0xDC00 <= cp && cp <= 0xDFFF || cp == 0 }
        let mut uc = Jzon::parseHex4(&bytes[0..4])?;
        if invalid(uc) {
            return Err(ParseErr::expect("invalid codepoint"));
        }

        if 0xD800 <= uc && uc <= 0xDBFF {
            if !(bytes[4] == b'\\' && bytes[5] == b'u') {
                return Err(ParseErr::expect("need succeed codepoint"));
            }

            let uc2 = Jzon::parseHex4(&bytes[6..10])?;
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

        /*
        match len {
            4 => {parsed[3] = (uc | 0x80) & 0xBF; uc >>= 6;},
            3 => {parsed[2] = (uc | 0x80) & 0xBF; uc >>= 6;},
            2 => {parsed[1] = (uc | 0x80) & 0xBF; uc >>= 6;},
            1 => {parsed[0] = (uc | firstByteMarkMap[len]);},
            _ => (),
        };
        */

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

        return Err(ParseErr::expect(""));
    }

    #[allow(non_snake_case)]
    fn parseHex4(bytes: &[u8]) -> ParseResult<u32> {
        bytes[0..4].iter().enumerate().fold(
            Ok(0u32),
            |init, (i, ch)| {
                (*ch as char)
                    .to_digit(16)
                    .ok_or(ParseErr::expect("invalid hex digit character"))
                    .map(|res| init.unwrap() + res * (0x1000u32 >> (i * 4)))
            },
        )
    }
}

mod tests {
    use super::*;
    #[test]
    fn test_parse_hex() {
        let bytes: [u8; 4] = [b'4', b'e', b'2', b'd'];
        let res = Jzon::parseHex4(&bytes);
        assert_eq!(0x4e2d, res.unwrap());
    }
}
