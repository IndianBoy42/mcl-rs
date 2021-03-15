use nalgebra::{RealField, Scalar};

use crate::{Pose, PoseCovariance};

pub trait OdometryModel<N: Scalar + Copy> {
    fn update_all(&self, dt: N, poses: &mut [Pose<N>]) {
        for pose in poses {
            *pose = self.update_one(dt, *pose);
        }
    }
    fn update_one(&self, dt: N, pose: Pose<N>) -> Pose<N>;
}

pub trait GaussianOdometryModel<N: Scalar> {
    fn mean(&self, dt: N, curr: Pose<N>) -> Pose<N>;
    fn covariance(&self, dt: N, curr: Pose<N>) -> PoseCovariance<N>;
}

pub trait LinearOdometryModel<N> {}
impl<LOM: LinearOdometryModel<N>, N: Scalar + Copy> OdometryModel<N> for LOM {
    fn update_one(&self, dt: N, pose: Pose<N>) -> Pose<N> {
        // self.mean(dt, pose) + randomly_sample_from_gaussian
        todo!();
    }
    fn update_all(&self, dt: N, poses: &mut [Pose<N>]) {
        // let mean = self.mean(dt, pose);
        // mean + randomly_sample_alot_from_gaussian
        todo!();
    }
}

pub struct UnscentedOdometryModel<M> {
    model: M,
}
impl<N, M> GaussianOdometryModel<N> for UnscentedOdometryModel<M>
where
    N: Scalar + Copy,
    M: OdometryModel<N>,
{
    fn mean(&self, dt: N, curr: Pose<N>) -> Pose<N> {
        todo!()
    }

    fn covariance(&self, dt: N, curr: Pose<N>) -> PoseCovariance<N> {
        todo!()
    }
}
