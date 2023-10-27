fn main() {
    // ? Vectors
    // Only one data type allowed in vector
    // Vectors are resigsable elements allocated on a head

    let mut nums = vec![1, 2, 3];

    nums.push(4); // add to Vectors

    println!("{:#?}", nums); // <= pretty debug print
    println!("{:?}", nums); // <= default debug print

    nums.pop(); // remove last element from vec

    println!("{:?}", nums);

    let mut vec = Vec::new(); // vec!
    vec.push("String");
    vec.push("Test");
    // vec.push(42);

    println!("{:?}", vec);

    vec.reverse();

    println!("{:?}", vec);

    // ? Specify veec size
    let mut vect = Vec::<i32>::with_capacity(2);

    println!("{}", vect.capacity());

    // ? Creat vector using iterator

    let v: Vec<i32> = (0..5).collect();

    println!("{:?}", v)
}
