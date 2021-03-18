use super::raycast::Raycaster;
use super::spatial_index::SpatialIndex;
use crate::{GridMapView, Pose};
use nalgebra::{Scalar, SimdRealField, SimdValue};
use std::ops::{Add, Mul};
pub use vecsi::VecSpatialIndexForCDDT;
type DefaultSpatialIndexForCDDT<N> = VecSpatialIndexForCDDT<N, DDTSlice<N>>;

pub struct CDDT<N, SI: SpatialIndex<N, DDTSlice<N>> = DefaultSpatialIndexForCDDT<N>> {
    inv_angle_res: N,
    inv_y_res: N,
    y_size: usize,
    lut: SI,
}

impl<N, SI: SpatialIndex<N, DDTSlice<N>>> Raycaster<N> for CDDT<N, SI>
where
    N: Scalar + num::FromPrimitive + num::ToPrimitive + SimdRealField + PartialOrd,
    <N as SimdValue>::Element: SimdRealField,
    for<'a> &'a N: Mul<Output = N>,
{
    fn build(&mut self, _grid_res: N, _angle_res: N, _map: GridMapView) {
        // inv_y_res = N::one() / grid_res;
        // self.y_size = map.nrows();
        // self.lut.clear();
        // self.lut.reserve(self.theta_size() * self.y_size);
        todo!()
    }

    fn get(&self, pose: Pose<N>) -> Option<N> {
        let ang = pose.rotation.angle();
        let rot = pose.rotation;
        let idx = rot * pose.translation.vector;
        // let idx = pose.translation * &rot;
        self.lut.query_point(ang, idx.y)?.get(idx.x)
        // let yn = (idx.y * self.inv_y_res).to_usize().unwrap();
        // let thn = (ang * self.inv_angle_res).to_usize().unwrap();
    }

    fn new() -> Self {
        todo!()
    }
}

impl<N, SI: SpatialIndex<N, DDTSlice<N>>> CDDT<N, SI>
where
    N: num::FromPrimitive + num::ToPrimitive + Add<N, Output = N>,
    for<'a> &'a N: Mul<Output = N>,
{
    fn theta_size(&self) -> usize {
        (&self.inv_angle_res * &N::from_f64(360.0).unwrap())
            .to_usize()
            .unwrap()
    }
}

pub struct DDTSlice<N> {
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
            Ok(_exact) => N::zero(),
            Err(insertat) => {
                let &ub = self.zeroes.get(insertat)?;
                ub - x
            }
        };
        Some(res)
    }
}

mod vecsi {
    use crate::sensor_models::utils::spatial_index::SpatialIndex;

    /// Makes assumptions on the points involved, making it only suitable for CDDT
    pub struct VecSpatialIndexForCDDT<N, V> {
        inv_th_res: N,
        inv_y_res: N,
        y_size: usize,
        map: Vec<V>,
    }
    impl<N, V> VecSpatialIndexForCDDT<N, V>
    where
        N: num::Float,
    {
        fn discretize(&self, th: N, y: N) -> (usize, usize) {
            let yoffs = y * th.sin();
            let thn = (th * self.inv_th_res).to_usize().unwrap();
            let yn = ((y + yoffs) * self.inv_y_res).to_usize().unwrap();

            (thn, yn)
        }
    }

    impl<N, V> SpatialIndex<N, V> for VecSpatialIndexForCDDT<N, V>
    where
        N: num::Float,
    {
        #[allow(unreachable_code)]
        fn new<I>(thres: N, yres: N, th: N, y: N, vec: I) -> Self
        where
            I: IntoIterator<Item = ((N, N), V)>,
        {
            Self {
                inv_th_res: N::one() / thres,
                inv_y_res: N::one() / yres,
                y_size: (y / yres).to_usize().unwrap(),
                map: todo!(),
            }
        }

        fn query_point(&self, th: N, y: N) -> Option<&V> {
            let (thn, yn) = self.discretize(th, y);
            let i = thn * self.y_size + yn;
            self.map.get(i)
        }

        fn build<I>(&mut self, xres: N, yres: N, x: N, y: N, vec: I)
        where
            I: IntoIterator<Item = ((N, N), V)>,
        {
            todo!()
        }
    }
}

#[cfg(test)]
mod test {

    #[test]
    fn test1() {}
}
