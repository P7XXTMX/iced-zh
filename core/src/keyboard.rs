//! 监听键盘事件。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Listen to keyboard events.
//!
//! </details>
pub mod key;

mod event;
mod location;
mod modifiers;

pub use event::Event;
pub use key::Key;
pub use location::Location;
pub use modifiers::Modifiers;
