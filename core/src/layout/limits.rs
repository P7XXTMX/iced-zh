#![allow(clippy::manual_clamp)]
use crate::length;
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
    /// 最小边界。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The minimum bounds.
    ///
    /// </details>
    pub min: Size,
    /// 最大边界。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The maximum bounds.
    ///
    /// </details>
    pub max: Size,
    /// 流动长度是否应该被压缩到固有尺寸。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether fluid lengths should be compressed to intrinsic dimensions.
    ///
    /// </details>
    pub compression: Size<bool>,
    /// 固有尺寸是否可以超出最大边界。在这种情况下，
    /// 最大边界变成一个提示。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Whether intrinsic dimensions can exceed maximum bounds. The maximum
    /// boundary becomes a hint in this case.
    ///
    /// </details>
    pub infinite: Size<bool>,
}

impl Limits {
    /// 使用给定的最小和最大 [`Size`] 创建新的 [`Limits`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates new [`Limits`] with the given minimum and maximum [`Size`].
    ///
    /// </details>
    pub const fn new(min: Size, max: Size) -> Limits {
        Limits::with_flags(min, max, Size::new(false, false), Size::new(false, false))
    }

    /// 使用给定的最小和最大 [`Size`]、
    /// 流动长度是否应该被压缩到固有尺寸，
    /// 以及上界是否可以无限增长，创建新的 [`Limits`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates new [`Limits`] with the given minimum and maximum [`Size`],
    /// whether fluid lengths should be compressed to intrinsic dimensions,
    /// and whether the upper bounds may grow indefinitely.
    ///
    /// </details>
    pub const fn with_flags(
        min: Size,
        max: Size,
        compression: Size<bool>,
        infinite: Size<bool>,
    ) -> Self {
        Limits {
            min,
            max,
            compression,
            infinite,
        }
    }

    /// 返回 [`Limits`] 的最大 [`Size`]。
    ///
    /// 在边界为 [`infinite`](Self::infinite) 的轴上，该值为
    /// [`f32::INFINITY`]，因为在这种情况下最大边界
    /// 变成了一个提示。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the maximum [`Size`] of the [`Limits`].
    ///
    /// On axes with [`infinite`](Self::infinite) bounds, the value is
    /// [`f32::INFINITY`], as the maximum boundary becomes a hint in this
    /// case.
    ///
    /// </details>
    pub fn bounds(&self) -> Size {
        Size::new(
            if self.infinite.width {
                f32::INFINITY
            } else {
                self.max.width
            },
            if self.infinite.height {
                f32::INFINITY
            } else {
                self.max.height
            },
        )
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
            Length::Fit | Length::Fluid(_) => {
                self.compression.width = false;
            }
            Length::Fixed(amount) => {
                let new_width = if self.infinite.width {
                    amount
                } else {
                    amount.min(self.max.width)
                }
                .max(self.min.width);

                self.min.width = new_width;
                self.max.width = new_width;
                self.compression.width = false;
                self.infinite.width = false;
            }
            Length::Bounded { bounds, sizing } => {
                match bounds {
                    length::Bounds::Min(min) => {
                        self.min.width = if self.infinite.width {
                            min
                        } else {
                            min.min(self.max.width)
                        };
                    }
                    length::Bounds::Max(max) => {
                        self.max.width = if self.infinite.width {
                            max
                        } else {
                            max.min(self.max.width)
                        }
                        .max(self.min.width);

                        self.infinite.width = false;
                    }
                    length::Bounds::Both { min, max } => {
                        self.min.width = min.min(self.max.width).max(self.min.width);
                        self.max.width = max.min(self.max.width).max(self.min.width);
                    }
                }

                match sizing {
                    length::Sizing::Shrink => {
                        self.compression.width = true;
                    }
                    length::Sizing::Fit => {
                        self.compression.width = false;
                    }
                    length::Sizing::Fill(_) => {}
                }
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
            Length::Fit | Length::Fluid(_) => {
                self.compression.height = false;
            }
            Length::Fixed(amount) => {
                let new_height = if self.infinite.height {
                    amount
                } else {
                    amount.min(self.max.height)
                }
                .max(self.min.height);

                self.min.height = new_height;
                self.max.height = new_height;
                self.compression.height = false;
                self.infinite.height = false;
            }
            Length::Bounded { bounds, sizing } => {
                match bounds {
                    length::Bounds::Min(min) => {
                        self.min.height = if self.infinite.height {
                            min
                        } else {
                            min.min(self.max.height)
                        };
                    }
                    length::Bounds::Max(max) => {
                        self.max.height = if self.infinite.height {
                            max
                        } else {
                            max.min(self.max.height)
                        }
                        .max(self.min.height);

                        self.infinite.height = false;
                    }
                    length::Bounds::Both { min, max } => {
                        self.min.height = min.min(self.max.height).max(self.min.height);
                        self.max.height = max.min(self.max.height).max(self.min.height);
                    }
                }

                match sizing {
                    length::Sizing::Shrink => {
                        self.compression.height = true;
                    }
                    length::Sizing::Fit => {
                        self.compression.height = false;
                    }
                    length::Sizing::Fill(_) => {}
                }
            }
            Length::Fill | Length::FillPortion(_) => {}
        }

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
            (self.min.width - size.width).max(0.0),
            (self.min.height - size.height).max(0.0),
        );

        let max = Size::new(
            (self.max.width - size.width).max(0.0),
            (self.max.height - size.height).max(0.0),
        );

        Limits {
            min,
            max,
            compression: self.compression,
            infinite: self.infinite,
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
            infinite: self.infinite,
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
        Size::new(
            self.resolve_width(width, intrinsic_size.width),
            self.resolve_height(height, intrinsic_size.height),
        )
    }

    /// 只[解析](Self::resolve) [`Limits`] 的宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// [Resolves](Self::resolve) only the width of the [`Limits`].
    ///
    /// </details>
    pub fn resolve_width(&self, width: impl Into<Length>, intrinsic_width: f32) -> f32 {
        resolve(
            self.min.width,
            self.max.width,
            self.compression.width,
            self.infinite.width,
            width.into(),
            intrinsic_width,
        )
    }

    /// 只[解析](Self::resolve) [`Limits`] 的高度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// [Resolves](Self::resolve) only the height of the [`Limits`].
    ///
    /// </details>
    pub fn resolve_height(&self, height: impl Into<Length>, intrinsic_height: f32) -> f32 {
        resolve(
            self.min.height,
            self.max.height,
            self.compression.height,
            self.infinite.height,
            height.into(),
            intrinsic_height,
        )
    }
}

fn resolve(
    min: f32,
    max: f32,
    compression: bool,
    infinite: bool,
    length: Length,
    intrinsic: f32,
) -> f32 {
    match length {
        Length::Fill
        | Length::FillPortion(_)
        | Length::Bounded {
            sizing: length::Sizing::Fill(_),
            ..
        } if !compression => if infinite { max.max(intrinsic) } else { max }.max(min),
        Length::Fixed(amount) => if infinite { amount } else { amount.min(max) }.max(min),
        _ => if infinite {
            intrinsic
        } else {
            intrinsic.min(max)
        }
        .max(min),
    }
}
