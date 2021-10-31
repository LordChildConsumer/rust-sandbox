/**
 * This file demonstrates how have command-line arguments for your program.
 */

use std::env;

pub fn run() {
    let args: Vec<String> = env::args().collect();
    let command = args[1].clone();
    let name = "Tom";

    if command == "hello" {
        println!("Hello there, {}!", name);
    } else if command == "status" {
        println!("uhh I'm alive I guess");
    }
}