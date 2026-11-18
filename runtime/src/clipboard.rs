//! 访问剪贴板。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Access the clipboard.
//!
//! </details>
use crate::core::clipboard::Kind;
use crate::futures::futures::channel::oneshot;
use crate::task::{self, Task};

/// 由某个 [`Task`] 执行的剪贴板动作。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A clipboard action to be performed by some [`Task`].
///
/// </details>
///
/// [`Task`]: crate::Task
#[derive(Debug)]
pub enum Action {
    /// 读取剪贴板，并用结果产生 `T`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Read the clipboard and produce `T` with the result.
    ///
    /// </details>
    Read {
        /// 剪贴板目标。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The clipboard target.
        ///
        /// </details>
        target: Kind,
        /// 用于发送读取到的内容的通道。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The channel to send the read contents.
        ///
        /// </details>
        channel: oneshot::Sender<Option<String>>,
    },

    /// 把给定的内容写入剪贴板。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Write the given contents to the clipboard.
    ///
    /// </details>
    Write {
        /// 剪贴板目标。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The clipboard target.
        ///
        /// </details>
        target: Kind,
        /// 要写入的内容。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The contents to be written.
        ///
        /// </details>
        contents: String,
    },
}

/// 读取剪贴板的当前内容。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Read the current contents of the clipboard.
///
/// </details>
pub fn read() -> Task<Option<String>> {
    task::oneshot(|channel| {
        crate::Action::Clipboard(Action::Read {
            target: Kind::Standard,
            channel,
        })
    })
}

/// 读取主选区剪贴板的当前内容。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Read the current contents of the primary clipboard.
///
/// </details>
pub fn read_primary() -> Task<Option<String>> {
    task::oneshot(|channel| {
        crate::Action::Clipboard(Action::Read {
            target: Kind::Primary,
            channel,
        })
    })
}

/// 把给定的内容写入剪贴板。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Write the given contents to the clipboard.
///
/// </details>
pub fn write<T>(contents: String) -> Task<T> {
    task::effect(crate::Action::Clipboard(Action::Write {
        target: Kind::Standard,
        contents,
    }))
}

/// 把给定的内容写入主选区剪贴板。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Write the given contents to the primary clipboard.
///
/// </details>
pub fn write_primary<Message>(contents: String) -> Task<Message> {
    task::effect(crate::Action::Clipboard(Action::Write {
        target: Kind::Primary,
        contents,
    }))
}
