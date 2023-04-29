fn replace_with_84(s: &mut Box<i32>) {
    *s = Box::new(84);
    /*
    // this is not okay, as *s would be empty:
    // let was = *s;
    // but this is:
    let was = std::mem::take(s);
    // so is this:
    *s = was;
    // we can exchange values behind &mut:
    let mut r = Box::new(84);
    std::mem::swap(s, &mut r);
    assert_ne!(*r, 84);
    */
}
fn main() {
    let mut s = Box::new(42);
    println!("main> s: {}", s);
    replace_with_84(&mut s);
    println!("(replace_with_84) main> s: {}", s);
}
