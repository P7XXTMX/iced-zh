//! 处理鼠标事件。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Handle mouse events.
//!
//! </details>
pub mod click;

mod button;
mod cursor;
mod event;
mod interaction;

pub use button::Button;
pub use click::Click;
pub use cursor::Cursor;
pub use event::{Event, ScrollDelta};
pub use interaction::Interaction;
