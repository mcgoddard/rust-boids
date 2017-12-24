extern crate fungine;
extern crate cgmath;
extern crate stopwatch;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate rand;

use std::sync::Arc;

mod boids {
    use std::sync::Arc;
    use std::ops::Add;
    use std::ops::Sub;
    use std::ops::Div;
    use std::f32;
    use fungine::fungine::{GameObject, Message};
    use cgmath::{ Vector3, InnerSpace };

    const NEIGHBOUR_DISTANCE: f32 = 10.0;
    const SEPARATION_DISTANCE: f32 = 1.0;

    fn euclidian_distance(first: Vector3<f32>, second: Vector3<f32>) -> f32 {
        ((second.x - first.x).powi(2) +
            (second.y - first.y).powi(2) +
            (second.z - first.z).powi(2)).sqrt()
    }

    // Add serde serialization to cgmath's Vector3 type
    #[derive(Serialize, Deserialize)]
    #[serde(remote = "Vector3")]
    struct Vector3Def<S> {
        /// The x component of the vector.
        pub x: S,
        /// The y component of the vector.
        pub y: S,
        /// The z component of the vector.
        pub z: S,
    }

    #[derive(Clone, Serialize, Deserialize)]
    pub enum BoidColourKind {
        Green,
        Blue,
        Red,
        Orange,
        Purple,
        Yellow
    }

    #[derive(Clone, Serialize, Deserialize)]
    pub struct Boid {
        #[serde(with = "Vector3Def")]
        pub position: Vector3<f32>,
        #[serde(with = "Vector3Def")]
        pub direction: Vector3<f32>,
        pub colour: BoidColourKind,
        pub id: i32
    }

    impl GameObject for Boid {
        fn box_clone(&self) -> Box<GameObject> {
            Box::new((*self).clone())
        }

        fn update(&self, current_state: Arc<Vec<Arc<Box<GameObject>>>>, _messages: Vec<Message>) -> Box<GameObject> {
            let mut centre_vector = Vector3::new(0.0f32, 0.0f32, 0.0f32);
            let mut align_vector = Vector3::new(0.0f32, 0.0f32, 0.0f32);
            let mut separation_vector = Vector3::new(0.0f32, 0.0f32, 0.0f32);
            let mut neighbour_count = 0.0f32;
            for boid in current_state.iter() {
                let boid: Box<GameObject> = boid.box_clone();
                if let Some(boid) = boid.downcast_ref::<Boid>() {
                    let distance = euclidian_distance(boid.position, self.position);
                    if distance < NEIGHBOUR_DISTANCE {
                        centre_vector = centre_vector.add(boid.position);
                        align_vector = align_vector.add(boid.direction);
                        neighbour_count += 1.0f32;
                        if distance < SEPARATION_DISTANCE {
                            separation_vector = separation_vector.sub(boid.position.sub(self.position))
                        }
                    }
                }
            }
            centre_vector = centre_vector.div(neighbour_count as f32).sub(self.position).div(100.0f32);
            align_vector = align_vector.div(neighbour_count as f32).sub(self.direction).div(8.0f32);
            let mut new_direction = self.direction;
            new_direction = new_direction.add(centre_vector);
            new_direction = new_direction.add(align_vector);
            new_direction = new_direction.add(separation_vector);
            new_direction = new_direction.normalize();
            // TODO : the divide by 120 here should be frame time dependant
            let new_position = self.position.add(new_direction.div(120.0f32));
            Box::new(Boid {
                position: new_position,
                direction: new_direction,
                colour: self.colour.clone(),
                id: self.id
            })
        }
    }
    unsafe impl Send for Boid {}
    unsafe impl Sync for Boid {}
}

use boids::{Boid, BoidColourKind};
use std::str::FromStr;
use fungine::fungine::{Fungine, GameObject, Message};
use cgmath::{ Vector3, InnerSpace };
use rand::Rng;

fn main() {
    let mut rng = rand::thread_rng();
    let mut initial_state = Vec::new();
    for i in 0i32..100i32 {
        let boid_colour = match i % 6 {
            0 => BoidColourKind::Green,
            1 => BoidColourKind::Blue,
            2 => BoidColourKind::Red,
            3 => BoidColourKind::Orange,
            4 => BoidColourKind::Purple,
            _ => BoidColourKind::Yellow,
        };
        let x = ((rng.gen::<u32>() % 300) as f32) / 10.0;
        let y = ((rng.gen::<u32>() % 300) as f32) / 10.0;
        let z = ((rng.gen::<u32>() % 300) as f32) / 10.0;
        let initial_object = Boid {
            position: Vector3::new(x, y, z),
            direction: Vector3::new(rng.gen::<f32>(),rng.gen::<f32>(),rng.gen::<f32>()).normalize(),
            colour: boid_colour,
            id: i
        };
        let initial_object = Box::new(initial_object) as Box<GameObject>;
        let initial_object = Arc::new(initial_object);
        initial_state.push(initial_object);
    }
    let port = "4794";
    let engine = Fungine::new(Arc::new(initial_state), Some(String::from_str(port)).unwrap().ok());
    let _next_states = engine.run();
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
