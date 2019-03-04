#[allow(dead_code)]
mod jzon;
fn main() {
    let jz = jzon::Jzon::Bool(true);
    println!("jzon {:?}", jz);
}
