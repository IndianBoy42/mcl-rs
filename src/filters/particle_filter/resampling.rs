use nalgebra::Scalar;

use crate::Pose;

pub mod basic;
pub mod kld;

pub trait ParticleResampler<N: Scalar> {
    fn resample(&self, poses: &[Pose<N>], weights: &[N]) -> Vec<Pose<N>>;
}
