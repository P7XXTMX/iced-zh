use crate::container;
use crate::core::{self, Element, Widget};

/// [`Pane`] 的控件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The controls of a [`Pane`].
///
/// </details>
///
/// [`Pane`]: super::Pane
pub struct Controls<'a, Message, Theme = crate::Theme, Renderer = crate::Renderer>
where
    Theme: container::Catalog,
{
    pub(super) full: Element<'a, Message, Theme, Renderer>,
    pub(super) compact: Option<Element<'a, Message, Theme, Renderer>>,
}

impl<'a, Message, Theme, Renderer> Controls<'a, Message, Theme, Renderer>
where
    Theme: container::Catalog,
{
    /// 使用给定的内容创建一个新的 [`Controls`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Controls`] with the given content.
    ///
    /// </details>
    pub fn new(content: impl Widget<Message, Theme, Renderer> + 'a) -> Self {
        Self {
            full: content._boxed(),
            compact: None,
        }
    }

    /// 创建一个带有完整版本和紧凑版本的新 [`Controls`]。
    /// 如果空间不足以在不重叠的情况下显示完整版本，
    /// 就会改为显示紧凑版本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Controls`] with a full and compact variant.
    /// If there is not enough room to show the full variant without overlap,
    /// then the compact variant will be shown instead.
    ///
    /// </details>
    pub fn dynamic(
        full: impl Widget<Message, Theme, Renderer> + 'a,
        compact: impl Widget<Message, Theme, Renderer> + 'a,
    ) -> Self {
        Self {
            full: full._boxed(),
            compact: Some(compact._boxed()),
        }
    }
}

impl<'a, Message, Theme, Renderer> From<Element<'a, Message, Theme, Renderer>>
    for Controls<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: container::Catalog + 'a,
    Renderer: core::Renderer,
{
    fn from(value: Element<'a, Message, Theme, Renderer>) -> Self {
        Self::new(value)
    }
}
