use nalgebra;

pub type Pose<F> = nalgebra::geometry::Isometry2<F>;
pub type PoseCovariance<F> = nalgebra::base::Matrix3<F>;
pub type Point<F> = nalgebra::geometry::Point2<F>;
pub type PointCovariance<F> = nalgebra::base::Matrix2<F>;

pub type GridMap = nalgebra::DMatrix<i8>;
pub type GridMapView<'a> = nalgebra::DMatrixSlice<'a, i8>;
pub type GridMapViewMut<'a> = nalgebra::DMatrixSliceMut<'a, i8>;

pub type PointCloud<F> = nalgebra::Matrix2xX<F>;
