/// [`PaneGrid`] 中用于显示部件的矩形区域。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A rectangular region in a [`PaneGrid`] used to display widgets.
///
/// </details>
///
/// [`PaneGrid`]: super::PaneGrid
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Pane(pub(super) usize);
