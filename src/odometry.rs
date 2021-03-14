use nalgebra::{RealField, Scalar};

use crate::{Pose, PoseCovariance};

pub trait OdometryModel<N: Scalar + RealField> {
    fn update_all(&self, dt: N, particles: &mut [Pose<N>]) {
        for particle in particles {
            *particle = self.update_one(dt, *particle);
        }
    }
    fn update_one(&self, dt: N, particle: Pose<N>) -> Pose<N>;
}

pub trait GaussianOdometryModel<N: Scalar + RealField> {
    fn mean(&self, dt: N, curr: Pose<N>) -> Pose<N>;
    fn covariance(&self, dt: N, curr: Pose<N>) -> PoseCovariance<N>;
}

pub trait LinearOdometryModel<N: Scalar + RealField> {}
impl<LOM: LinearOdometryModel<N>, N: Scalar + RealField> OdometryModel<N> for LOM {
    fn update_one(&self, dt: N, particle: Pose<N>) -> Pose<N> {
        // self.mean(dt, particle) + randomly_sample_from_gaussian
        todo!();
    }
    fn update_all(&self, dt: N, particles: &mut [Pose<N>]) {
        // let mean = self.mean(dt, particle);
        // mean + randomly_sample_alot_from_gaussian
        todo!();
    }
}

pub struct UnscentedOdometryModel<M> {
    model: M,
}
impl<N, M> GaussianOdometryModel<N> for UnscentedOdometryModel<M>
where
    N: Scalar + RealField,
    M: OdometryModel<N>,
{
    fn mean(&self, dt: N, curr: Pose<N>) -> Pose<N> {
        todo!()
    }

    fn covariance(&self, dt: N, curr: Pose<N>) -> PoseCovariance<N> {
        todo!()
    }
}
