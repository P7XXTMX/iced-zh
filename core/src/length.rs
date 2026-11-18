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
    /// 填充所有剩余空间
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Fill all the remaining space
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

    /// 填充最少的空间
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Fill the least amount of space
    ///
    /// </details>
    Shrink,

    /// 填充固定大小的空间
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Fill a fixed amount of space
    ///
    /// </details>
    Fixed(f32),
}

impl Length {
    /// 返回 [`Length`] 的 *填充因子*。
    ///
    /// *填充因子* 是一个相对单位，描述与其他元素相比，
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
            Length::FillPortion(factor) => *factor,
            Length::Shrink => 0,
            Length::Fixed(_) => 0,
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

    /// 返回 [`Length`] 的“流动”变体。
    ///
    /// 具体来说：
    /// - 如果是 [`Length::Shrink`] 或 [`Length::Fixed`]，返回 [`Length::Shrink`]。
    /// - 否则返回 [`Length::Fill`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the "fluid" variant of the [`Length`].
    ///
    /// Specifically:
    /// - [`Length::Shrink`] if [`Length::Shrink`] or [`Length::Fixed`].
    /// - [`Length::Fill`] otherwise.
    ///
    /// </details>
    pub fn fluid(&self) -> Self {
        match self {
            Length::Fill | Length::FillPortion(_) => Length::Fill,
            Length::Shrink | Length::Fixed(_) => Length::Shrink,
        }
    }

    /// 调整 [`Length`]，使它能够容纳另一个 [`Length`]，
    /// 并与其流动性保持一致。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Adapts the [`Length`] so it can contain the other [`Length`] and
    /// match its fluidity.
    ///
    /// </details>
    #[inline]
    pub fn enclose(self, other: Length) -> Self {
        match (self, other) {
            (Length::Shrink, Length::Fill | Length::FillPortion(_)) => other,
            _ => self,
        }
    }
}

impl From<Pixels> for Length {
    fn from(amount: Pixels) -> Self {
        Length::Fixed(f32::from(amount))
    }
}

impl From<f32> for Length {
    fn from(amount: f32) -> Self {
        Length::Fixed(amount)
    }
}

impl From<u32> for Length {
    fn from(units: u32) -> Self {
        Length::Fixed(units as f32)
    }
}
