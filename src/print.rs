/**
 * This file is to demonstrate some of the different ways to use println!
 */

pub fn run(){
    // Print to console
    println!("Hello, World! (sent from my print file)");

    // Print the number 1
    println!("Number: {}", 1);

    // Positional parameters
    println!("{0} am {1} and {0} want to {2}", "I", "epic", "die");

    // Named arguments
    println!("{name} likes to play {game}", name = "John", game = "Minecraft");

    // Placeholder traits
    println!("Binary: {:b} Hex: {:x}, Octal: {:o}", 10, 10, 10);

    // Placeholder for debug trait
    println!("{:?}", (12, true, "debug"));

    // Basic math
    println!("10 + 10 = {}", 10 + 10);
}