use nalgebra::{RealField, Scalar};

use crate::Pose;

pub trait OdometryModel<N: Scalar + RealField> {
    fn update_all(dt: N, particles: &mut Vec<Pose<N>>);
    fn update_one(dt: N, particle: Pose<N>) -> Pose<N>;
}
