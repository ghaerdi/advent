#[allow(dead_code)]
mod challenges;

use challenges::*;

fn main() {
    let room = vec![
        vec![" ", " ", " "],
        vec![" ", " ", "m"],
        vec![" ", " ", "f"],
    ];
    can_mouse_eat("up", &room);
    can_mouse_eat("down", &room);
    can_mouse_eat("left", &room);
    can_mouse_eat("right", &room);
    println!("Midudev's Advent 2021 but in rust");
}
