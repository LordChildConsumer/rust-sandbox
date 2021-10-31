/**
 * Arrays are a fixed length and all the elements in them must be of the same type.
 * Use Vectors if you need them to be a dynamic length
 * 
 * If the array is not the size or datatypes aren't the ones specified, it will give an error
 * 
 * Arrays are allocated in the memory stack
 */

use std::mem;

pub fn run() {
    // To declare an array you must specify the datatype and length using this format
    // let array: [type; size] = [data];

    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    // To print out the whole array use the debug placeholder (:?)
    println!("Whole Array: {:?}", numbers);

    // Get single value from array
    println!("Single Value: {}", numbers[0]);

    // Re-assign a value
    numbers[2] = 20;

    println!("Changed Array: {}", numbers[2]);

    // Get array length
    println!("Array Length: {}", numbers.len());

    // Get the amount of memory the array takes up
    println!("Array occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice from array
    // You can get a range of values by using "value..value"
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);
}