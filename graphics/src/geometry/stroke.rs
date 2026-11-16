//! 根据 [`Path`] 创建线条，并为其指定各种属性/样式。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Create lines from a [`Path`] and assigns them various attributes/styles.
//!
//! </details>
//!
//! [`Path`]: super::Path
pub use crate::geometry::Style;

use iced_core::Color;

/// 描边的样式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The style of a stroke.
///
/// </details>
#[derive(Debug, Clone, Copy)]
pub struct Stroke<'a> {
    /// 描边的颜色或渐变。
    ///
    /// 默认设置为 [`Color::BLACK`] 的 [`Style::Solid`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The color or gradient of the stroke.
    ///
    /// By default, it is set to a [`Style::Solid`] with [`Color::BLACK`].
    ///
    /// </details>
    pub style: Style,
    /// 描边两条边缘之间的距离。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The distance between the two edges of the stroke.
    ///
    /// </details>
    pub width: f32,
    /// 对开放子路径描边时，其端点处使用的形状。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The shape to be used at the end of open subpaths when they are stroked.
    ///
    /// </details>
    pub line_cap: LineCap,
    /// 对路径或基本形状描边时，其拐角处
    /// 使用的形状。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The shape to be used at the corners of paths or basic shapes when they
    /// are stroked.
    ///
    /// </details>
    pub line_join: LineJoin,
    /// 对线条描边时使用的虚线样式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The dash pattern used when stroking the line.
    ///
    /// </details>
    pub line_dash: LineDash<'a>,
}

impl Stroke<'_> {
    /// 设置 [`Stroke`] 的颜色。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the color of the [`Stroke`].
    ///
    /// </details>
    pub fn with_color(self, color: Color) -> Self {
        Stroke {
            style: Style::Solid(color),
            ..self
        }
    }

    /// 设置 [`Stroke`] 的宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the width of the [`Stroke`].
    ///
    /// </details>
    pub fn with_width(self, width: f32) -> Self {
        Stroke { width, ..self }
    }

    /// 设置 [`Stroke`] 的 [`LineCap`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`LineCap`] of the [`Stroke`].
    ///
    /// </details>
    pub fn with_line_cap(self, line_cap: LineCap) -> Self {
        Stroke { line_cap, ..self }
    }

    /// 设置 [`Stroke`] 的 [`LineJoin`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`LineJoin`] of the [`Stroke`].
    ///
    /// </details>
    pub fn with_line_join(self, line_join: LineJoin) -> Self {
        Stroke { line_join, ..self }
    }
}

impl Default for Stroke<'_> {
    fn default() -> Self {
        Stroke {
            style: Style::Solid(Color::BLACK),
            width: 1.0,
            line_cap: LineCap::default(),
            line_join: LineJoin::default(),
            line_dash: LineDash::default(),
        }
    }
}

/// 对开放子路径描边时，其端点处使用的形状。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The shape used at the end of open subpaths when they are stroked.
///
/// </details>
#[derive(Debug, Clone, Copy, Default)]
pub enum LineCap {
    /// 每条子路径的描边都不会超出它的两个端点。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The stroke for each sub-path does not extend beyond its two endpoints.
    ///
    /// </details>
    #[default]
    Butt,
    /// 在每条子路径的末端，表示描边的形状会
    /// 延伸出一个方形。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// At the end of each sub-path, the shape representing the stroke will be
    /// extended by a square.
    ///
    /// </details>
    Square,
    /// 在每条子路径的末端，表示描边的形状会
    /// 延伸出一个半圆。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// At the end of each sub-path, the shape representing the stroke will be
    /// extended by a semicircle.
    ///
    /// </details>
    Round,
}

/// 对路径或基本形状描边时，其拐角处使用的
/// 形状。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The shape used at the corners of paths or basic shapes when they are
/// stroked.
///
/// </details>
#[derive(Debug, Clone, Copy, Default)]
pub enum LineJoin {
    /// 尖角。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A sharp corner.
    ///
    /// </details>
    #[default]
    Miter,
    /// 圆角。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A round corner.
    ///
    /// </details>
    Round,
    /// 斜角。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A bevelled corner.
    ///
    /// </details>
    Bevel,
}

/// 对线条描边时使用的虚线样式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The dash pattern used when stroking the line.
///
/// </details>
#[derive(Debug, Clone, Copy, Default)]
pub struct LineDash<'a> {
    /// 线段与间隙交替出现的长度，用于描述该样式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The alternating lengths of lines and gaps which describe the pattern.
    ///
    /// </details>
    pub segments: &'a [f32],

    /// 样式在 [`LineDash::segments`] 中开始的偏移量。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The offset of [`LineDash::segments`] to start the pattern.
    ///
    /// </details>
    pub offset: usize,
}
