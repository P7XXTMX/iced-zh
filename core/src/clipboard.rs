//! 访问剪贴板。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Access the clipboard.
//!
//! </details>

/// 用于在应用程序内部和应用程序之间
/// 短期存储和传输数据的缓冲区。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A buffer for short-term storage and transfer within and between
/// applications.
///
/// </details>
pub trait Clipboard {
    /// 以文本形式读取 [`Clipboard`] 当前的内容。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Reads the current content of the [`Clipboard`] as text.
    ///
    /// </details>
    fn read(&self, kind: Kind) -> Option<String>;

    /// 把给定的文本内容写入 [`Clipboard`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Writes the given text contents to the [`Clipboard`].
    ///
    /// </details>
    fn write(&mut self, kind: Kind, contents: String);
}

/// [`Clipboard`] 的种类。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The kind of [`Clipboard`].
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// 标准剪贴板。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The standard clipboard.
    ///
    /// </details>
    Standard,
    /// 主选区剪贴板。
    ///
    /// 通常只存在于 X11 和 Wayland 中。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The primary clipboard.
    ///
    /// Normally only present in X11 and Wayland.
    ///
    /// </details>
    Primary,
}

/// [`Clipboard`] 特质的空实现。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A null implementation of the [`Clipboard`] trait.
///
/// </details>
#[derive(Debug, Clone, Copy)]
pub struct Null;

impl Clipboard for Null {
    fn read(&self, _kind: Kind) -> Option<String> {
        None
    }

    fn write(&mut self, _kind: Kind, _contents: String) {}
}
