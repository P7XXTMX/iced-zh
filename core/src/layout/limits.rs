#![allow(clippy::manual_clamp)]
use crate::{Length, Size};

/// 用于布局的一组尺寸约束。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A set of size constraints for layouting.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Limits {
    min: Size,
    max: Size,
    compression: Size<bool>,
}

impl Limits {
    /// 没有限制
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// No limits
    ///
    /// </details>
    pub const NONE: Limits = Limits {
        min: Size::ZERO,
        max: Size::INFINITE,
        compression: Size::new(false, false),
    };

    /// 使用给定的最小和最大 [`Size`] 创建新的 [`Limits`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates new [`Limits`] with the given minimum and maximum [`Size`].
    ///
    /// </details>
    pub const fn new(min: Size, max: Size) -> Limits {
        Limits::with_compression(min, max, Size::new(false, false))
    }

    /// 使用给定的最小和最大 [`Size`]，以及
    /// 流动长度是否应该被压缩到固有尺寸，创建新的 [`Limits`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates new [`Limits`] with the given minimun and maximum [`Size`], and
    /// whether fluid lengths should be compressed to intrinsic dimensions.
    ///
    /// </details>
    pub const fn with_compression(
        min: Size,
        max: Size,
        compress: Size<bool>,
    ) -> Self {
        Limits {
            min,
            max,
            compression: compress,
        }
    }

    /// 返回 [`Limits`] 的最小 [`Size`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the minimum [`Size`] of the [`Limits`].
    ///
    /// </details>
    pub fn min(&self) -> Size {
        self.min
    }

    /// 返回 [`Limits`] 的最大 [`Size`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the maximum [`Size`] of the [`Limits`].
    ///
    /// </details>
    pub fn max(&self) -> Size {
        self.max
    }

    /// 返回 [`Limits`] 的压缩设置。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the compression of the [`Limits`].
    ///
    /// </details>
    pub fn compression(&self) -> Size<bool> {
        self.compression
    }

    /// 对当前的 [`Limits`] 应用宽度约束。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Applies a width constraint to the current [`Limits`].
    ///
    /// </details>
    pub fn width(mut self, width: impl Into<Length>) -> Limits {
        match width.into() {
            Length::Shrink => {
                self.compression.width = true;
            }
            Length::Fixed(amount) => {
                let new_width = amount.min(self.max.width).max(self.min.width);

                self.min.width = new_width;
                self.max.width = new_width;
                self.compression.width = false;
            }
            Length::Fill | Length::FillPortion(_) => {}
        }

        self
    }

    /// 对当前的 [`Limits`] 应用高度约束。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Applies a height constraint to the current [`Limits`].
    ///
    /// </details>
    pub fn height(mut self, height: impl Into<Length>) -> Limits {
        match height.into() {
            Length::Shrink => {
                self.compression.height = true;
            }
            Length::Fixed(amount) => {
                let new_height =
                    amount.min(self.max.height).max(self.min.height);

                self.min.height = new_height;
                self.max.height = new_height;
                self.compression.height = false;
            }
            Length::Fill | Length::FillPortion(_) => {}
        }

        self
    }

    /// 对当前的 [`Limits`] 应用最小宽度约束。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Applies a minimum width constraint to the current [`Limits`].
    ///
    /// </details>
    pub fn min_width(mut self, min_width: f32) -> Limits {
        self.min.width = self.min.width.max(min_width).min(self.max.width);

        self
    }

    /// 对当前的 [`Limits`] 应用最大宽度约束。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Applies a maximum width constraint to the current [`Limits`].
    ///
    /// </details>
    pub fn max_width(mut self, max_width: f32) -> Limits {
        self.max.width = self.max.width.min(max_width).max(self.min.width);

        self
    }

    /// 对当前的 [`Limits`] 应用最小高度约束。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Applies a minimum height constraint to the current [`Limits`].
    ///
    /// </details>
    pub fn min_height(mut self, min_height: f32) -> Limits {
        self.min.height = self.min.height.max(min_height).min(self.max.height);

        self
    }

    /// 对当前的 [`Limits`] 应用最大高度约束。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Applies a maximum height constraint to the current [`Limits`].
    ///
    /// </details>
    pub fn max_height(mut self, max_height: f32) -> Limits {
        self.max.height = self.max.height.min(max_height).max(self.min.height);

        self
    }

    /// 把当前的 [`Limits`] 缩小给定的 [`Size`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Shrinks the current [`Limits`] by the given [`Size`].
    ///
    /// </details>
    pub fn shrink(&self, size: impl Into<Size>) -> Limits {
        let size = size.into();

        let min = Size::new(
            (self.min().width - size.width).max(0.0),
            (self.min().height - size.height).max(0.0),
        );

        let max = Size::new(
            (self.max().width - size.width).max(0.0),
            (self.max().height - size.height).max(0.0),
        );

        Limits {
            min,
            max,
            compression: self.compression,
        }
    }

    /// 移除当前 [`Limits`] 的最小 [`Size`] 约束。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Removes the minimum [`Size`] constraint for the current [`Limits`].
    ///
    /// </details>
    pub fn loose(&self) -> Limits {
        Limits {
            min: Size::ZERO,
            max: self.max,
            compression: self.compression,
        }
    }

    /// 给定宽度和高度要求，以及某些内容的
    /// 固有尺寸，
    /// 计算出符合 [`Limits`] 的 [`Size`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Computes the resulting [`Size`] that fits the [`Limits`] given
    /// some width and height requirements and the intrinsic size of
    /// some content.
    ///
    /// </details>
    pub fn resolve(
        &self,
        width: impl Into<Length>,
        height: impl Into<Length>,
        intrinsic_size: Size,
    ) -> Size {
        let width = match width.into() {
            Length::Fill | Length::FillPortion(_)
                if !self.compression.width =>
            {
                self.max.width
            }
            Length::Fixed(amount) => {
                amount.min(self.max.width).max(self.min.width)
            }
            _ => intrinsic_size.width.min(self.max.width).max(self.min.width),
        };

        let height = match height.into() {
            Length::Fill | Length::FillPortion(_)
                if !self.compression.height =>
            {
                self.max.height
            }
            Length::Fixed(amount) => {
                amount.min(self.max.height).max(self.min.height)
            }
            _ => intrinsic_size
                .height
                .min(self.max.height)
                .max(self.min.height),
        };

        Size::new(width, height)
    }
}
