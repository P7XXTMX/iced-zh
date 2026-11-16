//! 从渲染器控制窗口运行时。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Control the windowing runtime from a renderer.
//!
//! </details>
use std::sync::Arc;

/// 窗口外壳。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A windowing shell.
///
/// </details>
#[derive(Clone)]
pub struct Shell(Arc<dyn Notifier>);

impl Shell {
    /// 创建一个新的 [`Shell`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Shell`].
    ///
    /// </details>
    pub fn new(notifier: impl Notifier) -> Self {
        Self(Arc::new(notifier))
    }

    /// 创建一个无头 [`Shell`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a headless [`Shell`].
    ///
    /// </details>
    pub fn headless() -> Self {
        struct Headless;

        impl Notifier for Headless {
            fn tick(&self) {}
            fn request_redraw(&self) {}
            fn invalidate_layout(&self) {}
        }

        Self::new(Headless)
    }

    /// 请求 [`Shell`] 调用
    /// [`Renderer::tick`](crate::core::Renderer::tick)。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Requests for [`Renderer::tick`](crate::core::Renderer::tick) to
    /// be called by the [`Shell`].
    ///
    /// </details>
    pub fn tick(&self) {
        self.0.tick();
    }

    /// 请求重绘 [`Shell`] 的所有窗口。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Requests for all windows of the [`Shell`] to be redrawn.
    ///
    /// </details>
    pub fn request_redraw(&self) {
        self.0.request_redraw();
    }

    /// 请求重新计算 [`Shell`] 的所有布局。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Requests for all layouts of the [`Shell`] to be recomputed.
    ///
    /// </details>
    pub fn invalidate_layout(&self) {
        self.0.invalidate_layout();
    }
}

/// 可以把某些事件通知给外壳的类型。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A type that can notify a shell of certain events.
///
/// </details>
pub trait Notifier: Send + Sync + 'static {
    /// 请求 [`Shell`] 调用
    /// [`Renderer::tick`](crate::core::Renderer::tick)。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Requests for [`Renderer::tick`](crate::core::Renderer::tick) to
    /// be called by the [`Shell`].
    ///
    /// </details>
    fn tick(&self);

    /// 请求重绘 [`Shell`] 的所有窗口。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Requests for all windows of the [`Shell`] to be redrawn.
    ///
    /// </details>
    fn request_redraw(&self);

    /// 请求重新计算 [`Shell`] 的所有布局。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Requests for all layouts of the [`Shell`] to be recomputed.
    ///
    /// </details>
    fn invalidate_layout(&self);
}
