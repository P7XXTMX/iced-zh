//! 与渲染器无关的原生 GUI 运行时。
//!
//! ![Iced 生态系统的原生路径](https://github.com/iced-rs/iced/blob/master/docs/graphs/native.png?raw=true)
//!
//! `iced_runtime` 以 [`iced_core`] 为基础，在其上构建了一个原生运行时。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! A renderer-agnostic native GUI runtime.
//!
//! ![The native path of the Iced ecosystem](https://github.com/iced-rs/iced/blob/master/docs/graphs/native.png?raw=true)
//!
//! `iced_runtime` takes [`iced_core`] and builds a native runtime on top of it.
//!
//! </details>
//!
//! [`iced_core`]: https://github.com/iced-rs/iced/tree/0.14/core
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/iced-rs/iced/9ab6923e943f784985e9ef9ca28b10278297225d/docs/logo.svg"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
pub mod clipboard;
pub mod font;
pub mod image;
pub mod keyboard;
pub mod system;
pub mod task;
pub mod user_interface;
pub mod widget;
pub mod window;

pub use iced_core as core;
pub use iced_futures as futures;

pub use task::Task;
pub use user_interface::UserInterface;
pub use window::Window;

use crate::futures::futures::channel::oneshot;

use std::borrow::Cow;
use std::fmt;

/// iced 运行时可以执行的动作。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An action that the iced runtime can perform.
///
/// </details>
pub enum Action<T> {
    /// 输出某个值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Output some value.
    ///
    /// </details>
    Output(T),

    /// 根据字节数据加载字体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Load a font from its bytes.
    ///
    /// </details>
    LoadFont {
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
        channel: oneshot::Sender<Result<(), font::Error>>,
    },

    /// 运行一个部件操作。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Run a widget operation.
    ///
    /// </details>
    Widget(Box<dyn core::widget::Operation>),

    /// 运行一个剪贴板动作。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Run a clipboard action.
    ///
    /// </details>
    Clipboard(clipboard::Action),

    /// 运行一个窗口动作。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Run a window action.
    ///
    /// </details>
    Window(window::Action),

    /// 运行一个系统动作。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Run a system action.
    ///
    /// </details>
    System(system::Action),

    /// 一个图像动作。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// An image action.
    ///
    /// </details>
    Image(image::Action),

    /// 重新创建所有用户界面并重绘所有窗口。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Recreate all user interfaces and redraw all windows.
    ///
    /// </details>
    Reload,

    /// 退出运行时。
    ///
    /// 这通常会关闭应用程序的所有窗口，并
    /// 终止运行时循环。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Exits the runtime.
    ///
    /// This will normally close any application windows and
    /// terminate the runtime loop.
    ///
    /// </details>
    Exit,
}

impl<T> Action<T> {
    /// 使用给定的 [`widget::Operation`](core::widget::Operation) 创建一个新的 [`Action::Widget`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Action::Widget`] with the given [`widget::Operation`](core::widget::Operation).
    ///
    /// </details>
    pub fn widget(operation: impl core::widget::Operation + 'static) -> Self {
        Self::Widget(Box::new(operation))
    }

    fn output<O>(self) -> Result<T, Action<O>> {
        match self {
            Action::Output(output) => Ok(output),
            Action::LoadFont { bytes, channel } => {
                Err(Action::LoadFont { bytes, channel })
            }
            Action::Widget(operation) => Err(Action::Widget(operation)),
            Action::Clipboard(action) => Err(Action::Clipboard(action)),
            Action::Window(action) => Err(Action::Window(action)),
            Action::System(action) => Err(Action::System(action)),
            Action::Image(action) => Err(Action::Image(action)),
            Action::Reload => Err(Action::Reload),
            Action::Exit => Err(Action::Exit),
        }
    }
}

impl<T> fmt::Debug for Action<T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Action::Output(output) => write!(f, "Action::Output({output:?})"),
            Action::LoadFont { .. } => {
                write!(f, "Action::LoadFont")
            }
            Action::Widget { .. } => {
                write!(f, "Action::Widget")
            }
            Action::Clipboard(action) => {
                write!(f, "Action::Clipboard({action:?})")
            }
            Action::Window(_) => write!(f, "Action::Window"),
            Action::System(action) => write!(f, "Action::System({action:?})"),
            Action::Image(_) => write!(f, "Action::Image"),
            Action::Reload => write!(f, "Action::Reload"),
            Action::Exit => write!(f, "Action::Exit"),
        }
    }
}

/// 创建一个退出 iced 运行时的 [`Task`]。
///
/// 这通常会关闭应用程序的所有窗口，并
/// 终止运行时循环。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a [`Task`] that exits the iced runtime.
///
/// This will normally close any application windows and
/// terminate the runtime loop.
///
/// </details>
pub fn exit<T>() -> Task<T> {
    task::effect(Action::Exit)
}
