extern crate fungine;
extern crate cgmath;
extern crate stopwatch;
extern crate rand;

pub mod boids;

use std::sync::Arc;
use std::mem::transmute;
use fungine::fungine::{ Fungine, GameObject, GameObjectWithID, MessageWithID, 
                        Message };
use boids::{ Boid, BoidColourKind, Player, MoveMessage };
use cgmath::{ Vector3, InnerSpace, Vector2 };
use rand::Rng;

#[repr(C)]
pub struct ReturnObj {
    pub id: u64,
    pub obj_type: ObjType,
    pub boid: Boid,
    pub player: Player
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum ObjType {
    Boid,
    Player,
    NoObj
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
    let mut initial_state = Vec::with_capacity(boid_num+1);
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
        initial_state.push(GameObjectWithID {
            id: i, 
            game_object: initial_object
        });
    }
    let initial_object = Player {
        position: Vector3::new(0f32, 0f32, -10f32),
        direction: Vector3::new(0f32, 0f32, 1f32),
        mouse_look: Vector2::new(0f32, 0f32),
        smooth_look: Vector2::new(0f32, 0f32)
    };
    let initial_object = Box::new(initial_object) as Box<GameObject>;
    let initial_object = Arc::new(initial_object);
    initial_state.push(GameObjectWithID {
        id: boid_num as u64,
        game_object: initial_object
    });
    let engine = Fungine::new(&Arc::new(initial_state));

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
pub unsafe extern fn getObj(sim_ptr: *mut Fungine, index: usize) -> ReturnObj {
    let sim = &mut *sim_ptr;
    let game_object = &sim.current_state[index];
    let obj_type: ObjType;
    let boid: Boid;
    let player: Player;
    if let Some(b) = game_object.game_object.downcast_ref::<Boid>() {
        obj_type = ObjType::Boid;
        boid = *b;
        player = Default::default();
    }
    else if let Some(p) = game_object.game_object.downcast_ref::<Player>() {
        obj_type = ObjType::Player;
        boid = Default::default();
        player = *p;
    }
    else {
        obj_type = ObjType::NoObj;
        boid = Default::default();
        player = Default::default();
    }
    ReturnObj {
        id: game_object.id, 
        obj_type:  obj_type,
        boid: boid,
        player: player
    }

}

#[allow(dead_code)]
#[no_mangle]
pub unsafe extern fn addMovement(sim_ptr: *mut Fungine, id: u64, forward: f32, 
    strafe: f32, mouse: Vector2<f32>) {
    let sim: &mut Fungine = &mut *sim_ptr;
    let message: Arc<Box<Message>> = Arc::new(Box::new(MoveMessage {
        forward: forward,
        strafe: strafe,
        mouse_input: mouse
    }));
    let message_pair = MessageWithID {
        id: id,
        message: message
    };
    sim.push_message(message_pair);
}

#[no_mangle]
pub unsafe extern fn destroySim(sim_ptr: *mut Fungine) {
    let _sim: Box<Fungine> = transmute(sim_ptr);
    // Drop to free
}