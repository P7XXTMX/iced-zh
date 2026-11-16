use crate::pane_grid::Axis;

/// [`PaneGrid`] 的排布方式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The arrangement of a [`PaneGrid`].
///
/// </details>
///
/// [`PaneGrid`]: super::PaneGrid
#[derive(Debug, Clone)]
pub enum Configuration<T> {
    /// 对可用空间的一次分割。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A split of the available space.
    ///
    /// </details>
    Split {
        /// 分割的方向。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The direction of the split.
        ///
        /// </details>
        axis: Axis,

        /// 分割的比例，取值范围为 [0.0, 1.0]。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The ratio of the split in [0.0, 1.0].
        ///
        /// </details>
        ratio: f32,

        /// 分割左侧/上方的 [`Configuration`]。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The left/top [`Configuration`] of the split.
        ///
        /// </details>
        a: Box<Configuration<T>>,

        /// 分割右侧/下方的 [`Configuration`]。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The right/bottom [`Configuration`] of the split.
        ///
        /// </details>
        b: Box<Configuration<T>>,
    },
    /// 一个 [`Pane`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A [`Pane`].
    ///
    /// </details>
    ///
    /// [`Pane`]: super::Pane
    Pane(T),
}
