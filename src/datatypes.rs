use nalgebra;

pub type Pose<F> = nalgebra::geometry::Isometry2<F>;
pub type PoseCovariance<F> = nalgebra::base::Matrix3<F>;
pub type Point<F> = nalgebra::geometry::Point2<F>;
pub type PointCovariance<F> = nalgebra::base::Matrix2<F>;

pub type Map<F> = nalgebra::DMatrix<F>;
pub type MapView<'a, F> = nalgebra::DMatrixSlice<'a, F>;
pub type MapViewMut<'a, F> = nalgebra::DMatrixSliceMut<'a, F>;

pub type PointCloud<F> = nalgebra::Matrix2xX<F>;
