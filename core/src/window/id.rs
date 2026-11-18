use std::fmt;
use std::hash::Hash;
use std::sync::atomic::{self, AtomicU64};

/// 窗口的 id。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The id of the window.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Id(u64);

static COUNT: AtomicU64 = AtomicU64::new(1);

impl Id {
    /// 创建一个新的唯一窗口 [`Id`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new unique window [`Id`].
    ///
    /// </details>
    pub fn unique() -> Id {
        Id(COUNT.fetch_add(1, atomic::Ordering::Relaxed))
    }
}

impl fmt::Display for Id {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}
