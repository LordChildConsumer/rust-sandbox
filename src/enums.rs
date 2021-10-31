/**
 * Enums are types which have a few definite values.
 */


// Pretend this is some sort of game.
enum Movement {
    // Variants
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement) {
    // Perform action based on movement.
    match m {
        Movement::Up => println!("Avatar is moving up"),
        Movement::Down => println!("Avatar is moving down"),
        Movement::Left => println!("Avatar is moving left"),
        Movement::Right => println!("Avatar is moving right")
    }
}

pub fn run(){
    let avatar1 = Movement::Up;
    let avatar2 = Movement::Down;
    let avatar3 = Movement::Left;
    let avatar4 = Movement::Right;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}