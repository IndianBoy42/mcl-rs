pub mod basic;
pub mod kld;

pub trait ParticleResampler<N> {
    fn resample(&self, poses: &[Pose<N>], weights: &[N]) -> Vec<Pose<N>>;
}
