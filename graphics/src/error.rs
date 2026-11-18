//! 了解创建图形后端时可能出现的问题。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! See what can go wrong when creating graphical backends.
//!
//! </details>

/// 创建应用程序的图形上下文时发生的错误。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An error that occurred while creating an application's graphical context.
///
/// </details>
#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum Error {
    /// 不支持所请求的后端版本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The requested backend version is not supported.
    ///
    /// </details>
    #[error("the requested backend version is not supported")]
    VersionNotSupported,

    /// 找不到任何符合条件的像素格式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Failed to find any pixel format that matches the criteria.
    ///
    /// </details>
    #[error("failed to find any pixel format that matches the criteria")]
    NoAvailablePixelFormat,

    /// 找不到合适的图形适配器或设备。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A suitable graphics adapter or device could not be found.
    ///
    /// </details>
    #[error("a suitable graphics adapter or device could not be found")]
    GraphicsAdapterNotFound {
        /// 发生错误的后端的名称
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The name of the backend where the error happened
        ///
        /// </details>
        backend: &'static str,
        /// 这个后端无法使用的原因
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The reason why this backend could not be used
        ///
        /// </details>
        reason: Reason,
    },

    /// 上下文的内部后端发生了错误
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// An error occurred in the context's internal backend
    ///
    /// </details>
    #[error("an error occurred in the context's internal backend")]
    BackendError(String),

    /// 发生了多个错误
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Multiple errors occurred
    ///
    /// </details>
    #[error("multiple errors occurred: {0:?}")]
    List(Vec<Self>),
}

/// 找不到图形适配器的原因
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The reason why a graphics adapter could not be found
///
/// </details>
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reason {
    /// 后端与偏好不匹配
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The backend did not match the preference
    ///
    /// </details>
    DidNotMatch {
        /// 偏好的后端
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The preferred backend
        ///
        /// </details>
        preferred_backend: String,
    },
    /// 创建后端的请求失败
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The request to create the backend failed
    ///
    /// </details>
    RequestFailed(String),
}
