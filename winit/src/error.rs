use crate::core::backend;
use crate::futures::futures;

/// 运行应用程序时发生的错误。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An error that occurred while running an application.
///
/// </details>
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// 无法创建 futures 执行器。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The futures executor could not be created.
    ///
    /// </details>
    #[error("the futures executor could not be created")]
    ExecutorCreationFailed(futures::io::Error),

    /// 无法创建应用程序窗口。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The application window could not be created.
    ///
    /// </details>
    #[error("the application window could not be created")]
    WindowCreationFailed(winit::error::OsError),

    /// 无法创建应用程序的图形上下文。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The application graphics context could not be created.
    ///
    /// </details>
    #[error("the application graphics context could not be created")]
    GraphicsCreationFailed(backend::Error),
}

impl From<backend::Error> for Error {
    fn from(error: backend::Error) -> Error {
        Error::GraphicsCreationFailed(error)
    }
}
