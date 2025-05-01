//use std::marker::PhantomData;

//use glam::Vec3;
use bevy_math::Vec3;
use bevy_transform::prelude::Transform;

use crate::{
    driver::RigDriver,
    //handedness::Handedness,
    rig::RigUpdateParams,
    //transform::Transform,
};

/// Offsets the camera along a vector, in the coordinate space of the parent.
#[derive(Debug)]
pub struct Arm {
    pub offset: mint::Vector3<f32>,
}

impl Arm {
    pub fn new<V>(offset: V) -> Self
    where
        V: Into<mint::Vector3<f32>>,
    {
        let offset = offset.into();

        Self { offset }
    }
}

impl RigDriver for Arm {
    fn update(&mut self, params: RigUpdateParams) -> Transform {
        Transform {
            translation: params.parent.translation + params.parent.rotation * self.offset,
            rotation: params.parent.rotation,
            scale: Vec3::ONE,
        }
    }
}
