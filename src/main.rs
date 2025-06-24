use std::thread::sleep;

#[derive(Debug)]
struct Particle {
    coordinates: Coordinates,
    energy: i32,  // Joules
    mass: i32  // Atomic Mass
}

impl Particle {
    fn new(coordinates: Coordinates, energy: i32, mass: i32) -> Particle {
        Particle {
            coordinates,
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

fn display_all_particle_info(particles: &Vec<Particle>) {
    for i in particles {
        println!("Particles: {i:?}")
    }
}

fn tick(mut particles: Vec<Particle>) {
    for particle in &mut particles {
        particle.coordinates.x += 1;
    }
    
    display_all_particle_info(&particles);
    let sleep_time = core::time::Duration::new(1, 0);
    sleep(sleep_time);
    tick(particles);
}

fn main() {
    // let mut container: Vec<Particle> = Vec::new();
    let mut particles: Vec<Particle> = Vec::new();
    for i in 0..4 {
        let coordinates = Coordinates::new(i, 0, 0);
        particles.push(Particle::new(coordinates, 37982, 1));
    }
    display_all_particle_info(&particles);
    
    tick(particles);
}
