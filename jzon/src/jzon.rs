#![allow(non_snake_case)]
#![allow(dead_code)]
use std::string::String;
use std::vec::Vec;
use std::collections::HashMap;
use std::char;
use std::convert::From;

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
    ExpectBool,
    ExpectNull,
    ExpectColon,
    ExpectValue,
    ExpectQuote,
    ExpectPrefix,
    ExpectEscaped,
    ExpectHexDigit,
    ExpectCodePoint,
    ExpectCommaBrace,
    ExpectCommaBracket,
}

pub struct State<T> {
    value: T,
    consumed: usize,
}

pub type ParseResult<T> = Result<State<T>, ParseErr>;

impl From<std::option::NoneError> for ParseErr {
    fn from(_ : std::option::NoneError) -> Self {
        ParseErr::ExpectCodePoint
    }

}

impl Jzon {
    const VALUE_TRUE  : Jzon = Jzon::Bool(true);
    const VALUE_FALSE : Jzon = Jzon::Bool(false);
    const VALUE_NULL  : Jzon = Jzon::Null;

    pub fn parse(bytes: &[u8]) -> ParseResult<Jzon> {
        match bytes[0] as char {
            ' ' | '\t' | '\r' | '\n' => Jzon::parse(&bytes[1..]),
            '-' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' | '0' => Jzon::parse_number(bytes),
            't' => Jzon::parse_true(bytes),
            'f' => Jzon::parse_false(bytes),
            'n' => Jzon::parse_null(bytes),
            '"' => Jzon::parse_string(bytes),
            '{' => Jzon::parse_object(bytes),
            '[' => Jzon::parse_array(bytes),
            _ => Err(ParseErr::ExpectPrefix),
        }
    }

    fn parse_object(bytes: &[u8]) -> ParseResult<Jzon> {
        let mut state = State{ value: Jzon::Object(HashMap::new()), consumed: 0 };

        if let Jzon::Object(ref mut map) = state.value {
            loop {
                match Jzon::parse_pair(&bytes[(1+state.consumed)..]) {
                    Ok(State{value: (key, value), consumed: n}) => {
                        map.insert(key, value);
                        state.consumed += n;
                    },
                    Err(e) => return Err(e)
                }

                match bytes[1+state.consumed] as char {
                    ',' => continue,
                    '}' => break,
                    ' ' | '\t' | '\n' | '\r' => continue,
                    _    => return Err(ParseErr::ExpectCommaBrace)
                }
            }

            Ok(state)
        } else {
            Err(ParseErr::ExpectCommaBrace)
        }
    }

    fn parse_pair(bytes: &[u8]) -> ParseResult<(String, Jzon)> {
        if bytes[0] as char == '"' {
            match Jzon::parse_string_literal(bytes) {
                Ok(State{value: k, consumed: nk}) => {
                    if bytes[nk] as char != ':' {
                        return Err(ParseErr::ExpectColon)
                    }

                    if let Ok(State{value, consumed: nv}) = Jzon::parse(&bytes[nk+1..]) {
                        return Ok(State{value: (k, value), consumed: nk+1+nv});
                    } else {
                        return Err(ParseErr::ExpectValue);
                    }
                },
                Err(e) => return Err(e),
            }
        } else {
            Err(ParseErr::ExpectQuote)
        }
    }

    fn parse_array(bytes: &[u8]) -> ParseResult<Jzon> {
        let mut state = State{ value: Jzon::Array(Vec::new()), consumed: 0 };

        if let Jzon::Array(ref mut vec) = state.value {
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
                    _    => return Err(ParseErr::ExpectCommaBracket)
                }
            }

            Ok(state)
        } else {
            Err(ParseErr::ExpectCommaBracket)
        }
    }

    fn parse_true(bytes: &[u8]) -> ParseResult<Jzon> {
        match bytes[0..4] {
            [b't', b'r', b'u', b'e'] => Ok(State{value: Jzon::Bool(true), consumed: 4}),
            _ => Err(ParseErr::ExpectBool)
        } 
    }

    fn parse_false(bytes: &[u8]) -> ParseResult<Jzon> {
        match bytes[0..5] {
            [b'f', b'a', b'l', b's', b'e'] => Ok(State{value: Jzon::Bool(true), consumed: 5}),
            _ => Err(ParseErr::ExpectBool)
        } 
    }

    fn parse_null(bytes: &[u8]) -> ParseResult<Jzon> {
        match bytes[0..4] {
            [b'n', b'u', b'l', b'l'] => Ok(State{value: Jzon::Bool(true), consumed: 4}),
            _ => Err(ParseErr::ExpectNull)
        } 
    }

    fn parse_number(_bytes: &[u8]) -> ParseResult<Jzon> {
        unimplemented!()
    }

    fn parse_string(bytes: &[u8]) -> ParseResult<Jzon> {
        let State{value, consumed} = Jzon::parse_string_literal(&bytes)?;
        Ok(State{value: Jzon::String(value), consumed})
    }

    fn parse_string_literal(bytes: &[u8]) -> ParseResult<String> {
        let mut s = String::new();
        let remain_bytes = &bytes[1..];
        loop {
            s.push(match remain_bytes[0] as char {
                '\\' => {
                    let State{value, consumed: _} = Jzon::parse_escaped(&bytes[1..])?;
                    value
                },
                ch => ch,
            });
        }
    }

    fn parse_escaped(bytes: &[u8]) -> ParseResult<char> {
        let ch = match bytes[1] as char {
            'b' => 8 as char,
            't' => '\t',
            'n' => '\n',
            'r' => '\r',
            '"' => '\"',
            '/' => '/',
            '\\' => '\\',
            'u' => return Jzon::parse_unicode(&bytes),
            _ => return Err(ParseErr::ExpectEscaped),
        };

        Ok(State{value: ch, consumed: 2})
    }

    fn parse_unicode(bytes: &[u8]) -> ParseResult<char> {
        assert_eq!(bytes[0], b'\\');
        assert_eq!(bytes[1], b'u');
        let mut consumed = 2;
        let state = Jzon::parse_hex4(&bytes[2..6])?;
        consumed += 4;
        let mut uc = state.value;

        if Jzon::invalid(uc) {
            return Err(ParseErr::ExpectCodePoint);
        }

        if 0xD800 <= uc && uc <= 0xDBFF {
            if !(bytes[6] == b'\\' && bytes[7] == b'u') {
                return Err(ParseErr::ExpectCodePoint);
            }
            consumed += 2;

            let State{value: uc2, consumed: _} = Jzon::parse_hex4(&bytes[8..12])?;
            consumed += 4;
            uc = 0x10000 + (((uc&0x3FF) << 10 | uc2) & 0x3FF);
        }

        let value = char::from_u32(uc)?;
        Ok(State{value, consumed})
    }

    fn parse_hex4(bytes: &[u8]) -> ParseResult<u32> {
        // and_then:: m a -> (a -> m b) -> m b
        if let Some(hex) = bytes[0..4].iter().enumerate().fold(
            Some(0u32), 
            |init, (i, ch)| (*ch as char).to_digit(16).and_then(|d| init.and_then(|x| Some(x + d * (0x1000u32 >> (i * 4))))))
        {
            Ok(State{value: hex, consumed: 4})
        } else {
            Err(ParseErr::ExpectHexDigit)
        }
    }

    fn invalid(cp: u32) -> bool { 
        0xDC00 <= cp && cp <= 0xDFFF || cp == 0 
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_unicode() {
        let s = Jzon::parse_unicode("\\u963f".as_bytes()).unwrap();
        assert_eq!('阿', s.value);
        assert_eq!(6, s.consumed);

        let s = Jzon::parse_unicode("\\u1FFc".as_bytes()).unwrap();
        assert_eq!('ῼ', s.value);
        assert_eq!(6, s.consumed);

        let s = Jzon::parse_unicode("\\ud801\\udc37".as_bytes()).unwrap();
        assert_eq!('𐐷', s.value);
        assert_eq!(12, s.consumed);
    }

    #[test]
    fn test_parse_hex4() {
        let state = Jzon::parse_hex4("aa01".as_bytes()).unwrap(); 
        assert_eq!(state.value, 0xaa01u32);

        let state = Jzon::parse_hex4("ffff".as_bytes()).unwrap(); 
        assert_eq!(state.value, 0xffffu32);
        
        assert!(Jzon::parse_hex4("fhff".as_bytes()).is_err());
    }
}
