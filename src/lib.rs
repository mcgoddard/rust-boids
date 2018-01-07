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
use boids::{Boid, BoidColourKind};
use cgmath::{ Vector3, InnerSpace };
use rand::Rng;

#[repr(C)]
pub struct BoidObj {
    pub id: u64,
    pub boid: Boid
}

#[allow(dead_code)]
#[no_mangle]
pub extern fn newSim500() -> *mut Fungine {
    newSim(500)
}

#[allow(dead_code)]
#[no_mangle]
pub extern fn newSim(boid_num: usize) -> *mut Fungine {
    let mut rng = rand::thread_rng();
    let mut initial_state = Vec::with_capacity(boid_num);
    for i in 0u64..boid_num as u64 {
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
            colour: boid_colour
        };
        let initial_object = Box::new(initial_object) as Box<GameObject>;
        let initial_object = Arc::new(initial_object);
        initial_state.push((i, initial_object));
    }
    let engine = Fungine::new(&Arc::new(initial_state), None);

    unsafe { transmute(Box::new(engine)) }
}

#[allow(dead_code)]
#[no_mangle]
pub unsafe extern fn step(sim_ptr: *mut Fungine, frame_time: f32) -> usize {
    let sim = &mut *sim_ptr;
    let _ = sim.run_steps_cont(1, frame_time);
    sim.current_state.len()
}

#[allow(dead_code)]
#[no_mangle]
pub unsafe extern fn getBoid(sim_ptr: *mut Fungine, index: usize) -> BoidObj {
    let sim = &mut *sim_ptr;
    let game_object = &sim.current_state[index];
    if let Some(boid) = game_object.1.downcast_ref::<Boid>() {
        BoidObj {
            id: game_object.0, 
            boid: *boid
        }
    }
    else {
        BoidObj {
            id: game_object.0, 
            boid: Boid {
                position: Vector3::new(0f32, 0f32, 0f32),
                direction: Vector3::new(0f32, 0f32, 0f32),
                colour: BoidColourKind::Green
            }
        }
    }
}

#[no_mangle]
pub unsafe extern fn destroySim(sim_ptr: *mut Fungine) {
    let _sim: Box<Fungine> = transmute(sim_ptr);
    // Drop to free
}