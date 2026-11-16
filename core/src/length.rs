//! 准确地描述空间的大小。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Describe amounts of space accurately.
//!
//! </details>
use crate::Pixels;

/// 在某个特定维度上填充空间所用的策略。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The strategy used to fill space in a specific dimension.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Length {
    /// 根据元素的固有尺寸填充最少的空间；
    /// 固有尺寸通常由其内容决定。
    ///
    /// 这是大多数部件的默认尺寸策略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Fill the minimum amount of space based on the intrinsic size of the
    /// element; normally defined by its contents.
    ///
    /// This is the default sizing strategy of most widgets.
    ///
    /// </details>
    Fit,

    /// 填充所有剩余空间。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Fill all the remaining space.
    ///
    /// </details>
    Fill,

    /// 相对于其他元素，填充剩余空间的一部分。
    ///
    /// 假设我们有两个元素：一个是 `FillPortion(2)`，另一个是
    /// `FillPortion(3)`。第一个会得到可用空间的 2 份，
    /// 而第二个会得到 3 份。
    ///
    /// `Length::Fill` 等价于 `Length::FillPortion(1)`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Fill a portion of the remaining space relative to other elements.
    ///
    /// Let's say we have two elements: one with `FillPortion(2)` and one with
    /// `FillPortion(3)`. The first will get 2 portions of the available space,
    /// while the second one would get 3.
    ///
    /// `Length::Fill` is equivalent to `Length::FillPortion(1)`.
    ///
    /// </details>
    FillPortion(u16),

    /// 填充最少的空间；如果可能，会压缩内容。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Fill the least amount of space; compressing contents if possible.
    ///
    /// </details>
    Shrink,

    /// 填充固定大小的空间，以像素为单位。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Fill a fixed amount of space in pixels.
    ///
    /// </details>
    Fixed(f32),

    /// 使用某种 [`Sizing`] 策略，在给定的 [`Bounds`] 内填充一定大小的空间。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Fill a certain amount of space inside the given [`Bounds`] with some [`Sizing`] strategy.
    ///
    /// </details>
    Bounded {
        /// 可以被填充的空间的 [`Bounds`]。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The [`Bounds`] of space that can be filled.
        ///
        /// </details>
        bounds: Bounds,
        /// 填充 [`Bounds`] 时应该采用的 [`Sizing`] 策略。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The [`Sizing`] strategy with which the [`Bounds`] should be filled.
        ///
        /// </details>
        sizing: Sizing,
    },

    /// 像 [`Fill`](Self::Fill) 一样填充剩余空间，但受单个
    /// 开放式 [`Constraint`] 的约束。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Fill the remaining space like [`Fill`](Self::Fill), but subject to a single
    /// open-ended [`Constraint`].
    ///
    /// </details>
    Fluid(Constraint),
}

#[derive(Debug, Clone, Copy, PartialEq)]
/// [`Fluid`](Length::Fluid) 长度的限制。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The limit of a [`Fluid`](Length::Fluid) length.
///
/// </details>
pub enum Constraint {
    /// 填充可用空间，但绝不会收缩到给定的大小以下。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Fill available space, but never shrinks below the given amount.
    ///
    /// </details>
    Min(f32),
    /// 填充可用空间，但绝不会超出元素自身解析出的尺寸；
    /// 并把它没有用到的份额归还给它的兄弟元素。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Fill available space, but never beyond the size the element resolves to on its own;
    /// releasing any share it doesn't use back to its siblings.
    ///
    /// </details>
    Max,
}

impl Constraint {
    fn stack(self, other: Self) -> Self {
        match (self, other) {
            (Constraint::Min(a), Constraint::Min(b)) => Self::Min(a + b),
            (Constraint::Min(min), Constraint::Max) | (Constraint::Max, Constraint::Min(min)) => {
                Constraint::Min(min)
            }
            (Constraint::Max, Constraint::Max) => Self::Max,
        }
    }

    fn cross(self, other: Self) -> Self {
        match (self, other) {
            (Constraint::Min(a), Constraint::Min(b)) => Self::Min(a.max(b)),
            (Constraint::Min(min), Constraint::Max) | (Constraint::Max, Constraint::Min(min)) => {
                Constraint::Min(min)
            }
            (Constraint::Max, Constraint::Max) => Self::Max,
        }
    }
}

/// [`Bounded`](Length::Bounded) 长度的空间限制。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The space limits of a [`Bounded`](Length::Bounded) length.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Bounds {
    /// 长度必须至少为一定数量的像素。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The length must be at least a certain amount of pixels.
    ///
    /// </details>
    Min(f32),
    /// 长度不得超过一定数量的像素。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The length must not exceed a certain amount of pixels.
    ///
    /// </details>
    Max(f32),
    /// 长度必须在某个像素范围之内。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The length must be inside a range of pixels.
    ///
    /// </details>
    Both {
        /// 最小边界。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The minimum boundary.
        ///
        /// </details>
        min: f32,
        /// 最大边界。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The maximum boundary.
        ///
        /// </details>
        max: f32,
    },
}

impl Bounds {
    /// 对当前的 [`Bounds`] 应用新的最小边界。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Applies a new minimum boundary to the current [`Bounds`].
    ///
    /// </details>
    pub fn min(self, min: f32) -> Self {
        match self {
            Self::Min(_) => Self::Min(min),
            Self::Max(max) | Self::Both { max, .. } => Self::Both { min, max },
        }
    }

    /// 对当前的 [`Bounds`] 应用新的最大边界。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Applies a new maximum boundary to the current [`Bounds`].
    ///
    /// </details>
    pub fn max(self, max: f32) -> Self {
        match self {
            Self::Max(_) => Self::Max(max),
            Self::Min(min) | Self::Both { min, .. } => Self::Both { min, max },
        }
    }

    /// 返回一个表示当前 [`Bounds`] 的 [`Constraint`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns a [`Constraint`] that represents the current [`Bounds`].
    ///
    /// </details>
    pub fn constraint(self) -> Constraint {
        match self {
            Bounds::Min(min) | Bounds::Both { min, .. } => Constraint::Min(min),
            Bounds::Max(_) => Constraint::Max,
        }
    }
}

/// [`Bounded`](Length::Bounded) 长度的增长策略。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The growth strategy of a [`Bounded`](Length::Bounded) length.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Sizing {
    /// 等价于 [`Length::Fit`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Equivalent to [`Length::Fit`].
    ///
    /// </details>
    Fit,
    /// 等价于 [`Length::FillPortion`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Equivalent to [`Length::FillPortion`].
    ///
    /// </details>
    Fill(u16),
    /// 等价于 [`Length::Shrink`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Equivalent to [`Length::Shrink`].
    ///
    /// </details>
    Shrink,
}

impl Length {
    /// 返回一个具有给定最小边界的新 [`Bounded`](Self::Bounded) 长度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns a new [`Bounded`](Self::Bounded) length with the given minimum bounds.
    ///
    /// </details>
    pub fn min(self, min: impl Into<Pixels>) -> Self {
        let min = min.into().0;

        let with = match self {
            Self::Fit | Self::Fluid(_) => Sizing::Fit,
            Self::Fill => Sizing::Fill(1),
            Self::FillPortion(factor) => Sizing::Fill(factor),
            Self::Shrink => Sizing::Shrink,
            Self::Fixed(_) => return self,
            Self::Bounded {
                bounds,
                sizing: with,
            } => {
                return Self::Bounded {
                    bounds: bounds.min(min),
                    sizing: with,
                };
            }
        };

        Self::Bounded {
            bounds: Bounds::Min(min),
            sizing: with,
        }
    }

    /// 返回一个具有给定最大边界的新 [`Bounded`](Self::Bounded) 长度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns a new [`Bounded`](Self::Bounded) length with the given maximum bounds.
    ///
    /// </details>
    pub fn max(self, max: impl Into<Pixels>) -> Self {
        let max = max.into().0;

        let with = match self {
            Self::Fit | Self::Fluid(_) => Sizing::Fit,
            Self::Fill => Sizing::Fill(1),
            Self::FillPortion(factor) => Sizing::Fill(factor),
            Self::Shrink => Sizing::Shrink,
            Self::Fixed(_) => return self,
            Self::Bounded {
                bounds,
                sizing: with,
            } => {
                return Self::Bounded {
                    bounds: bounds.max(max),
                    sizing: with,
                };
            }
        };

        Self::Bounded {
            bounds: Bounds::Max(max),
            sizing: with,
        }
    }

    /// 返回 [`Length`] 的 _填充因子_。
    ///
    /// _填充因子_ 是一个相对单位，描述与其他元素相比，
    /// 应该填充多少剩余空间。它
    /// 只供布局引擎使用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the _fill factor_ of the [`Length`].
    ///
    /// The _fill factor_ is a relative unit describing how much of the
    /// remaining space should be filled when compared to other elements. It
    /// is only meant to be used by layout engines.
    ///
    /// </details>
    pub fn fill_factor(&self) -> u16 {
        match self {
            Length::Fill => 1,
            Length::FillPortion(factor)
            | Length::Bounded {
                sizing: Sizing::Fill(factor),
                ..
            } => *factor,
            Length::Fluid(_) => 1,
            Length::Shrink | Length::Fit | Length::Fixed(_) | Length::Bounded { .. } => 0,
        }
    }

    /// 如果 [`Length`] 是 [`Length::Fill`] 或
    /// [`Length::FillPortion`]，返回 `true`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns `true` if the [`Length`] is either [`Length::Fill`] or
    /// [`Length::FillPortion`].
    ///
    /// </details>
    pub fn is_fill(&self) -> bool {
        self.fill_factor() != 0
    }

    /// 如果 [`Length`] 是 [`Fit`](Self::Fit)，返回 `true`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns `true` if the [`Length`] is [`Fit`](Self::Fit).
    ///
    /// </details>
    pub fn is_fit(&self) -> bool {
        matches!(self, Self::Fit)
    }

    /// 如果适用，把当前 [`Length`] 的约束与给定 [`Length`] 的约束叠加起来。
    ///
    /// 具体来说，最小约束会被 _相加_ 并累积。
    ///
    /// 当容器沿着某个轴布局多个元素，并且需要在 _主_ 轴上
    /// 继承它们的约束时，应该使用它。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Stacks the constraints of the current [`Length`] with the given one, if applicable.
    ///
    /// Specifically, minimum constraints will be _added_ together and accumulated.
    ///
    /// You should use this when a container lays out multiple elements along a given axis and
    /// need to inherit their constraints in the _main_ axis.
    ///
    /// </details>
    pub fn stack(self, other: Length) -> Self {
        self.merge_with(other, Constraint::stack)
    }

    /// 如果适用，把当前 [`Length`] 的约束与给定 [`Length`] 的约束交叉合并。
    ///
    /// 具体来说，会比较最小约束，并返回其中的 _最大值_。
    ///
    /// 当容器沿着某个轴布局多个元素，并且需要在 _交叉_ 轴上
    /// 继承它们的约束时，应该使用它。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Crosses the constraints of the current [`Length`] with the given one, if applicable.
    ///
    /// Specifically, minimum constraints will be compared and the _maximum_ will be returned.
    ///
    /// You should use this when a container lays out multiple elements along a given axis and
    /// need to inherit their constraints in the _cross_ axis.
    ///
    /// </details>
    pub fn cross(self, other: Length) -> Self {
        self.merge_with(other, Constraint::cross)
    }

    fn merge_with(self, other: Self, merge: impl Fn(Constraint, Constraint) -> Constraint) -> Self {
        match (self, other) {
            // Shrink, Fixed, and Fill are unmergeable
            // Shrink、Fixed 和 Fill 无法合并
            (Length::Shrink | Length::Fixed(_) | Length::Fill | Length::FillPortion(_), _) => self,

            // Fluid elements are merged
            // 流动元素会被合并
            (Length::Fluid(a), Length::Fluid(b)) => Length::Fluid(merge(a, b)),
            (
                Length::Fluid(a),
                Length::Bounded {
                    bounds,
                    sizing: Sizing::Fill(_),
                },
            ) => Length::Fluid(merge(a, bounds.constraint())),
            (Length::Fluid(constraint), _) => Length::Fluid(constraint),

            (
                Length::Bounded {
                    bounds,
                    sizing: Sizing::Fit,
                },
                Length::Fill
                | Length::FillPortion(_)
                | Length::Bounded {
                    sizing: Sizing::Fill(_),
                    ..
                },
            ) => Length::Bounded {
                bounds,
                sizing: Sizing::Fill(1),
            },
            (
                Length::Bounded {
                    bounds,
                    sizing: with,
                },
                _,
            ) => Length::Bounded {
                bounds,
                sizing: with,
            },

            // Fluid and bounded constraints must be propagated
            // 流动约束和有界约束必须向上传播
            (
                _,
                Length::Bounded {
                    bounds,
                    sizing: Sizing::Fill(_),
                },
            ) => Length::Fluid(bounds.constraint()),
            (_, Length::Fluid(constraint)) => Length::Fluid(constraint),

            // Fill wins over Fit
            // Fill 优先于 Fit
            (_, Length::Fill | Length::FillPortion(_)) => Length::Fill,

            // Fall back to Fit
            // 回退到 Fit
            _ => Length::Fit,
        }
    }
}

impl From<Pixels> for Length {
    fn from(amount: Pixels) -> Self {
        Self::Fixed(f32::from(amount))
    }
}

impl From<f32> for Length {
    fn from(amount: f32) -> Self {
        Self::Fixed(amount)
    }
}

impl From<u32> for Length {
    fn from(units: u32) -> Self {
        Self::Fixed(units as f32)
    }
}
