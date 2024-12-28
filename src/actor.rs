// An actor is an orbital body

use crate::vector3::Vector3;

use std::fmt;

// Defines an actor on our celestial stage
pub struct Actor {
    pub name: String,

    pub mass: f64,
    pub radius: f64,

    pub pos: Vector3,
    pub vel: Vector3,
    pub acc: Vector3,
    pub facc: Vector3, // Fake acceleration, used to show acceleration despite resets
}

impl Actor {
    pub fn new(name: &str, mass: f64, radius: f64) -> Self {
        Self {
            name: name.to_string(),
            mass,
            radius,
            pos: Vector3::new(0.0, 0.0, 0.0),
            vel: Vector3::new(0.0, 0.0, 0.0),
            acc: Vector3::new(0.0, 0.0, 0.0),
            facc: Vector3::new(0.0, 0.0, 0.0),
        }
    }

    // Performs t seconds worth of simulation
    pub fn step(&mut self, s: f64) {

        // Moves the actor on the stage
        self.vel += self.acc * s;
        self.pos += self.vel * s;

        // Stores acceleration for printout
        self.facc = self.acc;

        // Resets acceleration for the next step
        self.acc = Vector3::new(0.0, 0.0, 0.0);
    }
}


impl fmt::Display for Actor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, 
            "{}:\n\tMass:\t\t{:.2e} kg\n\tRadius:\t\t{:.2e} m\n\tPosition:\t{} m\n\tVelocity:\t{} m/s\n\tAcceleration:\t{} m/s^2",
            self.name,
            self.mass,
            self.radius,
            self.pos,
            self.vel,
            self.facc
        )
    }
}