use std::marker::PhantomData;

use nalgebra::Scalar;
use rstar::primitives::PointWithData;
use rstar::{RTree, RTreeNum};

use crate::{GridMap, GridMapView, Pose};

pub trait Raycaster<N: Scalar> {
    fn build(grid_res: N, angle_res: N, map: GridMapView) -> Self;
    fn get(&self, pose: Pose<N>) -> Option<N>;
}

pub struct OccupancyGridWrapper<N: Scalar> {
    map: GridMap,
    grid_res: N,
    angle_res: N,
}

impl<N: Scalar> OccupancyGridWrapper<N> {
    fn new(grid_res: N, angle_res: N, map: GridMap) -> Self {
        Self {
            map,
            grid_res,
            angle_res,
        }
    }

    fn get_pt(&self, pose: crate::Pose<N>) -> Option<i8>
    where
        N: num::Float,
    {
        let xn = (pose.translation.vector.x / self.grid_res)
            .round()
            .to_usize()
            .unwrap();
        let yn = (pose.translation.vector.y / self.grid_res)
            .round()
            .to_usize()
            .unwrap();
        self.map.get((xn, yn)).copied()
    }
}
pub struct RayMarchingCaster<N: Scalar>(OccupancyGridWrapper<N>);
pub struct BresenhamsLineCaster<N: Scalar>(OccupancyGridWrapper<N>);

impl<N: Scalar> Raycaster<N> for RayMarchingCaster<N> {
    fn build(grid_res: N, angle_res: N, map: GridMapView) -> Self {
        Self(OccupancyGridWrapper::new(grid_res, angle_res, map.into()))
    }

    fn get(&self, pose: crate::Pose<N>) -> Option<N> {
        let Self(occ) = self;
        todo!()
    }
}

impl<N: Scalar> Raycaster<N> for BresenhamsLineCaster<N> {
    fn build(grid_res: N, angle_res: N, map: GridMapView) -> Self {
        Self(OccupancyGridWrapper::new(grid_res, angle_res, map.into()))
    }

    fn get(&self, pose: crate::Pose<N>) -> Option<N> {
        let Self(occ) = self;
        todo!()
    }
}

pub struct LUTRayCaster<N, C>(RTree<PointWithData<N, [N; 3]>>, PhantomData<C>)
where
    N: Scalar + RTreeNum,
    C: Raycaster<N>;
