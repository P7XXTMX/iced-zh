//! 构建触摸事件。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Build touch events.
//!
//! </details>
use crate::Point;

/// 触摸交互。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A touch interaction.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
#[allow(missing_docs)]
pub enum Event {
    /// 一次触摸交互开始了。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A touch interaction was started.
    ///
    /// </details>
    FingerPressed { id: Finger, position: Point },

    /// 一次正在进行的触摸交互发生了移动。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// An on-going touch interaction was moved.
    ///
    /// </details>
    FingerMoved { id: Finger, position: Point },

    /// 一次触摸交互结束了。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A touch interaction was ended.
    ///
    /// </details>
    FingerLifted { id: Finger, position: Point },

    /// 一次触摸交互被取消了。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A touch interaction was canceled.
    ///
    /// </details>
    FingerLost { id: Finger, position: Point },
}

/// 唯一标识符，表示触摸交互中的一根手指。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A unique identifier representing a finger on a touch interaction.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Finger(pub u64);
