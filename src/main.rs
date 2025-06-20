use std::collections::HashMap;

#[derive(Debug)]
struct Particle {
    energy: i32,  // Joules
    mass: i32  // Atomic Mass
}

impl Particle {
    fn new(energy: i32, mass: i32) -> Particle {
        Particle {
            energy,
            mass
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct Coordinates {
    x: i32,
    y: i32,
    z: i32
}

impl Coordinates {
    fn new(x: i32, y:i32, z:i32) -> Coordinates {
        Coordinates {
            x,
            y,
            z
        }
    }
}

// struct Container {
//     x_range: Range(i32),
//     y_range: Range(i32),
//     z_range: Range(i32)
// }

// impl Container {
//     fn new(x: i32, y: i32, z: i32) -> Container {
//         x_vec = Vec::new();
//         y_vec = Vec::new();
//         z_vec = Vec::new();
//         Container {
//             x,
//             y,
//             z
//         }
//     }
// }

fn main() {
    // let mut container: Vec<Particle> = Vec::new();
    let mut particles: HashMap<Coordinates, Particle> = HashMap::new();
    for i in 0..4 {
        particles.insert(
            Coordinates::new(i, 0, 0),
            Particle::new(37982, 1)
        );
    }
    for i in &particles {
        println!("Particles: {i:?}")
    }
}
