/// 鼠标的按键。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The button of a mouse.
///
/// </details>
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum Button {
    /// 鼠标左键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The left mouse button.
    ///
    /// </details>
    Left,

    /// 鼠标右键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The right mouse button.
    ///
    /// </details>
    Right,

    /// 中键（滚轮键）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The middle (wheel) button.
    ///
    /// </details>
    Middle,

    /// 鼠标后退键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The back mouse button.
    ///
    /// </details>
    Back,

    /// 鼠标前进键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The forward mouse button.
    ///
    /// </details>
    Forward,

    /// 其他按键。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Some other button.
    ///
    /// </details>
    Other(u16),
}
