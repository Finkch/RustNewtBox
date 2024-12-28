// Another iteration of my orbital simulator.
// This time its in Rust!
// It's been so long since I used a C-like language...

mod actor;
mod vector3;

use actor::Actor;

fn main() {
    println!("Hello, world!");


    let actor = Actor::new("Terra", 1e4, 1e4);

    println!("{}", actor);
}
