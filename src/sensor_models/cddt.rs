use std::ops::{Add, Mul};

use nalgebra::{RealField, Rotation2, Scalar, SimdRealField, SimdValue, UnitComplex};
use num::FromPrimitive;

use crate::Pose;

use super::beam_rangefinder_model::Raycaster;

pub struct CDDT<N> {
    inv_angle_res: N,
    inv_y_res: N,
    y_max: N,
    lut: Vec<DDTSlice<N>>,
}

impl<N> Raycaster<N> for CDDT<N>
where
    N: Scalar + num::FromPrimitive + num::ToPrimitive + SimdRealField + PartialOrd,
    <N as SimdValue>::Element: SimdRealField,
    for<'a> &'a N: Mul<Output = N>,
{
    fn build(&mut self, map: crate::MapView<N>) -> Self {
        self.lut.clear();
        self.y_max = N::from_usize(map.nrows() / 2).unwrap() / self.inv_y_res;
        self.lut.reserve(self.theta_size() * self.y_size());
        todo!()
    }

    fn get(&self, pose: Pose<N>) -> Option<N> {
        let ang = pose.rotation.angle();
        let rot = pose.rotation;
        let idx = rot * pose.translation.vector;
        // let idx = pose.translation * &rot;
        let yn = (idx[1] * self.inv_y_res).to_usize().unwrap();
        let thn = (ang * self.inv_angle_res).to_usize().unwrap();
        let x = idx[0];
        self.lut.get(thn * self.theta_size() + yn)?.get(x)
    }
}

impl<N> CDDT<N>
where
    N: num::FromPrimitive + num::ToPrimitive + Add<N, Output = N>,
    for<'a> &'a N: Mul<Output = N>,
{
    fn new() -> Self {
        Self {
            inv_angle_res: N::from_f64(1.0).unwrap(),
            inv_y_res: N::from_f64(1.0 / 0.01).unwrap(),
            y_max: N::from_f64(10.0).unwrap(),
            lut: Vec::new(),
        }
    }
    fn y_size(&self) -> usize {
        (&self.inv_y_res * &self.y_max).to_usize().unwrap()
    }
    fn theta_size(&self) -> usize {
        (&self.inv_angle_res * &N::from_f64(360.0).unwrap())
            .to_usize()
            .unwrap()
    }
}

struct DDTSlice<N> {
    zeroes: Vec<N>,
}
impl<N> DDTSlice<N> {
    pub fn get(&self, x: N) -> Option<N>
    where
        N: PartialOrd + SimdRealField,
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

#[cfg(test)]
mod test {
    use super::*;
    struct No {}

    fn check<N>(cddt: CDDT<N>)
    where
        N: Scalar + num::FromPrimitive + num::ToPrimitive + SimdRealField + PartialOrd,
        <N as SimdValue>::Element: SimdRealField,
        for<'a> &'a N: Mul<N, Output = N>,
    {
    }

    #[test]
    fn test1() {
        let a: CDDT<f64>;
        check(a);
        let a: CDDT<f32>;
        check(a);
        // let a: CDDT<[f32; 4]>;
        // check(a);
        // let a: CDDT<No>;
        // check(a);
    }
}
