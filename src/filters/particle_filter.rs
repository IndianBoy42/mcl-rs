use nalgebra::{RealField, Scalar};

use crate::Pose;

use crate::filters::StateFilter;

#[derive(Clone, Debug)]
pub struct ParticleFilter<N: Scalar> {
    pose: Vec<Pose<N>>,
    weights: Vec<N>,
}

// impl<N> StateFilter<N> for ParticleFilter<N> {}
