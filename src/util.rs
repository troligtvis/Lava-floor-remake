extern crate nalgebra as na;
use na::{Vector2, Point2, Scalar, Isometry2};

use nphysics2d::algebra::Velocity2;

pub fn point_to_old<N: Copy + Scalar>(point: Point2<N>) -> ggez::nalgebra::Point2<N> {
    ggez::nalgebra::Point2::new(point.x, point.y)
}

pub fn vector_to_old<N: Copy + Scalar>(vector: Vector2<N>) -> ggez::nalgebra::Vector2<N> {
    ggez::nalgebra::Vector2::new(vector.x, vector.y)
}

pub fn isometry_to_point<N: na::RealField + Copy + Scalar>(isometry: Isometry2<N>) -> Point2<N> {
    isometry.translation.vector.into()
}

pub fn point_to_isometry<N: na::RealField + Copy + Scalar>(point: Point2<N>) -> Isometry2<N> {
    Isometry2::translation(point.x, point.y)
}

pub fn point_to_vector<N: Copy + Scalar>(point: Point2<N>) -> Vector2<N> {
    Vector2::new(point.x, point.y)
}

pub fn point_to_velocity<N: na::RealField + Copy + Scalar>(point: Point2<N>) -> Velocity2<N> {
    Velocity2::linear(point.x, point.y)
}

pub fn add<N: std::ops::Add<Output = N> + Copy + Scalar>(
    point1: Point2<N>,
    point2: Point2<N>,
) -> Point2<N> {
    Point2::new(point1.x + point2.x, point1.y + point2.y)
}

pub fn sub<N: std::ops::Sub<Output = N> + Copy + Scalar>(
    point1: Point2<N>,
    point2: Point2<N>,
) -> Point2<N> {
    Point2::new(point1.x - point2.x, point1.y - point2.y)
}