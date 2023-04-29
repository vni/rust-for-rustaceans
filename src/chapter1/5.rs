// Rust assumes that mutable references are exclusive.

fn noalias(input: &i32, output: &mut i32) {
    if *input == 1 {
        *output = 2;
    }
    if *input != 1 {
        *output = 3;
    }
}

fn main() {
    let x = 1;
    let mut output = 50;

    println!("x: {x}, output: {output}");
    noalias(&x, &mut output);
    println!("x: {x}, output: {output}");
}
