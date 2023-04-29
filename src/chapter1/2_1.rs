fn main() {
    let x: usize;

    // println!("&x: {:p}", &x); // compile error: possibly uninitialized

    x = 10;
    println!("&x: {:p}, x: {}", &x, x);
    let old_x_addr = &x;

    let x: usize;
    x = 11;
    println!("&x: {:p}, x: {}", &x, x);
    let new_x_addr = &x;

    assert_ne!(old_x_addr, new_x_addr);
}
