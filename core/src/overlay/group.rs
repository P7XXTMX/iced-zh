use crate::layout;
use crate::mouse;
use crate::overlay;
use crate::renderer;
use crate::widget;
use crate::{Clipboard, Event, Layout, Overlay, Shell, Size};

/// 一个 [`Overlay`] 容器，显示多个覆盖层 [`overlay::Element`]
/// 子级。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An [`Overlay`] container that displays multiple overlay [`overlay::Element`]
/// children.
///
/// </details>
pub struct Group<'a, Message, Theme, Renderer> {
    children: Vec<overlay::Element<'a, Message, Theme, Renderer>>,
}

impl<'a, Message, Theme, Renderer> Group<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: 'a + crate::Renderer,
{
    /// 创建一个空的 [`Group`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates an empty [`Group`].
    ///
    /// </details>
    pub fn new() -> Self {
        Self::default()
    }

    /// 使用给定的元素创建一个 [`Group`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a [`Group`] with the given elements.
    ///
    /// </details>
    pub fn with_children(
        mut children: Vec<overlay::Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        use std::cmp;

        children.sort_unstable_by(|a, b| {
            a.as_overlay()
                .index()
                .partial_cmp(&b.as_overlay().index())
                .unwrap_or(cmp::Ordering::Equal)
        });

        Group { children }
    }

    /// 把 [`Group`] 转换成一个覆盖层 [`overlay::Element`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Turns the [`Group`] into an overlay [`overlay::Element`].
    ///
    /// </details>
    pub fn overlay(self) -> overlay::Element<'a, Message, Theme, Renderer> {
        overlay::Element::new(Box::new(self))
    }
}

impl<'a, Message, Theme, Renderer> Default
    for Group<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: 'a + crate::Renderer,
{
    fn default() -> Self {
        Self::with_children(Vec::new())
    }
}

impl<Message, Theme, Renderer> Overlay<Message, Theme, Renderer>
    for Group<'_, Message, Theme, Renderer>
where
    Renderer: crate::Renderer,
{
    fn layout(&mut self, renderer: &Renderer, bounds: Size) -> layout::Node {
        layout::Node::with_children(
            bounds,
            self.children
                .iter_mut()
                .map(|child| child.as_overlay_mut().layout(renderer, bounds))
                .collect(),
        )
    }

    fn update(
        &mut self,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
    ) {
        for (child, layout) in self.children.iter_mut().zip(layout.children()) {
            child
                .as_overlay_mut()
                .update(event, layout, cursor, renderer, clipboard, shell);
        }
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
    ) {
        for (child, layout) in self.children.iter().zip(layout.children()) {
            child
                .as_overlay()
                .draw(renderer, theme, style, layout, cursor);
        }
    }

    fn mouse_interaction(
        &self,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.children
            .iter()
            .zip(layout.children())
            .map(|(child, layout)| {
                child
                    .as_overlay()
                    .mouse_interaction(layout, cursor, renderer)
            })
            .max()
            .unwrap_or_default()
    }

    fn operate(
        &mut self,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        operation.traverse(&mut |operation| {
            self.children.iter_mut().zip(layout.children()).for_each(
                |(child, layout)| {
                    child.as_overlay_mut().operate(layout, renderer, operation);
                },
            );
        });
    }

    fn overlay<'a>(
        &'a mut self,
        layout: Layout<'a>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'a, Message, Theme, Renderer>> {
        let children = self
            .children
            .iter_mut()
            .zip(layout.children())
            .filter_map(|(child, layout)| {
                child.as_overlay_mut().overlay(layout, renderer)
            })
            .collect::<Vec<_>>();

        (!children.is_empty()).then(|| Group::with_children(children).overlay())
    }

    fn index(&self) -> f32 {
        self.children
            .first()
            .map(|child| child.as_overlay().index())
            .unwrap_or(1.0)
    }
}

impl<'a, Message, Theme, Renderer> From<Group<'a, Message, Theme, Renderer>>
    for overlay::Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: 'a + crate::Renderer,
{
    fn from(group: Group<'a, Message, Theme, Renderer>) -> Self {
        group.overlay()
    }
}
