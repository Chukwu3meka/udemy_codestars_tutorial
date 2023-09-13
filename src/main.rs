fn main() {
    // * 4 Primary Scalar Types
    // Boolean, integers, floating point and characters

    // ? Integers (8 bit, 16 bit, 32 bit, 64 bit and 128 bit) which can either be signed/unsigned (i8, i16, i32, i64, i128) or (u8, u16, u32, u64, u128)

    // let x: i8 = -10; // <= Signed

    // * To hide errors for unused variables prefix them with _

    let _x: i8 = -10; // <= Signed
    let _y: u8 = 10; // <= Unsigned

    let decimal = 02_5555_33;
    let hex: i32 = 0xff;
    let octal = 0o377;
    let binary = 0b1111_1111;

    let byte = b'A';

    println!(
        "decimal: {}, hex: {}, octal: {}, binary: {}, byte{}",
        decimal,
        hex,
        octal,
        binary,
        byte
    );

    let i32: f64 = 2.5;

    println!("{}", i32);

    // ? Boolean

    let t = true;
    let f: bool = false;

    println!("t: {}, f: {}", t, f);

    // ? Arithmetic operations (+ - * / %)

    let a = 10;
    let b = 4;

    let remainder = a % b;

    println!("remainder: {}", remainder)
}
