/**
 * This file demonstrates how to use variables
 * 
 * Variables hold primitive data or references to data
 * Varaibles are immutable by default   |   By default you cannot reassign them
 * Rust is a block-scoped language      |   A variable is not accessable outside of the brackets it's in
 */

pub fn run(){
    let name = "Tom";
    let mut age = 37;

    println!("My name is {}, and I am {} years old", name, age);

    // Happy birthday Tom!
    age = 38;

    println!("My name is {}, and I am now {} years old", name, age);


    // Define constant
    const ID: i32 = 001; // Rust will ignore the 0s if a value starts with it.
    println!("ID: {}", ID);

    // Assign multiple variables
    let (my_name, my_age) = ("Tom", 37);

    println!("{} is {}", my_name, my_age);
}