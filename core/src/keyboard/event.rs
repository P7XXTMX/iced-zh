use crate::SmolStr;
use crate::keyboard::key;
use crate::keyboard::{Key, Location, Modifiers};

/// 键盘事件。
///
/// _**注意：** 这个类型还很不完整！如果你需要跟踪
/// 其他事件，欢迎[提交 issue][open an issue] 并分享你的使用场景！_
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A keyboard event.
///
/// _**Note:** This type is largely incomplete! If you need to track
/// additional events, feel free to [open an issue] and share your use case!_
///
/// </details>
///
/// [open an issue]: https://github.com/iced-rs/iced/issues
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// 某个键盘按键被按下。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A keyboard key was pressed.
    ///
    /// </details>
    KeyPressed {
        /// 被按下的按键。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The key pressed.
        ///
        /// </details>
        key: Key,

        /// 应用了除 Ctrl 之外所有键盘修饰键之后的被按下的按键。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The key pressed with all keyboard modifiers applied, except Ctrl.
        ///
        /// </details>
        modified_key: Key,

        /// 被按下的物理按键。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The physical key pressed.
        ///
        /// </details>
        physical_key: key::Physical,

        /// 按键的位置。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The location of the key.
        ///
        /// </details>
        location: Location,

        /// 修饰键的状态。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The state of the modifier keys.
        ///
        /// </details>
        modifiers: Modifiers,

        /// 这次按键产生的文本（如果有）。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The text produced by the key press, if any.
        ///
        /// </details>
        text: Option<SmolStr>,

        /// 该事件是否由按键重复产生。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// Whether the event was the result of key repeat.
        ///
        /// </details>
        repeat: bool,
    },

    /// 某个键盘按键被释放。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A keyboard key was released.
    ///
    /// </details>
    KeyReleased {
        /// 被释放的按键。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The key released.
        ///
        /// </details>
        key: Key,

        /// 应用了除 Ctrl 之外所有键盘修饰键之后的被释放的按键。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The key released with all keyboard modifiers applied, except Ctrl.
        ///
        /// </details>
        modified_key: Key,

        /// 被释放的物理按键。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The physical key released.
        ///
        /// </details>
        physical_key: key::Physical,

        /// 按键的位置。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The location of the key.
        ///
        /// </details>
        location: Location,

        /// 修饰键的状态。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The state of the modifier keys.
        ///
        /// </details>
        modifiers: Modifiers,
    },

    /// 键盘修饰键发生了变化。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The keyboard modifiers have changed.
    ///
    /// </details>
    ModifiersChanged(Modifiers),
}
