fn main() {
    let mut x;

    // assert_eq!(x, 42);
    x = 42;

    let y = &x;

    x = 43;

    // assert_eq!(*y, 42);
}
