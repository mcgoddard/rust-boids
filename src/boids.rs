extern crate fungine;
extern crate cgmath;
extern crate stopwatch;

use std::sync::Arc;
use std::ops::{ Add, Sub, Div, Mul };
use std::f32;
use std::f32::consts::PI;
use fungine::fungine::{ GameObject, Message, GameObjectWithID };
use cgmath::{ Vector3, InnerSpace, Rad, Angle };

const NEIGHBOUR_DISTANCE: f32 = 10.0;
const SEPARATION_DISTANCE: f32 = 1.0;
const MAX_TURN: Rad<f32> = Rad(PI / 4f32);

fn euclidian_distance(first: Vector3<f32>, second: Vector3<f32>) -> f32 {
    ((second.x - first.x).powi(2) +
        (second.y - first.y).powi(2) +
        (second.z - first.z).powi(2)).sqrt()
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum BoidColourKind {
    Green,
    Blue,
    Red,
    Orange,
    Purple,
    Yellow
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Boid {
    pub position: Vector3<f32>,
    pub direction: Vector3<f32>,
    pub colour: BoidColourKind
}

impl GameObject for Boid {
    fn box_clone(&self) -> Box<GameObject> {
        Box::new(*self)
    }

    fn update(&self, current_state: Arc<Vec<GameObjectWithID>>, _messages: Vec<Box<Message>>, frame_time: f32) -> Box<GameObject> {
        let mut centre_vector = Vector3::new(0.0f32, 0.0f32, 0.0f32);
        let mut align_vector = Vector3::new(0.0f32, 0.0f32, 0.0f32);
        let mut separation_vector = Vector3::new(0.0f32, 0.0f32, 0.0f32);
        let mut neighbour_count = 0.0f32;
        for boid in current_state.iter() {
            let boid: Box<GameObject> = boid.game_object.box_clone();
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
        let angle_between = new_direction.angle(self.direction);
        let frame_angle = MAX_TURN * frame_time;
        if angle_between > frame_angle {
            let d_tick = ((self.direction.cross(new_direction)).cross(self.direction)).normalize();
            new_direction = frame_angle.cos() * self.direction + frame_angle.sin() * d_tick;
        }
        let new_position = self.position.add(new_direction.mul(frame_time));
        Box::new(Boid {
            position: new_position,
            direction: new_direction,
            colour: self.colour
        })
    }
}
unsafe impl Send for Boid {}
unsafe impl Sync for Boid {}