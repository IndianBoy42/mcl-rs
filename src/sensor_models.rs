use nalgebra::{RealField, Scalar};

use crate::Pose;

pub mod beam_rangefinder_model;
pub mod known_landmark_model;
pub mod likelihood_field_model;
pub mod unknown_landmark_model;

pub trait SensorModel<F: Scalar + RealField> {
    fn probability(&self, pos: Pose<F>) -> F;

    // TODO: how to apply this for kalman filters? the above may work for UKF but not others
}
