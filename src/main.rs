extern crate fungine;
extern crate cgmath;
extern crate stopwatch;
#[macro_use]
extern crate serde_derive;
extern crate serde;

use std::sync::Arc;

mod boids {
    use std::sync::Arc;
    use std::ops::Add;
    use std::ops::Sub;
    use std::ops::Div;
    use std::f32;
    use fungine::fungine::{GameObject, Message};
    use cgmath::Vector3;
    use cgmath::InnerSpace;

    const NEIGHBOUR_DISTANCE: f32 = 10.0;

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
                        separation_vector = separation_vector.sub(self.position.sub(boid.position))
                    }
                }
            }
            centre_vector = centre_vector.div(neighbour_count as f32).sub(self.position).div(100.0f32);
            align_vector = align_vector.div(neighbour_count as f32).sub(self.position).div(8.0f32);
            let new_direction = self.direction.add(centre_vector).add(align_vector).add(separation_vector).normalize();
            let new_position = self.position.add(new_direction);
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
use fungine::fungine::{Fungine, GameObject, Message};
use cgmath::Vector3;

fn main() {
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
        let initial_object = Boid {
            position: Vector3::new(i as f32,i as f32,i as f32),
            direction: Vector3::new(1.0f32,1.0f32,1.0f32),
            colour: boid_colour,
            id: i
        };
        let initial_object = Box::new(initial_object) as Box<GameObject>;
        let initial_object = Arc::new(initial_object);
        initial_state.push(initial_object);
    }
    let engine = Fungine::new(Arc::new(initial_state), None);
    let _next_states = engine.run();
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
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
}
