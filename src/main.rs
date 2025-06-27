use std::thread::sleep;

#[derive(Debug)]
struct Particle {
    id: i32,
    energy: i32,  // Joules
    mass: i32  // Atomic Mass
}

impl Particle {
    fn new(id: i32, energy: i32, mass: i32) -> Particle {
        Particle {
            id,
            energy,
            mass
        }
    }
}

#[derive(Eq, Hash, PartialEq, Debug)]
struct Coordinate {
    x: i32,
    y: i32,
    z: i32,
    particle_id: i32
}

impl Coordinate {
    fn new(x: i32, y:i32, z:i32, particle_id: i32) -> Coordinate {
        Coordinate {
            x,
            y,
            z,
            particle_id
        }
    }
}

fn display_all_particle_info(particles: &Vec<Particle>, coordinates: &Vec<Coordinate>) {
    for i in particles {
        let coordinate = &coordinates[i.id as usize];
        println!("{i:?}\n{coordinate:?}");
    }
}

fn tick(mut particles: Vec<Particle>, mut coordinates: Vec<Coordinate>) {
    for particle in &mut particles {
        coordinates[particle.id as usize].x += 1;
    }
    
    display_all_particle_info(&particles, &coordinates);
    let sleep_time = core::time::Duration::new(1, 0);
    sleep(sleep_time);
    tick(particles, coordinates);
}

fn main() {
    // let mut container: Vec<Particle> = Vec::new();
    let mut particles: Vec<Particle> = Vec::new();
    let mut coordinates: Vec<Coordinate> = Vec::new();
    let mut partical_id = 0;
    for x in 0..10 {
        for y in 0..10 {
            for z in 0..10 {
                if (x ^ y ^ z) != 0 && !(x > 0 && y > 0 && z > 0) {  // todo don't think this is right. Re-think
                    let coordinate = Coordinate::new(x, y, z, partical_id);
                    particles.push(Particle::new(partical_id, 37982, 1));
                    coordinates.push(coordinate);
                    partical_id += 1;
                }
            }
        }
    }
    for i in 0..4 {
        let coordinate = Coordinate::new(i, 0, 0, i);
        particles.push(Particle::new(i, 37982, 1));
        coordinates.push(coordinate);
    }
    
    tick(particles, coordinates);
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
