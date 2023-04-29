// Listing 1-8: Lifetimes do not need to be contiguous.

fn rand() -> f32 {
    0.42
}

fn main() {
    let mut x = Box::new(42);
    let r = &x;
    if rand() > 0.5 {
        *x = 84;
        println!("*x: {}", *x);
    } else {
        println!("{}", r);
    }

    // println!("main> r: {}", r);
}
