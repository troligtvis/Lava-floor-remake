use crate::{util};

use nphysics2d::world::{DefaultMechanicalWorld, DefaultGeometricalWorld};
use nphysics2d::object::{DefaultBodySet, DefaultColliderSet, DefaultBodyHandle, DefaultColliderHandle, Collider, ColliderDesc, BodyPartHandle, RigidBodyDesc};
use nphysics2d::joint::DefaultJointConstraintSet;
use nphysics2d::force_generator::DefaultForceGeneratorSet;
use nphysics2d::solver::SignoriniModel;

use ncollide2d::query::ContactManifold;

use nalgebra as na;
use na::{Point2, Vector2};

const TIME_STEP: f32 = 1.0 / 60.0;
const GRAVITY: f32 = 30.;

#[derive(PartialEq, Debug, Clone, Copy)]
pub enum ObjectType {
    Player, 
    // Enemy,
    Platform,
}

pub struct Physics2D {
    /** 
     * The mechanical world contains all the data structures and algorithms necessary to
     * perform efficiently the simulation of physical phenomena like 
     * gravity, contact forces, deformations, etc.
    */
    mechanical_world: DefaultMechanicalWorld<f32>,

    /**
     * The geometrical world contains all the data structures and algorithms necessary to perform efficiently 
     * geometric operations like computing sets of contacts between touching objects, detecting when two objects 
     * start interacting with each other, etc.
    */
    geometrical_world: DefaultGeometricalWorld<f32>,

    pub bodies: DefaultBodySet<f32>,
    pub colliders: DefaultColliderSet<f32>,
    joint_constraint_set: DefaultJointConstraintSet<f32>,
    force_generator_set: DefaultForceGeneratorSet<f32>,
    
    pub ticks: usize,
}

impl Physics2D {
    pub fn new() -> Self {
        let mut mechanical_world = DefaultMechanicalWorld::new(Vector2::new(0.0, 9.81));
        mechanical_world.set_timestep(TIME_STEP);
        mechanical_world
            .solver
            .set_contact_model(Box::new(SignoriniModel::new()));

        let geometrical_world = DefaultGeometricalWorld::new();

        let bodies = DefaultBodySet::new();
        let colliders = DefaultColliderSet::new();

        let joint_constraint_set = DefaultJointConstraintSet::new(); 
        let force_generator_set = DefaultForceGeneratorSet::new();

        Self {
            geometrical_world,
            mechanical_world,
            bodies,
            colliders,
            joint_constraint_set,
            force_generator_set,
            ticks: 0,
        }
    }

    pub fn step(&mut self) {
        self.mechanical_world.step(
            &mut self.geometrical_world,
            &mut self.bodies,
            &mut self.colliders,
            &mut self.joint_constraint_set,
            &mut self.force_generator_set,
        )
    }

    pub fn get_gravity(&self) -> f32 {
        GRAVITY
    }

    pub fn get_position(&self, handle: DefaultBodyHandle) -> Point2<f32> {
        util::isometry_to_point(
            *self.bodies.rigid_body(handle)
                .unwrap()
                .position()
        )
    }

    pub fn set_position(&mut self, handle: DefaultBodyHandle, point: Point2<f32>) {
        self.bodies.rigid_body_mut(handle)
            .unwrap()
            .set_position(util::point_to_isometry(point));
    }

    pub fn get_velocity(&self, handle: DefaultBodyHandle) -> Point2<f32> {
        self.bodies.rigid_body(handle)
            .unwrap()
            .velocity()
            .linear
            .into()
    }

    pub fn set_velocity(&mut self, handle: DefaultBodyHandle, velocity: Point2<f32>) {
        self.bodies.rigid_body_mut(handle)
            .unwrap()
            .set_linear_velocity(velocity.coords);
    }

    pub fn get_collider(&self, handle: DefaultColliderHandle) -> &Collider<f32, DefaultBodyHandle> {
        self.colliders
            .get(handle)
            .expect("No collider found for handle.")
    }

    pub fn collisions(
        &self,
        handle: DefaultColliderHandle,
    ) -> Vec<(
        (ObjectType, ObjectType),
        ContactManifold<f32>,
    )> {
        self.geometrical_world
            .contacts_with(&self.colliders, handle, true)
            .into_iter()
            .flatten()
            .map(|(handle1, _, handle2, _, _, manifold)| {
                (self.retrieve_user_datas(handle1, handle2), manifold.clone())
            })
            .collect()
    }

    pub fn retrieve_user_datas(
        &self,
        this_handle: DefaultColliderHandle,
        that_handle: DefaultColliderHandle,
    ) -> (ObjectType, ObjectType) {

        fn retrieve_user_data(collider: &Collider<f32, DefaultBodyHandle>) -> ObjectType {
            *collider
                .user_data()
                .expect("Tile has no user_data.")
                .downcast_ref::<ObjectType>()
                .expect("user_data has an invalid type.")
        };

        (
            retrieve_user_data(self.get_collider(this_handle)),
            retrieve_user_data(self.get_collider(that_handle)),
        )
    }

    fn on_ground(&self, manifold: &ContactManifold<f32>) -> bool {
        manifold.contacts().any(|tracked_contact| {
            tracked_contact.contact.normal[0usize].round() == 0.
        })
    }

    pub fn ground_check(
        &self, 
        collider_handle: DefaultColliderHandle,
        _object_type: ObjectType) -> bool {
        let mut on_ground = false;

        for (user_datas, manifold) in self.collisions(collider_handle) {
            match user_datas {
                (_object_type, _other) => match _other {
                    _other @ ObjectType::Platform => {
                        if self.on_ground(&manifold) {
                            on_ground = true;
                        }
                    },
                    _ => (),
                },
            }
        }

        on_ground
    }

    pub fn add_rigid_body(
        &mut self, 
        rigid_body_desc: RigidBodyDesc<f32>
    ) -> DefaultBodyHandle {
        let rb = rigid_body_desc.build();

        self.bodies.insert(rb)
    }

    pub fn add_collider(
        &mut self,
        body_handle: DefaultBodyHandle,
        collider_desc: ColliderDesc<f32>,
    ) -> DefaultColliderHandle {
        let collider = collider_desc
            .build(BodyPartHandle(body_handle, 0));

        self.colliders.insert(collider)
    }
}