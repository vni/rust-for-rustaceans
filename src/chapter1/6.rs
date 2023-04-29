// Mutability applies only to the immediately referenced memory

fn main() {
    let x = 42;
    println!("x: {x}, &x: {:p}", &x);

    let mut y = &x; // y is of type &i32
    println!("y: {y} ({y:p}), &y: {:p}", &y);

    let z = &mut y; // z is of type &mut &i32
    println!("z: {z} ({z:p}), &z: {:p}, *z: {}", &z, *z);

    let b = 84;
    println!("b: {b}, &b: {:p}", &b);

    *z = &b;
    // println!("y: {y} ({y:p}), &y: {:p}", &y);
    println!("z: {z} ({z:p}), &z: {:p}, *z: {}", &z, *z);
}
