/**
 * This file demonstrates the different datatypes in rust and how they can be used.
 * 
 * 
 * Primitive Types--
 * Integers: u8, i8, u16, i16, u32, i32, u64, i64, u128, i128 (number of bits they take in memory)
 * Floats: f32, f64
 * Boolean (bool)
 * Characters (char)
 * Tuples
 * Arrays
 * 
 * Unsigned integers means there is no positive or negative sign, so you can only have positive numbers.
 * 
 * Characters are sort of like a string with only one character (hence the name.)
 * 
 * Tuples are basically lists
 * 
 * Arrays are a fixed length. Vectors are like growable arrays.
 * 
 * 
 * Rust is a statically typed language so it needs to know the type of all variables when compiled.
 * The compiler, however, can usually infer the datatype from the value and how it's used. 
 */

pub fn run(){
    // Default is "i32"
    let x = 1;

    // Default is "f64"
    let y = 2.5;

    // Add explicit type
    let z: i64 = 123123123123;

    // Find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    // Boolean
    let is_active: bool = true;

    // Get boolean from expression
    let is_greater: bool = 10 > 5;

    // Character (char)
    let a1: char = 'a';
    let face = '\u{1F600}';



    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}