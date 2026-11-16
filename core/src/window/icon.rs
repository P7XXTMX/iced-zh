//! 更改窗口的图标。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Change the icon of a window.
//!
//! </details>
use crate::Size;

use std::mem;

/// 根据 `sRGB` 色彩空间中的 RGBA 像素构建一个 [`Icon`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Builds an  [`Icon`] from its RGBA pixels in the `sRGB` color space.
///
/// </details>
pub fn from_rgba(rgba: Vec<u8>, width: u32, height: u32) -> Result<Icon, Error> {
    const PIXEL_SIZE: usize = mem::size_of::<u8>() * 4;

    if !rgba.len().is_multiple_of(PIXEL_SIZE) {
        return Err(Error::ByteCountNotDivisibleBy4 {
            byte_count: rgba.len(),
        });
    }

    let pixel_count = rgba.len() / PIXEL_SIZE;

    if pixel_count != (width * height) as usize {
        return Err(Error::DimensionsVsPixelCount {
            width,
            height,
            width_x_height: (width * height) as usize,
            pixel_count,
        });
    }

    Ok(Icon {
        rgba,
        size: Size::new(width, height),
    })
}

/// 窗口图标，通常用于标题栏或任务栏。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An window icon normally used for the titlebar or taskbar.
///
/// </details>
#[derive(Clone)]
pub struct Icon {
    rgba: Vec<u8>,
    size: Size<u32>,
}

impl Icon {
    /// 返回 [`Icon`] 的原始数据。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the raw data of the [`Icon`].
    ///
    /// </details>
    pub fn into_raw(self) -> (Vec<u8>, Size<u32>) {
        (self.rgba, self.size)
    }
}

impl std::fmt::Debug for Icon {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Icon")
            .field("rgba", &format!("{} pixels", self.rgba.len() / 4))
            .field("size", &self.size)
            .finish()
    }
}

#[derive(Debug, thiserror::Error)]
/// 使用无效参数调用 [`from_rgba`] 时产生的错误。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An error produced when using [`from_rgba`] with invalid arguments.
///
/// </details>
pub enum Error {
    /// 当 `rgba` 参数的长度不能被 4 整除时产生，此时 `rgba` 无法
    /// 被安全地解释为 32bpp 的 RGBA 像素。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Produced when the length of the `rgba` argument isn't divisible by 4, thus `rgba` can't be
    /// safely interpreted as 32bpp RGBA pixels.
    ///
    /// </details>
    #[error(
        "The provided RGBA data (with length {byte_count}) isn't divisible \
        by 4. Therefore, it cannot be safely interpreted as 32bpp RGBA pixels"
    )]
    ByteCountNotDivisibleBy4 {
        /// 所提供 RGBA 数据的长度。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The length of the provided RGBA data.
        ///
        /// </details>
        byte_count: usize,
    },
    /// 当像素数量（`rgba.len() / 4`）不等于 `width * height` 时产生。
    /// 你的参数中至少有一个不正确。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Produced when the number of pixels (`rgba.len() / 4`) isn't equal to `width * height`.
    /// At least one of your arguments is incorrect.
    ///
    /// </details>
    #[error(
        "The number of RGBA pixels ({pixel_count}) does not match the \
        provided dimensions ({width}x{height})."
    )]
    DimensionsVsPixelCount {
        /// 所提供的宽度。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The provided width.
        ///
        /// </details>
        width: u32,
        /// 所提供的高度。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The provided height.
        ///
        /// </details>
        height: u32,
        /// `width` 与 `height` 的乘积。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The product of `width` and `height`.
        ///
        /// </details>
        width_x_height: usize,
        /// 所提供 RGBA 数据的像素数量。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The amount of pixels of the provided RGBA data.
        ///
        /// </details>
        pixel_count: usize,
    },
}
