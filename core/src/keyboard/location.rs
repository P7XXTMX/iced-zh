/// 按键在键盘上的位置。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The location of a key on the keyboard.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Location {
    /// 键盘上的标准按键区。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The standard group of keys on the keyboard.
    ///
    /// </details>
    Standard,
    /// 键盘的左侧。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The left side of the keyboard.
    ///
    /// </details>
    Left,
    /// 键盘的右侧。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The right side of the keyboard.
    ///
    /// </details>
    Right,
    /// 键盘的数字小键盘。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The numpad of the keyboard.
    ///
    /// </details>
    Numpad,
}
