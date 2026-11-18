//! 控制某些内容（比如图像）在一块空间内的旋转。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Control the rotation of some content (like an image) within a space.
//!
//! </details>
use crate::{Degrees, Radians, Size};

/// 旋转内容所用的策略。
///
/// 它用于控制内容被旋转一定角度时
/// 布局的行为。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The strategy used to rotate the content.
///
/// This is used to control the behavior of the layout when the content is rotated
/// by a certain angle.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Rotation {
    /// 元素在旋转时会浮动。布局会保持与旋转之前
    /// 完全一致。
    ///
    /// 这在用于动画时尤其有用，因为在平滑地旋转（例如一个图标）时，
    /// 它可以避免布局发生偏移或尺寸变化。
    ///
    /// 这是默认值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The element will float while rotating. The layout will be kept exactly as it was
    /// before the rotation.
    ///
    /// This is especially useful when used for animations, as it will avoid the
    /// layout being shifted or resized when smoothly i.e. an icon.
    ///
    /// This is the default.
    ///
    /// </details>
    Floating(Radians),
    /// 元素在旋转时是实心的。布局会被调整，以适应
    /// 旋转后的内容。
    ///
    /// 这让你可以旋转图像，并让布局根据图像的新尺寸
    /// 进行调整。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The element will be solid while rotating. The layout will be adjusted to fit
    /// the rotated content.
    ///
    /// This allows you to rotate an image and have the layout adjust to fit the new
    /// size of the image.
    ///
    /// </details>
    Solid(Radians),
}

impl Rotation {
    /// 返回 [`Rotation`] 的角度，以 [`Radians`] 表示。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the angle of the [`Rotation`] in [`Radians`].
    ///
    /// </details>
    pub fn radians(self) -> Radians {
        match self {
            Rotation::Floating(radians) | Rotation::Solid(radians) => radians,
        }
    }

    /// 返回 [`Rotation`] 的角度（以 [`Radians`] 表示）的可变引用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns a mutable reference to the angle of the [`Rotation`] in [`Radians`].
    ///
    /// </details>
    pub fn radians_mut(&mut self) -> &mut Radians {
        match self {
            Rotation::Floating(radians) | Rotation::Solid(radians) => radians,
        }
    }

    /// 返回 [`Rotation`] 的角度，以 [`Degrees`] 表示。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the angle of the [`Rotation`] in [`Degrees`].
    ///
    /// </details>
    pub fn degrees(self) -> Degrees {
        Degrees(self.radians().0.to_degrees())
    }

    /// 把 [`Rotation`] 应用到给定的 [`Size`] 上，返回
    /// 能包含旋转后尺寸的最小 [`Size`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Applies the [`Rotation`] to the given [`Size`], returning
    /// the minimum [`Size`] containing the rotated one.
    ///
    /// </details>
    pub fn apply(self, size: Size) -> Size {
        match self {
            Self::Floating(_) => size,
            Self::Solid(rotation) => size.rotate(rotation),
        }
    }
}

impl Default for Rotation {
    fn default() -> Self {
        Self::Floating(Radians(0.0))
    }
}

impl From<Radians> for Rotation {
    fn from(radians: Radians) -> Self {
        Self::Floating(radians)
    }
}

impl From<f32> for Rotation {
    fn from(radians: f32) -> Self {
        Self::Floating(Radians(radians))
    }
}
