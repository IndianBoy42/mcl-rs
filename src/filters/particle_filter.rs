use nalgebra::{RealField, Scalar};

use crate::Pose;

use crate::filters::StateFilter;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParticleFilter<N: Scalar + RealField> {
    particles: Vec<Pose<N>>,
}

impl<N: Scalar + RealField> StateFilter<N> for ParticleFilter<N> {
    fn apply_odom<T: crate::odometry::OdometryModel<N>>(&mut self, odom: T, time: N) {
        todo!()
    }

    fn apply_sensor<T: crate::sensor_models::SensorModel<N>>(&mut self, sensor: T) {
        todo!()
    }
}
