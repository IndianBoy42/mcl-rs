use rstar::AABB;
use rstar::{primitives::PointWithData, RTree, RTreeNum};

use crate::fmap::FMap;

/// Spatial Index for storing data associated with some point in 2-D space.
///
/// This is not a full spatial index
pub trait SpatialIndex<N, V> {
    /// Create a new index data structure
    fn new<I>(xres: N, yres: N, x: N, y: N, vec: I) -> Self
    where
        I: IntoIterator<Item = ((N, N), V)>;

    fn build<I>(&mut self, xres: N, yres: N, x: N, y: N, vec: I)
    where
        I: IntoIterator<Item = ((N, N), V)>,
        Self: Sized,
    {
        *self = Self::new(xres, yres, x, y, vec);
    }
    /// Get the related point and its associated data
    /// Should only return a point within (xres, yres) of the query point
    fn query_point(&self, x: N, y: N) -> Option<&V>;
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

    fn new<I>(xres: N, yres: N, x: N, y: N, vec: I) -> Self
    where
        I: IntoIterator<Item = ((N, N), V)>,
    {
        todo!()
    }

    fn build<I>(&mut self, xres: N, yres: N, x: N, y: N, vec: I)
    where
        I: IntoIterator<Item = ((N, N), V)>,
    {
        todo!()
    }
}

pub struct RTreeSpatialIndex<N, V>
where
    N: RTreeNum + num::Float,
{
    rtree: RTree<PointWithData<V, [N; 2]>>,
    xres2: N,
    yres2: N,
}

impl<N, V> SpatialIndex<N, V> for RTreeSpatialIndex<N, V>
where
    N: RTreeNum + num::Float + num::FromPrimitive,
{
    fn new<I>(xres: N, yres: N, x: N, y: N, vec: I) -> Self
    where
        I: IntoIterator<Item = ((N, N), V)>,
    {
        let vec = vec
            .into_iter()
            .map(|((x, y), v)| PointWithData::new(v, [x, y]))
            .collect();
        Self {
            xres2: xres / N::from_f64(2.0).unwrap(),
            yres2: yres / N::from_f64(2.0).unwrap(),
            rtree: RTree::bulk_load(vec),
        }
    }

    fn query_point(&self, x: N, y: N) -> Option<&V> {
        let bbox = AABB::from_corners(
            [x - self.xres2, y - self.yres2],
            [x + self.xres2, y + self.yres2],
        );
        self.rtree.locate_in_envelope(&bbox).next().map(|v| &v.data)
    }
}
