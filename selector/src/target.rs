use crate::core::widget::Id;
use crate::core::widget::operation::{Focusable, Scrollable, TextInput};
use crate::core::{Rectangle, Size, Vector};

use std::any::Any;

/// 选择过程中产生的通用部件匹配结果。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A generic widget match produced during selection.
///
/// </details>
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq)]
pub enum Target {
    Container {
        id: Option<Id>,
        bounds: Rectangle,
        visible_bounds: Option<Rectangle>,
    },
    Focusable {
        id: Option<Id>,
        bounds: Rectangle,
        visible_bounds: Option<Rectangle>,
    },
    Scrollable {
        id: Option<Id>,
        bounds: Rectangle,
        visible_bounds: Option<Rectangle>,
        content: Size,
        translation: Vector,
    },
    TextInput {
        id: Option<Id>,
        bounds: Rectangle,
        visible_bounds: Option<Rectangle>,
        content: String,
    },
    Text {
        id: Option<Id>,
        bounds: Rectangle,
        visible_bounds: Option<Rectangle>,
        content: String,
    },
    Custom {
        id: Option<Id>,
        bounds: Rectangle,
        visible_bounds: Option<Rectangle>,
    },
}

impl Target {
    /// 返回 [`Target`] 的布局边界。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the layout bounds of the [`Target`].
    ///
    /// </details>
    pub fn bounds(&self) -> Rectangle {
        match self {
            Target::Container { bounds, .. }
            | Target::Focusable { bounds, .. }
            | Target::Scrollable { bounds, .. }
            | Target::TextInput { bounds, .. }
            | Target::Text { bounds, .. }
            | Target::Custom { bounds, .. } => *bounds,
        }
    }

    /// 返回 [`Target`] 的可见边界，以屏幕坐标表示。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the visible bounds of the [`Target`], in screen coordinates.
    ///
    /// </details>
    pub fn visible_bounds(&self) -> Option<Rectangle> {
        match self {
            Target::Container { visible_bounds, .. }
            | Target::Focusable { visible_bounds, .. }
            | Target::Scrollable { visible_bounds, .. }
            | Target::TextInput { visible_bounds, .. }
            | Target::Text { visible_bounds, .. }
            | Target::Custom { visible_bounds, .. } => *visible_bounds,
        }
    }
}

impl From<Candidate<'_>> for Target {
    fn from(candidate: Candidate<'_>) -> Self {
        match candidate {
            Candidate::Container {
                id,
                bounds,
                visible_bounds,
            } => Self::Container {
                id: id.cloned(),
                bounds,
                visible_bounds,
            },
            Candidate::Focusable {
                id,
                bounds,
                visible_bounds,
                ..
            } => Self::Focusable {
                id: id.cloned(),
                bounds,
                visible_bounds,
            },
            Candidate::Scrollable {
                id,
                bounds,
                visible_bounds,
                content,
                translation,
                ..
            } => Self::Scrollable {
                id: id.cloned(),
                bounds,
                visible_bounds,
                content,
                translation,
            },
            Candidate::TextInput {
                id,
                bounds,
                visible_bounds,
                state,
            } => Self::TextInput {
                id: id.cloned(),
                bounds,
                visible_bounds,
                content: state.text().into_owned(),
            },
            Candidate::Text {
                id,
                bounds,
                visible_bounds,
                content,
            } => Self::Text {
                id: id.cloned(),
                bounds,
                visible_bounds,
                content: content.to_owned(),
            },
            Candidate::Custom {
                id,
                bounds,
                visible_bounds,
                ..
            } => Self::Custom {
                id: id.cloned(),
                bounds,
                visible_bounds,
            },
        }
    }
}

impl Bounded for Target {
    fn bounds(&self) -> Rectangle {
        self.bounds()
    }

    fn visible_bounds(&self) -> Option<Rectangle> {
        self.visible_bounds()
    }
}

/// 一个选择候选项。
///
/// 它会被传给 [`Selector::select`](crate::Selector::select)。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A selection candidate.
///
/// This is provided to [`Selector::select`](crate::Selector::select).
///
/// </details>
#[allow(missing_docs)]
#[derive(Clone)]
pub enum Candidate<'a> {
    Container {
        id: Option<&'a Id>,
        bounds: Rectangle,
        visible_bounds: Option<Rectangle>,
    },
    Focusable {
        id: Option<&'a Id>,
        bounds: Rectangle,
        visible_bounds: Option<Rectangle>,
        state: &'a dyn Focusable,
    },
    Scrollable {
        id: Option<&'a Id>,
        bounds: Rectangle,
        content: Size,
        visible_bounds: Option<Rectangle>,
        translation: Vector,
        state: &'a dyn Scrollable,
    },
    TextInput {
        id: Option<&'a Id>,
        bounds: Rectangle,
        visible_bounds: Option<Rectangle>,
        state: &'a dyn TextInput,
    },
    Text {
        id: Option<&'a Id>,
        bounds: Rectangle,
        visible_bounds: Option<Rectangle>,
        content: &'a str,
    },
    Custom {
        id: Option<&'a Id>,
        bounds: Rectangle,
        visible_bounds: Option<Rectangle>,
        state: &'a dyn Any,
    },
}

impl<'a> Candidate<'a> {
    /// 返回 [`Candidate`] 的部件 [`Id`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the widget [`Id`] of the [`Candidate`].
    ///
    /// </details>
    pub fn id(&self) -> Option<&'a Id> {
        match self {
            Candidate::Container { id, .. }
            | Candidate::Focusable { id, .. }
            | Candidate::Scrollable { id, .. }
            | Candidate::TextInput { id, .. }
            | Candidate::Text { id, .. }
            | Candidate::Custom { id, .. } => *id,
        }
    }

    /// 返回 [`Candidate`] 的布局边界。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the layout bounds of the [`Candidate`].
    ///
    /// </details>
    pub fn bounds(&self) -> Rectangle {
        match self {
            Candidate::Container { bounds, .. }
            | Candidate::Focusable { bounds, .. }
            | Candidate::Scrollable { bounds, .. }
            | Candidate::TextInput { bounds, .. }
            | Candidate::Text { bounds, .. }
            | Candidate::Custom { bounds, .. } => *bounds,
        }
    }

    /// 返回 [`Candidate`] 的可见边界，以屏幕坐标表示。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the visible bounds of the [`Candidate`], in screen coordinates.
    ///
    /// </details>
    pub fn visible_bounds(&self) -> Option<Rectangle> {
        match self {
            Candidate::Container { visible_bounds, .. }
            | Candidate::Focusable { visible_bounds, .. }
            | Candidate::Scrollable { visible_bounds, .. }
            | Candidate::TextInput { visible_bounds, .. }
            | Candidate::Text { visible_bounds, .. }
            | Candidate::Custom { visible_bounds, .. } => *visible_bounds,
        }
    }
}

/// 有界类型同时具有布局边界和可见边界。
///
/// 这个特质让我们可以针对 [`Selector`](crate::Selector) 的
/// [`Output`](crate::Selector::Output) 编写泛型代码。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A bounded type has both layout bounds and visible bounds.
///
/// This trait lets us write generic code over the [`Output`](crate::Selector::Output)
/// of a [`Selector`](crate::Selector).
///
/// </details>
pub trait Bounded: std::fmt::Debug {
    /// 返回布局边界。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the layout bounds.
    ///
    /// </details>
    fn bounds(&self) -> Rectangle;

    /// 返回可见边界，以屏幕坐标表示。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the visible bounds, in screen coordinates.
    ///
    /// </details>
    fn visible_bounds(&self) -> Option<Rectangle>;
}

/// 一个文本匹配结果。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A text match.
///
/// </details>
#[allow(missing_docs)]
#[derive(Debug, Clone, PartialEq)]
pub enum Text {
    Raw {
        id: Option<Id>,
        bounds: Rectangle,
        visible_bounds: Option<Rectangle>,
    },
    Input {
        id: Option<Id>,
        bounds: Rectangle,
        visible_bounds: Option<Rectangle>,
    },
}

impl Text {
    /// 返回 [`Text`] 的布局边界。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the layout bounds of the [`Text`].
    ///
    /// </details>
    pub fn bounds(&self) -> Rectangle {
        match self {
            Text::Raw { bounds, .. } | Text::Input { bounds, .. } => *bounds,
        }
    }

    /// 返回 [`Text`] 的可见边界，以屏幕坐标表示。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the visible bounds of the [`Text`], in screen coordinates.
    ///
    /// </details>
    pub fn visible_bounds(&self) -> Option<Rectangle> {
        match self {
            Text::Raw { visible_bounds, .. } | Text::Input { visible_bounds, .. } => {
                *visible_bounds
            }
        }
    }
}

impl Bounded for Text {
    fn bounds(&self) -> Rectangle {
        self.bounds()
    }

    fn visible_bounds(&self) -> Option<Rectangle> {
        self.visible_bounds()
    }
}
