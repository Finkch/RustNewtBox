// Another iteration of my orbital simulator.
// This time its in Rust!
// It's been so long since I used a C-like language...

mod actor;
mod vector3;
mod solarsystems;

use actor::Actor;
use solarsystems::sol_terra;

fn main() {

    let mut actors = sol_terra();

    println!("{}", actors[0]);
    println!("{}", actors[1]);
}


// Applies gravity between a pair of actors
fn gravity(a: &mut Actor, b: &mut Actor) {

    let g = 6.67430e-11;

    let r = &a.pos - &b.pos;

    let d = &r.mag();

    let f = r * (g / d.powi(3));

    a.acc = &a.acc + &(f * b.mass);
    b.acc = &b.acc - &(f * a.mass);
}