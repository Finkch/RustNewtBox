// Another iteration of my orbital simulator.
// This time its in Rust!
// It's been so long since I used a C-like language...

mod planet;
mod vector3;

use planet::Planet;

fn main() {
    println!("Hello, world!");


    let actor = Planet::new("Terra", 1e4, 1e4);

    println!("{}", actor);
}
