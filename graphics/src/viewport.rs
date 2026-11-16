use crate::core::renderer::Scale;
use crate::core::{Size, Transformation};

/// 用于显示计算机图形的可视区域。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A viewing region for displaying computer graphics.
///
/// </details>
#[derive(Debug, Clone)]
pub struct Viewport {
    physical_size: Size<u32>,
    logical_size: Size<f32>,
    scale: Scale,
    projection: Transformation,
}

impl Viewport {
    /// 使用给定的物理尺寸和缩放因子创建一个新的
    /// [`Viewport`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Viewport`] with the given physical dimensions and scale
    /// factor.
    ///
    /// </details>
    pub fn with_physical_size(size: Size<u32>, scale: Scale) -> Viewport {
        let scale_factor = scale.total();

        Viewport {
            physical_size: size,
            logical_size: Size::new(
                size.width as f32 / scale_factor,
                size.height as f32 / scale_factor,
            ),
            scale,
            projection: Transformation::orthographic(size.width, size.height),
        }
    }

    /// 返回 [`Viewport`] 的物理尺寸。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the physical size of the [`Viewport`].
    ///
    /// </details>
    pub fn physical_size(&self) -> Size<u32> {
        self.physical_size
    }

    /// 返回 [`Viewport`] 的物理宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the physical width of the [`Viewport`].
    ///
    /// </details>
    pub fn physical_width(&self) -> u32 {
        self.physical_size.width
    }

    /// 返回 [`Viewport`] 的物理高度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the physical height of the [`Viewport`].
    ///
    /// </details>
    pub fn physical_height(&self) -> u32 {
        self.physical_size.height
    }

    /// 返回 [`Viewport`] 的逻辑尺寸。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the logical size of the [`Viewport`].
    ///
    /// </details>
    pub fn logical_size(&self) -> Size<f32> {
        self.logical_size
    }

    /// 返回 [`Viewport`] 的 [`Scale`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`Scale`] of the [`Viewport`].
    ///
    /// </details>
    pub fn scale(&self) -> Scale {
        self.scale
    }

    /// 返回 [`Viewport`] 的缩放因子。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the scale factor of the [`Viewport`].
    ///
    /// </details>
    pub fn scale_factor(&self) -> f32 {
        self.scale.total()
    }

    /// 返回 [`Viewport`] 的投影变换。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the projection transformation of the [`Viewport`].
    ///
    /// </details>
    pub fn projection(&self) -> Transformation {
        self.projection
    }
}
