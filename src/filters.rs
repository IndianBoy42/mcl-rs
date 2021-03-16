use nalgebra::{Scalar, SimdRealField};

use crate::odometry::OdometryModel;
use crate::sensor_models::SensorModel;
use crate::{Pose, PoseCovariance};

pub mod kalman_filter;
pub mod particle_filter;
pub trait StateFilter<N: Scalar + SimdRealField> {
    fn pose(&self) -> Pose<N>;
    fn covariance(&self) -> PoseCovariance<N>;

    fn apply_odom<T: OdometryModel<N>>(&mut self, odom: T, time: N);
    fn apply_sensor<T: SensorModel<N>>(&mut self, sensor: T)
    where
        N: Copy;
}
