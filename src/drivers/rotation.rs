//use std::marker::PhantomData;

//use glam::Quat;
use bevy_math::{Quat, Vec3};
use bevy_transform::prelude::Transform;

use crate::{driver::RigDriver, rig::RigUpdateParams};

/*
use crate::{
    driver::RigDriver, handedness::Handedness, rig::RigUpdateParams, transform::Transform,
};
*/

/// Directly sets the rotation of the camera
#[derive(Debug)]
pub struct Rotation {
    pub rotation: Quat,
}

impl Default for Rotation {
    fn default() -> Self {
        Self {
            rotation: Quat::default().into(),
        }
    }
}

impl Rotation {
    pub fn new<Q>(rotation: Q) -> Self
    where
        Q: Into<Quat>,
    {
        let rotation = rotation.into();

        Self { rotation }
    }
}

impl RigDriver for Rotation {
    fn update(&mut self, params: RigUpdateParams) -> Transform {
        Transform {
            translation: params.parent.translation,
            rotation: self.rotation,
            scale: Vec3::ONE,
        }
    }
}
