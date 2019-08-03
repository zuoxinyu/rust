#![feature(try_trait)]
extern crate jzon;
use jzon::jzon::Jzon;
use std::fs;
use std::io;
use std::path::Path;
use std::time;

const UNITS: [&str; 6] = ["B", "K", "M", "G", "T", "P"];
const PASSED_MARK: &str = ":heavy_check_mark:";
const FAILED_MARK: &str = ":x:";

fn main() {
    print!(r#"## Jzon
A simple and ease-of-use JSON library in Rust.

## TODO";
- TODO: impl Display trait with more options
- TODO: impl Index trait with lifetime
- TODO: impl Iterator trait
- TODO: impl Deref trait
- TODO: impl From trait
- FIXME: float point number parsing precision

## Sample Results
Sample files from [JSON\_checker](http://www.json.org/JSON\_checker/).
P.S.: `fail01.json` is excluded as it is relaxed in RFC7159. `fail18.json` is excluded as depth of JSON is not specified.

### Roundtrip
"#);
    print_table_header();
    let _ = test_json_dir(&Path::new("data/roundtrip"));

    println!("\n### Corner Cases");
    print_table_header();
    let _ = test_json_dir(&Path::new("data/jsonchecker"));

    println!("\n### Big Files");
    print_table_header();
    test_json_file(&Path::new("data/canada.json"));
    test_json_file(&Path::new("data/twitter.json"));
    test_json_file(&Path::new("data/citm_catalog.json"));
}

// m a -> (a -> m b) -> m b
fn test_json_dir(dir: &Path) -> io::Result<()> {
    let mut entries: Vec<_> = dir.read_dir()?.filter_map(|e| e.ok()).collect();
    entries.sort_by_key(|file| file.path());
    for entry in entries {
        let path = entry.path();
        if let Some(ext) = path.extension() {
            if ext != "json" {
                continue;
            }
            let _ = test_json_file(&path);
        }
    }
    Ok(())
}

fn test_json_file(path: &Path) {
    let file = path.file_name().unwrap().to_str().unwrap();
    let content = fs::read_to_string(path).unwrap();
    let start = time::Instant::now();
    let parsed = Jzon::parse(content.as_bytes());
    let cost = start.elapsed();
    let size = content.len();
    let should_fail = file.starts_with("fail");
    let passed = if parsed.is_ok() && !should_fail || parsed.is_err() && should_fail {
        PASSED_MARK
    } else {
        FAILED_MARK
    };
    print_table_line(file, passed, &size_str(size), &format!("{:.3?}", cost));
}

fn size_str(len: usize) -> String {
    let mut size: f64 = len as f64;
    let mut e = 0;
    while size > 1024.0 {
        size /= 1024.0;
        e += 1;
    }

    format!("{:>4.1}{}", size, UNITS[e])
}

fn print_table_line(file: &str, pass: &str, size: &str, cost: &str) {
    let len = PASSED_MARK.len();
    let (file_len, pass_len, size_len, cost_len) = (19, len, 6, 9);
    println!(
        "| {:<file_len$} | {:^pass_len$} | {:>size_len$} | {:>cost_len$} |",
        file,
        pass,
        size,
        cost,
        file_len = file_len,
        pass_len = pass_len,
        size_len = size_len,
        cost_len = cost_len,
    );
}

fn print_table_header() {
    let len = PASSED_MARK.len();
    let (file_len, pass_len, size_len, cost_len) = (19, len, 6, 9);
    print_table_line("file", "passed", "size", "cost");
    print_table_line(
        &format!(":{}-", "-".repeat(file_len - 2)),
        &format!(":{}:", "-".repeat(pass_len - 2)),
        &format!("-{}:", "-".repeat(size_len - 2)),
        &format!("-{}:", "-".repeat(cost_len - 2)),
    );
}
