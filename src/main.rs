// Another iteration of my orbital simulator.
// This time its in Rust!
// It's been so long since I used a C-like language...

mod actor;
mod vector3;

use actor::Actor;

fn main() {

    let actor = Actor::new("Terra", 1e4, 1e4);

    println!("{}", actor);
}


// Applies gravity between a pair of actors
fn gravity(a: &mut Actor, b: &mut Actor) {

    let g = 6.67430e-11;

    let r = a.pos - b.pos;

    let f = g / r.mag().powi(3) * r;

    a.acc = a.acc + f * b.mass;
    b.acc = b.acc - f * a.mass;
}