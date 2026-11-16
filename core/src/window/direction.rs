/// 相对于窗口中心的基本方位。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The cardinal directions relative to the center of a window.
///
/// </details>
#[derive(Debug, Clone, Copy)]
pub enum Direction {
    /// 指向窗口的上边缘。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Points to the top edge of a window.
    ///
    /// </details>
    North,

    /// 指向窗口的下边缘。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Points to the bottom edge of a window.
    ///
    /// </details>
    South,

    /// 指向窗口的右边缘。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Points to the right edge of a window.
    ///
    /// </details>
    East,

    /// 指向窗口的左边缘。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Points to the left edge of a window.
    ///
    /// </details>
    West,

    /// 指向窗口的右上角。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Points to the top-right corner of a window.
    ///
    /// </details>
    NorthEast,

    /// 指向窗口的左上角。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Points to the top-left corner of a window.
    ///
    /// </details>
    NorthWest,

    /// 指向窗口的右下角。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Points to the bottom-right corner of a window.
    ///
    /// </details>
    SouthEast,

    /// 指向窗口的左下角。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Points to the bottom-left corner of a window.
    ///
    /// </details>
    SouthWest,
}
