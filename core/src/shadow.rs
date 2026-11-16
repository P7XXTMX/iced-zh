use crate::{Color, Vector};

/// 阴影。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A shadow.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Shadow {
    /// 阴影的颜色。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The color of the shadow.
    ///
    /// </details>
    pub color: Color,

    /// 阴影的偏移量。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The offset of the shadow.
    ///
    /// </details>
    pub offset: Vector,

    /// 阴影的模糊半径。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The blur radius of the shadow.
    ///
    /// </details>
    pub blur_radius: f32,
}
