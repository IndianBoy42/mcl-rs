use nalgebra::{RealField, Scalar};

use crate::{MapView, Pose};

pub trait Raycaster<N: Scalar + RealField> {
    fn new(map: MapView<N>) -> Self;
    fn get(&self, pose: Pose<N>) -> Option<N>;
}
