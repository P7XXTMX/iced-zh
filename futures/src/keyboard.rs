//! 监听键盘事件。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Listen to keyboard events.
//!
//! </details>
use crate::core;
use crate::core::keyboard::Event;
use crate::subscription::{self, Subscription};

/// 返回一个 [`Subscription`]，监听被忽略的键盘事件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Returns a [`Subscription`] that listens to ignored keyboard events.
///
/// </details>
pub fn listen() -> Subscription<Event> {
    #[derive(Hash)]
    struct Listen;

    subscription::filter_map(Listen, move |event| match event {
        subscription::Event::Interaction {
            event: core::Event::Keyboard(event),
            status: core::event::Status::Ignored,
            ..
        } => Some(event),
        _ => None,
    })
}
