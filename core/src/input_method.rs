//! 监听输入法事件。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Listen to input method events.
//!
//! </details>
use crate::{Pixels, Rectangle};

use std::ops::Range;

/// 部件的输入法策略。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The input method strategy of a widget.
///
/// </details>
#[derive(Debug, Clone, PartialEq)]
pub enum InputMethod<T = String> {
    /// 输入法已禁用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Input method is disabled.
    ///
    /// </details>
    Disabled,
    /// 输入法已启用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Input method is enabled.
    ///
    /// </details>
    Enabled {
        /// 输入法光标所在的区域。
        ///
        /// 这个区域不应该被遮挡。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The area of the cursor of the input method.
        ///
        /// This area should not be covered.
        ///
        /// </details>
        cursor: Rectangle,
        /// 输入法的 [`Purpose`]。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The [`Purpose`] of the input method.
        ///
        /// </details>
        purpose: Purpose,
        /// 需要时，覆盖显示在输入法对话框之上的预编辑内容。
        ///
        /// 理想情况下，你的部件会就地（on-the-spot）显示预编辑内容；但由于这
        /// 可能比较棘手，你也可以在这里提供当前的预编辑内容，
        /// 运行时会把它显示为覆盖层（即“Over-the-spot IME”）。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The preedit to overlay on top of the input method dialog, if needed.
        ///
        /// Ideally, your widget will show pre-edits on-the-spot; but, since that can
        /// be tricky, you can instead provide the current pre-edit here and the
        /// runtime will display it as an overlay (i.e. "Over-the-spot IME").
        ///
        /// </details>
        preedit: Option<Preedit<T>>,
    },
}

/// [`InputMethod`] 的预编辑内容。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The pre-edit of an [`InputMethod`].
///
/// </details>
#[derive(Debug, Clone, PartialEq, Default)]
pub struct Preedit<T = String> {
    /// 当前的内容。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The current content.
    ///
    /// </details>
    pub content: T,
    /// 内容中被选中的范围。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The selected range of the content.
    ///
    /// </details>
    pub selection: Option<Range<usize>>,
    /// 内容的文本大小。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The text size of the content.
    ///
    /// </details>
    pub text_size: Option<Pixels>,
}

impl<T> Preedit<T> {
    /// 创建一个新的空 [`Preedit`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new empty [`Preedit`].
    ///
    /// </details>
    pub fn new() -> Self
    where
        T: Default,
    {
        Self::default()
    }

    /// 把 [`Preedit`] 转换为拥有所有权的版本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Turns a [`Preedit`] into its owned version.
    ///
    /// </details>
    pub fn to_owned(&self) -> Preedit
    where
        T: AsRef<str>,
    {
        Preedit {
            content: self.content.as_ref().to_owned(),
            selection: self.selection.clone(),
            text_size: self.text_size,
        }
    }
}

impl Preedit {
    /// 借用 [`Preedit`] 的内容。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Borrows the contents of a [`Preedit`].
    ///
    /// </details>
    pub fn as_ref(&self) -> Preedit<&str> {
        Preedit {
            content: &self.content,
            selection: self.selection.clone(),
            text_size: self.text_size,
        }
    }
}

/// [`InputMethod`] 的用途。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The purpose of an [`InputMethod`].
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Purpose {
    /// 对输入法（IME）没有特殊提示（默认）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// No special hints for the IME (default).
    ///
    /// </details>
    #[default]
    Normal,
    /// 输入法（IME）用于安全输入（例如密码）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The IME is used for secure input (e.g. passwords).
    ///
    /// </details>
    Secure,
    /// 输入法（IME）用于向终端输入。
    ///
    /// 例如，这可以让 Wayland 上的屏幕键盘（OSK）显示额外的按钮。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The IME is used to input into a terminal.
    ///
    /// For example, that could alter OSK on Wayland to show extra buttons.
    ///
    /// </details>
    Terminal,
}

impl InputMethod {
    /// 合并两个 [`InputMethod`] 策略；两者都处于打开状态时，优先采用第一个：
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Merges two [`InputMethod`] strategies, prioritizing the first one when both open:
    ///
    /// </details>
    ///
    /// ```
    /// # use iced_core::input_method::{InputMethod, Purpose, Preedit};
    /// # use iced_core::{Point, Rectangle, Size};
    ///
    /// let open = InputMethod::Enabled {
    ///     cursor: Rectangle::new(Point::ORIGIN, Size::UNIT),
    ///     purpose: Purpose::Normal,
    ///     preedit: Some(Preedit { content: "1".to_owned(), selection: None, text_size: None }),
    /// };
    ///
    /// let open_2 = InputMethod::Enabled {
    ///     cursor: Rectangle::new(Point::ORIGIN, Size::UNIT),
    ///     purpose: Purpose::Secure,
    ///     preedit: Some(Preedit { content: "2".to_owned(), selection: None, text_size: None }),
    /// };
    ///
    /// let mut ime = InputMethod::Disabled;
    ///
    /// ime.merge(&open);
    /// assert_eq!(ime, open);
    ///
    /// ime.merge(&open_2);
    /// assert_eq!(ime, open);
    /// ```
    pub fn merge<T: AsRef<str>>(&mut self, other: &InputMethod<T>) {
        if let InputMethod::Enabled { .. } = self {
            return;
        }

        *self = other.to_owned();
    }

    /// 如果 [`InputMethod`] 处于打开状态，返回 true。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns true if the [`InputMethod`] is open.
    ///
    /// </details>
    pub fn is_enabled(&self) -> bool {
        matches!(self, Self::Enabled { .. })
    }
}

impl<T> InputMethod<T> {
    /// 把 [`InputMethod`] 转换为拥有所有权的版本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Turns an [`InputMethod`] into its owned version.
    ///
    /// </details>
    pub fn to_owned(&self) -> InputMethod
    where
        T: AsRef<str>,
    {
        match self {
            Self::Disabled => InputMethod::Disabled,
            Self::Enabled {
                cursor,
                purpose,
                preedit,
            } => InputMethod::Enabled {
                cursor: *cursor,
                purpose: *purpose,
                preedit: preedit.as_ref().map(Preedit::to_owned),
            },
        }
    }
}

/// 描述[输入法](https://en.wikipedia.org/wiki/Input_method)事件。
///
/// 它也被称为“组合事件（composition event）”。
///
/// 使用类拉丁键盘布局时，大多数按键只会产生一个
/// [`keyboard::Event::KeyPressed`](crate::keyboard::Event::KeyPressed)。
/// 然而，不可能为用户想要输入的每一个
/// unicode 字符都配备一个按键。操作系统采用的解决方案是
/// 允许用户改用 _一系列按键_ 来输入这些字符。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Describes [input method](https://en.wikipedia.org/wiki/Input_method) events.
///
/// This is also called a "composition event".
///
/// Most keypresses using a latin-like keyboard layout simply generate a
/// [`keyboard::Event::KeyPressed`](crate::keyboard::Event::KeyPressed).
/// However, one couldn't possibly have a key for every single
/// unicode character that the user might want to type. The solution operating systems employ is
/// to allow the user to type these using _a sequence of keypresses_ instead.
///
/// </details>
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Event {
    /// 输入法（IME）被打开时发出通知。
    ///
    /// 收到此事件之后，你可能会收到 [`Preedit`][Self::Preedit] 和
    /// [`Commit`][Self::Commit] 事件。你也应该开始执行与输入法相关的请求，
    /// 例如 [`Shell::request_input_method`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Notifies when the IME was opened.
    ///
    /// After getting this event you could receive [`Preedit`][Self::Preedit] and
    /// [`Commit`][Self::Commit] events. You should also start performing IME related requests
    /// like [`Shell::request_input_method`].
    ///
    /// </details>
    ///
    /// [`Shell::request_input_method`]: crate::Shell::request_input_method
    Opened,

    /// 需要在光标位置设置新的组合文本时发出通知。
    ///
    /// 该值是一个二元组，由预编辑字符串以及光标的起始位置和结束位置
    /// 组成。为 `None` 时，应该隐藏光标。`String` 为空字符串时，
    /// 表示预编辑内容已被清除。
    ///
    /// 光标范围按字节索引。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Notifies when a new composing text should be set at the cursor position.
    ///
    /// The value represents a pair of the preedit string and the cursor begin position and end
    /// position. When it's `None`, the cursor should be hidden. When `String` is an empty string
    /// this indicates that preedit was cleared.
    ///
    /// The cursor range is byte-wise indexed.
    ///
    /// </details>
    Preedit(String, Option<Range<usize>>),

    /// 需要向编辑器部件中插入文本时发出通知。
    ///
    /// 在此事件之前，会先发出一个空的 [`Self::Preedit`] 事件。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Notifies when text should be inserted into the editor widget.
    ///
    /// Right before this event, an empty [`Self::Preedit`] event will be issued.
    ///
    /// </details>
    Commit(String),

    /// 输入法（IME）被禁用时发出通知。
    ///
    /// 收到此事件之后，在下一个 [`Opened`][Self::Opened] 事件到来之前，
    /// 你不会再收到任何 [`Preedit`][Self::Preedit] 或
    /// [`Commit`][Self::Commit] 事件。你也应该
    /// 停止发出与输入法相关的请求（例如 [`Shell::request_input_method`]），并清除
    /// 待处理的预编辑文本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Notifies when the IME was disabled.
    ///
    /// After receiving this event you won't get any more [`Preedit`][Self::Preedit] or
    /// [`Commit`][Self::Commit] events until the next [`Opened`][Self::Opened] event. You should
    /// also stop issuing IME related requests like [`Shell::request_input_method`] and clear
    /// pending preedit text.
    ///
    /// </details>
    ///
    /// [`Shell::request_input_method`]: crate::Shell::request_input_method
    Closed,
}
