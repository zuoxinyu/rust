#![allow(non_snake_case)]
#![allow(dead_code)]
use std::char;
use std::collections::HashMap;
use std::convert::From;
use std::string::String;
use std::vec::Vec;

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
    ExpectPair,
    ExpectBool,
    ExpectNull,
    ExpectColon,
    ExpectValue,
    ExpectQuote,
    ExpectDigit,
    ExpectPrefix,
    ExpectEscaped,
    ExpectHexDigit,
    ExpectCodePoint,
    ExpectCommaBrace,
    ExpectNoneControl,
    ExpectCommaBracket,
}

#[derive(Debug)]
pub struct State<T> {
    pub value: T,
    pub consumed: usize,
}

pub type ParseResult<T> = Result<State<T>, ParseErr>;

impl From<std::option::NoneError> for ParseErr {
    fn from(_: std::option::NoneError) -> Self {
        ParseErr::ExpectCodePoint
    }
}

const START: u16 = 2 << 0; // start
const ZERO: u16 = 2 << 1; // 0
const DOT: u16 = 2 << 2; // .
const DIGIT0: u16 = 2 << 3; // 0-9 after _nNoneZero
const DIGIT1: u16 = 2 << 4; // 0-9 after _nDot
const DIGIT2: u16 = 2 << 5; // 0-9 after _nExp or _nPlus or _Minus
const NONE_ZERO: u16 = 2 << 6; // 1-9
const EXP: u16 = 2 << 7; // e E
const PLUS: u16 = 2 << 8; // +
const MINUS: u16 = 2 << 9; // -
const NEG: u16 = 2 << 10; // -

macro_rules! or{
    ($s:expr, $e:expr) => {{
        $s | $e
    }};

    ($e:expr, $($es:expr),+) => {{
        $e | (or! { $($es),+ })
    }};

}

macro_rules! matchs {
    ($s:expr, $e:expr) => {{
        $s & $e > 0
    }};

    ($e:expr, $($es:expr),+) => {{
        $e & (or! { $($es),+ }) > 0
    }};
}

macro_rules! ImplParialEqForJzon {
    ($t:ty, $jt:path) => {
        impl PartialEq<$t> for Jzon {
            fn eq(&self, other: &$t) -> bool {
                if let $jt(v) = self {
                    v == other
                } else {
                    false
                }
            }
        }

        impl PartialEq<Jzon> for $t {
            fn eq(&self, other :&Jzon) -> bool {
                if let $jt(v) = other {
                    v == self
                } else {
                    false
                }

            }

        }
    };
}

ImplParialEqForJzon!(i64, Jzon::Integer);
ImplParialEqForJzon!(f64, Jzon::Double);
ImplParialEqForJzon!(&str, Jzon::String);
ImplParialEqForJzon!(String, Jzon::String);
ImplParialEqForJzon!(bool, Jzon::Bool);

impl Jzon {
    const VALUE_NULL: Jzon = Jzon::Null;
    const VALUE_TRUE: Jzon = Jzon::Bool(true);
    const VALUE_FALSE: Jzon = Jzon::Bool(false);

    pub fn parse(bytes: &[u8]) -> ParseResult<Jzon> {
        let spaces = Jzon::parse_space(bytes).unwrap();
        let bytes = &bytes[spaces.consumed..];
        let parsed = match bytes[0] as char {
            '-' | '0'..='9' => Jzon::parse_number(bytes),
            't' => Jzon::parse_true(bytes),
            'f' => Jzon::parse_false(bytes),
            'n' => Jzon::parse_null(bytes),
            '"' => Jzon::parse_string(bytes),
            '{' => Jzon::parse_object(bytes),
            '[' => Jzon::parse_array(bytes),
            _ => Err(ParseErr::ExpectPrefix),
        }?;

        Ok(State {
            value: parsed.value,
            consumed: parsed.consumed + spaces.consumed,
        })
    }

    fn parse_object(bytes: &[u8]) -> ParseResult<Jzon> {
        assert_eq!(bytes[0], b'{');
        let mut map = HashMap::new();
        let mut consumed = 1;
        let mut extra_comma = false;

        loop {
            match bytes[consumed] as char {
                ',' => {
                    if !extra_comma {
                        if map.is_empty() {
                            return Err(ParseErr::ExpectPair);
                        }
                        extra_comma = true;
                        consumed += 1;
                        continue;
                    }
                }
                ' ' | '\t' | '\n' | '\r' => {
                    consumed += 1;
                    continue;
                }
                '}' if !extra_comma => {
                    consumed += 1;
                    break;
                }
                _ => {
                    extra_comma = false;
                    let pair = Jzon::parse_pair(&bytes[consumed..])?;
                    map.insert(pair.value.0, pair.value.1);
                    consumed += pair.consumed;
                    continue;
                }
            }
        }

        Ok(State {
            value: Jzon::Object(map),
            consumed,
        })
    }

    fn parse_array(bytes: &[u8]) -> ParseResult<Jzon> {
        assert_eq!(bytes[0], b'[');
        let mut vec = Vec::new();
        let mut consumed = 1;
        let mut extra_comma = false;

        loop {
            match bytes[consumed] as char {
                ',' => {
                    if !extra_comma {
                        if vec.is_empty() {
                            return Err(ParseErr::ExpectValue);
                        }
                        extra_comma = true;
                        consumed += 1;
                        continue;
                    }
                }
                ' ' | '\t' | '\n' | '\r' => {
                    consumed += 1;
                    continue;
                }
                ']' if !extra_comma => {
                    consumed += 1;
                    break;
                }
                _ => {
                    extra_comma = false;
                    let elem = Jzon::parse(&bytes[consumed..])?;
                    vec.push(elem.value);
                    consumed += elem.consumed;
                    continue;
                }
            }
        }

        Ok(State {
            value: Jzon::Array(vec),
            consumed,
        })
    }

    fn parse_true(bytes: &[u8]) -> ParseResult<Jzon> {
        assert_eq!(bytes[0], b't');
        match bytes[0..4] {
            [b't', b'r', b'u', b'e'] => Ok(State {
                value: Jzon::VALUE_TRUE,
                consumed: 4,
            }),
            _ => Err(ParseErr::ExpectBool),
        }
    }

    fn parse_false(bytes: &[u8]) -> ParseResult<Jzon> {
        assert_eq!(bytes[0], b'f');
        match bytes[0..5] {
            [b'f', b'a', b'l', b's', b'e'] => Ok(State {
                value: Jzon::VALUE_FALSE,
                consumed: 5,
            }),
            _ => Err(ParseErr::ExpectBool),
        }
    }

    fn parse_null(bytes: &[u8]) -> ParseResult<Jzon> {
        assert_eq!(bytes[0], b'n');
        match bytes[0..4] {
            [b'n', b'u', b'l', b'l'] => Ok(State {
                value: Jzon::VALUE_NULL,
                consumed: 4,
            }),
            _ => Err(ParseErr::ExpectNull),
        }
    }
    fn parse_number(bytes: &[u8]) -> ParseResult<Jzon> {
        let mut consumed = 0;
        let mut n = 0i64;
        let mut e = 0i64;
        let mut t = 1.0f64;
        let mut f = 0.0f64;
        let mut negtive = 1;
        let mut is_float = false;
        let mut exp_pos = 1;
        let mut st = START;
        let mut ex = ZERO | NONE_ZERO;

        loop {
            match bytes[consumed] {
                b'-' if matchs!(st, START) => {
                    st = NEG;
                    ex = START;
                    negtive = -1;
                }
                b'0' if matchs!(st, START, NEG) => {
                    st = ZERO;
                    ex = DOT | EXP;
                }
                b'.' if matchs!(st, ZERO, DIGIT0, NONE_ZERO) => {
                    st = DOT;
                    ex = DIGIT1;
                    is_float = true;
                }
                d @ b'0'..=b'9' if matchs!(st, DOT, DIGIT1) => {
                    st = DIGIT1;
                    ex = DIGIT1 | EXP;
                    t *= 10f64;
                    f += (d - b'0') as f64 / t;
                }
                d @ b'1'..=b'9' if matchs!(st, START, NEG) => {
                    st = NONE_ZERO;
                    ex = DOT | DIGIT0 | EXP;
                    n = (d - b'0') as i64;
                    f = (d - b'0') as f64;
                }
                d @ b'0'..=b'9' if matchs!(st, DIGIT0, NONE_ZERO) => {
                    st = DIGIT0;
                    ex = DIGIT0 | EXP | DOT;
                    n = n * 10i64 + (d - b'0') as i64;
                    f = f * 10f64 + (d - b'0') as f64;
                }
                b'e' | b'E' if matchs!(st, ZERO, NONE_ZERO, DIGIT0, DIGIT1) => {
                    st = EXP;
                    ex = PLUS | MINUS | DIGIT2;
                    is_float = true;
                }
                b'+' if matchs!(st, EXP) => {
                    st = PLUS;
                    ex = DIGIT2;
                }
                b'-' if matchs!(st, EXP) => {
                    st = MINUS;
                    ex = DIGIT2;
                    exp_pos = -1;
                }
                d @ b'0'..=b'9' if matchs!(st, EXP, MINUS, PLUS, DIGIT2) => {
                    st = DIGIT2;
                    ex = DIGIT2;
                    e = e * 10 + (d - b'0') as i64;
                    f *= 10f64.powf((e * exp_pos) as f64);
                }
                _ => {
                    break;
                }
            }
            consumed += 1;
        }

        if matchs!(st, ZERO | NONE_ZERO | DIGIT0 | DIGIT1 | DIGIT2) {
            Ok(State {
                value: if is_float {
                    Jzon::Double(f * negtive as f64)
                } else {
                    Jzon::Integer(n * negtive)
                },
                consumed,
            })
        } else {
            println!("ex:{}", ex);
            Err(ParseErr::ExpectPrefix)
        }
    }

    fn parse_string(bytes: &[u8]) -> ParseResult<Jzon> {
        assert_eq!(bytes[0], b'"');
        let State { value, consumed } = Jzon::parse_string_literal(&bytes)?;
        Ok(State {
            value: Jzon::String(value),
            consumed,
        })
    }

    fn parse_pair(bytes: &[u8]) -> ParseResult<(String, Jzon)> {
        assert_eq!(bytes[0], b'"');
        if bytes[0] != b'"' {
            return Err(ParseErr::ExpectQuote);
        }

        let key = Jzon::parse_string_literal(bytes)?;
        let spaces = Jzon::parse_space(&bytes[key.consumed..]).unwrap();

        if bytes[key.consumed + spaces.consumed] != b':' {
            return Err(ParseErr::ExpectColon);
        }

        let val = Jzon::parse(&bytes[key.consumed + spaces.consumed + 1..])?;
        Ok(State {
            value: (key.value, val.value),
            consumed: key.consumed + spaces.consumed + 1 + val.consumed,
        })
    }

    fn parse_string_literal(bytes: &[u8]) -> ParseResult<String> {
        assert_eq!(bytes[0], b'"');
        let mut value = String::new();
        let mut consumed = 1;
        loop {
            match bytes[consumed] {
                b'\\' => {
                    let escaped = Jzon::parse_escaped(&bytes[consumed..])?;
                    value.push(escaped.value);
                    consumed += escaped.consumed;
                }
                b'\"' => {
                    consumed += 1;
                    break;
                }
                ch if (ch as char).is_control() => {
                    return Err(ParseErr::ExpectNoneControl);
                }
                ch => {
                    value.push(ch as char);
                    consumed += 1;
                }
            };
        }

        Ok(State { value, consumed })
    }

    fn parse_escaped(bytes: &[u8]) -> ParseResult<char> {
        assert_eq!(bytes[0], b'\\');
        let consumed = 2;
        let value = match bytes[1] as char {
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

        Ok(State { value, consumed })
    }

    fn parse_unicode(bytes: &[u8]) -> ParseResult<char> {
        assert_eq!(bytes[0], b'\\');
        assert_eq!(bytes[1], b'u');
        fn invalid(cp: u32) -> bool {
            0xDC00 <= cp && cp <= 0xDFFF || cp == 0
        }
        let mut consumed = 2;
        let state = Jzon::parse_hex4(&bytes[2..6])?;

        consumed += 4;
        let mut uc = state.value;

        if invalid(uc) {
            return Err(ParseErr::ExpectCodePoint);
        }

        if 0xD800 <= uc && uc <= 0xDBFF {
            if !(bytes[6] == b'\\' && bytes[7] == b'u') {
                return Err(ParseErr::ExpectCodePoint);
            }
            consumed += 2;

            let State {
                value: uc2,
                consumed: _,
            } = Jzon::parse_hex4(&bytes[8..12])?;
            consumed += 4;
            uc = 0x10000 + (((uc & 0x3FF) << 10 | uc2) & 0x3FF);
        }

        let value = char::from_u32(uc)?;
        Ok(State { value, consumed })
    }

    fn parse_hex4(bytes: &[u8]) -> ParseResult<u32> {
        // and_then:: m a -> (a -> m b) -> m b
        if let Some(hex) = bytes[0..4]
            .iter()
            .enumerate()
            .fold(Some(0u32), |init, (i, ch)| {
                (*ch as char)
                    .to_digit(16)
                    .and_then(|d| init.and_then(|x| Some(x + d * (0x1000u32 >> (i * 4)))))
            })
        {
            Ok(State {
                value: hex,
                consumed: 4,
            })
        } else {
            Err(ParseErr::ExpectHexDigit)
        }
    }

    #[inline]
    fn parse_space(bytes: &[u8]) -> ParseResult<()> {
        let value = ();
        let mut consumed = 0;
        loop {
            match bytes[consumed] as char {
                ' ' | '\t' | '\n' | '\r' => {
                    consumed += 1;
                    continue;
                }
                _ => break,
            }
        }

        Ok(State { value, consumed })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const JSON: &'static str = r#"
    {
    "string": "a string literal",
        "integer": -142,
        "double": -0.34E+12,
        "array": ["a", "b", "c", "d"],
        "object": {
            "nest-key": "nest value",
            "nest-int": 1
        }
    }"#;

    #[test]
    fn parse() {
        let jz = Jzon::parse(JSON.as_bytes());
        assert!(jz.is_ok());
    }

    #[test]
    fn parse_number() {
        let jz = Jzon::parse_number("0,".as_bytes()).unwrap();
        assert_eq!(jz.value, 0i64);
        let jz = Jzon::parse_number("-0,".as_bytes()).unwrap();
        assert_eq!(jz.value, 0);
        let jz = Jzon::parse_number("123,".as_bytes()).unwrap();
        assert_eq!(jz.value, 123);
        let jz = Jzon::parse_number("-123,".as_bytes()).unwrap();
        assert_eq!(jz.value, -123);
        let jz = Jzon::parse_number("123.45,".as_bytes()).unwrap();
        assert_eq!(jz.value, 123.45);
        let jz = Jzon::parse_number("1.23E10,".as_bytes()).unwrap();
        assert_eq!(jz.value, 1.23E10);
        let jz = Jzon::parse_number("1.23E+10,".as_bytes()).unwrap();
        assert_eq!(jz.value, 1.23E10);
        let jz = Jzon::parse_number("-1.23E-10,".as_bytes()).unwrap();
        assert_eq!(jz.value, -1.23E-10);
        let jz = Jzon::parse_number("-1E-10,".as_bytes()).unwrap();
        assert_eq!(jz.value, -1E-10);

        let jz = Jzon::parse_number("--1.23E-10,".as_bytes());
        assert!(jz.is_err());
        let jz = Jzon::parse_number("-1..23E-10,".as_bytes());
        assert!(jz.is_err());
        let jz = Jzon::parse_number("-1..23EE-10,".as_bytes());
        assert!(jz.is_err());
        let jz = Jzon::parse_number("-1..23E--10,".as_bytes());
        assert!(jz.is_err());
    }

    #[test]
    fn parse_string() {
        let jz = Jzon::parse_string(r#""a string literal","#.as_bytes()).unwrap().value;
        assert_eq!("a string literal", jz);
    }

    #[test]
    fn parse_unicode() {
        let s = Jzon::parse_unicode("\\u963f".as_bytes()).unwrap();
        assert_eq!('阿', s.value);
        assert_eq!(6, s.consumed);

        let s = Jzon::parse_unicode("\\u1FFc".as_bytes()).unwrap();
        assert_eq!('ῼ', s.value);
        assert_eq!(6, s.consumed);

        let s = Jzon::parse_unicode("\\ud801\\udc37".as_bytes()).unwrap();
        assert_eq!('𐀷', s.value);
        assert_eq!(12, s.consumed);
    }

    #[test]
    fn parse_hex4() {
        let state = Jzon::parse_hex4("aa01".as_bytes()).unwrap();
        assert_eq!(state.value, 0xaa01u32);

        let state = Jzon::parse_hex4("ffff".as_bytes()).unwrap();
        assert_eq!(state.value, 0xffffu32);

        assert!(Jzon::parse_hex4("fhff".as_bytes()).is_err());
    }
}
