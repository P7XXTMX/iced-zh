/// 鼠标光标的交互形态。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The interaction of a mouse cursor.
///
/// </details>
#[derive(Debug, Eq, PartialEq, Clone, Copy, PartialOrd, Ord, Default)]
#[allow(missing_docs)]
pub enum Interaction {
    #[default]
    None,
    Hidden,
    Idle,
    ContextMenu,
    Help,
    Pointer,
    Progress,
    Wait,
    Cell,
    Crosshair,
    Text,
    Alias,
    Copy,
    Move,
    NoDrop,
    NotAllowed,
    Grab,
    Grabbing,
    ResizingHorizontally,
    ResizingVertically,
    ResizingDiagonallyUp,
    ResizingDiagonallyDown,
    ResizingColumn,
    ResizingRow,
    AllScroll,
    ZoomIn,
    ZoomOut,
}
