//! 截取窗口的屏幕截图。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Take screenshots of a window.
//!
//! </details>
use crate::{Bytes, Rectangle, Size};

use std::fmt::{Debug, Formatter};

/// 屏幕截图的数据，通过 `window::screenshot()` 捕获。
///
/// 此截图的 `bytes` 始终按 `sRGB` 色彩空间中的 `RGBA` 顺序排列。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Data of a screenshot, captured with `window::screenshot()`.
///
/// The `bytes` of this screenshot will always be ordered as `RGBA` in the `sRGB` color space.
///
/// </details>
#[derive(Clone)]
pub struct Screenshot {
    /// [`Screenshot`] 的 RGBA 字节。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The RGBA bytes of the [`Screenshot`].
    ///
    /// </details>
    pub rgba: Bytes,
    /// [`Screenshot`] 的大小，单位为物理像素。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The size of the [`Screenshot`] in physical pixels.
    ///
    /// </details>
    pub size: Size<u32>,
    /// [`Screenshot`] 的缩放因子。在根据部件边界（单位为逻辑像素）
    /// 换算并裁剪截图时，它会很有用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The scale factor of the [`Screenshot`]. This can be useful when converting between widget
    /// bounds (which are in logical pixels) to crop screenshots.
    ///
    /// </details>
    pub scale_factor: f32,
}

impl Debug for Screenshot {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Screenshot: {{ \n bytes: {}\n scale: {}\n size: {:?} }}",
            self.rgba.len(),
            self.scale_factor,
            self.size
        )
    }
}

impl Screenshot {
    /// 创建一个新的 [`Screenshot`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Screenshot`].
    ///
    /// </details>
    pub fn new(
        rgba: impl Into<Bytes>,
        size: Size<u32>,
        scale_factor: f32,
    ) -> Self {
        Self {
            rgba: rgba.into(),
            size,
            scale_factor,
        }
    }

    /// 把 [`Screenshot`] 裁剪到给定的 `region`。该区域始终相对于
    /// [`Screenshot`] 的左上角。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Crops a [`Screenshot`] to the provided `region`. This will always be relative to the
    /// top-left corner of the [`Screenshot`].
    ///
    /// </details>
    pub fn crop(&self, region: Rectangle<u32>) -> Result<Self, CropError> {
        if region.width == 0 || region.height == 0 {
            return Err(CropError::Zero);
        }

        if region.x + region.width > self.size.width
            || region.y + region.height > self.size.height
        {
            return Err(CropError::OutOfBounds);
        }

        // Image is always RGBA8 = 4 bytes per pixel
        // 图像始终是 RGBA8 = 每像素 4 字节
        const PIXEL_SIZE: usize = 4;

        let bytes_per_row = self.size.width as usize * PIXEL_SIZE;
        let row_range = region.y as usize..(region.y + region.height) as usize;
        let column_range = region.x as usize * PIXEL_SIZE
            ..(region.x + region.width) as usize * PIXEL_SIZE;

        let chopped = self.rgba.chunks(bytes_per_row).enumerate().fold(
            vec![],
            |mut acc, (row, bytes)| {
                if row_range.contains(&row) {
                    acc.extend(&bytes[column_range.clone()]);
                }

                acc
            },
        );

        Ok(Self {
            rgba: Bytes::from(chopped),
            size: Size::new(region.width, region.height),
            scale_factor: self.scale_factor,
        })
    }
}

impl AsRef<[u8]> for Screenshot {
    fn as_ref(&self) -> &[u8] {
        &self.rgba
    }
}

impl From<Screenshot> for Bytes {
    fn from(screenshot: Screenshot) -> Self {
        screenshot.rgba
    }
}

#[derive(Debug, thiserror::Error)]
/// 裁剪 [`Screenshot`] 时可能发生的错误。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Errors that can occur when cropping a [`Screenshot`].
///
/// </details>
pub enum CropError {
    #[error("The cropped region is out of bounds.")]
    /// 裁剪区域的大小超出了边界。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The cropped region's size is out of bounds.
    ///
    /// </details>
    OutOfBounds,
    #[error("The cropped region is not visible.")]
    /// 裁剪区域的大小为零。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The cropped region's size is zero.
    ///
    /// </details>
    Zero,
}
