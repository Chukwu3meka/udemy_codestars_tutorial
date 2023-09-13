fn main() {
    // ? Slices
    // A Slice is a region of an array oor vector that can be any length
    // It cannot be stored directly as varaibles or pass as function arguments

    let v: Vec<i32> = (0..5).collect();
    println!("{:?}", v);

    let sv: &[i32] = &v; // get all elements
    println!("{:?}", sv);

    let su: &[i32] = &v[2..4]; // get all elements
    println!("{:?}", su);
}
