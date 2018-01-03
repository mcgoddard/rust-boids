extern crate fungine;
extern crate cgmath;
extern crate stopwatch;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate rand;

pub mod boids;

use std::sync::Arc;
use std::mem::transmute;
use fungine::fungine::{Fungine, GameObject};
use boids::boids::{Boid, BoidColourKind};
use cgmath::{ Vector3, InnerSpace };
use rand::Rng;

#[allow(dead_code)]
#[no_mangle]
pub extern fn newSim() -> *mut Fungine {
    let mut rng = rand::thread_rng();
    let mut initial_state = Vec::new();
    for i in 0i32..1000i32 {
        let boid_colour = match i % 6 {
            0 => BoidColourKind::Green,
            1 => BoidColourKind::Blue,
            2 => BoidColourKind::Red,
            3 => BoidColourKind::Orange,
            4 => BoidColourKind::Purple,
            _ => BoidColourKind::Yellow,
        };
        let x = (((rng.gen::<u32>() % 300) as f32) / 10.0) - 15.0;
        let y = (((rng.gen::<u32>() % 300) as f32) / 10.0) - 15.0;
        let z = (((rng.gen::<u32>() % 300) as f32) / 10.0) - 15.0;
        let x_dir = rng.gen::<f32>() - 0.5f32;
        let y_dr = rng.gen::<f32>() - 0.5f32;
        let z_dir = rng.gen::<f32>() - 0.5f32;
        let initial_object = Boid {
            position: Vector3::new(x, y, z),
            direction: Vector3::new(x_dir, y_dr, z_dir).normalize(),
            colour: boid_colour,
            id: i
        };
        let initial_object = Box::new(initial_object) as Box<GameObject>;
        let initial_object = Arc::new(initial_object);
        initial_state.push(initial_object);
    }
    let engine = Fungine::new(Arc::new(initial_state), None);

    let sim_ref = unsafe { transmute(Box::new(engine)) };
    sim_ref
}

#[allow(dead_code)]
#[no_mangle]
pub extern fn step(sim_ptr: *mut Fungine) -> usize {
    let sim = unsafe { &mut *sim_ptr };
    let _ = sim.run_steps_cont(1);
    sim.current_state.len()
}

#[allow(dead_code)]
#[no_mangle]
pub extern fn getBoid(sim_ptr: *mut Fungine, index: usize) -> boids::boids::Boid {
    let sim = unsafe { &mut *sim_ptr };
    let game_object = &sim.current_state[index];
    if let Some(boid) = game_object.downcast_ref::<Boid>() {
        *boid
    }
    else {
        Boid {
            position: Vector3::new(0f32, 0f32, 0f32),
            direction: Vector3::new(0f32, 0f32, 0f32),
            colour: BoidColourKind::Green,
            id: 0
        }
    }
}

#[no_mangle]
pub extern fn destroySim(sim_ptr: *mut Fungine) {
    let _sim: Box<Fungine> = unsafe{ transmute(sim_ptr) };
    // Drop to free
}