//! 在你的应用程序中查找并查询部件。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Find and query widgets in your applications.
//!
//! </details>
pub use iced_selector::{Bounded, Candidate, Selector, Target, Text, id, is_focused};

use crate::Task;
use crate::task;

/// 查找一个与给定 [`Selector`] 匹配的部件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Finds a widget matching the given [`Selector`].
///
/// </details>
pub fn find<S>(selector: S) -> Task<Option<S::Output>>
where
    S: Selector + Send + 'static,
    S::Output: Send + Clone + 'static,
{
    task::widget(selector.find())
}

/// 查找所有与给定 [`Selector`] 匹配的部件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Finds all widgets matching the given [`Selector`].
///
/// </details>
pub fn find_all<S>(selector: S) -> Task<Vec<S::Output>>
where
    S: Selector + Send + 'static,
    S::Output: Send + Clone + 'static,
{
    task::widget(selector.find_all())
}
