//! 在四周留出空间。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Space stuff around the perimeter.
//!
//! </details>
use crate::{Pixels, Size};

/// 盒子每一侧要填充的空间大小
///
/// 你可以利用 `From` 特质方便地构建 [`Padding`]：
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An amount of space to pad for each side of a box
///
/// You can leverage the `From` trait to build [`Padding`] conveniently:
///
/// </details>
///
/// ```
/// # use iced_core::Padding;
/// #
/// let padding = Padding::from(20);              // 20px on all sides（四边各 20px）
/// let padding = Padding::from([10, 20]);        // top/bottom, left/right（上下、左右）
/// ```
///
/// 通常，部件的 `padding` 方法会要求一个 `Into<Padding>`，
/// 因此你可以轻松地这样写：
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Normally, the `padding` method of a widget will ask for an `Into<Padding>`,
/// so you can easily write:
///
/// </details>
///
/// ```
/// # use iced_core::Padding;
/// #
/// # struct Widget;
/// #
/// impl Widget {
///     # pub fn new() -> Self { Self }
///     #
///     pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
///         // ...
///         self
///     }
/// }
///
/// let widget = Widget::new().padding(20);              // 20px on all sides（四边各 20px）
/// let widget = Widget::new().padding([10, 20]);        // top/bottom, left/right（上下、左右）
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Default)]
pub struct Padding {
    /// 上内边距
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Top padding
    ///
    /// </details>
    pub top: f32,
    /// 右内边距
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Right padding
    ///
    /// </details>
    pub right: f32,
    /// 下内边距
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Bottom padding
    ///
    /// </details>
    pub bottom: f32,
    /// 左内边距
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Left padding
    ///
    /// </details>
    pub left: f32,
}

/// 创建一个四边相等的 [`Padding`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Create a [`Padding`] that is equal on all sides.
///
/// </details>
pub fn all(padding: impl Into<Pixels>) -> Padding {
    Padding::new(padding.into().0)
}

/// 创建一个只有上边的 [`Padding`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Create some top [`Padding`].
///
/// </details>
pub fn top(padding: impl Into<Pixels>) -> Padding {
    Padding::default().top(padding)
}

/// 创建一个只有下边的 [`Padding`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Create some bottom [`Padding`].
///
/// </details>
pub fn bottom(padding: impl Into<Pixels>) -> Padding {
    Padding::default().bottom(padding)
}

/// 创建一个只有左边的 [`Padding`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Create some left [`Padding`].
///
/// </details>
pub fn left(padding: impl Into<Pixels>) -> Padding {
    Padding::default().left(padding)
}

/// 创建一个只有右边的 [`Padding`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Create some right [`Padding`].
///
/// </details>
pub fn right(padding: impl Into<Pixels>) -> Padding {
    Padding::default().right(padding)
}

/// 创建一个左右两边相等的 [`Padding`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Create some [`Padding`] with equal left and right sides.
///
/// </details>
pub fn horizontal(padding: impl Into<Pixels>) -> Padding {
    Padding::default().horizontal(padding)
}

/// 创建一个上下两边相等的 [`Padding`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Create some [`Padding`] with equal top and bottom sides.
///
/// </details>
pub fn vertical(padding: impl Into<Pixels>) -> Padding {
    Padding::default().vertical(padding)
}

impl Padding {
    /// 为零的内边距
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Padding of zero
    ///
    /// </details>
    pub const ZERO: Padding = Padding {
        top: 0.0,
        right: 0.0,
        bottom: 0.0,
        left: 0.0,
    };

    /// 创建一个四边相等的 [`Padding`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Create a [`Padding`] that is equal on all sides.
    ///
    /// </details>
    pub const fn new(padding: f32) -> Padding {
        Padding {
            top: padding,
            right: padding,
            bottom: padding,
            left: padding,
        }
    }

    /// 设置 [`Padding`] 的 [`top`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`top`] of the [`Padding`].
    ///
    /// </details>
    ///
    /// [`top`]: Self::top
    pub fn top(self, top: impl Into<Pixels>) -> Self {
        Self {
            top: top.into().0,
            ..self
        }
    }

    /// 设置 [`Padding`] 的 [`bottom`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`bottom`] of the [`Padding`].
    ///
    /// </details>
    ///
    /// [`bottom`]: Self::bottom
    pub fn bottom(self, bottom: impl Into<Pixels>) -> Self {
        Self {
            bottom: bottom.into().0,
            ..self
        }
    }

    /// 设置 [`Padding`] 的 [`left`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`left`] of the [`Padding`].
    ///
    /// </details>
    ///
    /// [`left`]: Self::left
    pub fn left(self, left: impl Into<Pixels>) -> Self {
        Self {
            left: left.into().0,
            ..self
        }
    }

    /// 设置 [`Padding`] 的 [`right`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`right`] of the [`Padding`].
    ///
    /// </details>
    ///
    /// [`right`]: Self::right
    pub fn right(self, right: impl Into<Pixels>) -> Self {
        Self {
            right: right.into().0,
            ..self
        }
    }

    /// 设置 [`Padding`] 的 [`left`] 和 [`right`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`left`] and [`right`] of the [`Padding`].
    ///
    /// </details>
    ///
    /// [`left`]: Self::left
    /// [`right`]: Self::right
    pub fn horizontal(self, horizontal: impl Into<Pixels>) -> Self {
        let horizontal = horizontal.into();

        Self {
            left: horizontal.0,
            right: horizontal.0,
            ..self
        }
    }

    /// 设置 [`Padding`] 的 [`top`] 和 [`bottom`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`top`] and [`bottom`] of the [`Padding`].
    ///
    /// </details>
    ///
    /// [`top`]: Self::top
    /// [`bottom`]: Self::bottom
    pub fn vertical(self, vertical: impl Into<Pixels>) -> Self {
        let vertical = vertical.into();

        Self {
            top: vertical.0,
            bottom: vertical.0,
            ..self
        }
    }

    /// 返回水平方向 [`Padding`] 的总量。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the total amount of horizontal [`Padding`].
    ///
    /// </details>
    pub fn x(self) -> f32 {
        self.left + self.right
    }

    /// 返回垂直方向 [`Padding`] 的总量。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the total amount of vertical [`Padding`].
    ///
    /// </details>
    pub fn y(self) -> f32 {
        self.top + self.bottom
    }

    /// 调整 [`Padding`]，使其适合给定的 `inner` 和 `outer` [`Size`] 之间的空间。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Fits the [`Padding`] between the provided `inner` and `outer` [`Size`].
    ///
    /// </details>
    pub fn fit(self, inner: Size, outer: Size) -> Self {
        let available = (outer - inner).max(Size::ZERO);
        let new_top = self.top.min(available.height);
        let new_left = self.left.min(available.width);

        Padding {
            top: new_top,
            bottom: self.bottom.min(available.height - new_top),
            left: new_left,
            right: self.right.min(available.width - new_left),
        }
    }
}

impl From<u16> for Padding {
    fn from(p: u16) -> Self {
        Padding {
            top: f32::from(p),
            right: f32::from(p),
            bottom: f32::from(p),
            left: f32::from(p),
        }
    }
}

impl From<[u16; 2]> for Padding {
    fn from(p: [u16; 2]) -> Self {
        Padding {
            top: f32::from(p[0]),
            right: f32::from(p[1]),
            bottom: f32::from(p[0]),
            left: f32::from(p[1]),
        }
    }
}

impl From<f32> for Padding {
    fn from(p: f32) -> Self {
        Padding {
            top: p,
            right: p,
            bottom: p,
            left: p,
        }
    }
}

impl From<[f32; 2]> for Padding {
    fn from(p: [f32; 2]) -> Self {
        Padding {
            top: p[0],
            right: p[1],
            bottom: p[0],
            left: p[1],
        }
    }
}

impl From<Padding> for Size {
    fn from(padding: Padding) -> Self {
        Self::new(padding.x(), padding.y())
    }
}

impl From<Pixels> for Padding {
    fn from(pixels: Pixels) -> Self {
        Self::from(pixels.0)
    }
}
