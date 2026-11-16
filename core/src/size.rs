use crate::{Alignment, Radians, Vector};

/// 二维空间的大小。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An amount of space in 2 dimensions.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub struct Size<T = f32> {
    /// 宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The width.
    ///
    /// </details>
    pub width: T,
    /// 高度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The height.
    ///
    /// </details>
    pub height: T,
}

impl<T> Size<T> {
    /// 使用给定的宽度和高度创建一个新的 [`Size`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new  [`Size`] with the given width and height.
    ///
    /// </details>
    pub const fn new(width: T, height: T) -> Self {
        Size { width, height }
    }
}

impl Size {
    /// 宽度和高度均为零的 [`Size`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A [`Size`] with zero width and height.
    ///
    /// </details>
    pub const ZERO: Size = Size::new(0., 0.);

    /// 宽度和高度均为 1 个单位的 [`Size`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A [`Size`] with a width and height of 1 unit.
    ///
    /// </details>
    pub const UNIT: Size = Size::new(1., 1.);

    /// 宽度和高度均为无穷大的 [`Size`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A [`Size`] with infinite width and height.
    ///
    /// </details>
    pub const INFINITE: Size = Size::new(f32::INFINITY, f32::INFINITY);

    /// 返回此尺寸与另一个尺寸各分量的最小值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the minimum of each component of this size and another.
    ///
    /// </details>
    pub fn min(self, other: Self) -> Self {
        Size {
            width: self.width.min(other.width),
            height: self.height.min(other.height),
        }
    }

    /// 返回此尺寸与另一个尺寸各分量的最大值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the maximum of each component of this size and another.
    ///
    /// </details>
    pub fn max(self, other: Self) -> Self {
        Size {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }

    /// 把此 [`Size`] 扩大给定的量。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Expands this [`Size`] by the given amount.
    ///
    /// </details>
    pub fn expand(self, other: impl Into<Size>) -> Self {
        let other = other.into();

        Size {
            width: self.width + other.width,
            height: self.height + other.height,
        }
    }

    /// 旋转此 [`Size`]，并返回包含它的
    /// 最小 [`Size`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Rotates this [`Size`] and returns the minimum [`Size`]
    /// containing it.
    ///
    /// </details>
    pub fn rotate(self, rotation: Radians) -> Size {
        let radians = f32::from(rotation);

        Size {
            width: (self.width * radians.cos()).abs() + (self.height * radians.sin()).abs(),
            height: (self.width * radians.sin()).abs() + (self.height * radians.cos()).abs(),
        }
    }

    /// 在不超出其边界的前提下，对此 [`Size`]
    /// 应用一个宽高比。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Applies an aspect ratio to this [`Size`] without
    /// exceeding its bounds.
    ///
    /// </details>
    pub const fn ratio(self, aspect_ratio: f32) -> Size {
        Size {
            width: (self.height * aspect_ratio).min(self.width),
            height: (self.width / aspect_ratio).min(self.height),
        }
    }

    /// 返回使用给定的对齐方式把此 [`Size`] 在给定的
    /// `container` 内对齐所需的偏移量。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the offset needed to align this [`Size`] within the given
    /// `container` using the provided alignments.
    ///
    /// </details>
    pub fn align(self, container: Size, align_x: Alignment, align_y: Alignment) -> Vector {
        let x = match align_x {
            Alignment::Start => 0.0,
            Alignment::Center => (container.width - self.width) / 2.0,
            Alignment::End => container.width - self.width,
        };

        let y = match align_y {
            Alignment::Start => 0.0,
            Alignment::Center => (container.height - self.height) / 2.0,
            Alignment::End => container.height - self.height,
        };

        Vector { x, y }
    }
}

impl<T> From<[T; 2]> for Size<T> {
    fn from([width, height]: [T; 2]) -> Self {
        Size { width, height }
    }
}

impl<T> From<(T, T)> for Size<T> {
    fn from((width, height): (T, T)) -> Self {
        Self { width, height }
    }
}

impl From<(u32, u32)> for Size {
    fn from((width, height): (u32, u32)) -> Self {
        Size::new(width as f32, height as f32)
    }
}

impl<T> From<Vector<T>> for Size<T> {
    fn from(vector: Vector<T>) -> Self {
        Size {
            width: vector.x,
            height: vector.y,
        }
    }
}

impl<T> From<Size<T>> for [T; 2] {
    fn from(size: Size<T>) -> Self {
        [size.width, size.height]
    }
}

impl<T> From<Size<T>> for Vector<T> {
    fn from(size: Size<T>) -> Self {
        Vector::new(size.width, size.height)
    }
}

impl<T> std::ops::Add for Size<T>
where
    T: std::ops::Add<Output = T>,
{
    type Output = Size<T>;

    fn add(self, rhs: Self) -> Self::Output {
        Size {
            width: self.width + rhs.width,
            height: self.height + rhs.height,
        }
    }
}

impl<T> std::ops::Sub for Size<T>
where
    T: std::ops::Sub<Output = T>,
{
    type Output = Size<T>;

    fn sub(self, rhs: Self) -> Self::Output {
        Size {
            width: self.width - rhs.width,
            height: self.height - rhs.height,
        }
    }
}

impl<T> std::ops::Mul<T> for Size<T>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    type Output = Size<T>;

    fn mul(self, rhs: T) -> Self::Output {
        Size {
            width: self.width * rhs,
            height: self.height * rhs,
        }
    }
}

impl<T> std::ops::Div<T> for Size<T>
where
    T: std::ops::Div<Output = T> + Copy,
{
    type Output = Size<T>;

    fn div(self, rhs: T) -> Self::Output {
        Size {
            width: self.width / rhs,
            height: self.height / rhs,
        }
    }
}

impl<T> std::ops::Mul<Vector<T>> for Size<T>
where
    T: std::ops::Mul<Output = T> + Copy,
{
    type Output = Size<T>;

    fn mul(self, scale: Vector<T>) -> Self::Output {
        Size {
            width: self.width * scale.x,
            height: self.height * scale.y,
        }
    }
}
