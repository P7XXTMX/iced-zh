//! 为着色器管理颜色。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Manage colors for shaders.
//!
//! </details>
use crate::core::Color;

use bytemuck::{Pod, Zeroable};

/// 打包为 4 个浮点数的颜色，分别表示 RGBA 通道。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A color packed as 4 floats representing RGBA channels.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Zeroable, Pod)]
#[repr(C)]
pub struct Packed([f32; 4]);

impl Packed {
    /// 返回 [`Packed`] 颜色的内部分量。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the internal components of the [`Packed`] color.
    ///
    /// </details>
    pub fn components(self) -> [f32; 4] {
        self.0
    }
}

/// 指示渲染器是否应当执行伽马校正的标志。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A flag that indicates whether the renderer should perform gamma correction.
///
/// </details>
pub const GAMMA_CORRECTION: bool = internal::GAMMA_CORRECTION;

/// 打包一个 [`Color`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Packs a [`Color`].
///
/// </details>
pub fn pack(color: impl Into<Color>) -> Packed {
    Packed(internal::pack(color.into()))
}

#[cfg(not(feature = "web-colors"))]
mod internal {
    use crate::core::Color;

    pub const GAMMA_CORRECTION: bool = true;

    pub fn pack(color: Color) -> [f32; 4] {
        color.into_linear()
    }
}

#[cfg(feature = "web-colors")]
mod internal {
    use crate::core::Color;

    pub const GAMMA_CORRECTION: bool = false;

    pub fn pack(color: Color) -> [f32; 4] {
        [color.r, color.g, color.b, color.a]
    }
}
