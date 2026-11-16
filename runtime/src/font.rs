//! 加载并使用字体。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Load and use fonts.
//!
//! </details>
use crate::core::Pixels;
use crate::core::font::{Error, Family, Font};
use crate::futures::futures::channel::oneshot;
use crate::task::{self, Task};

use std::borrow::Cow;
use std::fmt;

/// 字体动作。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A font action.
///
/// </details>
pub enum Action {
    /// 根据字节数据加载字体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Load a font from its bytes.
    ///
    /// </details>
    Load {
        /// 要加载的字体的字节数据。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The bytes of the font to load.
        ///
        /// </details>
        bytes: Cow<'static, [u8]>,
        /// 用于传回加载结果的通道。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The channel to send back the load result.
        ///
        /// </details>
        channel: oneshot::Sender<Result<(), Error>>,
    },

    /// 列出所有系统字体族。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Lists all system font families.
    ///
    /// </details>
    List {
        /// 用于传回列表结果的通道。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The channel to send back the list result.
        ///
        /// </details>
        channel: oneshot::Sender<Result<Vec<Family>, Error>>,
    },

    /// 为正在运行的应用程序设置新的字体默认值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the new font defaults for the running application.
    ///
    /// </details>
    SetDefaults {
        /// 新的默认 [`Font`]。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The new default [`Font`].
        ///
        /// </details>
        font: Font,
        /// 新的默认文本大小。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The new default text size.
        ///
        /// </details>
        text_size: Pixels,
    },
}

impl fmt::Debug for Action {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Load { .. } => f.write_str("Load"),
            Self::List { .. } => f.write_str("List"),
            Self::SetDefaults { font, text_size } => f
                .debug_struct("SetDefaults")
                .field("font", font)
                .field("text_size", text_size)
                .finish(),
        }
    }
}

/// 根据字节数据加载字体。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Load a font from its bytes.
///
/// </details>
pub fn load(bytes: impl Into<Cow<'static, [u8]>>) -> Task<Result<(), Error>> {
    task::oneshot(|channel| {
        crate::Action::Font(Action::Load {
            bytes: bytes.into(),
            channel,
        })
    })
}

/// 列出系统中所有可用的字体族。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Lists all the available font families in the system.
///
/// </details>
pub fn list() -> Task<Result<Vec<Family>, Error>> {
    task::oneshot(|channel| crate::Action::Font(Action::List { channel }))
}

/// 为正在运行的应用程序设置新的默认 [`Font`] 和文本大小。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Sets a new default [`Font`] and text size for the running application.
///
/// </details>
pub fn set_defaults<Message>(font: Font, text_size: impl Into<Pixels>) -> Task<Message> {
    task::effect(crate::Action::Font(Action::SetDefaults {
        font,
        text_size: text_size.into(),
    }))
}
