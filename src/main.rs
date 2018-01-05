extern crate fungine;
extern crate cgmath;
extern crate stopwatch;
extern crate rand;
extern crate rustboidslib;

use rustboidslib::boids::{Boid, BoidColourKind};
use std::str::FromStr;
use std::sync::Arc;
use fungine::fungine::{Fungine, GameObject};
use cgmath::{ Vector3, InnerSpace };
use rand::Rng;

fn main() {
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
    let port = "4794";
    let engine = Fungine::new(&Arc::new(initial_state), Some(String::from_str(port)).unwrap().ok());
    engine.run();
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use std::str::FromStr;
    use fungine::fungine::{Fungine, GameObject, Message};
    use stopwatch::{Stopwatch};
    use cgmath::Vector3;
    use boids::{Boid, BoidColourKind};

    #[test]
    fn speed_test() {
        let mut initial_state = Vec::new();
        for i in 0i32..1000i32 {
            let initial_object = Boid {
                position: Vector3::new(i as f32,i as f32,i as f32),
                direction: Vector3::new(1.0f32,1.0f32,1.0f32),
                colour: BoidColourKind::Green,
                id: i
            };
            let initial_object = Box::new(initial_object) as Box<GameObject>;
            let initial_object = Arc::new(initial_object);
            initial_state.push(initial_object);
        }
        let engine = Fungine::new(Arc::new(initial_state), None);
        let sw = Stopwatch::start_new();
        let _final_states = engine.run_steps(60);
        println!("Time taken: {}ms", sw.elapsed_ms());
    }

    #[test]
    fn speed_with_networking_test() {
        let mut initial_state = Vec::new();
        for i in 0i32..500i32 {
            let initial_object = Boid {
                position: Vector3::new(i as f32,i as f32,i as f32),
                direction: Vector3::new(1.0f32,1.0f32,1.0f32),
                colour: BoidColourKind::Green,
                id: i
            };
            let initial_object = Box::new(initial_object) as Box<GameObject>;
            let initial_object = Arc::new(initial_object);
            initial_state.push(initial_object);
        }
        let port = "4794";
        let engine = Fungine::new(Arc::new(initial_state), Some(String::from_str(port)).unwrap().ok());
        let sw = Stopwatch::start_new();
        let _final_states = engine.run_steps(60);
        println!("Time taken: {}ms", sw.elapsed_ms());
    }
}
