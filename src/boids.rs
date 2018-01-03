extern crate fungine;
extern crate cgmath;
extern crate stopwatch;
extern crate serde;

pub mod boids {
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

    #[repr(C)]
    #[derive(Clone, Serialize, Deserialize, Copy)]
    pub enum BoidColourKind {
        Green,
        Blue,
        Red,
        Orange,
        Purple,
        Yellow
    }

    #[repr(C)]
    #[derive(Clone, Serialize, Deserialize, Copy)]
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