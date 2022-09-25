/*
Primitive str = Immutable fixed-length string somewhere in memory
String = Growable, heap-allocated data structure - Use when you need to modify or own string data
*/

pub fn run() {
    let hello_str: &str = "Hello";
    let mut hello_string: String = String::from("Hello");

    // Get length
    println!("Length: {}", hello_str.len());
    println!("Length: {}", hello_string.len());

    // Push a char
    hello_string.push(' ');
    hello_string.push_str("World");

    println!("{}", hello_string);

    // Capacity in bytes
    println!("Capacity hello_string: {}", hello_string.capacity());

    // Check if is empty
    println!("Is Empty: {}", hello_string.is_empty());

    // Contains
    println!("Contains 'World' {}", hello_string.contains("World"));

    // Replace
    println!("Replace: {}", hello_string.replace("World", "There"));

    println!("{}", hello_string);

    // Loop through string by whitespace
    for word in hello_string.split_whitespace() {
        println!("{}", word);
    }

    // Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    // Assertion testing
    assert_eq!(2, s.len());
    assert_eq!(10, s.capacity());

    println!("{}", s);
}
