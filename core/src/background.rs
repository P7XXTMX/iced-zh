use crate::Color;
use crate::gradient::{self, Gradient};

/// 某个元素的背景。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The background of some element.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Background {
    /// 纯色。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A solid color.
    ///
    /// </details>
    Color(Color),
    /// 在多种颜色之间线性插值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Linearly interpolate between several colors.
    ///
    /// </details>
    Gradient(Gradient),
    // TODO: Add image variant
    // 待办：添加图像变体
}

impl Background {
    /// 按给定的系数缩放 [`Background`] 的 alpha
    /// 通道。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Scales the alpha channel of the [`Background`] by the given
    /// factor.
    ///
    /// </details>
    pub fn scale_alpha(self, factor: f32) -> Self {
        match self {
            Self::Color(color) => Self::Color(color.scale_alpha(factor)),
            Self::Gradient(gradient) => {
                Self::Gradient(gradient.scale_alpha(factor))
            }
        }
    }
}

impl From<Color> for Background {
    fn from(color: Color) -> Self {
        Background::Color(color)
    }
}

impl From<Gradient> for Background {
    fn from(gradient: Gradient) -> Self {
        Background::Gradient(gradient)
    }
}

impl From<gradient::Linear> for Background {
    fn from(gradient: gradient::Linear) -> Self {
        Background::Gradient(Gradient::Linear(gradient))
    }
}
