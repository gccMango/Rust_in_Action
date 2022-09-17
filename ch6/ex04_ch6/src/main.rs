fn main() {
    let ptr = 42 as *const Vec<String>;

    unsafe {
        let new_addr = ptr.offset(1);

        println!("ptr value: {:p} --> offseted new_addr: {:p}",ptr,new_addr);
    }
}
