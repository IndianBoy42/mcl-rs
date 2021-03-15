use nalgebra::{RealField, Scalar};

use crate::Pose;

mod cddt;
mod icp;

pub mod beam_rangefinder_model;
pub mod known_landmark_model;
pub mod likelihood_field_model;
pub mod unknown_landmark_model;

pub trait SensorModel<F: Scalar + Copy> {
    fn probability(&self, pos: Pose<F>) -> F;
    fn probability_all(&self, pos: &[Pose<F>]) -> Vec<F> {
        pos.iter().map(|&f| self.probability(f)).collect()
    }
}

pub trait GaussianSensorModel<F: Scalar> {
    // These probably have to be constrained,
    // will figure out nalgebra generics later
    type MeasurementVec;
    type MeasurementMat;

    fn mean(pos: Pose<F>) -> Self::MeasurementVec;
    fn covariance(pos: Pose<F>) -> Self::MeasurementMat;
}

pub trait LinearSensorModel<F: Scalar>: GaussianSensorModel<F> {
    fn get_measurment(&self) -> Self::MeasurementVec;
}
impl<LSM: LinearSensorModel<F>, F: Scalar + Copy> SensorModel<F> for LSM {
    fn probability(&self, pos: Pose<F>) -> F {
        let mean = Self::mean(pos);
        let cov = Self::covariance(pos);
        let meas = self.get_measurment();
        // use self.meas
        todo!()
    }
}

pub struct UnscentedSensorModel<M> {
    model: M,
}
// impl<F, M> GaussianSensorModel<F> for UnscentedSensorModel<M>
// where
//     F,
//     M: SensorModel<F>,
// {}
