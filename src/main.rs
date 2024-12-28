// Another iteration of my orbital simulator.
// This time its in Rust!
// It's been so long since I used a C-like language...

mod actor;
mod vector3;
mod solarsystems;

use actor::Actor;
use solarsystems::sol_terra;

fn main() {

    // Gets a set of actors
    let mut actors = sol_terra();

    // Tracks time (t) and step size (s)
    let mut t: f64 = 0.0;
    let s: f64 = 1.0;

    // Simulation loop
    while t < 1e8 {

        // Prints state
        println!("{:.2e}", t);
        printout(&actors);

        // Applies gravity
        gravity_pairs(&mut actors);

        // Moves actors on the stage
        for actor in actors.iter_mut() {
            actor.step(s);
        }

        // Updates current time
        t += s;

    }
}


// Applies gravity between a pair of actors
fn gravity_pairs(actors: &mut [Actor]) {

    // Consideres each pair exactly once
    for i in 0..(actors.len() - 1) {

        // Splits into left and right to prevent mutability overlap
        let (l, r) = actors.split_at_mut(i + 1);

        // Considers each pair to actor_i
        for j in 0..(r.len()) {
            gravity(&mut l[i], &mut r[j]);
        }
    }
}

fn gravity(a: &mut Actor, b: &mut Actor) {

    // Gravitational constant
    let g = 6.67430e-11;

    // Vector distance
    let r = &a.pos - &b.pos;

    // Scalar distance
    let d = &r.mag();

    // Finds the vector force before accounting for massess
    let f = r * (g / d.powi(3));

    // Applies forces to actors
    a.acc = &a.acc + &(f * b.mass);
    b.acc = &b.acc - &(f * a.mass);
}

// Prints each actors state in each step
fn printout(actors: &[Actor]) {
    for actor in actors.iter() {
        println!("{}", actor);
    }
}