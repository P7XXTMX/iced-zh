//! 绘制文本并与之交互。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Draw and interact with text.
//!
//! </details>
mod rich;

pub use crate::core::text::{Fragment, Highlighter, IntoFragment, Span};
pub use crate::core::widget::text::*;
pub use rich::Rich;

/// 一段文本。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A bunch of text.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # pub type State = ();
/// # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
/// use iced::widget::text;
/// use iced::color;
///
/// enum Message {
///     // ...
/// }
///
/// fn view(state: &State) -> Element<'_, Message> {
///     text("Hello, this is iced!")
///         .size(20)
///         .color(color!(0x0000ff))
///         .into()
/// }
/// ```
pub type Text<'a, Theme = crate::Theme, Renderer = crate::Renderer> =
    crate::core::widget::Text<'a, Theme, Renderer>;
