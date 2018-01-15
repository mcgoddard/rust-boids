extern crate fungine;
extern crate cgmath;
extern crate stopwatch;
extern crate rand;
extern crate rustboidslib;

use rustboidslib::boids::{ Boid, BoidColourKind };
use std::sync::Arc;
use fungine::fungine::{ Fungine, GameObject, GameObjectWithID };
use cgmath::{ Vector3, InnerSpace };
use rand::Rng;

// Start the boids simulation up with 1000 boids
fn main() {
    let mut rng = rand::thread_rng();
    let mut initial_state = Vec::new();
    for i in 0u64..1000u64 {
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
        initial_state.push(GameObjectWithID {
            id: i, 
            game_object: initial_object
        });
    }
    let engine = Fungine::new(&Arc::new(initial_state));
    engine.run();
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use fungine::fungine::{ Fungine, GameObject, GameObjectWithID };
    use stopwatch::{ Stopwatch };
    use cgmath::Vector3;
    use rustboidslib::boids::{ Boid, BoidColourKind };

    #[test]
    fn speed_test() {
        let mut initial_state = Vec::new();
        for i in 0u64..1000u64 {
            let initial_object = Boid {
                position: Vector3::new(i as f32,i as f32,i as f32),
                direction: Vector3::new(1.0f32,1.0f32,1.0f32),
                colour: BoidColourKind::Green
            };
            let initial_object = Box::new(initial_object) as Box<GameObject>;
            let initial_object = Arc::new(initial_object);
            initial_state.push(GameObjectWithID {
                id: i,
                game_object: initial_object
            });
        }
        let engine = Fungine::new(&Arc::new(initial_state));
        let sw = Stopwatch::start_new();
        let _final_states = engine.run_steps(60, 1f32);
        println!("Time taken: {}ms", sw.elapsed_ms());
    }
}
