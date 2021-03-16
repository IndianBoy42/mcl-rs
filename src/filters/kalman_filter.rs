use nalgebra::Scalar;

use crate::odometry::GaussianOdometryModel;
use crate::sensor_models::GaussianSensorModel;
use crate::{Pose, PoseCovariance};

pub trait KalmanFilter<N: Scalar> {
    fn pose(&self) -> Pose<N>;
    fn covariance(&self) -> PoseCovariance<N>;

    fn apply_odom<T: GaussianOdometryModel<N>>(&mut self, odom: T, time: N);
    fn apply_sensor<T: GaussianSensorModel<N>>(&mut self, sensor: T)
    where
        N: Copy;
}
