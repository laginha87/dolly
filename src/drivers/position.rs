//use std::marker::PhantomData;

//use glam::Vec3;
use bevy_math::Vec3;
use bevy_transform::prelude::Transform;

use crate::{driver::RigDriver, rig::RigUpdateParams};

/*
use crate::{
    driver::RigDriver, handedness::Handedness, rig::RigUpdateParams, transform::Transform,
};
*/

/// Directly sets the position of the camera
#[derive(Debug)]
pub struct Position {
    pub position: mint::Point3<f32>,
}

impl Default for Position {
    fn default() -> Self {
        Self {
            position: Vec3::default().into(),
        }
    }
}

impl Position {
    pub fn new<P>(position: P) -> Self
    where
        P: Into<mint::Point3<f32>>,
    {
        let position = position.into();

        Self { position }
    }

    /// Add the specified vector to the position of this component
    pub fn translate<V>(&mut self, move_vec: V)
    where
        V: Into<mint::Vector3<f32>>,
    {
        let position: Vec3 = From::from(self.position);
        let move_vec: Vec3 = move_vec.into().into();
        self.position = (position + move_vec).into();
    }
}

impl RigDriver for Position {
    fn update(&mut self, params: RigUpdateParams) -> Transform {
        Transform {
            translation: self.position,
            rotation: params.parent.rotation,
            scale: Vec3::ONE,
        }
    }
}
