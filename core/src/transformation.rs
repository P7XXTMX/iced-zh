use crate::{Point, Rectangle, Size, Vector};

use glam::{Mat4, Vec3, Vec4};
use std::ops::Mul;

/// 二维变换矩阵。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A 2D transformation matrix.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transformation(Mat4);

impl Transformation {
    /// 让被变换的对象保持不变的 [`Transformation`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A [`Transformation`] that preserves whatever is transformed.
    ///
    /// </details>
    pub const IDENTITY: Self = Self(Mat4::IDENTITY);

    /// 创建一个正交投影。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates an orthographic projection.
    ///
    /// </details>
    #[rustfmt::skip]
    pub fn orthographic(width: u32, height: u32) -> Self{
        Self(Mat4::orthographic_rh_gl(
            0.0, width as f32,
            height as f32, 0.0,
            -1.0, 1.0
        ))
    }

    /// 创建一个平移变换。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a translate transformation.
    ///
    /// </details>
    pub fn translate(x: f32, y: f32) -> Self {
        Self(Mat4::from_translation(Vec3::new(x, y, 0.0)))
    }

    /// 创建一个等比缩放变换。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a uniform scaling transformation.
    ///
    /// </details>
    pub fn scale(scaling: f32) -> Self {
        Self(Mat4::from_scale(Vec3::new(scaling, scaling, 1.0)))
    }

    /// 返回 [`Transformation`] 的逆变换。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the inverse of the [`Transformation`].
    ///
    /// </details>
    pub fn inverse(self) -> Self {
        Self(self.0.inverse())
    }

    /// 返回 [`Transformation`] 的缩放因子。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the scale factor of the [`Transformation`].
    ///
    /// </details>
    pub fn scale_factor(&self) -> f32 {
        self.0.x_axis.x
    }

    /// 返回 [`Transformation`] 的平移量。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the translation of the [`Transformation`].
    ///
    /// </details>
    pub fn translation(&self) -> Vector {
        Vector::new(self.0.w_axis.x, self.0.w_axis.y)
    }
}

impl Default for Transformation {
    fn default() -> Self {
        Transformation::IDENTITY
    }
}

impl Mul for Transformation {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        Self(self.0 * rhs.0)
    }
}

impl Mul<Transformation> for Point {
    type Output = Self;

    fn mul(self, transformation: Transformation) -> Self {
        let point = transformation
            .0
            .mul_vec4(Vec4::new(self.x, self.y, 1.0, 1.0));

        Point::new(point.x, point.y)
    }
}

impl Mul<Transformation> for Vector {
    type Output = Self;

    fn mul(self, transformation: Transformation) -> Self {
        let new_vector = transformation
            .0
            .mul_vec4(Vec4::new(self.x, self.y, 1.0, 0.0));

        Vector::new(new_vector.x, new_vector.y)
    }
}

impl Mul<Transformation> for Size {
    type Output = Self;

    fn mul(self, transformation: Transformation) -> Self {
        let new_size = transformation.0.mul_vec4(Vec4::new(
            self.width,
            self.height,
            1.0,
            0.0,
        ));

        Size::new(new_size.x, new_size.y)
    }
}

impl Mul<Transformation> for Rectangle {
    type Output = Self;

    fn mul(self, transformation: Transformation) -> Self {
        let position = self.position();
        let size = self.size();

        Self::new(position * transformation, size * transformation)
    }
}

impl AsRef<[f32; 16]> for Transformation {
    fn as_ref(&self) -> &[f32; 16] {
        self.0.as_ref()
    }
}

impl From<Transformation> for [f32; 16] {
    fn from(t: Transformation) -> [f32; 16] {
        *t.as_ref()
    }
}

impl From<Transformation> for Mat4 {
    fn from(transformation: Transformation) -> Self {
        transformation.0
    }
}
