use std::mem;

pub fn run() {
    let numbers: [i32; 5] = [1, 2, 3, 4, 5];
    let mut numbers_mut: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);
    println!("{:?}", numbers_mut);

    // Re-assign value
    numbers_mut[2] = 20;

    println!("{:?}", numbers_mut);

    // Get single value
    println!("Single value: {}", numbers[0]);

    // Get array length
    println!("Array length: {}", numbers.len());

    // Arrays are stack allocated
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("{:?}", slice);
}
