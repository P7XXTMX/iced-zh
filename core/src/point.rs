use crate::Vector;

use num_traits::{Float, Num};
use std::fmt;

/// 二维的点。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A 2D point.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Point<T = f32> {
    /// X 坐标。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The X coordinate.
    ///
    /// </details>
    pub x: T,

    /// Y 坐标。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The Y coordinate.
    ///
    /// </details>
    pub y: T,
}

impl Point {
    /// 原点（即位于 (0, 0) 的 [`Point`]）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The origin (i.e. a [`Point`] at (0, 0)).
    ///
    /// </details>
    pub const ORIGIN: Self = Self::new(0.0, 0.0);
}

impl<T> Point<T> {
    /// 使用给定的坐标创建一个新的 [`Point`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Point`] with the given coordinates.
    ///
    /// </details>
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }

    /// 计算到另一个 [`Point`] 的距离。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Computes the distance to another [`Point`].
    ///
    /// </details>
    pub fn distance(&self, to: Self) -> T
    where
        T: Float,
    {
        let a = self.x - to.x;
        let b = self.y - to.y;

        a.hypot(b)
    }
}

impl<T> From<[T; 2]> for Point<T>
where
    T: Num,
{
    fn from([x, y]: [T; 2]) -> Self {
        Point { x, y }
    }
}

impl<T> From<(T, T)> for Point<T>
where
    T: Num,
{
    fn from((x, y): (T, T)) -> Self {
        Self { x, y }
    }
}

impl<T> From<Point<T>> for [T; 2] {
    fn from(point: Point<T>) -> [T; 2] {
        [point.x, point.y]
    }
}

impl<T> std::ops::Add<Vector<T>> for Point<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Self;

    fn add(self, vector: Vector<T>) -> Self {
        Self {
            x: self.x + vector.x,
            y: self.y + vector.y,
        }
    }
}

impl<T> std::ops::AddAssign<Vector<T>> for Point<T>
where
    T: std::ops::AddAssign,
{
    fn add_assign(&mut self, vector: Vector<T>) {
        self.x += vector.x;
        self.y += vector.y;
    }
}

impl<T> std::ops::Sub<Vector<T>> for Point<T>
where
    T: std::ops::Sub<Output = T>,
{
    type Output = Self;

    fn sub(self, vector: Vector<T>) -> Self {
        Self {
            x: self.x - vector.x,
            y: self.y - vector.y,
        }
    }
}

impl<T> std::ops::SubAssign<Vector<T>> for Point<T>
where
    T: std::ops::SubAssign,
{
    fn sub_assign(&mut self, vector: Vector<T>) {
        self.x -= vector.x;
        self.y -= vector.y;
    }
}

impl<T> std::ops::Sub<Point<T>> for Point<T>
where
    T: std::ops::Sub<Output = T>,
{
    type Output = Vector<T>;

    fn sub(self, point: Self) -> Vector<T> {
        Vector::new(self.x - point.x, self.y - point.y)
    }
}

impl<T> fmt::Display for Point<T>
where
    T: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point {{ x: {}, y: {} }}", self.x, self.y)
    }
}

impl Point<f32> {
    /// 对 [`Point`] 的坐标进行四舍五入。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Rounds the [`Point`] coordinates.
    ///
    /// </details>
    pub fn round(self) -> Self {
        Point {
            x: self.x.round(),
            y: self.y.round(),
        }
    }

    /// 把 [`Point`] 吸附到 __无符号__ 整数坐标。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Snaps the [`Point`] to __unsigned__ integer coordinates.
    ///
    /// </details>
    pub fn snap(self) -> Point<u32> {
        Point {
            x: self.x.round() as u32,
            y: self.y.round() as u32,
        }
    }
}
