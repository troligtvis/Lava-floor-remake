use crate::{Context, world::World, physics::ObjectType, util, timer};
extern crate nalgebra as na;
use na::{Vector2, Point2};

use ncollide2d::shape::{ ShapeHandle, Cuboid };
use nphysics2d::object::{DefaultBodyHandle, DefaultColliderHandle, RigidBodyDesc, ColliderDesc};

const MAX_VEL: f32 = 100.;
const FALL_MULTIPLIER: f32 = 12.5;
const LOW_JUMP_MULTIPLIER: f32 = 12.;
const JUMP_POWER: f32 = 20.;

#[derive(Default)]
pub struct PlayerInput {
    pub left: bool,
    pub right: bool,
    pub jump: bool,
}

pub struct Player {
    pub input: PlayerInput,
    pub position: Point2<f32>,
    pub velocity: Vector2<f32>,
    body_handle: DefaultBodyHandle,
    collider_handle: DefaultColliderHandle,
    
    has_jumped: bool,
    last_on_ground: (bool, bool),
}

impl Player {
    pub fn new(world: &mut World) -> Self {
        let rigid_body_desc = RigidBodyDesc::new()
            .translation(Vector2::new(0., 0.))
            .mass(10.2);

        let body_handle = world.physics.add_rigid_body(rigid_body_desc);

        let rad = 10.;

        let shape_handle = ShapeHandle::new(Cuboid::new(Vector2::repeat(rad)));
        let collider_desc = ColliderDesc::new(shape_handle)
            .user_data(ObjectType::Player);
       
        let collider_handle = world.physics.add_collider(
            body_handle, 
            collider_desc,
        );
        
        Self {
            input: Default::default(),
            position: Point2::new(0., 0.),
            velocity: Vector2::zeros(),
            body_handle,
            collider_handle,
            has_jumped: false,
            last_on_ground: (true, true),
        }
    }

    pub fn update(&mut self, ctx: &Context, world: &mut World) {
        let dt: f32 = timer::delta(ctx).as_secs_f32();

        let direction = Vector2::new(
            self.input.right as i32 as f32 - self.input.left as i32 as f32,
            0.0,
        );
        
        let movement_direction = if direction.x > 0. {    
            Point2::new(1., 0.)
        } else if direction.x < 0. {
            Point2::new(-1., 0.)
        } else {
            Point2::new(0., 0.)
        };

        let mut velocity: Point2<f32> = world.physics.get_velocity(self.body_handle);
        velocity[0] = 0.;   // stop the velocity.x
        
        let is_grounded = self.last_on_ground.0 || self.last_on_ground.1;

        let max_vel = if is_grounded {
            MAX_VEL
        } else {
            MAX_VEL - 20.
        };

        world.physics.set_velocity(self.body_handle, util::add(velocity, movement_direction * max_vel));
        self.position = world.physics.get_position(self.body_handle);

        let on_ground = world.physics.ground_check(self.collider_handle, ObjectType::Player);
        let in_air = !(on_ground || self.last_on_ground.0 || !self.has_jumped);

        let gravity = world.physics.get_gravity();
        let up = Point2::new(0., 1.);

        if (is_grounded || !self.has_jumped) && self.input.jump {
            self.has_jumped = true;
            let jump_vector = up * -JUMP_POWER;
            velocity[0] = 0.;
            world.physics.set_velocity(self.body_handle, util::add(velocity, jump_vector));
            println!("JUMP {}", self.input.jump);
        }

        // Better jumping
        let velocity: Point2<f32> = world.physics.get_velocity(self.body_handle);
        if velocity.y > 0.0 {
            let vel: Point2<f32> =  up * gravity * (FALL_MULTIPLIER - 1.) * dt;
            world.physics.set_velocity(self.body_handle, util::add(velocity, vel));
        } else if velocity.y < 0.3 && !self.input.jump {
            let vel: Point2<f32> = up * gravity * (LOW_JUMP_MULTIPLIER - 1.) * dt;
            world.physics.set_velocity(self.body_handle, util::add(velocity, vel));
        }

        
        self.last_on_ground.0 = self.last_on_ground.1;
        self.last_on_ground.1 = on_ground;
    } 
}