#[repr(C)]
struct StructReprC {
    a: bool,
    b: f64,
    c: u8,
    d: u32,
    e: u16,
}

#[repr(C)]
struct TupleReprC(bool, f64, u8, u32, u16);

// #[repr(Rust)]
struct StructReprRust {
    a: bool,
    b: f64,
    c: u8,
    d: u32,
    e: u16,
}

// #[repr(Rust)]
struct TupleReprRust(bool, f64, u8, u32, u16);

#[repr(packed)]
struct StructReprPacked {
    a: bool,
    b: f64,
    c: u8,
    d: u32,
    e: u16,
    // f: u8,
}

#[repr(packed)]
struct TupleReprPacked(bool, f64, u8, u32, u16 /*, u8*/);

#[repr(C, align(1024))]
struct ReprCAlign1024(u16);

#[repr(transparent)]
struct Bigu16(u16);

fn main() {
    println!("The layout of every struct (be it a struct or a tuple): {{");
    println!("\tbool,");
    println!("\tf64,");
    println!("\tu8,");
    println!("\tu32,");
    println!("\tu16,");
    println!("}}\n\n");

    let a = StructReprC {
        a: true,
        b: 0.0f64,
        c: 255,
        d: 255,
        e: 255,
    };
    println!("size_of_val(&StructReprC): {}", std::mem::size_of_val(&a));

    let a = TupleReprC(true, 0.0f64, 255, 255, 255);
    println!("size_of_val(&TupleReprC): {}", std::mem::size_of_val(&a));

    let b = StructReprRust {
        a: true,
        b: 0.0f64,
        c: 255,
        d: 255,
        e: 255,
    };
    println!(
        "size_of_val(&StructReprRust): {}",
        std::mem::size_of_val(&b)
    );

    let b = TupleReprRust(true, 0.0f64, 255, 255, 255);
    println!("size_of_val(&TupleReprRust): {}", std::mem::size_of_val(&b));

    let c = StructReprPacked {
        a: true,
        b: 0.0f64,
        c: 255,
        d: 255,
        e: 255,
        // f: 255,
    };
    println!(
        "size_of_val(&StructReprPacked): {}",
        std::mem::size_of_val(&c)
    );

    let c = TupleReprPacked(true, 0.0f64, 255, 255, 255 /*, 255*/);
    println!(
        "size_of_val(&TupleReprPacked): {}",
        std::mem::size_of_val(&c)
    );

    let d = ReprCAlign1024(22);
    println!(
        "size_of_val(&ReprCAlign1024): {}",
        std::mem::size_of_val(&d)
    );

    let vec_of_align = [ReprCAlign1024(10), ReprCAlign1024(20)];
    println!(
        "size_of_val(vec_of_align  &ReprCAlign1024): {}",
        std::mem::size_of_val(&vec_of_align)
    );

    let e = Bigu16(30);
    println!(
        "size_of_val(transparent  &Bigu16): {}",
        std::mem::size_of_val(&e)
    );
}
