//! 访问原生系统。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Access the native system.
//!
//! </details>
use crate::core::theme;
use crate::futures::futures::channel::oneshot;
use crate::futures::subscription::{self, Subscription};
use crate::task::{self, Task};

/// 要在系统上执行的操作。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An operation to be performed on the system.
///
/// </details>
#[derive(Debug)]
pub enum Action {
    /// 发送可用的系统信息。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Send available system information.
    ///
    /// </details>
    GetInformation(oneshot::Sender<Information>),

    /// 发送当前的系统主题模式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Send the current system theme mode.
    ///
    /// </details>
    GetTheme(oneshot::Sender<theme::Mode>),

    /// 通知运行时系统主题已经改变。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Notify to the runtime that the system theme has changed.
    ///
    /// </details>
    NotifyTheme(theme::Mode),
}

/// 包含系统的相关信息（例如系统名称、处理器、内存、图形适配器）。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Contains information about the system (e.g. system name, processor, memory, graphics adapter).
///
/// </details>
#[derive(Clone, Debug)]
pub struct Information {
    /// 操作系统名称
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The operating system name
    ///
    /// </details>
    pub system_name: Option<String>,
    /// 操作系统内核版本
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Operating system kernel version
    ///
    /// </details>
    pub system_kernel: Option<String>,
    /// 完整的操作系统版本
    ///
    /// 示例：
    /// - MacOS 10.15 Catalina
    /// - Windows 10 Pro
    /// - Ubuntu 20.04 LTS (Focal Fossa)
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Long operating system version
    ///
    /// Examples:
    /// - MacOS 10.15 Catalina
    /// - Windows 10 Pro
    /// - Ubuntu 20.04 LTS (Focal Fossa)
    ///
    /// </details>
    pub system_version: Option<String>,
    /// 简短的操作系统版本号
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Short operating system version number
    ///
    /// </details>
    pub system_short_version: Option<String>,
    /// 详细的处理器型号信息
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Detailed processor model information
    ///
    /// </details>
    pub cpu_brand: String,
    /// 处理器的物理核心数
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The number of physical cores on the processor
    ///
    /// </details>
    pub cpu_cores: Option<usize>,
    /// 内存总大小，以字节为单位
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Total RAM size, in bytes
    ///
    /// </details>
    pub memory_total: u64,
    /// 本进程使用的内存，以字节为单位
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Memory used by this process, in bytes
    ///
    /// </details>
    pub memory_used: Option<u64>,
    /// 用于渲染的底层图形后端
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Underlying graphics backend for rendering
    ///
    /// </details>
    pub graphics_backend: String,
    /// 当前使用的图形适配器的型号信息
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Model information for the active graphics adapter
    ///
    /// </details>
    pub graphics_adapter: String,
}

/// 返回可用的系统信息。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Returns available system information.
///
/// </details>
pub fn information() -> Task<Information> {
    task::oneshot(|channel| crate::Action::System(Action::GetInformation(channel)))
}

/// 返回当前的系统主题。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Returns the current system theme.
///
/// </details>
pub fn theme() -> Task<theme::Mode> {
    task::oneshot(|sender| crate::Action::System(Action::GetTheme(sender)))
}

/// 订阅系统主题的变化。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Subscribes to system theme changes.
///
/// </details>
pub fn theme_changes() -> Subscription<theme::Mode> {
    #[derive(Hash)]
    struct ThemeChanges;

    subscription::filter_map(ThemeChanges, |event| {
        let subscription::Event::SystemThemeChanged(mode) = event else {
            return None;
        };

        Some(mode)
    })
}
