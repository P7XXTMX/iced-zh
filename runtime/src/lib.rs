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
//! [`iced_core`]: https://github.com/iced-rs/iced/tree/master/core
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/iced-rs/iced/9ab6923e943f784985e9ef9ca28b10278297225d/docs/logo.svg"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
pub mod backend;
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

use crate::core::Event;

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

    /// 运行一个字体动作。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Run a font action.
    ///
    /// </details>
    Font(font::Action),

    /// 运行一个图像动作。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Run an image action.
    ///
    /// </details>
    Image(image::Action),

    /// 运行一个后端动作。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Run a backend action.
    ///
    /// </details>
    Backend(backend::Action),

    /// 产生一个事件。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Produce an event.
    ///
    /// </details>
    Event {
        /// 事件的 [`window::Id`](core::window::Id)。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The [`window::Id`](core::window::Id) of the event.
        ///
        /// </details>
        window: core::window::Id,
        /// 要产生的 [`Event`]。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The [`Event`] to be produced.
        ///
        /// </details>
        event: Event,
    },

    /// 轮询任何可能有待处理计算的资源。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Poll any resources that may have pending computations.
    ///
    /// </details>
    Tick,

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
            Action::Widget(operation) => Err(Action::Widget(operation)),
            Action::Clipboard(action) => Err(Action::Clipboard(action)),
            Action::Window(action) => Err(Action::Window(action)),
            Action::System(action) => Err(Action::System(action)),
            Action::Font(action) => Err(Action::Font(action)),
            Action::Image(action) => Err(Action::Image(action)),
            Action::Backend(action) => Err(Action::Backend(action)),
            Action::Event { window, event } => Err(Action::Event { window, event }),
            Action::Tick => Err(Action::Tick),
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
            Action::Widget { .. } => {
                write!(f, "Action::Widget")
            }
            Action::Clipboard(action) => {
                write!(f, "Action::Clipboard({action:?})")
            }
            Action::Window(_) => write!(f, "Action::Window"),
            Action::System(action) => write!(f, "Action::System({action:?})"),
            Action::Font(action) => {
                write!(f, "Action::Font({action:?})")
            }
            Action::Image(action) => write!(f, "Action::Image({action:?})"),
            Action::Backend(action) => write!(f, "Action::Backend({action:?})"),
            Action::Event { window, event } => write!(
                f,
                "Action::Event {{ window: {window:?}, event: {event:?} }}"
            ),
            Action::Tick => write!(f, "Action::Tick"),
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
