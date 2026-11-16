pub use crate::Overlay;

use crate::mouse;
use crate::renderer;
use crate::shell;
use crate::widget;
use crate::{Event, Shell};

/// 泛型的 [`Overlay`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A generic [`Overlay`].
///
/// </details>
pub struct Element<'a, Message, Theme, Renderer> {
    overlay: Box<dyn Overlay<Message, Theme, Renderer> + 'a>,
}

impl<'a, Message, Theme, Renderer> Element<'a, Message, Theme, Renderer> {
    /// 创建一个包含给定 [`Overlay`] 的新 [`Element`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Element`] containing the given [`Overlay`].
    ///
    /// </details>
    pub fn new(overlay: Box<dyn Overlay<Message, Theme, Renderer> + 'a>) -> Self {
        Self { overlay }
    }

    /// 返回 [`Element`] 的 [`Overlay`] 的引用，
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns a reference to the [`Overlay`] of the [`Element`],
    ///
    /// </details>
    pub fn as_overlay(&self) -> &dyn Overlay<Message, Theme, Renderer> {
        self.overlay.as_ref()
    }

    /// 返回 [`Element`] 的 [`Overlay`] 的可变引用，
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns a mutable reference to the [`Overlay`] of the [`Element`],
    ///
    /// </details>
    pub fn as_overlay_mut(&mut self) -> &mut dyn Overlay<Message, Theme, Renderer> {
        self.overlay.as_mut()
    }

    /// 对 [`Element`] 产生的消息应用一个变换。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Applies a transformation to the produced message of the [`Element`].
    ///
    /// </details>
    pub fn map<B>(self, f: &'a dyn Fn(Message) -> B) -> Element<'a, B, Theme, Renderer>
    where
        Message: 'a,
        Theme: 'a,
        Renderer: 'a,
        B: 'a,
    {
        Element {
            overlay: Box::new(Map::new(self.overlay, f)),
        }
    }
}

struct Map<'a, A, B, Theme, Renderer> {
    content: Box<dyn Overlay<A, Theme, Renderer> + 'a>,
    mapper: &'a dyn Fn(A) -> B,
}

impl<'a, A, B, Theme, Renderer> Map<'a, A, B, Theme, Renderer> {
    pub fn new(
        content: Box<dyn Overlay<A, Theme, Renderer> + 'a>,
        mapper: &'a dyn Fn(A) -> B,
    ) -> Map<'a, A, B, Theme, Renderer> {
        Map { content, mapper }
    }
}

impl<A, B, Theme, Renderer> Overlay<B, Theme, Renderer> for Map<'_, A, B, Theme, Renderer> {
    fn operate(&mut self, renderer: &Renderer, operation: &mut dyn widget::Operation) {
        self.content.operate(renderer, operation);
    }

    fn update(
        &mut self,
        event: &Event,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, B>,
    ) {
        let mut local_messages = shell::Bus::new();
        let mut local_shell = shell.local(&mut local_messages);

        self.content
            .update(event, cursor, renderer, &mut local_shell);

        shell.merge(local_shell, self.mapper);
    }

    fn mouse_interaction(&self, cursor: mouse::Cursor, renderer: &Renderer) -> mouse::Interaction {
        self.content.mouse_interaction(cursor, renderer)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        cursor: mouse::Cursor,
    ) {
        self.content.draw(renderer, theme, style, cursor);
    }

    fn overlay<'a>(&'a mut self, renderer: &Renderer) -> Vec<Element<'a, B, Theme, Renderer>> {
        self.content
            .overlay(renderer)
            .into_iter()
            .map(|overlay| overlay.map(self.mapper))
            .collect()
    }

    fn index(&self) -> f32 {
        self.content.index()
    }
}
