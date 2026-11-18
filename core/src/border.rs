//! 在容器周围绘制线条。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Draw lines around containers.
//!
//! </details>
use crate::{Color, Pixels};

/// 边框。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A border.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Border {
    /// 边框的颜色。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The color of the border.
    ///
    /// </details>
    pub color: Color,

    /// 边框的宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The width of the border.
    ///
    /// </details>
    pub width: f32,

    /// 边框的 [`Radius`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Radius`] of the border.
    ///
    /// </details>
    pub radius: Radius,
}

/// 使用给定的 [`Radius`] 创建一个新的 [`Border`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Border`] with the given [`Radius`].
///
/// </details>
///
/// ```
/// # use iced_core::border::{self, Border};
/// #
/// assert_eq!(border::rounded(10), Border::default().rounded(10));
/// ```
pub fn rounded(radius: impl Into<Radius>) -> Border {
    Border::default().rounded(radius)
}

/// 使用给定的 [`Color`] 创建一个新的 [`Border`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Border`] with the given [`Color`].
///
/// </details>
///
/// ```
/// # use iced_core::border::{self, Border};
/// # use iced_core::Color;
/// #
/// assert_eq!(border::color(Color::BLACK), Border::default().color(Color::BLACK));
/// ```
pub fn color(color: impl Into<Color>) -> Border {
    Border::default().color(color)
}

/// 使用给定的 `width` 创建一个新的 [`Border`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Border`] with the given `width`.
///
/// </details>
///
/// ```
/// # use iced_core::border::{self, Border};
/// # use iced_core::Color;
/// #
/// assert_eq!(border::width(10), Border::default().width(10));
/// ```
pub fn width(width: impl Into<Pixels>) -> Border {
    Border::default().width(width)
}

impl Border {
    /// 设置 [`Border`] 的 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Color`] of the [`Border`].
    ///
    /// </details>
    pub fn color(self, color: impl Into<Color>) -> Self {
        Self {
            color: color.into(),
            ..self
        }
    }

    /// 设置 [`Border`] 的 [`Radius`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Radius`] of the [`Border`].
    ///
    /// </details>
    pub fn rounded(self, radius: impl Into<Radius>) -> Self {
        Self {
            radius: radius.into(),
            ..self
        }
    }

    /// 设置 [`Border`] 的宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the width of the [`Border`].
    ///
    /// </details>
    pub fn width(self, width: impl Into<Pixels>) -> Self {
        Self {
            width: width.into().0,
            ..self
        }
    }
}

/// 图元各个角的边框半径，顺序为：
/// 左上、右上、右下、左下。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The border radii for the corners of a graphics primitive in the order:
/// top-left, top-right, bottom-right, bottom-left.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Radius {
    /// 左上角半径
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Top left radius
    ///
    /// </details>
    pub top_left: f32,
    /// 右上角半径
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Top right radius
    ///
    /// </details>
    pub top_right: f32,
    /// 右下角半径
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Bottom right radius
    ///
    /// </details>
    pub bottom_right: f32,
    /// 左下角半径
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Bottom left radius
    ///
    /// </details>
    pub bottom_left: f32,
}

/// 创建一个新的 [`Radius`]，每个角使用相同的值。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Radius`] with the same value for each corner.
///
/// </details>
pub fn radius(value: impl Into<Pixels>) -> Radius {
    Radius::new(value)
}

/// 使用给定的左上角值创建一个新的 [`Radius`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Radius`] with the given top left value.
///
/// </details>
pub fn top_left(value: impl Into<Pixels>) -> Radius {
    Radius::default().top_left(value)
}

/// 使用给定的右上角值创建一个新的 [`Radius`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Radius`] with the given top right value.
///
/// </details>
pub fn top_right(value: impl Into<Pixels>) -> Radius {
    Radius::default().top_right(value)
}

/// 使用给定的右下角值创建一个新的 [`Radius`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Radius`] with the given bottom right value.
///
/// </details>
pub fn bottom_right(value: impl Into<Pixels>) -> Radius {
    Radius::default().bottom_right(value)
}

/// 使用给定的左下角值创建一个新的 [`Radius`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Radius`] with the given bottom left value.
///
/// </details>
pub fn bottom_left(value: impl Into<Pixels>) -> Radius {
    Radius::default().bottom_left(value)
}

/// 创建一个新的 [`Radius`]，把给定的值用作左上角和右上角。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Radius`] with the given value as top left and top right.
///
/// </details>
pub fn top(value: impl Into<Pixels>) -> Radius {
    Radius::default().top(value)
}

/// 创建一个新的 [`Radius`]，把给定的值用作左下角和右下角。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Radius`] with the given value as bottom left and bottom right.
///
/// </details>
pub fn bottom(value: impl Into<Pixels>) -> Radius {
    Radius::default().bottom(value)
}

/// 创建一个新的 [`Radius`]，把给定的值用作左上角和左下角。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Radius`] with the given value as top left and bottom left.
///
/// </details>
pub fn left(value: impl Into<Pixels>) -> Radius {
    Radius::default().left(value)
}

/// 创建一个新的 [`Radius`]，把给定的值用作右上角和右下角。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a new [`Radius`] with the given value as top right and bottom right.
///
/// </details>
pub fn right(value: impl Into<Pixels>) -> Radius {
    Radius::default().right(value)
}

impl Radius {
    /// 创建一个新的 [`Radius`]，每个角使用相同的值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Radius`] with the same value for each corner.
    ///
    /// </details>
    pub fn new(value: impl Into<Pixels>) -> Self {
        let value = value.into().0;

        Self {
            top_left: value,
            top_right: value,
            bottom_right: value,
            bottom_left: value,
        }
    }

    /// 设置 [`Radius`] 的左上角值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the top left value of the [`Radius`].
    ///
    /// </details>
    pub fn top_left(self, value: impl Into<Pixels>) -> Self {
        Self {
            top_left: value.into().0,
            ..self
        }
    }

    /// 设置 [`Radius`] 的右上角值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the top right value of the [`Radius`].
    ///
    /// </details>
    pub fn top_right(self, value: impl Into<Pixels>) -> Self {
        Self {
            top_right: value.into().0,
            ..self
        }
    }

    /// 设置 [`Radius`] 的右下角值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the bottom right value of the [`Radius`].
    ///
    /// </details>
    pub fn bottom_right(self, value: impl Into<Pixels>) -> Self {
        Self {
            bottom_right: value.into().0,
            ..self
        }
    }

    /// 设置 [`Radius`] 的左下角值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the bottom left value of the [`Radius`].
    ///
    /// </details>
    pub fn bottom_left(self, value: impl Into<Pixels>) -> Self {
        Self {
            bottom_left: value.into().0,
            ..self
        }
    }

    /// 设置 [`Radius`] 的左上角和右上角值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the top left and top right values of the [`Radius`].
    ///
    /// </details>
    pub fn top(self, value: impl Into<Pixels>) -> Self {
        let value = value.into().0;

        Self {
            top_left: value,
            top_right: value,
            ..self
        }
    }

    /// 设置 [`Radius`] 的左下角和右下角值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the bottom left and bottom right values of the [`Radius`].
    ///
    /// </details>
    pub fn bottom(self, value: impl Into<Pixels>) -> Self {
        let value = value.into().0;

        Self {
            bottom_left: value,
            bottom_right: value,
            ..self
        }
    }

    /// 设置 [`Radius`] 的左上角和左下角值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the top left and bottom left values of the [`Radius`].
    ///
    /// </details>
    pub fn left(self, value: impl Into<Pixels>) -> Self {
        let value = value.into().0;

        Self {
            top_left: value,
            bottom_left: value,
            ..self
        }
    }

    /// 设置 [`Radius`] 的右上角和右下角值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the top right and bottom right values of the [`Radius`].
    ///
    /// </details>
    pub fn right(self, value: impl Into<Pixels>) -> Self {
        let value = value.into().0;

        Self {
            top_right: value,
            bottom_right: value,
            ..self
        }
    }
}

impl From<f32> for Radius {
    fn from(radius: f32) -> Self {
        Self {
            top_left: radius,
            top_right: radius,
            bottom_right: radius,
            bottom_left: radius,
        }
    }
}

impl From<u8> for Radius {
    fn from(w: u8) -> Self {
        Self::from(f32::from(w))
    }
}

impl From<u32> for Radius {
    fn from(w: u32) -> Self {
        Self::from(w as f32)
    }
}

impl From<i32> for Radius {
    fn from(w: i32) -> Self {
        Self::from(w as f32)
    }
}

impl From<Radius> for [f32; 4] {
    fn from(radi: Radius) -> Self {
        [
            radi.top_left,
            radi.top_right,
            radi.bottom_right,
            radi.bottom_left,
        ]
    }
}

impl std::ops::Mul<f32> for Radius {
    type Output = Self;

    fn mul(self, scale: f32) -> Self::Output {
        Self {
            top_left: self.top_left * scale,
            top_right: self.top_right * scale,
            bottom_right: self.bottom_right * scale,
            bottom_left: self.bottom_left * scale,
        }
    }
}
