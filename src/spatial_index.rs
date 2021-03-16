use num::Float;

use rstar::{primitives::PointWithData, RTree, RTreeNum};

use crate::fmap::{FMap};

/// Spatial Index for storing data associated with some point in 2-D space.
///
/// This is not a full spatial index
pub trait SpatialIndex<K, V> {
    /// Create a new index data structure
    fn new_index(xres: K, yres: K, x: K, y: K, vec: Vec<((K, K), V)>) -> Self;
    /// Get the related point and its associated data
    /// Should only return a point within (xres, yres) of the query point
    fn query_point(&self, x: K, y: K) -> Option<&V>;
}

pub struct FMapSpatialIndex<N, V> {
    inv_xres: N,
    inv_yres: N,
    map: FMap<(i32, i32), V>,
}
impl<N, V> SpatialIndex<N, V> for FMapSpatialIndex<N, V>
where
    N: num::Float,
{
    fn query_point(&self, x: N, y: N) -> Option<&V> {
        self.map.get(&(
            (x * self.inv_xres).round().to_i32().unwrap(),
            (y * self.inv_yres).round().to_i32().unwrap(),
        ))
    }

    fn new_index(_xres: N, _yres: N, _x: N, _y: N, _vec: Vec<((N, N), V)>) -> Self {
        todo!()
    }
}

impl<N, V> SpatialIndex<N, V> for RTree<PointWithData<V, [N; 2]>>
where
    N: RTreeNum,
{
    fn new_index(_xres: N, _yres: N, _x: N, _y: N, _vec: Vec<((N, N), V)>) -> Self {
        todo!()
    }

    fn query_point(&self, _x: N, _y: N) -> Option<&V> {
        todo!()
    }
}
