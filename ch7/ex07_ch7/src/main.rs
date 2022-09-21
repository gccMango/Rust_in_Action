// use std::mem;

fn basic_hash(key : &str) -> u32 {
    let first = key.chars()
                .next()
                .unwrap_or('\0');
    unsafe {
        std::mem::transmute::<char, u32>(first)
    }
}
fn main() {
    println!("Hash translate \"Hello world\" => {}", basic_hash("Hello world"));
    println!("Hash translate \"Tonga\" => {}", basic_hash("Tonga"));
    println!("Hash translate \"Tuvalu\" => {}", basic_hash("Tuvalu"));

}
