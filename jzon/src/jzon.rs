#[allow(dead_code)]
use std::string::String;
use std::vec::Vec;
use std::mem;

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

trait JSON<T> {
    fn into_json(self) -> Vec<u8>;
    fn from_json(json: &[u8]) -> Result<T, ParseErr>;
}

impl ParseErr {
    pub fn new() -> ParseErr {
        ParseErr::ParseError(String::from("parse error"))
    }

    pub fn expect(str: &str) -> ParseErr {
        ParseErr::ParseError(String::from(str))
    }
}

pub type ParseResult<T> = Result<T, ParseErr>;

impl Jzon {
    pub fn parse(bytes: &[u8]) -> ParseResult<Jzon> {
        match bytes[0] as char {
            ' ' | '\t' | '\r' | '\n' => Jzon::parse(&bytes[1..]),
            '-' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => Jzon::parseNumber(
                bytes,
            ),
            't' => Jzon::parseTrue(bytes),
            'f' => Jzon::parseFalse(bytes),
            'n' => Jzon::parseNull(bytes),
            '"' => Jzon::parseString(bytes),
            '{' => Jzon::parseObject(bytes),
            '[' => Jzon::parseArray(bytes),
            _ => Err(ParseErr::new()),
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
        Err(ParseErr::new())
    }

    #[allow(non_snake_case)]
    fn parseTrue(bytes: &[u8]) -> Result<Jzon, ParseErr> {
        Err(ParseErr::new())
    }

    #[allow(non_snake_case)]
    fn parseFalse(bytes: &[u8]) -> Result<Jzon, ParseErr> {
        Err(ParseErr::new())
    }

    #[allow(non_snake_case)]
    fn parseNull(bytes: &[u8]) -> Result<Jzon, ParseErr> {
        Err(ParseErr::new())
    }

    #[allow(non_snake_case)]
    fn parseNumber(bytes: &[u8]) -> Result<Jzon, ParseErr> {
        Err(ParseErr::new())
    }

    #[allow(non_snake_case)]
    fn parseString(bytes: &[u8]) -> Result<Jzon, ParseErr> {
        Err(ParseErr::new())
    }

    #[allow(non_snake_case)]
    fn parseStringLiteral(bytes: &[u8]) -> Result<String, ParseErr> {
        let mut s = String::new();
        let remain_bytes = &bytes[1..];
        loop {
            s.push(match remain_bytes[0] as char {
                '\\' => Jzon::parseEscapedChar(&bytes[1..])?,
                ch => ch,
            });
        }
    }

    #[allow(non_snake_case)]
    fn parseEscapedChar(bytes: &[u8]) -> Result<char, ParseErr> {
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
            // the `return` expression is type of `!` which is the subtype of all other types
        })
    }

    #[allow(non_snake_case)]
    fn parseUnicodePoint(bytes: &[u8]) -> Result<char, ParseErr> {
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
    fn parseHex4(bytes: &[u8]) -> Result<u32, ParseErr> {
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
