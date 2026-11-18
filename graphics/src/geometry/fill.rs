//! 用某种样式填充 [`Geometry`]。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Fill [`Geometry`] with a certain style.
//!
//! </details>
//!
//! [`Geometry`]: super::Renderer::Geometry
pub use crate::geometry::Style;

use crate::core::Color;
use crate::gradient::{self, Gradient};

/// 用于填充几何图形的样式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The style used to fill geometry.
///
/// </details>
#[derive(Debug, Clone, Copy)]
pub struct Fill {
    /// 填充的颜色或渐变。
    ///
    /// 默认设置为 [`Color::BLACK`] 的 [`Style::Solid`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The color or gradient of the fill.
    ///
    /// By default, it is set to [`Style::Solid`] with [`Color::BLACK`].
    ///
    /// </details>
    pub style: Style,

    /// 填充规则定义了如何判断哪些部分位于形状内部、
    /// 哪些部分位于形状外部。
    ///
    /// 详见 [SVG 规范][1]。
    ///
    /// 默认设置为 `NonZero`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The fill rule defines how to determine what is inside and what is
    /// outside of a shape.
    ///
    /// See the [SVG specification][1] for more details.
    ///
    /// By default, it is set to `NonZero`.
    ///
    /// </details>
    ///
    /// [1]: https://www.w3.org/TR/SVG/painting.html#FillRuleProperty
    pub rule: Rule,
}

impl Default for Fill {
    fn default() -> Self {
        Self {
            style: Style::Solid(Color::BLACK),
            rule: Rule::NonZero,
        }
    }
}

impl From<Color> for Fill {
    fn from(color: Color) -> Fill {
        Fill {
            style: Style::Solid(color),
            ..Fill::default()
        }
    }
}

impl From<Gradient> for Fill {
    fn from(gradient: Gradient) -> Self {
        Fill {
            style: Style::Gradient(gradient),
            ..Default::default()
        }
    }
}

impl From<gradient::Linear> for Fill {
    fn from(gradient: gradient::Linear) -> Self {
        Fill {
            style: Style::Gradient(Gradient::Linear(gradient)),
            ..Default::default()
        }
    }
}

/// 填充规则定义了如何判断哪些部分位于形状内部、
/// 哪些部分位于形状外部。
///
/// 参见 [SVG 规范][1]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The fill rule defines how to determine what is inside and what is outside of
/// a shape.
///
/// See the [SVG specification][1].
///
/// </details>
///
/// [1]: https://www.w3.org/TR/SVG/painting.html#FillRuleProperty
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(missing_docs)]
pub enum Rule {
    NonZero,
    EvenOdd,
}
