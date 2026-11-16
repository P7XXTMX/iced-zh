//! 在运行时配置 [`Backend`](crate::core::Backend)。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Configure a [`Backend`](crate::core::Backend) at runtime.
//!
//! </details>
use crate::Task;
use crate::core::backend;
use crate::futures::futures::channel::oneshot;
use crate::task;

/// 后端操作。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An backend operation.
///
/// </details>
#[derive(Debug)]
pub enum Action {
    /// 切换当前应用程序的 [`backend::Settings`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Switches the [`backend::Settings`] of the current application.
    ///
    /// </details>
    Configure(
        backend::Settings,
        oneshot::Sender<Result<(), backend::Error>>,
    ),
}

/// 返回一个切换当前应用程序 [`backend::Settings`] 的 [`Task`]。
///
/// 可以利用它在运行时切换渲染器。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Returns a [`Task`] that switches the [`backend::Settings`] of the current application.
///
/// This can be leveraged to switch renderers at runtime.
///
/// </details>
pub fn configure(settings: backend::Settings) -> Task<Result<(), backend::Error>> {
    task::oneshot(|sender| crate::Action::Backend(Action::Configure(settings, sender)))
}
