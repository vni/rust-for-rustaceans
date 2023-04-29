// Rust assumes that shared references are immutable.

fn cache(input: &i32, sum: &mut i32) {
    *sum = *input + *input;
    assert_eq!(*sum, 2 * *input);
}

fn main() {
    let a = 10;
    let mut sum = 0;

    cache(&a, &mut sum);
}
