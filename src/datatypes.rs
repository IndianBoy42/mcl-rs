use nalgebra;

pub type Pose<F> = nalgebra::geometry::Isometry2<F>;
pub type PoseCovariance<F> = nalgebra::base::Matrix3<F>;
pub type Point<F> = nalgebra::geometry::Point2<F>;
pub type PointCovariance<F> = nalgebra::base::Matrix2<F>;

const GRID_OCCU: i8 = 1;
const GRID_FREE: i8 = -1;
const GRID_NONE: i8 = 0;
#[inline(always)]
pub const fn grid_occupied(x: i8) -> bool {
    x > GRID_NONE
}
#[inline(always)]
pub const fn grid_mapped(x: i8) -> bool {
    x != GRID_NONE
}
#[inline(always)]
pub const fn grid_freecell(x: i8) -> bool {
    x < GRID_NONE
}

pub type GridMap = nalgebra::DMatrix<i8>;
pub type GridMapView<'a> = nalgebra::DMatrixSlice<'a, i8>;
pub type GridMapViewMut<'a> = nalgebra::DMatrixSliceMut<'a, i8>;

pub type PointCloud<F> = nalgebra::Matrix2xX<F>;
