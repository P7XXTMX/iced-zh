//! 显式分配图像以控制呈现。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Allocate images explicitly to control presentation.
//!
//! </details>
use crate::core::image::Handle;
use crate::futures::futures::channel::oneshot;
use crate::task::{self, Task};

pub use crate::core::image::{Allocation, Error};

/// 图像动作。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An image action.
///
/// </details>
#[derive(Debug)]
pub enum Action {
    /// 分配给定的 [`Handle`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Allocates the given [`Handle`].
    ///
    /// </details>
    Allocate(Handle, oneshot::Sender<Result<Allocation, Error>>),
}

/// 分配一个图像 [`Handle`]。
///
/// 显式取得 [`Allocation`] 之后，你就得到一个保证：
/// 使用这个 [`Handle`] 时，对应的图像会在下一帧
/// 立即绘制出来。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Allocates an image [`Handle`].
///
/// When you obtain an [`Allocation`] explicitly, you get the guarantee
/// that using a [`Handle`] will draw the corresponding image immediately
/// in the next frame.
///
/// </details>
pub fn allocate(handle: impl Into<Handle>) -> Task<Result<Allocation, Error>> {
    task::oneshot(|sender| crate::Action::Image(Action::Allocate(handle.into(), sender)))
}
