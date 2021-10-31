/**
 * Reference Pointers point to a resource in memory
 * 
 * Basically, if we have a primitive variable, we can create a variable that points to another variable.
 * 
 * With non-primitives, if you assign another variable to a piece of data, the first variable will no longer hold that value.
 * You'll need to use a reference (&) to point to the resource.
 */

pub fn run(){
    // Primitive Array
    let arr1= [1, 2, 3];
    let arr2 = arr1;

    println!("Arr Values: {:?}", (arr1, arr2));

    // Non-Primitives
    let vec1 = vec![1, 2, 3];
    let vec2 = &vec1;

    println!("Vec Values: {:?}", (&vec1, vec2));
}