/**
 * Vectors are resizeable vectors allocated in the heap.
 * 
 * Vectors work basically just like arrays except you define them different
 * and obviously the size of them can change.
 */

use std::mem;

pub fn run() {
    
    // Define them using this template
    // let name: Vec<type> = vec![data];
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    // To print out the whole vector use the debug placeholder (:?)
    println!("Whole Vector: {:?}", numbers);

    // Get single value from vector
    println!("Single Value: {}", numbers[0]);

    // Re-assign a value
    numbers[2] = 20;
    println!("Changed Vector: {}", numbers[2]);

    // Add on to vector
    numbers.push(6);

    // Detach last value
    numbers.pop();

    // Get vector length
    println!("Vector Length: {}", numbers.len());

    // Get the amount of memory the vector takes up
    println!("Vector occupies {} bytes", mem::size_of_val(&numbers));

    // Get slice from vector
    // You can get a range of values by using "value..value"
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    // Loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x);
    }

    // Loop through and mutate values
    for x in numbers.iter_mut() {
        // Multiple each value by 2
        *x *= 2;
    }

    println!("Doubled Number: {:?}", numbers);
}