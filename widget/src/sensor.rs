//! 在内容进入和离开视野时生成消息。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Generate messages when content pops in and out of view.
//!
//! </details>
use crate::core::layout;
use crate::core::mouse;
use crate::core::overlay;
use crate::core::renderer;
use crate::core::time::{Duration, Instant};
use crate::core::widget;
use crate::core::widget::tree::{self, Tree};
use crate::core::window;
use crate::core::{
    self, Clipboard, Element, Event, Layout, Length, Pixels, Rectangle, Shell,
    Size, Vector, Widget,
};

/// 在其内容进入和离开视野时可以生成消息的部件。
///
/// 它甚至可以在给定的距离处提前通知你！
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A widget that can generate messages when its content pops in and out of view.
///
/// It can even notify you with anticipation at a given distance!
///
/// </details>
pub struct Sensor<
    'a,
    Key,
    Message,
    Theme = crate::Theme,
    Renderer = crate::Renderer,
> {
    content: Element<'a, Message, Theme, Renderer>,
    key: Key,
    on_show: Option<Box<dyn Fn(Size) -> Message + 'a>>,
    on_resize: Option<Box<dyn Fn(Size) -> Message + 'a>>,
    on_hide: Option<Message>,
    anticipate: Pixels,
    delay: Duration,
}

impl<'a, Message, Theme, Renderer> Sensor<'a, (), Message, Theme, Renderer>
where
    Renderer: core::Renderer,
{
    /// 使用给定的内容创建一个新的 [`Sensor`] 部件。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Sensor`] widget with the given content.
    ///
    /// </details>
    pub fn new(
        content: impl Into<Element<'a, Message, Theme, Renderer>>,
    ) -> Self {
        Self {
            content: content.into(),
            key: (),
            on_show: None,
            on_resize: None,
            on_hide: None,
            anticipate: Pixels::ZERO,
            delay: Duration::ZERO,
        }
    }
}

impl<'a, Key, Message, Theme, Renderer>
    Sensor<'a, Key, Message, Theme, Renderer>
where
    Key: self::Key,
    Renderer: core::Renderer,
{
    /// 设置内容进入视野时要产生的消息。
    ///
    /// 闭包会收到内容在那一刻的 [`Size`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the message to be produced when the content pops into view.
    ///
    /// The closure will receive the [`Size`] of the content in that moment.
    ///
    /// </details>
    pub fn on_show(mut self, on_show: impl Fn(Size) -> Message + 'a) -> Self {
        self.on_show = Some(Box::new(on_show));
        self
    }

    /// 设置内容在视野内改变 [`Size`] 时要产生的消息。
    ///
    /// 闭包会收到内容的新 [`Size`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the message to be produced when the content changes [`Size`] once its in view.
    ///
    /// The closure will receive the new [`Size`] of the content.
    ///
    /// </details>
    pub fn on_resize(
        mut self,
        on_resize: impl Fn(Size) -> Message + 'a,
    ) -> Self {
        self.on_resize = Some(Box::new(on_resize));
        self
    }

    /// 设置内容离开视野时要产生的消息。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the message to be produced when the content pops out of view.
    ///
    /// </details>
    pub fn on_hide(mut self, on_hide: Message) -> Self {
        self.on_hide = Some(on_hide);
        self
    }

    /// 设置 [`Sensor`] 部件的键，用于保持连续性。
    ///
    /// 如果键发生变化，[`Sensor`] 部件会再次触发。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the key of the [`Sensor`] widget, for continuity.
    ///
    /// If the key changes, the [`Sensor`] widget will trigger again.
    ///
    /// </details>
    pub fn key<K>(
        self,
        key: K,
    ) -> Sensor<'a, impl self::Key, Message, Theme, Renderer>
    where
        K: Clone + PartialEq + 'static,
    {
        Sensor {
            content: self.content,
            key: OwnedKey(key),
            on_show: self.on_show,
            on_resize: self.on_resize,
            on_hide: self.on_hide,
            anticipate: self.anticipate,
            delay: self.delay,
        }
    }

    /// 设置 [`Sensor`] 的键，用于保持连续性；使用引用。
    ///
    /// 如果键发生变化，[`Sensor`] 会再次触发。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the key of the [`Sensor`], for continuity; using a reference.
    ///
    /// If the key changes, the [`Sensor`] will trigger again.
    ///
    /// </details>
    pub fn key_ref<K>(
        self,
        key: &'a K,
    ) -> Sensor<'a, &'a K, Message, Theme, Renderer>
    where
        K: ToOwned + PartialEq<K::Owned> + ?Sized,
        K::Owned: 'static,
    {
        Sensor {
            content: self.content,
            key,
            on_show: self.on_show,
            on_resize: self.on_resize,
            on_hide: self.on_hide,
            anticipate: self.anticipate,
            delay: self.delay,
        }
    }

    /// 设置以 [`Pixels`] 为单位的距离，用于在内容进入视野之前
    /// 提前触发。
    ///
    /// 这非常适合在用户察觉之前，在幕后惰性加载
    /// 一个很长的可滚动区域中的条目！
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the distance in [`Pixels`] to use in anticipation of the
    /// content popping into view.
    ///
    /// This can be quite useful to lazily load items in a long scrollable
    /// behind the scenes before the user can notice it!
    ///
    /// </details>
    pub fn anticipate(mut self, distance: impl Into<Pixels>) -> Self {
        self.anticipate = distance.into();
        self
    }

    /// 设置在内容显示或隐藏之后，触发 [`on_show`] 或
    /// [`on_hide`] 事件之前要等待的时间。
    ///
    /// 与 [`key`] 结合使用时，可以用来对键的变化进行防抖。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the amount of time to wait before firing an [`on_show`] or
    /// [`on_hide`] event; after the content is shown or hidden.
    ///
    /// When combined with [`key`], this can be useful to debounce key changes.
    ///
    /// </details>
    ///
    /// [`on_show`]: Self::on_show
    /// [`on_hide`]: Self::on_hide
    /// [`key`]: Self::key
    pub fn delay(mut self, delay: impl Into<Duration>) -> Self {
        self.delay = delay.into();
        self
    }
}

#[derive(Debug, Clone)]
struct State<Key> {
    has_popped_in: bool,
    should_notify_at: Option<(bool, Instant)>,
    last_size: Option<Size>,
    last_key: Key,
}

impl<Key, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Sensor<'_, Key, Message, Theme, Renderer>
where
    Key: self::Key,
    Renderer: core::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State<Key::Owned>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State {
            has_popped_in: false,
            should_notify_at: None,
            last_size: None,
            last_key: self.key.to_owned(),
        })
    }

    fn children(&self) -> Vec<Tree> {
        vec![Tree::new(&self.content)]
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&[&self.content]);
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        if let Event::Window(window::Event::RedrawRequested(now)) = &event {
            let state = tree.state.downcast_mut::<State<Key::Owned>>();

            if state.has_popped_in && !self.key.eq(&state.last_key) {
                state.has_popped_in = false;
                state.should_notify_at = None;
                state.last_key = self.key.to_owned();
            }

            let bounds = layout.bounds();
            let top_left_distance = viewport.distance(bounds.position());

            let bottom_right_distance = viewport
                .distance(bounds.position() + Vector::from(bounds.size()));

            let distance = top_left_distance.min(bottom_right_distance);

            if self.on_show.is_none() {
                if let Some(on_resize) = &self.on_resize {
                    let size = bounds.size();

                    if Some(size) != state.last_size {
                        state.last_size = Some(size);
                        shell.publish(on_resize(size));
                    }
                }
            } else if state.has_popped_in {
                if distance <= self.anticipate.0 {
                    if let Some(on_resize) = &self.on_resize {
                        let size = bounds.size();

                        if Some(size) != state.last_size {
                            state.last_size = Some(size);
                            shell.publish(on_resize(size));
                        }
                    }
                } else if self.on_hide.is_some() {
                    state.has_popped_in = false;
                    state.should_notify_at = Some((false, *now + self.delay));
                }
            } else if distance <= self.anticipate.0 {
                let size = bounds.size();

                state.has_popped_in = true;
                state.should_notify_at = Some((true, *now + self.delay));
                state.last_size = Some(size);
            }

            match &state.should_notify_at {
                Some((has_popped_in, at)) if at <= now => {
                    if *has_popped_in {
                        if let Some(on_show) = &self.on_show {
                            shell.publish(on_show(layout.bounds().size()));
                        }
                    } else if let Some(on_hide) = self.on_hide.take() {
                        shell.publish(on_hide);
                    }

                    state.should_notify_at = None;
                }
                Some((_, at)) => {
                    shell.request_redraw_at(*at);
                }
                None => {}
            }
        }

        self.content.as_widget_mut().update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        );
    }

    fn size(&self) -> Size<Length> {
        self.content.as_widget().size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.content.as_widget().size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        self.content.as_widget_mut().layout(
            &mut tree.children[0],
            renderer,
            limits,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: layout::Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.content.as_widget().draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: core::Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        self.content.as_widget_mut().operate(
            &mut tree.children[0],
            layout,
            renderer,
            operation,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: core::Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            &tree.children[0],
            layout,
            cursor,
            viewport,
            renderer,
        )
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: core::Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: core::Vector,
    ) -> Option<overlay::Element<'b, Message, Theme, Renderer>> {
        self.content.as_widget_mut().overlay(
            &mut tree.children[0],
            layout,
            renderer,
            viewport,
            translation,
        )
    }
}

impl<'a, Key, Message, Theme, Renderer>
    From<Sensor<'a, Key, Message, Theme, Renderer>>
    for Element<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Key: self::Key + 'a,
    Renderer: core::Renderer + 'a,
    Theme: 'a,
{
    fn from(pop: Sensor<'a, Key, Message, Theme, Renderer>) -> Self {
        Element::new(pop)
    }
}

/// 部件的键。
///
/// 通常你不需要关心这个特质。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The key of a widget.
///
/// You should generally not need to care about this trait.
///
/// </details>
pub trait Key {
    /// 键的拥有所有权的版本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The owned version of the key.
    ///
    /// </details>
    type Owned: 'static;

    /// 返回键的拥有所有权的版本。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the owned version of the key.
    ///
    /// </details>
    fn to_owned(&self) -> Self::Owned;

    /// 把键与给定的拥有所有权的版本进行比较。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Compares the key with the given owned version.
    ///
    /// </details>
    fn eq(&self, other: &Self::Owned) -> bool;
}

impl<T> Key for &T
where
    T: ToOwned + PartialEq<T::Owned> + ?Sized,
    T::Owned: 'static,
{
    type Owned = T::Owned;

    fn to_owned(&self) -> <Self as Key>::Owned {
        ToOwned::to_owned(*self)
    }

    fn eq(&self, other: &Self::Owned) -> bool {
        *self == other
    }
}

struct OwnedKey<T>(T);

impl<T> Key for OwnedKey<T>
where
    T: PartialEq + Clone + 'static,
{
    type Owned = T;

    fn to_owned(&self) -> Self::Owned {
        self.0.clone()
    }

    fn eq(&self, other: &Self::Owned) -> bool {
        &self.0 == other
    }
}

impl<T> PartialEq<T> for OwnedKey<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &T) -> bool {
        &self.0 == other
    }
}

impl Key for () {
    type Owned = ();

    fn to_owned(&self) -> Self::Owned {}

    fn eq(&self, _other: &Self::Owned) -> bool {
        true
    }
}
