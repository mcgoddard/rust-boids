extern crate fungine;
extern crate cgmath;

use std::sync::Arc;

mod boids {
    use std::sync::Arc;
    use fungine::fungine::{Fungine, GameObject, Message};
    use cgmath::Vector3;

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
            Box::new(Boid {
                position: self.position,
                direction: self.direction
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
