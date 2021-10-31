/**
 * Tuples group together values of different types.
 * The most you can have in a tuple is 12 elements
 */

pub fn run(){
    // To declare a tuple, you need to put the types of data it stores in parantheses
    let person: (&str, &str, i8) = ("Tom", "Chicago", 37);

    println!("{} is from {} and is {} years old", person.0, person.1, person.2);
}