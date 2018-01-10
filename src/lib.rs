extern crate fungine;
extern crate cgmath;
extern crate stopwatch;
extern crate rand;

pub mod boids;

use std::sync::Arc;
use std::mem::transmute;
use fungine::fungine::{ Fungine, GameObject, GameObjectWithID, MessageWithID, 
                        Message };
use boids::{ Boid, BoidColourKind, Player, MoveMessage, Plane, Tree, PlaneKind };
use cgmath::{ Vector3, InnerSpace, Vector2 };
use rand::Rng;
use rand::ThreadRng;

#[repr(C)]
pub struct ReturnObj {
    pub id: u64,
    pub obj_type: ObjType,
    pub boid: Boid,
    pub player: Player,
    pub plane: Plane,
    pub tree: Tree
}

#[repr(C)]
#[derive(Clone, Copy)]
pub enum ObjType {
    Boid,
    Player,
    Plane,
    Tree,
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
    let num_players = 1usize;
    let num_planes = 6usize;
    let num_trees = 10usize;
    let num_objects = boid_num + num_players + num_trees + num_planes;
    let mut initial_state: Vec<GameObjectWithID> = Vec::with_capacity(num_objects);
    set_up_boids(boid_num, &mut initial_state, &mut rng);
    set_up_player(boid_num as u64, &mut initial_state);
    set_up_planes((boid_num + num_players) as u64, &mut initial_state);
    set_up_trees((boid_num + num_players + num_planes) as u64, num_trees as u64, 
        &mut initial_state, &mut rng);
    let engine = Fungine::new(&Arc::new(initial_state));

    unsafe { transmute(Box::new(engine)) }
}

fn set_up_boids(boid_num: usize, states: &mut Vec<GameObjectWithID>, rng: &mut ThreadRng) {
    for i in 0u64..boid_num as u64 {
        let boid_colour = match i % 6 {
            0 => BoidColourKind::Green,
            1 => BoidColourKind::Blue,
            2 => BoidColourKind::Red,
            3 => BoidColourKind::Orange,
            4 => BoidColourKind::Purple,
            _ => BoidColourKind::Yellow,
        };
        let x = rng.gen_range::<f32>(5f32, 95f32);
        let y = rng.gen_range::<f32>(5f32, 95f32);
        let z = rng.gen_range::<f32>(5f32, 95f32);
        let x_dir = rng.gen_range::<f32>(-1f32, 1f32);
        let z_dir = rng.gen_range::<f32>(-1f32, 1f32);
        let initial_object = Boid {
            position: Vector3::new(x, y, z),
            direction: Vector3::new(x_dir, 0f32, z_dir).normalize(),
            colour: boid_colour
        };
        let initial_object = Box::new(initial_object) as Box<GameObject>;
        let initial_object = Arc::new(initial_object);
        states.push(GameObjectWithID {
            id: i, 
            game_object: initial_object
        });
    }
}

fn set_up_player(id: u64, states: &mut Vec<GameObjectWithID>) {
    let initial_object = Player {
        position: Vector3::new(50f32, 1f32, 50f32),
        direction: Vector3::new(0f32, 0f32, 1f32),
        mouse_look: Vector2::new(0f32, 0f32),
        smooth_look: Vector2::new(0f32, 0f32)
    };
    let initial_object = Box::new(initial_object) as Box<GameObject>;
    let initial_object = Arc::new(initial_object);
    states.push(GameObjectWithID {
        id: id,
        game_object: initial_object
    });
}

fn set_up_planes(start_id: u64, states: &mut Vec<GameObjectWithID>) {
    let ground = GameObjectWithID {
        id: start_id,
        game_object: Arc::new(Box::new(Plane {
            position: Vector3::new(50f32, 0f32, 50f32),
            direction: Vector3::new(0f32, 0f32, 1f32),
            texturing: PlaneKind::Ground
        }) as Box<GameObject>)
    };
    states.push(ground);
    let ceiling = GameObjectWithID {
        id: start_id + 1,
        game_object: Arc::new(Box::new(Plane {
            position: Vector3::new(50f32, 100f32, 50f32),
            direction: Vector3::new(0f32, 0f32, -1f32),
            texturing: PlaneKind::Transparent
        }) as Box<GameObject>)
    };
    states.push(ceiling);
    let wall = GameObjectWithID {
        id: start_id + 2,
        game_object: Arc::new(Box::new(Plane {
            position: Vector3::new(0f32, 50f32, 50f32),
            direction: Vector3::new(0f32, 0f32, -1f32),
            texturing: PlaneKind::Transparent
        }) as Box<GameObject>)
    };
    states.push(wall);
    let wall = GameObjectWithID {
        id: start_id + 3,
        game_object: Arc::new(Box::new(Plane {
            position: Vector3::new(100f32, 50f32, 50f32),
            direction: Vector3::new(0f32, 0f32, 1f32),
            texturing: PlaneKind::Transparent
        }) as Box<GameObject>)
    };
    states.push(wall);
    let wall = GameObjectWithID {
        id: start_id + 4,
        game_object: Arc::new(Box::new(Plane {
            position: Vector3::new(50f32, 50f32, 0f32),
            direction: Vector3::new(-1f32, 0f32, 0f32),
            texturing: PlaneKind::Transparent
        }) as Box<GameObject>)
    };
    states.push(wall);
    let wall = GameObjectWithID {
        id: start_id + 5,
        game_object: Arc::new(Box::new(Plane {
            position: Vector3::new(50f32, 50f32, 100f32),
            direction: Vector3::new(1f32, 0f32, 0f32),
            texturing: PlaneKind::Transparent
        }) as Box<GameObject>)
    };
    states.push(wall);
}

fn set_up_trees(start_id: u64, num_trees: u64, states: &mut Vec<GameObjectWithID>,
    rng: &mut ThreadRng) {
    for i in 0u64..num_trees as u64 {
        let x = rng.gen_range::<f32>(5f32, 95f32);
        let z = rng.gen_range::<f32>(5f32, 95f32);
        let x_dir = rng.gen_range::<f32>(-1f32, 1f32);
        let z_dir = rng.gen_range::<f32>(-1f32, 1f32);
        let initial_object = Arc::new(Box::new(Tree {
            position: Vector3::new(x, 0f32, z),
            direction: Vector3::new(x_dir, 0f32, z_dir).normalize()
        }) as Box<GameObject>);
        states.push(GameObjectWithID {
            id: start_id + i,
            game_object: initial_object
        });
    }
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
    let mut return_obj = ReturnObj {
        id: game_object.id,
        obj_type:  ObjType::NoObj,
        boid: Default::default(),
        player: Default::default(),
        plane: Default::default(),
        tree: Default::default()
    };
    if let Some(b) = game_object.game_object.downcast_ref::<Boid>() {
        return_obj.obj_type = ObjType::Boid;
        return_obj.boid = *b;
    }
    else if let Some(p) = game_object.game_object.downcast_ref::<Player>() {
        return_obj.obj_type = ObjType::Player;
        return_obj.player = *p;
    }
    else if let Some(pl) = game_object.game_object.downcast_ref::<Plane>() {
        return_obj.obj_type = ObjType::Plane;
        return_obj.plane = *pl;
    }
    else if let Some(t) = game_object.game_object.downcast_ref::<Tree>() {
        return_obj.obj_type = ObjType::Tree;
        return_obj.tree = *t;
    }
    return_obj
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