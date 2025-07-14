use std::thread::sleep;

// This module integrations this application with the graphics engine.

use std::f32::consts::TAU;

use graphics::{
    Camera, ControlScheme, DeviceEvent, EngineUpdates, Entity, InputSettings, LightType, Lighting,
    Mesh, PointLight, Scene, UiLayout, UiSettings, GraphicsSettings, WindowEvent, Gaussian,
    RIGHT_VEC, UP_VEC
};
use egui::{Context, Slider, TopBottomPanel};

use lin_alg::f32::{Quaternion, Vec3};

type Color = (f32, f32, f32);

const WINDOW_TITLE: &str = "Causal gravity model";
const WINDOW_SIZE_X: f32 = 1_600.;
const WINDOW_SIZE_Y: f32 = 1_000.;
const BACKGROUND_COLOR: Color = (0.5, 0.5, 0.5);

const RENDER_DIST: f32 = 200.;

pub const BODY_SPHERE_SIZE: f32 = 0.1;
pub const BODY_SHINYNESS: f32 = 2.;
pub const BODY_COLOR: Color = (0., 1.0, 0.5);

/// This runs whenever an event (e.g. keyboard, mouse etc) occurs, and provides information on the event.
fn event_dev_handler(
    state_: &mut State,
    event: DeviceEvent,
    scene: &mut Scene,
    _engine_inputs: bool,
    _dt: f32,
) -> EngineUpdates {
    match event {
        DeviceEvent::MouseMotion { delta } => {
            // let (dx, dy) = delta; // Relative cursor movement
            // println!("Relative cursor position change: dx: {}, dy: {}", dx, dy);
        }
        DeviceEvent::Button { button, state } => {}
        _ => (),
    }
    EngineUpdates::default()
}

fn event_win_handler(
    state: &mut State,
    event: WindowEvent,
    _scene: &mut Scene,
    _dt: f32,
) -> EngineUpdates {
    match event {
        WindowEvent::CursorMoved {device_id, position} => {}
        _ => (),
    }
    EngineUpdates::default()
}

/// This runs each frame.
fn render_handler(_state: &mut State, _scene: &mut Scene, _dt: f32) -> EngineUpdates {
    EngineUpdates::default()
}

const SLIDER_WIDTH: f32 = 460.;
const SLIDER_WIDTH_ORIENTATION: f32 = 100.;

pub const ROW_SPACING: f32 = 22.;
pub const COL_SPACING: f32 = 30.;

/// This function draws the (immediate-mode) GUI.
/// [UI items](https://docs.rs/egui/latest/egui/struct.Ui.html#method.heading)
fn ui_handler(state: &mut State, ctx: &Context, mut scene: &mut Scene) -> EngineUpdates {
    let engine_updates = EngineUpdates::default();

    TopBottomPanel::top("0").show(ctx, |ui| {
        ui.spacing_mut().slider_width = SLIDER_WIDTH;

        ui.horizontal(|ui| {
            ui.add_space(COL_SPACING);
            ui.label("Time:");
        });

        ui.add_space(ROW_SPACING / 2.);
    });

    engine_updates
}


fn draw_entities(entities: &mut Vec<Entity>, coordinates: &Vec<Coordinate>) {
    *entities = Vec::new();

    let mut containing_box = Entity::new(
        1, // Index of the mesh.
        Vec3::new(0., 0., 0.),
        Quaternion::default(),
        1.,
        (100.0, 100.0, 100.0),
        BODY_SHINYNESS,
    );
    containing_box.opacity = 0.1;
    entities.push(containing_box);

    let mut index = 0;
    for coordinate in coordinates {
        // Alternate way to construct: use its `Default` impl, overriding fields as required.
        entities.push(
            // manually set the `scale_partial` field with a `Vec3` if using non-uniform scaling. 
            Entity::new(
                0, // Index of the mesh.
                Vec3::new(
                    (coordinate.x as f32) * 5.,
                    (coordinate.y as f32) * 5.,
                    (coordinate.z as f32) * 5.
                ),
                Quaternion::default(),
                1.,
                (0.0, 0.0, 100.0),
                BODY_SHINYNESS,
            )
        );
        index += 1;
    }
}

/// Entry point to our render and event loop.
fn render(state: State, coordinates: &Vec<Coordinate>) {
    let mut scene = Scene {
        meshes: vec![
            Mesh::new_sphere(1., 3),
            Mesh::new_box(100., 100., 100.)
        ],
        entities: Vec::new(), // updated below.
        gaussians: vec![],
        camera: Camera {
            fov_y: TAU / 8.,
            position: Vec3::new(0., 10., -150.),
            far: RENDER_DIST,
            orientation: Quaternion::from_axis_angle(RIGHT_VEC, TAU / 16.),
            ..Default::default()
        },
        lighting: Lighting {
            ambient_color: [-1., 1., 1., 0.5],
            ambient_intensity: 0.03,
            point_lights: vec![
                // Light from above and to a side.
                PointLight {
                    type_: LightType::Omnidirectional,
                    position: Vec3::new(30., 50., 30.),
                    diffuse_color: [0.3, 0.4, 0.5, 1.],
                    specular_color: [0.3, 0.4, 0.5, 1.],
                    diffuse_intensity: 8_000.,
                    specular_intensity: 30_000.,
                },
            ],
        },
        input_settings: InputSettings {
            control_scheme: ControlScheme::FreeCamera,
            ..Default::default()
        },
        background_color: BACKGROUND_COLOR,
        window_size: (WINDOW_SIZE_X, WINDOW_SIZE_Y),
        window_title: WINDOW_TITLE.to_owned(),
    };

    let ui_settings = UiSettings {
        layout: UiLayout::Top,
        icon_path: Some("./resources/icon.png".to_owned()),
    };

    // Initialize entities.
    // if !state.snapshots.is_empty() {
    //     draw_entities(
    //         &mut scene.entities,
    //         &state.snapshots[state.ui.snapshot_selected],
    //     )
    // }

    // Initialize entities.
    draw_entities(&mut scene.entities, coordinates);
    

    // This starts the main event loop; program intereactions from here on out will 
    // be handled by one of the `_handler` callbacks defined above.
    graphics::run(
        state,
        scene,
        ui_settings,
        GraphicsSettings::default(),
        render_handler,
        event_dev_handler,
        event_win_handler,
        ui_handler,
    );
}

struct State {} // Set this up however you'd like.

impl State {}

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

fn tick_entities(mut scene: &mut Scene) {
    for entity in &mut scene.entities {
        entity.position.x += 1.;
    }
}

fn main() {
    // let mut container: Vec<Particle> = Vec::new();
    let mut particles: Vec<Particle> = Vec::new();
    let mut coordinates: Vec<Coordinate> = Vec::new();
    let mut partical_id = 0;
    let max: i32 = 10;
    for x in 0..max {
        for y in 0..max {
            for z in 0..max {
                if x == 0 || x == (max-1) || y == 0 || y == (max-1) || z == 0 || z == (max-1) {
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
    
    let state: State = State{};
    render(state, &coordinates);
    
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
