fn main() {
    // let x = 5;

    // println!("The value of x is {}", x);

    // x = 6;

    // println!("The value of x is {}", x);

    // ?  Mutability using let and mut
    let mut x = 5;

    println!("The value of x is {}", x);

    x = 6;

    println!("The value of x is {}", x);

    // ? CONST
    const SECONDS: i8 = 60; // <= data type must be defined

    println!("The value of seconds is {}", SECONDS)
}
