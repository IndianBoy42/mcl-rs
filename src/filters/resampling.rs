pub mod basic;
pub mod kld;

pub trait ParticleResampler<N: Scalar + RealField> {
    fn resample(&self, poses: &[Pose<N>], weights: &[N]) -> Vec<Pose<N>>;
}
