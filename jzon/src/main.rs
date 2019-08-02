extern crate jzon;
use jzon::jzon::Jzon;
use std::env;
use std::fs;
use std::time;

// TODO: Generate benchmark results into Readme.
fn main() {
    let args = env::args();
    if args.len() > 1 {
        let it = args.last();
        let text = it.unwrap();
        let result = Jzon::parse(text.as_bytes()).unwrap();
        println!("{:?}", result);
        return;
    }

    test_json_dir("data/roundtrip");
    test_json_dir("data/jsonchecker");
    test_json_file("data/canada.json");
    test_json_file("data/twitter.json");
    test_json_file("data/citm_catalog.json");
}

fn test_json_dir(dir_name: &str) {
    let dir = fs::read_dir(dir_name).unwrap();
    for e in dir {
        if let Ok(entry) = e {
            if let Some(ext) = entry.path().extension() {
                if ext == "json" {
                    test_json_file(entry.path().to_str().unwrap());
                }
            }
        }
    }
}

fn test_json_file(file: &str) {
    print!("{}: ", file);
    let content = fs::read_to_string(file).unwrap();
    let start = time::Instant::now();
    let parsed = Jzon::parse(content.as_bytes());
    let end = time::Instant::now();
    print!(
        "{}, cost: {:?}\n",
        if parsed.is_ok() { "pass" } else { "FAIL" },
        end - start
    );
}
