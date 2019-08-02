extern crate jzon;
use jzon::jzon::Jzon;
use std::fs;
use std::io;

const JSON: &'static str = r#"
{
    "string": "a string literal",
    "integer": -142,
    "double": -0.34E+12,
    "boolean": true,
    "null": null,
    "array": ["a", "b", "c", "d"],
    "object": {
        "nest-key": "nest value",
        "nest-int": 1.12
    }
}"#;

fn main() -> io::Result<()> {
    let jz = Jzon::parse(JSON.as_bytes()).unwrap();
    println!("{:?}", jz.value);

    let checker_dir = fs::read_dir("data/roundtrip")?;

    for entry in checker_dir {
        if let Ok(e) = entry {
            let name = e.file_name();
            let name = name.to_str().unwrap();
            print!("{}: ", name); 
            let content = fs::read_to_string(e.path()).unwrap();
            let parsed = Jzon::parse(content.as_bytes());
            print!("{}\n", if parsed.is_ok() {"pass"} else {"FAIL"});
        }
    }

    Ok(())
}

