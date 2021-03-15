use nalgebra::{RealField, Rotation2, Scalar, UnitComplex};
use num::FromPrimitive;

use crate::Pose;

use super::beam_rangefinder_model::Raycaster;

pub struct CDDT<N: Scalar + RealField> {
    inv_angle_res: N,
    inv_y_res: N,
    y_max: N,
    lut: Vec<DDTSlice<N>>,
}

impl<N: Scalar + RealField + num::FromPrimitive + num::ToPrimitive> Raycaster<N> for CDDT<N> {
    fn get(&self, pose: Pose<N>) -> Option<N> {
        let ang = pose.rotation.angle();
        let rot = pose.rotation;
        let idx = pose.translation * &rot;
        let yn = ((idx.vector[1] + self.y_max) * self.inv_y_res)
            .to_usize()
            .unwrap();
        let thn = (ang * self.inv_angle_res).round().to_usize().unwrap();
        let x = idx.vector[0];
        self.lut.get(thn * self.theta_size() + yn)?.get(x)
    }

    fn new(map: crate::MapView<N>) -> Self {
        todo!()
    }
}

impl<N: Scalar + RealField> CDDT<N> {
    fn theta_size(&self) -> usize
    where
        N: num::FromPrimitive + num::ToPrimitive,
    {
        (self.inv_angle_res * N::from_f64(360.0).unwrap())
            .to_usize()
            .unwrap()
    }
}

struct DDTSlice<N: Scalar + RealField> {
    zeroes: Vec<N>,
}
impl<N: Scalar + RealField> DDTSlice<N> {
    pub fn get(&self, x: N) -> Option<N>
    where
        N: PartialOrd,
    {
        let res = match self
            .zeroes
            .binary_search_by(|&test| test.partial_cmp(&x).unwrap())
            // .binary_search_by_key(&FloatOrd(x), |&x| FloatOrd(x))
        {
            Ok(exact) => N::zero(),
            Err(insertat) => {
                let &ub = self.zeroes.get(insertat)?;
                ub - x
            }
        };
        Some(res)
    }
}
