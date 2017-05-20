extern crate fungine;
extern crate cgmath;
extern crate stopwatch;

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

    #[derive(Clone)]
    pub struct Boid {
        pub position: Vector3<f32>,
        pub direction: Vector3<f32>
    }

    impl GameObject for Boid {
        fn box_clone(&self) -> Box<GameObject> {
            Box::new((*self).clone())
        }

        fn update(&self, current_state: Arc<Vec<Arc<Box<GameObject+Send+Sync>>>>, messages: Vec<Message>) -> Box<GameObject+Send+Sync> {
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
                direction: new_direction
            })
        }
    }
    unsafe impl Send for Boid {}
    unsafe impl Sync for Boid {}
}

use boids::Boid;
use fungine::fungine::{Fungine, GameObject, Message};
use cgmath::Vector3;

fn main() {
    let mut initial_state = Vec::new();
    for i in 0..1000 {
        let initial_object = Boid {
            position: Vector3::new(i as f32,i as f32,i as f32),
            direction: Vector3::new(1.0f32,1.0f32,1.0f32)
        };
        let initial_object = Box::new(initial_object) as Box<GameObject+Send+Sync>;
        let initial_object = Arc::new(initial_object);
        initial_state.push(initial_object);
    }
    let engine = Fungine::new(Arc::new(initial_state));
    let next_states = engine.run();
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use fungine::fungine::{Fungine, GameObject, Message};
    use stopwatch::{Stopwatch};
    use cgmath::Vector3;
    use boids::Boid;

    #[test]
    fn speed_test() {
        let mut initial_state = Vec::new();
        for i in 0..1000 {
            let initial_object = Boid {
                position: Vector3::new(i as f32,i as f32,i as f32),
                direction: Vector3::new(1.0f32,1.0f32,1.0f32)
            };
            let initial_object = Box::new(initial_object) as Box<GameObject+Send+Sync>;
            let initial_object = Arc::new(initial_object);
            initial_state.push(initial_object);
        }
        let engine = Fungine::new(Arc::new(initial_state));
        let sw = Stopwatch::start_new();
        let final_states = engine.run_steps(60);
        println!("Time taken: {}ms", sw.elapsed_ms());
    }
}
