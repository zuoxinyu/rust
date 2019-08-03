use std::char;
use std::collections::HashMap;
use std::convert::From;
use std::f64;
use std::fmt;
use std::str;
use std::string::String;
use std::vec::Vec;

#[derive(Debug)]
pub enum ParseErr {
    ExpectPair,
    ExpectBool,
    ExpectNull,
    ExpectColon,
    ExpectValue,
    ExpectQuote,
    ExpectDigit,
    ExpectNoMore,
    ExpectPrefix,
    ExpectNoneEOF,
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

macro_rules! matches {
    ($s:expr, $e:expr) => {{
        $s & $e > 0
    }};
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

impl fmt::Display for Jzon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let concat = |init: String, s: String| {
            if init.is_empty() {
                s.clone()
            } else {
                init.clone() + "," + &s
            }
        };
        match self {
            Jzon::Null => write!(f, "null"),
            Jzon::Bool(true) => write!(f, "true"),
            Jzon::Bool(false) => write!(f, "false"),
            Jzon::Double(v) => write!(f, "{}", v),
            Jzon::Integer(v) => write!(f, "{}", v),
            Jzon::String(v) => write!(f, "\"{}\"", v),
            Jzon::Object(map) => write!(
                f,
                "{{{}}}",
                map.iter()
                    .map(|(k, v)| format!("\"{}\":{}", k, v))
                    .fold("".to_owned(), concat)
            ),
            Jzon::Array(vec) => write!(
                f,
                "[{}]",
                vec.iter()
                    .map(|v| format!("{}", v))
                    .fold("".to_owned(), concat)
            ),
        }
    }
}

macro_rules! impl_parial_eq_for_jzon {
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
            fn eq(&self, other: &Jzon) -> bool {
                if let $jt(v) = other {
                    v == self
                } else {
                    false
                }
            }
        }
    };
}

impl_parial_eq_for_jzon!(i64, Jzon::Integer);
impl_parial_eq_for_jzon!(&str, Jzon::String);
impl_parial_eq_for_jzon!(String, Jzon::String);
impl_parial_eq_for_jzon!(bool, Jzon::Bool);

impl PartialEq<f64> for Jzon {
    fn eq(&self, other: &f64) -> bool {
        if let Jzon::Double(v) = self {
            f64::abs(v - other) < f64::EPSILON
        } else {
            false
        }
    }
}

impl PartialEq<Jzon> for f64 {
    fn eq(&self, other: &Jzon) -> bool {
        if let Jzon::Double(v) = other {
            f64::abs(v - self) < f64::EPSILON
        } else {
            false
        }
    }
}

use ParseErr::*;

impl Jzon {
    const VALUE_NULL: Jzon = Jzon::Null;
    const VALUE_TRUE: Jzon = Jzon::Bool(true);
    const VALUE_FALSE: Jzon = Jzon::Bool(false);

    pub fn parse(bytes: &[u8]) -> ParseResult<Jzon> {
        let mut v = Jzon::parse_value(bytes)?;
        let State { consumed, .. } = &mut v;

        loop {
            match bytes.iter().nth(*consumed) {
                Some(b' ') | Some(b'\t') | Some(b'\r') | Some(b'\n') => {
                    *consumed += 1;
                }
                Some(_) => break Err(ExpectNoMore),
                None => break Ok(v),
            }
        }
    }

    fn parse_value(bytes: &[u8]) -> ParseResult<Jzon> {
        let spaces = Jzon::parse_space(bytes).unwrap();
        let bytes = &bytes[spaces.consumed..];
        let mut it = bytes.iter();

        let parsed = match it.next() {
            Some(ch) => match *ch {
                b'-' | b'0'..=b'9' => Jzon::parse_number(bytes),
                b't' => Jzon::parse_true(bytes),
                b'f' => Jzon::parse_false(bytes),
                b'n' => Jzon::parse_null(bytes),
                b'"' => Jzon::parse_string(bytes),
                b'{' => Jzon::parse_object(bytes),
                b'[' => Jzon::parse_array(bytes),
                _ => Err(ExpectPrefix),
            },
            None => Err(ExpectNoneEOF),
        }?;

        let consumed = parsed.consumed + spaces.consumed;
        let value = parsed.value;
        Ok(State { value, consumed })
    }

    fn parse_object(bytes: &[u8]) -> ParseResult<Jzon> {
        let mut map = HashMap::new();
        let mut consumed = 1;
        let mut extra_comma = false;

        loop {
            match bytes.iter().nth(consumed) {
                Some(ch) => match *ch as char {
                    ',' if !extra_comma && !map.is_empty() => {
                        extra_comma = true;
                        consumed += 1;
                        continue;
                    }
                    ' ' | '\t' | '\n' | '\r' => {
                        consumed += 1;
                        continue;
                    }
                    '}' if !extra_comma => {
                        consumed += 1;
                        break;
                    }
                    '"' => {
                        extra_comma = false;
                        let pair = Jzon::parse_pair(&bytes[consumed..])?;
                        map.insert(pair.value.0, pair.value.1);
                        consumed += pair.consumed;
                        continue;
                    }
                    _ => {
                        return Err(ExpectPair);
                    }
                },
                None => return Err(ExpectNoneEOF),
            }
        }

        Ok(State {
            value: Jzon::Object(map),
            consumed,
        })
    }

    fn parse_array(bytes: &[u8]) -> ParseResult<Jzon> {
        let mut vec = Vec::new();
        let mut consumed = 1;
        let mut extra_comma = false;

        loop {
            match bytes.iter().nth(consumed) {
                Some(ch) => match *ch as char {
                    ',' if !extra_comma && !vec.is_empty() => {
                        extra_comma = true;
                        consumed += 1;
                        continue;
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
                        let elem = Jzon::parse_value(&bytes[consumed..])?;
                        vec.push(elem.value);
                        consumed += elem.consumed;
                        continue;
                    }
                },
                None => {
                    return Err(ExpectNoneEOF);
                }
            }
        }

        Ok(State {
            value: Jzon::Array(vec),
            consumed,
        })
    }

    fn parse_true(bytes: &[u8]) -> ParseResult<Jzon> {
        if bytes.len() < 4 {
            return Err(ExpectNoneEOF);
        }

        match bytes[0..4] {
            [b't', b'r', b'u', b'e'] => Ok(State {
                value: Jzon::VALUE_TRUE,
                consumed: 4,
            }),
            _ => Err(ExpectBool),
        }
    }

    fn parse_false(bytes: &[u8]) -> ParseResult<Jzon> {
        if bytes.len() < 5 {
            return Err(ExpectNoneEOF);
        }
        match bytes[0..5] {
            [b'f', b'a', b'l', b's', b'e'] => Ok(State {
                value: Jzon::VALUE_FALSE,
                consumed: 5,
            }),
            _ => Err(ExpectBool),
        }
    }

    fn parse_null(bytes: &[u8]) -> ParseResult<Jzon> {
        if bytes.len() < 4 {
            return Err(ExpectNoneEOF);
        }
        match bytes[0..4] {
            [b'n', b'u', b'l', b'l'] => Ok(State {
                value: Jzon::VALUE_NULL,
                consumed: 4,
            }),
            _ => Err(ExpectNull),
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
        let mut it = bytes.iter();

        loop {
            match it.next() {
                Some(ch) => match *ch {
                    b'-' if matches!(st, START) => {
                        st = NEG;
                        negtive = -1;
                    }
                    b'0' if matches!(st, START | NEG) => {
                        st = ZERO;
                    }
                    b'.' if matches!(st, ZERO | DIGIT0 | NONE_ZERO) => {
                        st = DOT;
                        is_float = true;
                    }
                    d @ b'0'..=b'9' if matches!(st, DOT | DIGIT1) => {
                        st = DIGIT1;
                        t *= 10f64;
                        f += (d - b'0') as f64 * negtive as f64 / t;
                    }
                    d @ b'1'..=b'9' if matches!(st, START | NEG) => {
                        st = NONE_ZERO;
                        n = negtive * (d - b'0') as i64;
                        f = negtive as f64 * (d - b'0') as f64;
                    }
                    d @ b'0'..=b'9' if matches!(st, DIGIT0 | NONE_ZERO) => {
                        st = DIGIT0;
                        n = n * 10i64 + negtive * (d - b'0') as i64;
                        f = f * 10f64 + negtive as f64 * ((d - b'0') as f64);
                    }
                    b'e' | b'E' if matches!(st, ZERO | NONE_ZERO | DIGIT0 | DIGIT1) => {
                        st = EXP;
                        is_float = true;
                    }
                    b'+' if matches!(st, EXP) => {
                        st = PLUS;
                    }
                    b'-' if matches!(st, EXP) => {
                        st = MINUS;
                        exp_pos = -1;
                    }
                    d @ b'0'..=b'9' if matches!(st, EXP | MINUS | PLUS | DIGIT2) => {
                        st = DIGIT2;
                        e = e * 10 + (d - b'0') as i64;
                        f *= 10f64.powf((e * exp_pos) as f64);
                    }
                    _ => break,
                },
                None => return Err(ExpectNoneEOF),
            }
            consumed += 1;
        }

        if !matches!(st, ZERO | NONE_ZERO | DIGIT0 | DIGIT1 | DIGIT2) {
            return Err(ExpectPrefix);
        }

        Ok(State {
            value: if is_float {
                Jzon::Double(f)
            } else {
                Jzon::Integer(n)
            },
            consumed,
        })
    }

    fn parse_string(bytes: &[u8]) -> ParseResult<Jzon> {
        let State { value, consumed } = Jzon::parse_string_literal(&bytes)?;
        Ok(State {
            value: Jzon::String(value),
            consumed,
        })
    }

    fn parse_pair(bytes: &[u8]) -> ParseResult<(String, Jzon)> {
        let key = Jzon::parse_string_literal(bytes)?;
        let spaces = Jzon::parse_space(&bytes[key.consumed..]).unwrap();

        if bytes[key.consumed + spaces.consumed] != b':' {
            return Err(ExpectColon);
        }

        let val = Jzon::parse_value(&bytes[key.consumed + 1 + spaces.consumed..])?;
        Ok(State {
            value: (key.value, val.value),
            consumed: key.consumed + spaces.consumed + 1 + val.consumed,
        })
    }

    fn parse_string_literal(bytes: &[u8]) -> ParseResult<String> {
        let mut value = String::new();
        let mut consumed = 1;
        loop {
            match bytes.iter().nth(consumed) {
                Some(ch) => match *ch {
                    b'\\' => {
                        let escaped = Jzon::parse_escaped(&bytes[consumed..])?;
                        value.push(escaped.value);
                        consumed += escaped.consumed;
                    }
                    b'\"' => {
                        consumed += 1;
                        break;
                    }
                    // according to ECMA-404
                    0x0000..=0x001F => {
                        return Err(ExpectNoneControl);
                    }
                    ch => {
                        value.push(ch as char);
                        consumed += 1;
                    }
                },
                None => return Err(ExpectNoneEOF),
            }
        }

        Ok(State { value, consumed })
    }

    fn parse_escaped(bytes: &[u8]) -> ParseResult<char> {
        let consumed = 2;
        let mut it = bytes[1..].iter();
        let value = match it.next() {
            Some(b'b') => 8 as char,
            Some(b't') => '\t',
            Some(b'f') => 12 as char,
            Some(b'n') => '\n',
            Some(b'r') => '\r',
            Some(b'"') => '\"',
            Some(b'/') => '/',
            Some(b'\\') => '\\',
            Some(b'u') => return Jzon::parse_unicode(&bytes),
            Some(_) => return Err(ExpectEscaped),
            None => return Err(ExpectNoneEOF),
        };

        Ok(State { value, consumed })
    }

    fn parse_unicode(bytes: &[u8]) -> ParseResult<char> {
        let mut consumed = 2;
        if bytes.len() < 6 {
            return Err(ExpectNoneEOF);
        }
        let state = Jzon::parse_hex4(&bytes[2..6])?;

        consumed += 4;
        let mut uc = state.value;

        if 0xDC00 <= uc && uc <= 0xDFFF || uc == 0 {
            return Err(ExpectCodePoint);
        }

        if 0xD800 <= uc && uc <= 0xDBFF {
            if bytes.len() < 12 {
                return Err(ExpectNoneEOF);
            }
            if !(bytes[6] == b'\\' && bytes[7] == b'u') {
                return Err(ExpectCodePoint);
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
        if bytes.len() < 4 {
            return Err(ExpectNoneEOF);
        }
        if let Some(hex) = bytes[0..4]
            .iter()
            .enumerate()
            .fold(Some(0u32), |init, (i, ch)| {
                (*ch as char)
                    .to_digit(16)
                    .and_then(|d| init.and_then(|x| Some(x + d * (0x1000u32 >> (i as u32 * 4)))))
            })
        {
            Ok(State {
                value: hex,
                consumed: 4,
            })
        } else {
            Err(ExpectHexDigit)
        }
    }

    #[inline]
    fn parse_space(bytes: &[u8]) -> ParseResult<()> {
        let value = ();
        let mut consumed = 0;
        let mut it = bytes.iter();
        loop {
            match it.next() {
                Some(ch) => match *ch as char {
                    ' ' | '\t' | '\n' | '\r' => {
                        consumed += 1;
                        continue;
                    }
                    _ => break,
                },
                None => break,
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
        let jz = Jzon::parse_value(JSON.as_bytes()).unwrap();
        if let Jzon::Object(v) = jz.value {
            assert_eq!(5, v.len());
        } else {
            panic!();
        }
    }

    #[test]
    fn parse_object() {
        parse();
    }

    #[test]
    fn parse_array() {
        let jz = Jzon::parse_array(r#"[1, 2, 3, true, false, "string"]"#.as_bytes()).unwrap();
        if let Jzon::Array(v) = jz.value {
            assert_eq!(6, v.len());
        } else {
            panic!();
        }
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
        let jz = Jzon::parse_string(r#""a string literal","#.as_bytes());
        assert_eq!("a string literal", jz.unwrap().value);
    }

    #[test]
    fn parse_null() {
        let jz = Jzon::parse_null("null".as_bytes()).unwrap();
        if let Jzon::Null = jz.value {
            ()
        } else {
            panic!();
        }
    }

    #[test]
    fn parse_bool() {
        let jz = Jzon::parse_true("true".as_bytes()).unwrap();
        if let Jzon::Bool(v) = jz.value {
            assert!(v);
        } else {
            panic!();
        }
        let jz = Jzon::parse_false("false".as_bytes()).unwrap();
        if let Jzon::Bool(v) = jz.value {
            assert!(!v);
        } else {
            panic!();
        }
    }

    #[test]
    fn parse_pair() {
        let jz = Jzon::parse_pair(r#""a string literal": 10,"#.as_bytes());
        let pair = jz.unwrap().value;
        assert_eq!("a string literal", pair.0);
        assert_eq!(10, pair.1);
    }

    #[test]
    fn parse_string_literal() {
        let jz = Jzon::parse_string_literal(r#""a string literal","#.as_bytes());
        assert_eq!("a string literal", jz.unwrap().value);
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

    #[test]
    fn fmt() {
        let jz = Jzon::parse(JSON.as_bytes()).unwrap();
        print!("value is {}", jz.value);
    }
}
