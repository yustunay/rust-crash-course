// Enums are types which have a few definite values

enum Movement {
    //Variants
    Up,
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement) {
    //Perform action depending on info
    match m { //match like switch
        Movement::Up => println!("avatar moving up"),
        Movement::Down => println!("avatar moving down"),
        Movement::Left => println!("avatar moving left"),
        Movement::Right => println!("avatar moving right")
    }
}


pub fn run() {
    let avatar1 = Movement::Left;
    let avatar2 = Movement::Up;
    let avatar3 = Movement::Right;
    let avatar4 = Movement::Down;

    move_avatar(avatar1);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar4);
}
