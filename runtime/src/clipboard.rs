//! 访问剪贴板。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Access the clipboard.
//!
//! </details>
use crate::core::clipboard::{Content, Error, Kind};
use crate::futures::futures::channel::oneshot;
use crate::task::{self, Task};

use std::path::PathBuf;
use std::sync::Arc;

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
        /// 要读取的 [`Content`] 的 [`Kind`]。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The [`Kind`] of [`Content`] to read.
        ///
        /// </details>
        kind: Kind,
        /// 用于发送读取到的内容的通道。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The channel to send the read contents.
        ///
        /// </details>
        channel: oneshot::Sender<Result<Content, Error>>,
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
        /// 要写入的 [`Content`]。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The [`Content`] to be written.
        ///
        /// </details>
        content: Content,

        /// 用于发送写入结果的通道。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The channel to send the write result.
        ///
        /// </details>
        channel: oneshot::Sender<Result<(), Error>>,
    },
}

/// 从剪贴板读取给定 [`Kind`] 的 [`Content`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Read the given [`Kind`] of [`Content`] from the clipboard.
///
/// </details>
pub fn read(kind: Kind) -> Task<Result<Arc<Content>, Error>> {
    task::oneshot(|channel| crate::Action::Clipboard(Action::Read { kind, channel }))
        .map(|result| result.map(Arc::new))
}

/// 读取剪贴板当前的文本内容。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Read the current text contents of the clipboard.
///
/// </details>
pub fn read_text() -> Task<Result<Arc<String>, Error>> {
    task::oneshot(|channel| {
        crate::Action::Clipboard(Action::Read {
            kind: Kind::Text,
            channel,
        })
    })
    .map(|result| {
        let Ok(Content::Text(text)) = result else {
            return Err(Error::ContentNotAvailable);
        };

        Ok(Arc::new(text))
    })
}

/// 读取剪贴板当前的 HTML 内容。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Read the current HTML contents of the clipboard.
///
/// </details>
pub fn read_html() -> Task<Result<Arc<String>, Error>> {
    task::oneshot(|channel| {
        crate::Action::Clipboard(Action::Read {
            kind: Kind::Html,
            channel,
        })
    })
    .map(|result| {
        let Ok(Content::Html(html)) = result else {
            return Err(Error::ContentNotAvailable);
        };

        Ok(Arc::new(html))
    })
}

/// 读取剪贴板当前的文件路径。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Read the current file paths of the clipboard.
///
/// </details>
pub fn read_files() -> Task<Result<Arc<[PathBuf]>, Error>> {
    task::oneshot(|channel| {
        crate::Action::Clipboard(Action::Read {
            kind: Kind::Files,
            channel,
        })
    })
    .map(|result| {
        let Ok(Content::Files(files)) = result else {
            return Err(Error::ContentNotAvailable);
        };

        Ok(Arc::from(files))
    })
}

/// 读取剪贴板当前的 [`Image`](crate::core::clipboard::Image)。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Read the current [`Image`](crate::core::clipboard::Image) of the clipboard.
///
/// </details>
#[cfg(feature = "image")]
pub fn read_image() -> Task<Result<crate::core::clipboard::Image, Error>> {
    task::oneshot(|channel| {
        crate::Action::Clipboard(Action::Read {
            kind: Kind::Image,
            channel,
        })
    })
    .map(|result| {
        let Ok(Content::Image(image)) = result else {
            return Err(Error::ContentNotAvailable);
        };

        Ok(image)
    })
}

/// 把给定的 [`Content`] 写入剪贴板。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Write the given [`Content`] to the clipboard.
///
/// </details>
pub fn write(content: impl Into<Content>) -> Task<Result<(), Error>> {
    let content = content.into();

    task::oneshot(|channel| crate::Action::Clipboard(Action::Write { content, channel }))
}
