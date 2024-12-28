// A place to store various solar systems

use crate::actor::Actor;

const AU: f64 = 1.496e11;
const G:  f64 = 6.67430e-11;

const MS: f64 = 1.989e30;
const RS: f64 = 696340e3;

const ME: f64 = 5.972e24;
const RE: f64 = 6378e3;

//      Actor parameters
// pub name: String,
// pub mass: f64,
// pub radius: f64,

fn vis_viva(parent: &Actor, r: f64) -> f64 {
    (G * parent.mass / r).sqrt()
}

// Simple sun-earth system
pub fn sol_terra() -> [Actor; 2] {
    let mut actors = [
        Actor::new(
            "Sol",
            MS,
            RS,
        ),
        Actor::new(
            "Terra",
            ME,
            RE,
        ),
    ];

    actors[1].pos.x = AU;
    actors[1].vel.y = vis_viva(&actors[0], AU);

    return actors;
}