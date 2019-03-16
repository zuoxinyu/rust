#[allow(dead_code)]
mod jzon;
#[derive(Debug)]
struct S {
    i : i32
}
fn main() {
    let jz = jzon::Jzon::Bool(true);
    println!("jzon {:?}", jz);
}

fn lifetime<'a>(i : &'a i32, j : &i32) -> &'a i32 {
    i
}
