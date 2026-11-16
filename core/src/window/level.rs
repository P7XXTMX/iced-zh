/// 窗口层级按 z 轴位置对窗口进行分组。
///
/// 不同窗口层级中的窗口之间的相对顺序是固定的。
/// 同一窗口层级内窗口的 z 轴顺序可能会随着
/// 用户交互而动态变化。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A window level groups windows with respect to their z-position.
///
/// The relative ordering between windows in different window levels is fixed.
/// The z-order of a window within the same window level may change dynamically
/// on user interaction.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Level {
    /// 默认行为。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The default behavior.
    ///
    /// </details>
    #[default]
    Normal,

    /// 窗口将始终位于普通窗口之下。
    ///
    /// 这对基于小组件（widget）的应用很有用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The window will always be below normal windows.
    ///
    /// This is useful for a widget-based app.
    ///
    /// </details>
    AlwaysOnBottom,

    /// 窗口将始终位于普通窗口之上。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The window will always be on top of normal windows.
    ///
    /// </details>
    AlwaysOnTop,
}
