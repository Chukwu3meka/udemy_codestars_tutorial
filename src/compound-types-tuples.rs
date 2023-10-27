fn main() {
    // ? Tuple
    // * A tuple is a way of grouping a number of values with vaiety of types to one
    // * Tuple has a fixed length, so once defined cannot grow or shrink in size

    let tup = (500, 'h', true);

    println!("{}", tup.0);

    let (x, y, z) = tup;

    println!("x: {}, y: {}, z: {},", x, y, z);
}
