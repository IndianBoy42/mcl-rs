use nalgebra::{RealField, Scalar};

use crate::odometry::OdometryModel;
use crate::sensor_models::SensorModel;

pub mod kalman_filter;
pub mod particle_filter;
pub trait StateFilter<N: Scalar + RealField> {
    fn apply_odom<T: OdometryModel<N>>(&mut self, odom: T, time: N);
    fn apply_sensor<T: SensorModel<N>>(&mut self, sensor: T);
}
