extern crate fungine;
extern crate cgmath;
extern crate stopwatch;

use std::sync::Arc;
use std::ops::{ Add, Sub, Div, Mul };
use std::f32;
use std::f32::consts::PI;
use fungine::fungine::{ GameObject, GameObjectWithID, MessageList, UpdateResult,
                        Message };
use cgmath::{ Vector3, Vector2, InnerSpace, Rad, Angle, ElementWise, Deg, 
              Quaternion, Rotation3, Rotation };

const NEIGHBOUR_DISTANCE: f32 = 10.0;
const SEPARATION_DISTANCE: f32 = 1.0;
const MAX_TURN: Rad<f32> = Rad(PI / 4f32);
const MOUSE_SENSITIVITY: f32 = 4.0f32;
const MOUSE_SMOOTHING: f32 = 2.0f32;
const MOUSE_SCALE_VEC: Vector2<f32> = Vector2 {
    x: MOUSE_SENSITIVITY * MOUSE_SMOOTHING,
    y: MOUSE_SENSITIVITY * MOUSE_SMOOTHING
};
pub const DIRECTION_RIGHT: Vector3<f32> = Vector3 { x: 1f32, y: 0f32, z: 0f32 };
pub const DIRECTION_FORWARD: Vector3<f32> = Vector3 { x: 0f32, y: 0f32, z: 1f32 };
pub const DIRECTION_UP: Vector3<f32> = Vector3 { x: 0f32, y: 1f32, z: 0f32 };

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

impl Boid {
    pub fn new() -> Self {
        Boid {
            position: Vector3::new(0.0f32, 0.0f32, 0.0f32),
            direction: Vector3::new(0.0f32, 0.0f32, 0.0f32),
            colour: BoidColourKind::Green
        }
    }
}

impl Default for Boid {
    fn default() -> Self {
        Self::new()
    }
}

impl GameObject for Boid {
    fn box_clone(&self) -> Box<GameObject> {
        Box::new(*self)
    }

    fn update(&self, id: u64, current_state: Arc<Vec<GameObjectWithID>>, 
            _messages: Arc<MessageList>, frame_time: f32) -> UpdateResult {
        let mut centre_vector = Vector3::new(0.0f32, 0.0f32, 0.0f32);
        let mut align_vector = Vector3::new(0.0f32, 0.0f32, 0.0f32);
        let mut separation_vector = Vector3::new(0.0f32, 0.0f32, 0.0f32);
        let mut neighbour_count = 0u32;
        for object_pair in current_state.iter() {
            let boid: Box<GameObject> = object_pair.game_object.box_clone();
            if let Some(boid) = boid.downcast_ref::<Boid>() {
                let distance = euclidian_distance(boid.position, self.position);
                if id != object_pair.id && distance < NEIGHBOUR_DISTANCE {
                    centre_vector = centre_vector.add(boid.position);
                    align_vector = align_vector.add(boid.direction);
                    neighbour_count += 1;
                    if distance < SEPARATION_DISTANCE {
                        separation_vector = separation_vector.sub(boid.position.sub(self.position))
                    }
                }
            }
        }
        if neighbour_count > 0 {
            centre_vector = centre_vector.div(neighbour_count as f32).sub(self.position).div(100.0f32);
            align_vector = align_vector.div(neighbour_count as f32).sub(self.direction).div(8.0f32);
        }
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
        UpdateResult {
            state: Box::new(Boid {
                position: new_position,
                direction: new_direction,
                colour: self.colour
            }),
            messages: vec![]
        }
    }
}
unsafe impl Send for Boid {}
unsafe impl Sync for Boid {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Player {
    pub position: Vector3<f32>,
    pub direction: Vector3<f32>,
    pub mouse_look: Vector2<f32>,
    pub smooth_look: Vector2<f32>
}

impl Player {
    pub fn new() -> Self {
        Player {
            position: Vector3::new(0.0f32, 0.0f32, 0.0f32),
            direction: Vector3::new(0.0f32, 0.0f32, 0.0f32),
            mouse_look: Vector2::new(0.0f32, 0.0f32),
            smooth_look: Vector2::new(0.0f32, 0.0f32)
        }
    }
}

impl Default for Player {
    fn default() -> Self {
        Self::new()
    }
}

impl GameObject for Player {
    fn box_clone(&self) -> Box<GameObject> {
        Box::new(*self)
    }

    fn update(&self, _id: u64, _current_state: Arc<Vec<GameObjectWithID>>, 
            messages: Arc<MessageList>, frame_time: f32) -> UpdateResult {
        let mut md = Vector2::new(0f32, 0f32);
        let mut forward: f32 = 0.0f32;
        let mut strafe: f32 = 0.0f32;
        for message in messages.clone().iter() {
            let message: Box<Message> = message.box_clone();
            if let Some(message) = message.downcast_ref::<MoveMessage>() {
                md += message.mouse_input;
                forward = message.forward;
                strafe = message.strafe;
            }
        }
        md = md.mul_element_wise(MOUSE_SCALE_VEC);
        let new_smooth_look = self.smooth_look.lerp(md, 1f32/MOUSE_SMOOTHING);
        let mut new_mouse_look = self.mouse_look + new_smooth_look;
        if new_mouse_look.y > 90f32 {
            new_mouse_look = Vector2::new(new_mouse_look.x, 90f32);
        }
        else if new_mouse_look.y < -90f32 {
            new_mouse_look = Vector2::new(new_mouse_look.x, -90f32);
        }
        let x_rotation = Quaternion::from_axis_angle(DIRECTION_RIGHT, Rad::from(Deg(-new_mouse_look.y)));
        let y_rotation = Quaternion::from_axis_angle(DIRECTION_UP, Rad::from(Deg(new_mouse_look.x)));
        let rotation = (y_rotation * x_rotation).normalize();
        let new_direction = rotation.rotate_vector(DIRECTION_FORWARD);
        let mut new_forward_direction = new_direction.mul(forward);
        new_forward_direction.y -= new_forward_direction.y;
        let mut new_strafe_direction = rotation.rotate_vector(DIRECTION_RIGHT).mul(strafe);
        new_strafe_direction.y -= new_strafe_direction.y;
        let mut new_position = self.position;
        new_position = new_position + (new_forward_direction.mul(frame_time));
        new_position = new_position + (new_strafe_direction.mul(frame_time));
        UpdateResult {
            state: Box::new(Player {
                position: new_position,
                direction: new_direction,
                mouse_look: new_mouse_look,
                smooth_look: new_smooth_look
            }),
            messages: vec![]
        }
    }
}
unsafe impl Send for Player {}
unsafe impl Sync for Player {}

#[repr(C)]
#[derive(Clone, Debug)]
pub struct MoveMessage {
    pub forward: f32,
    pub strafe: f32,
    pub mouse_input: Vector2<f32>
}

impl Message for MoveMessage {
    fn box_clone(&self) -> Box<Message> {
        Box::new((*self).clone())
    }
}
unsafe impl Send for MoveMessage {}
unsafe impl Sync for MoveMessage {}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum PlaneKind {
    Transparent,
    Ground
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Plane {
    pub position: Vector3<f32>,
    pub direction: Quaternion<f32>,
    pub texturing: PlaneKind
}

impl Plane {
    pub fn new() -> Self {
        Plane {
            position: Vector3::new(0f32, 0f32, 0f32),
            direction: Quaternion::new(0f32, 0f32, 0f32, 0f32),
            texturing: PlaneKind::Transparent
        }
    }
}

impl Default for Plane {
    fn default() -> Self {
        Self::new()
    }
}

impl GameObject for Plane {
    fn box_clone(&self) -> Box<GameObject> {
        Box::new(*self)
    }

    fn update(&self, _id: u64, _current_state: Arc<Vec<GameObjectWithID>>, 
            _messages: Arc<MessageList>, _frame_time: f32) -> UpdateResult {
        UpdateResult {
            state: Box::new(Plane {
                position: self.position,
                direction: self.direction,
                texturing: self.texturing
            }),
            messages: vec![]
        }
    }
}
unsafe impl Send for Plane {}
unsafe impl Sync for Plane {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Tree {
    pub position: Vector3<f32>,
    pub direction: Vector3<f32>
}

impl Tree {
    pub fn new() -> Self {
        Tree {
            position: Vector3::new(0.0f32, 0.0f32, 0.0f32),
            direction: Vector3::new(0.0f32, 0.0f32, 1.0f32)
        }
    }
}

impl Default for Tree {
    fn default() -> Self {
        Self::new()
    }
}

impl GameObject for Tree {
    fn box_clone(&self) -> Box<GameObject> {
        Box::new(*self)
    }

    fn update(&self, _id: u64, _current_state: Arc<Vec<GameObjectWithID>>, 
            _messages: Arc<MessageList>, _frame_time: f32) -> UpdateResult {
        UpdateResult {
            state: Box::new(Tree {
                position: self.position,
                direction: self.direction
            }),
            messages: vec![]
        }
    }
}
unsafe impl Send for Tree {}
unsafe impl Sync for Tree {}
