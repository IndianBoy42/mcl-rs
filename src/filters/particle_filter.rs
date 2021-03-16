use nalgebra::{Scalar};

use crate::Pose;

pub mod resampling;

#[derive(Clone, Debug)]
pub struct ParticleFilter<N: Scalar> {
    pose: Vec<Pose<N>>,
    weights: Vec<N>,
}

// impl<N> StateFilter<N> for ParticleFilter<N> {}
