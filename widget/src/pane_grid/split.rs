/// 把 [`PaneGrid`] 中的一个区域分成两个不同窗格的分隔条。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A divider that splits a region in a [`PaneGrid`] into two different panes.
///
/// </details>
///
/// [`PaneGrid`]: super::PaneGrid
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Split(pub(super) usize);
