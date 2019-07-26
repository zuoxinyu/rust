extern crate jzon;
use jzon::jzon::Jzon;

const JSON: &'static str = r#"
{
"string": "a string literal",
    "integer": 1,
    "array": ["a", "b", "c", "d"],
    "object": {
        "nest-key": "nest value",
        "nest-int": 1
    }
}"#;


fn main() {
    let jz = Jzon::parse(JSON.as_bytes()).unwrap();
    println!("{:?}", jz.value);
}
