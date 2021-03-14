use nalgebra::{RealField, Scalar};

use crate::Pose;

use crate::filters::StateFilter;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ParticleFilter<N: Scalar + RealField> {
    particles: Vec<Pose<N>>,
}

// impl<N: Scalar + RealField> StateFilter<N> for ParticleFilter<N> {}
