//! 存储图像。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Store images.
//!
//! </details>
use iced_core::Size;

use std::fmt::Debug;

/// 存储缓存的图像数据，供渲染时使用
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Stores cached image data for use in rendering
///
/// </details>
pub trait Storage {
    /// [`Storage`] 中 [`Entry`] 的类型。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The type of an [`Entry`] in the [`Storage`].
    ///
    /// </details>
    type Entry: Entry;

    /// 上传或移除 [`Self::Entry`] 时提供的状态。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// State provided to upload or remove a [`Self::Entry`].
    ///
    /// </details>
    type State<'a>;

    /// 上传某个 [`Self::Entry`] 的图像数据。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Upload the image data of a [`Self::Entry`].
    ///
    /// </details>
    fn upload(
        &mut self,
        width: u32,
        height: u32,
        data: &[u8],
        state: &mut Self::State<'_>,
    ) -> Option<Self::Entry>;

    /// 从 [`Storage`] 中移除一个 [`Self::Entry`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Remove a [`Self::Entry`] from the [`Storage`].
    ///
    /// </details>
    fn remove(&mut self, entry: &Self::Entry, state: &mut Self::State<'_>);
}

/// 某个 [`Storage`] 中的条目，
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An entry in some [`Storage`],
///
/// </details>
pub trait Entry: Debug {
    /// [`Entry`] 的 [`Size`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Size`] of the [`Entry`].
    ///
    /// </details>
    fn size(&self) -> Size<u32>;
}
