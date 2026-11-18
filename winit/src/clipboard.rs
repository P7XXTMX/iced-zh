//! 访问剪贴板。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Access the clipboard.
//!
//! </details>

use crate::core::clipboard::Kind;
use std::sync::Arc;
use winit::window::{Window, WindowId};

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
pub struct Clipboard {
    state: State,
}

enum State {
    Connected {
        clipboard: window_clipboard::Clipboard,
        // Held until drop to satisfy the safety invariants of
        // `window_clipboard::Clipboard`.
        //
        // Note that the field ordering is load-bearing.
        // 一直持有到被丢弃为止，以满足
        // `window_clipboard::Clipboard` 的安全不变量。
        //
        // 注意，字段的顺序是有实际作用的。
        #[allow(dead_code)]
        window: Arc<Window>,
    },
    Unavailable,
}

impl Clipboard {
    /// 为给定的窗口创建一个新的 [`Clipboard`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Clipboard`] for the given window.
    ///
    /// </details>
    pub fn connect(window: Arc<Window>) -> Clipboard {
        // SAFETY: The window handle will stay alive throughout the entire
        // lifetime of the `window_clipboard::Clipboard` because we hold
        // the `Arc<Window>` together with `State`, and enum variant fields
        // get dropped in declaration order.
        // SAFETY：窗口句柄在 `window_clipboard::Clipboard` 的整个
        // 生命周期内都会保持存活，因为我们把
        // `Arc<Window>` 和 `State` 放在一起持有，而枚举变体的字段
        // 按声明顺序丢弃。
        #[allow(unsafe_code)]
        let clipboard =
            unsafe { window_clipboard::Clipboard::connect(&window) };

        let state = match clipboard {
            Ok(clipboard) => State::Connected { clipboard, window },
            Err(_) => State::Unavailable,
        };

        Clipboard { state }
    }

    /// 创建一个不与任何窗口关联的新 [`Clipboard`]。
    /// 这个剪贴板永远不会包含复制的值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Clipboard`] that isn't associated with a window.
    /// This clipboard will never contain a copied value.
    ///
    /// </details>
    pub fn unconnected() -> Clipboard {
        Clipboard {
            state: State::Unavailable,
        }
    }

    /// 以文本形式读取 [`Clipboard`] 当前的内容。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Reads the current content of the [`Clipboard`] as text.
    ///
    /// </details>
    pub fn read(&self, kind: Kind) -> Option<String> {
        match &self.state {
            State::Connected { clipboard, .. } => match kind {
                Kind::Standard => clipboard.read().ok(),
                Kind::Primary => clipboard.read_primary().and_then(Result::ok),
            },
            State::Unavailable => None,
        }
    }

    /// 把给定的文本内容写入 [`Clipboard`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Writes the given text contents to the [`Clipboard`].
    ///
    /// </details>
    pub fn write(&mut self, kind: Kind, contents: String) {
        match &mut self.state {
            State::Connected { clipboard, .. } => {
                let result = match kind {
                    Kind::Standard => clipboard.write(contents),
                    Kind::Primary => {
                        clipboard.write_primary(contents).unwrap_or(Ok(()))
                    }
                };

                match result {
                    Ok(()) => {}
                    Err(error) => {
                        log::warn!("error writing to clipboard: {error}");
                    }
                }
            }
            State::Unavailable => {}
        }
    }

    /// 返回创建 [`Clipboard`] 时所用窗口的标识符（如果有）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the identifier of the window used to create the [`Clipboard`], if any.
    ///
    /// </details>
    pub fn window_id(&self) -> Option<WindowId> {
        match &self.state {
            State::Connected { window, .. } => Some(window.id()),
            State::Unavailable => None,
        }
    }
}

impl crate::core::Clipboard for Clipboard {
    fn read(&self, kind: Kind) -> Option<String> {
        self.read(kind)
    }

    fn write(&mut self, kind: Kind, contents: String) {
        self.write(kind, contents);
    }
}
