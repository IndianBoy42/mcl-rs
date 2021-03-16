pub mod cddt;
pub mod icp;
pub mod spatial_index;

use nalgebra::Scalar;

use crate::{MapView, Pose};

pub trait Raycaster<N: Scalar> {
    fn build(grid_res: N, angle_res: N, map: MapView<N>) -> Self;
    fn get(&self, pose: Pose<N>) -> Option<N>;
}
