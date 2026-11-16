//! 用于制作动画视图的部件。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! A widget to make animated views.
//!
//! </details>
use std::hash::{DefaultHasher, Hash, Hasher};

use crate::core::animation::{Animation, Float};
use crate::core::layout::{self, Layout};
use crate::core::mouse;
use crate::core::overlay;
use crate::core::renderer;
use crate::core::shell;
use crate::core::time::Instant;
use crate::core::widget::{self, Operation, Tree, tree};
use crate::core::{self, Event, Length, Rectangle, Shell, Size, Vector, Widget};

/// [`Transition`] 的逻辑。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The logic of a [`Transition`].
///
/// </details>
pub trait Program: 'static {
    /// [`Program`] 进行动画的值的类型。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The type of value that the [`Program`] animates.
    ///
    /// </details>
    type Value: Copy + 'static;

    /// 在给定的时刻，让 [`Program`] 从当前状态过渡到
    /// 给定的值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Transitions the [`Program`] from its current state towards the given value at
    /// the given time.
    ///
    /// </details>
    fn go(&mut self, value: Self::Value, now: Instant);

    /// 如果 [`Program`] 当前正在进行中，返回 true。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns true if the [`Program`] is currently in progress.
    ///
    /// </details>
    fn is_animating(&self, now: Instant) -> bool;
}

impl<I> Program for Animation<I>
where
    I: Float + Clone + Copy + PartialEq + 'static,
{
    type Value = I;

    fn go(&mut self, value: Self::Value, now: Instant) {
        self.go_mut(value, now);
    }

    fn is_animating(&self, now: Instant) -> bool {
        self.is_animating(now)
    }
}

/// 可以用来为其内容添加动画的部件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A widget that can be used to animate its contents.
///
/// </details>
pub struct Transition<'a, Message, W, P>
where
    P: Program,
{
    init: Box<dyn Fn() -> P + 'a>,
    view: Box<dyn Fn(&P, Instant) -> W + 'a>,
    on_finish: Option<Box<dyn Fn() -> Message + 'a>>,
    element: Option<W>,
    next_element: Option<W>,
    last_limits: layout::Limits,
    key: Key,
    id: Option<widget::Id>,
    value: P::Value,
}

impl<'a, Message, W, P> Transition<'a, Message, W, P>
where
    P: Program,
{
    /// 创建一个新的 [`Transition`]。
    ///
    /// `init` 闭包用于初始化动画。
    ///
    /// `view` 闭包会收到动画和一个 [`Instant`]，可以用来对值进行插值。
    /// 在到达给定的 `value` 之前，它每一帧都会被调用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Transition`].
    ///
    /// The `init` closure will be used to initialize an animation.
    ///
    /// The `view` closure will receive the animation and an [`Instant`], which can be used for interpolating values.
    /// This will be called every frame until the given `value` is reached.
    ///
    /// </details>
    pub fn new(
        init: impl Fn() -> P + 'a,
        value: P::Value,
        view: impl Fn(&P, Instant) -> W + 'a,
    ) -> Self {
        Self {
            init: Box::new(init),
            view: Box::new(view),
            on_finish: None,
            element: None,
            next_element: None,
            last_limits: layout::Limits::new(Size::ZERO, Size::ZERO),
            key: Key::default(),
            id: None,
            value,
        }
    }

    /// 设置 [`Transition`] 的 [`widget::Id`]。
    ///
    /// 之后可以使用这个 [`widget::Id`] 通过 [`reset`] 重置 [`Animation`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`widget::Id`] of the [`Transition`].
    ///
    /// The [`widget::Id`] can subsequently be used to reset the [`Animation`] via [`reset`].
    ///
    /// </details>
    pub fn id(mut self, id: impl Into<widget::Id>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// 设置 [`Transition`] 部件的键，用于保持连续性。
    ///
    /// 改变键会重置 [`Animation`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the key of the [`Transition`] widget for continuity.
    ///
    /// Changing the key will reset the [`Animation`].
    ///
    /// </details>
    pub fn key(mut self, key: impl Hash) -> Self {
        self.key = Key::new(key);
        self
    }

    /// 设置 [`Transition`] 完成动画时要产生的消息。
    ///
    /// 注意：如果 [`Animation`] 被设置为无限循环，这条消息永远不会产生！
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the message that will be produced when the [`Transition`] has finished animating.
    ///
    /// Note that if an [`Animation`] is set to loop forever, the message will never be produced!
    ///
    /// </details>
    pub fn on_finish(self, on_finish: Message) -> Self
    where
        Message: Clone + 'a,
    {
        self.on_finish_maybe(on_finish)
    }

    /// 如果为 [`Some`]，设置 [`Transition`] 完成动画时要产生的消息。
    ///
    /// 注意：如果 [`Animation`] 被设置为无限循环，这条消息永远不会产生！
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the message that will be produced when the [`Transition`] has finished animating, if [`Some`].
    ///
    /// Note that if an [`Animation`] is set to loop forever, the message will never be produced!
    ///
    /// </details>
    pub fn on_finish_maybe(mut self, on_finish: impl Into<Option<Message>>) -> Self
    where
        Message: Clone + 'a,
    {
        self.on_finish = on_finish
            .into()
            .map(|on_finish| Box::new(move || on_finish.clone()) as _);

        self
    }

    /// 设置 [`Transition`] 完成动画时要产生的消息。
    ///
    /// 它与 [`Transition::on_finish`] 类似，但使用闭包来产生
    /// 消息。
    ///
    /// 这个闭包只会在 [`Transition`] 真正完成动画时被调用，
    /// 因此，如果创建结果消息的开销较大，这个方法可以用来
    /// 减少开销。
    ///
    /// 注意：如果 [`Animation`] 被设置为无限循环，这条消息永远不会产生！
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the message that will be produced when the [`Transition`] has finished animating.
    ///
    /// This is analogous to [`Transition::on_finish`], but using a closure to produce
    /// the message.
    ///
    /// This closure will only be called when the [`Transition`] has actually finished animating and,
    /// therefore, this method is useful to reduce overhead if creating the resulting
    /// message is slow.
    ///
    /// Note that if an [`Animation`] is set to loop forever, the message will never be produced!
    ///
    /// </details>
    pub fn on_finish_with(mut self, on_finish: impl Fn() -> Message + 'a) -> Self {
        self.on_finish = Some(Box::new(on_finish));
        self
    }
}

impl<Message, W, P> widget::Meta for Transition<'_, Message, W, P> where P: Program {}

impl<Message, W, Theme, Renderer, P> Widget<Message, Theme, Renderer>
    for Transition<'_, Message, W, P>
where
    P: Program,
    W: Widget<Message, Theme, Renderer>,
{
    fn size(&self) -> Size<Length> {
        self.element.size()
    }

    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State<P>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State {
            animation: (self.init)(),
            instant: Instant::now(),
            key: self.key,
            should_reset: false,
            size: None,
        })
    }

    fn diff(&mut self, tree: &mut Tree) {
        let State::<P> {
            animation,
            key: old_key,
            should_reset,
            instant,
            ..
        } = tree.state.downcast_mut();

        if *old_key != self.key {
            *old_key = self.key;
            *animation = (self.init)();
            *should_reset = false;
        }

        if let Some(next_element) = self.next_element.take() {
            self.element = Some(next_element);
        } else {
            self.element = Some((self.view)(animation, *instant));
        }

        tree.diff_children(std::slice::from_mut(&mut self.element));
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        self.last_limits = *limits;

        self.element
            .layout(&mut tree.children[0], renderer, &limits.loose());

        tree.size = tree.children[0].size;
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        if let core::Event::Window(core::window::Event::RedrawRequested(redraw)) = event {
            let State::<P> {
                animation,
                instant,
                should_reset,
                size,
                ..
            } = tree.state.downcast_mut();

            if instant == redraw {
                shell.request_redraw();
            } else {
                let was_animating = animation.is_animating(*instant);

                if *should_reset {
                    *animation = (self.init)();
                    *should_reset = false;
                }

                *instant = *redraw;
                animation.go(self.value, *instant);

                let is_animating = animation.is_animating(*instant);
                let just_finished = was_animating && !is_animating;

                if is_animating || just_finished {
                    let size = *size;

                    let mut new = (self.view)(animation, *instant);
                    tree.diff_children(&mut [&mut new]);

                    let new_size = new.size();

                    if size != Some(new_size) {
                        self.next_element = Some(new);
                        shell.invalidate_layout_with(shell::Diff::Perform);

                        let state = tree.state.downcast_mut::<State<P>>();
                        state.size = Some(new_size);
                    } else {
                        self.element = Some(new);
                        self.element
                            .layout(&mut tree.children[0], renderer, &self.last_limits);
                    }

                    shell.request_redraw();
                }

                if just_finished && let Some(on_finish) = &self.on_finish {
                    shell.publish(on_finish());
                }
            }
        }

        self.element.update(
            &mut tree.children[0],
            event,
            layout,
            cursor,
            renderer,
            shell,
            viewport,
        );
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.element.draw(
            &tree.children[0],
            renderer,
            theme,
            style,
            layout,
            cursor,
            viewport,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.element
            .mouse_interaction(&tree.children[0], layout, cursor, viewport, renderer)
    }

    fn operate(
        &mut self,
        tree: &mut Tree,
        layout: Layout,
        viewport: &Rectangle,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        let mut should_reset = ShouldReset(false);
        operation.custom(self.id.as_ref(), layout.bounds(), &mut should_reset);

        if should_reset.0 {
            tree.state.downcast_mut::<State<P>>().should_reset = true;
        }

        self.element
            .operate(&mut tree.children[0], layout, viewport, renderer, operation);
    }

    fn overlay<'a>(
        &'a mut self,
        tree: &'a mut Tree,
        layout: Layout,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
        window: Size,
    ) -> Vec<overlay::Element<'a, Message, Theme, Renderer>> {
        self.element.overlay(
            &mut tree.children[0],
            layout,
            renderer,
            viewport,
            translation,
            window,
        )
    }
}

struct State<P: Program> {
    animation: P,
    instant: Instant,
    key: Key,
    should_reset: bool,
    size: Option<Size<Length>>,
}

#[derive(Default, Clone, Copy, Hash, PartialEq, Eq)]
struct Key(u64);

impl Key {
    fn new(data: impl Hash) -> Self {
        let mut hasher = DefaultHasher::new();
        data.hash(&mut hasher);
        Self(hasher.finish())
    }
}

struct ShouldReset(bool);

/// 重置 [`Transition`] 的 [`Animation`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Reset the [`Animation`] of a [`Transition`].
///
/// </details>
pub fn reset<Message>(id: impl Into<widget::Id>) -> iced_runtime::Task<Message>
where
    Message: iced_runtime::futures::MaybeSend + 'static,
{
    let id = id.into();
    iced_runtime::task::widget(reset_raw(id)).discard()
}

/// 用于重置 [`Transition`] 的 [`Animation`] 的 [`Operation`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An [`Operation`] to reset the [`Animation`] of a [`Transition`].
///
/// </details>
pub fn reset_raw(id: impl Into<widget::Id>) -> impl Operation {
    struct Reset(widget::Id);

    impl Operation for Reset {
        fn traverse(&mut self, operate: &mut dyn FnMut(&mut dyn Operation<()>)) {
            operate(self);
        }

        fn custom(
            &mut self,
            id: Option<&widget::Id>,
            _bounds: Rectangle,
            state: &mut dyn std::any::Any,
        ) {
            if id == Some(&self.0)
                && let Some(ShouldReset(should_reset)) = state.downcast_mut()
            {
                *should_reset = true;
            }
        }
    }

    Reset(id.into())
}
