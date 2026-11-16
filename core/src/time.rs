//! 跟踪时间，原生平台和 Web 平台都适用！
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Keep track of time, both in native and web platforms!
//!
//! </details>

pub use web_time::Duration;
pub use web_time::Instant;
pub use web_time::SystemTime;

/// 创建一个表示给定毫秒数的 [`Duration`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a [`Duration`] representing the given amount of milliseconds.
///
/// </details>
pub fn milliseconds(milliseconds: u64) -> Duration {
    Duration::from_millis(milliseconds)
}

/// 创建一个表示给定秒数的 [`Duration`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a [`Duration`] representing the given amount of seconds.
///
/// </details>
pub fn seconds(seconds: u64) -> Duration {
    Duration::from_secs(seconds)
}

/// 创建一个表示给定分钟数的 [`Duration`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a [`Duration`] representing the given amount of minutes.
///
/// </details>
pub fn minutes(minutes: u64) -> Duration {
    seconds(minutes * 60)
}

/// 创建一个表示给定小时数的 [`Duration`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a [`Duration`] representing the given amount of hours.
///
/// </details>
pub fn hours(hours: u64) -> Duration {
    minutes(hours * 60)
}

/// 创建一个表示给定天数的 [`Duration`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a [`Duration`] representing the given amount of days.
///
/// </details>
pub fn days(days: u64) -> Duration {
    hours(days * 24)
}
