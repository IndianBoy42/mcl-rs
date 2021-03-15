use nalgebra::{RealField, Scalar};

use crate::{MapView, Pose};

pub trait Raycaster<N: Scalar> {
    fn build(&mut self, map: crate::MapView<N>) -> Self;
    fn get(&self, pose: Pose<N>) -> Option<N>;
}
