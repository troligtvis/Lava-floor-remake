use crate::{physics::ObjectType, World};
use nalgebra::{Point2, Vector2};

use nphysics2d::object::{Ground, ColliderDesc, DefaultBodyHandle, DefaultColliderHandle,};

use ncollide2d::shape::{Cuboid, ShapeHandle};

pub struct Platform {
    pub object_type: ObjectType,
    pub position: Point2<f32>,
    pub shape: Cuboid<f32>,

    collider_handle: DefaultColliderHandle,
    body_handle: DefaultBodyHandle,
}

impl Platform {
    pub fn new(
        size: Vector2<f32>,
        translation: Vector2<f32>,
        position: Point2<f32>,
        world: &mut World,
    ) -> Self {
        let shape = Cuboid::new(size);
        let shape_handle = ShapeHandle::new(shape.clone());
        
        let body_handle = world.physics.bodies.insert(Ground::new());
        let collider_desc = ColliderDesc::new(shape_handle)
            .translation(translation)
            .user_data(ObjectType::Platform);

        let collider_handle = world.physics.add_collider(
            body_handle, 
            collider_desc
        );

        Self {
            object_type: ObjectType::Platform,
            position,
            shape: shape,
            collider_handle,
            body_handle,
        }
    }
}