fn main() {
    let a = 42 ;
    let r = &a; //reference to a
    let b = a +*r;//dereference r

    println!("b : {}", b);
}
