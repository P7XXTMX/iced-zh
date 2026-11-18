//! 对齐并定位部件。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Align and position widgets.
//!
//! </details>

/// 在容器某个轴上的对齐方式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Alignment on the axis of a container.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Alignment {
    /// 对齐到轴的起点。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Align at the start of the axis.
    ///
    /// </details>
    Start,

    /// 对齐到轴的中心。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Align at the center of the axis.
    ///
    /// </details>
    Center,

    /// 对齐到轴的终点。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Align at the end of the axis.
    ///
    /// </details>
    End,
}

impl From<Horizontal> for Alignment {
    fn from(horizontal: Horizontal) -> Self {
        match horizontal {
            Horizontal::Left => Self::Start,
            Horizontal::Center => Self::Center,
            Horizontal::Right => Self::End,
        }
    }
}

impl From<Vertical> for Alignment {
    fn from(vertical: Vertical) -> Self {
        match vertical {
            Vertical::Top => Self::Start,
            Vertical::Center => Self::Center,
            Vertical::Bottom => Self::End,
        }
    }
}

/// 某个资源的水平 [`Alignment`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The horizontal [`Alignment`] of some resource.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Horizontal {
    /// 左对齐
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Align left
    ///
    /// </details>
    Left,

    /// 水平居中
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Horizontally centered
    ///
    /// </details>
    Center,

    /// 右对齐
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Align right
    ///
    /// </details>
    Right,
}

impl From<Alignment> for Horizontal {
    fn from(alignment: Alignment) -> Self {
        match alignment {
            Alignment::Start => Self::Left,
            Alignment::Center => Self::Center,
            Alignment::End => Self::Right,
        }
    }
}

/// 某个资源的垂直 [`Alignment`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The vertical [`Alignment`] of some resource.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Vertical {
    /// 顶部对齐
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Align top
    ///
    /// </details>
    Top,

    /// 垂直居中
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Vertically centered
    ///
    /// </details>
    Center,

    /// 底部对齐
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Align bottom
    ///
    /// </details>
    Bottom,
}

impl From<Alignment> for Vertical {
    fn from(alignment: Alignment) -> Self {
        match alignment {
            Alignment::Start => Self::Top,
            Alignment::Center => Self::Center,
            Alignment::End => Self::Bottom,
        }
    }
}
