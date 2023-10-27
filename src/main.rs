fn main() {
    // ? String
    // Strings are similar to vectors just that they are always UTF-8
    // Stored on heap

    let name = String::from("Chukwuemeka");
    let course = "Rust Tutorial".to_string();

    println!("{} is studying {}", name, course);

    // replace string
    let new_name = name.replace("Chukwuemeka", "Vanilla");
    println!("Updated name from {} to {}", name, new_name)

    // String Slice or str (&str)
}
