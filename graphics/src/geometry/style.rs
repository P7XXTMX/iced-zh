use crate::core::Color;
use crate::geometry::Gradient;

/// 某个绘制内容的着色样式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The coloring style of some drawing.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Style {
    /// 纯色 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A solid [`Color`].
    ///
    /// </details>
    Solid(Color),

    /// [`Gradient`] 渐变色。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A [`Gradient`] color.
    ///
    /// </details>
    Gradient(Gradient),
}

impl From<Color> for Style {
    fn from(color: Color) -> Self {
        Self::Solid(color)
    }
}

impl From<Gradient> for Style {
    fn from(gradient: Gradient) -> Self {
        Self::Gradient(gradient)
    }
}
