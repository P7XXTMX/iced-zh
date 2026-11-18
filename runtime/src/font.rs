//! 加载并使用字体。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Load and use fonts.
//!
//! </details>
use crate::Action;
use crate::task::{self, Task};
use std::borrow::Cow;

/// 加载字体时发生的错误。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An error while loading a font.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {}

/// 根据字节数据加载字体。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Load a font from its bytes.
///
/// </details>
pub fn load(bytes: impl Into<Cow<'static, [u8]>>) -> Task<Result<(), Error>> {
    task::oneshot(|channel| Action::LoadFont {
        bytes: bytes.into(),
        channel,
    })
}
