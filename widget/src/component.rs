//! 使用 Elm 架构构建并复用自定义部件。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Build and reuse custom widgets using The Elm Architecture.
//!
//! </details>
use crate::Action;
use crate::core::event;
use crate::core::layout::{self, Layout};
use crate::core::mouse;
use crate::core::overlay;
use crate::core::renderer;
use crate::core::shell;
use crate::core::widget;
use crate::core::widget::tree::{self, Tree};
use crate::core::window;
use crate::core::{Element, Event, Length, Rectangle, Shell, Size, Vector, Widget};

use std::cell::{Cell, RefCell};

/// 使用 Elm 架构的可复用自定义部件。
///
/// [`Component`] 让你可以像编写带有封装状态的 `iced` 应用程序
/// 那样实现自定义部件。
///
/// 换句话说，[`Component`] 让你可以把 `iced` 应用程序变成
/// 自定义部件，并且无需繁琐的接线就能嵌入它们。
///
/// [`Component`] 产生的部件可以触发 [`Event`](Component::Event)
/// 并更新 [`Component`] 的内部状态。
///
/// 此外，[`Component`] 还能产生 `Message`，把相关的交互
/// 通知给父应用程序。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A reusable, custom widget that uses The Elm Architecture.
///
/// A [`Component`] allows you to implement custom widgets as if they were
/// `iced` applications with encapsulated state.
///
/// In other words, a [`Component`] allows you to turn `iced` applications into
/// custom widgets and embed them without cumbersome wiring.
///
/// A [`Component`] produces widgets that may fire an [`Event`](Component::Event)
/// and update the internal state of the [`Component`].
///
/// Additionally, a [`Component`] is capable of producing a `Message` to notify
/// the parent application of any relevant interactions.
///
/// </details>
///
/// # 状态（State）
/// 组件可以用两种方式存储状态：要么作为特质实现者
/// 内部的数据，要么放在由运行时管理、并提供给特质方法的
/// [`State`][Component::State] 类型中。这两种方式并不
/// 互斥，而且优缺点正好相反。
///
/// 例如，如果某个状态被位于部件树不同分支的多个组件所需要，
/// 那么让它们的公共祖先存储该状态并向下传递
/// 会更方便。
///
/// 另一方面，如果某个状态只被组件自身需要，
/// 你可以把它存为其内部 [`State`][Component::State] 的一部分。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A component can store its state in one of two ways: either as data within the
/// implementor of the trait, or in a type [`State`][Component::State] that is managed
/// by the runtime and provided to the trait methods. These two approaches are not
/// mutually exclusive and have opposite pros and cons.
///
/// For instance, if a piece of state is needed by multiple components that reside
/// in different branches of the tree, then it's more convenient to let a common
/// ancestor store it and pass it down.
///
/// On the other hand, if a piece of state is only needed by the component itself,
/// you can store it as part of its internal [`State`][Component::State].
///
/// </details>
pub trait Component<'a, Message, Theme = crate::Theme, Renderer = crate::Renderer> {
    /// 此 [`Component`] 的内部状态。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The internal state of this [`Component`].
    ///
    /// </details>
    type State: Default + 'static;

    /// 此 [`Component`] 在内部处理的事件类型。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The type of event this [`Component`] handles internally.
    ///
    /// </details>
    type Event: 'static;

    /// 处理一个 [`Event`](Component::Event)，并相应地更新 [`Component`] 的状态。
    ///
    /// 它可以为父应用程序产生一条 `Message`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Processes an [`Event`](Component::Event) and updates the [`Component`] state accordingly.
    ///
    /// It can produce a `Message` for the parent application.
    ///
    /// </details>
    fn update(
        &self,
        state: &mut Self::State,
        event: Self::Event,
        renderer: &Renderer,
    ) -> Option<Message>;

    /// 产生 [`Component`] 的部件，这些部件可以在用户交互时
    /// 触发 [`Event`](Component::Event)。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Produces the widgets of the [`Component`], which may trigger an [`Event`](Component::Event)
    /// on user interaction.
    ///
    /// </details>
    fn view(&self, state: &Self::State) -> impl crate::Widget<Self::Event, Theme, Renderer> + 'a;

    /// 监听运行时 [`Event`]，并据此执行一个 [`Action`]。
    ///
    /// 如果该 [`Action`] 发布了一个 [`Component::Event`]，它会被立即
    /// 送入 [`update`](Self::update)。
    ///
    /// 默认返回 [`Action::none`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Listens to a runtime [`Event`] and performs an [`Action`] as a result.
    ///
    /// If the [`Action`] publishes a [`Component::Event`], it will be immediately fed
    /// to [`update`](Self::update).
    ///
    /// By default, it returns [`Action::none`].
    ///
    /// </details>
    fn listen(
        &self,
        _state: &Self::State,
        _event: &Event,
        _bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Action<Self::Event> {
        Action::none()
    }

    /// 返回 [`Component`] 当前的 [`mouse::Interaction`]。
    ///
    /// 此交互会覆盖 [`Component`] 的 [`view`](Self::view)
    /// 所产生的任何交互。
    ///
    /// 默认返回 [`mouse::Interaction::None`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current [`mouse::Interaction`] of the [`Component`].
    ///
    /// This interaction will override any interaction produced by the [`view`](Self::view)
    /// of the [`Component`].
    ///
    /// By default, it returns [`mouse::Interaction::None`].
    ///
    /// </details>
    fn mouse_interaction(&self, _state: &Self::State) -> mouse::Interaction {
        mouse::Interaction::None
    }

    /// 让当前的 [`Component`] 与保存在部件树中的内部
    /// [`State`](Self::State) 进行协调。
    ///
    /// 每当部件树发生变化时，此方法都会被调用。你可以利用它来
    /// 检测 [`Component`] 的变化并作出响应。
    ///
    /// 默认什么都不做。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Reconciles the current [`Component`] with its internal [`State`](Self::State) persisted
    /// in the widget tree.
    ///
    /// This method will be called every time the widget tree changes. You can leverage it to
    /// detect and react to changes in the [`Component`].
    ///
    /// By default, it does nothing.
    ///
    /// </details>
    fn diff(&mut self, _state: &mut Self::State) {}

    /// 在 [`Component`] 上运行给定的 [`widget::Operation`]。
    ///
    /// 默认什么都不做。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Run the provided [`widget::Operation`] on the [`Component`].
    ///
    /// By default, it does nothing.
    ///
    /// </details>
    fn operate(
        &self,
        _state: &mut Self::State,
        _bounds: Rectangle,
        _operation: &mut dyn widget::Operation,
    ) {
    }
}

/// 把 [`Component`] 的实现者转换为可以嵌入任何应用程序的
/// [`Element`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Turns an implementor of [`Component`] into an [`Element`] that can be
/// embedded in any application.
///
/// </details>
pub fn component<'a, C, Message, Theme, Renderer>(
    component: C,
) -> impl Widget<Message, Theme, Renderer> + 'a
where
    C: Component<'a, Message, Theme, Renderer> + 'a,
    C::State: 'static,
    Message: 'a,
    Theme: 'a,
    Renderer: 'a,
{
    Instance {
        component,
        view: crate::space()._boxed(),
        limits: layout::Limits::new(Size::ZERO, Size::INFINITE),
        is_outdated: Cell::new(true),
        has_overlay: false,
    }
}

struct Instance<'a, C, Message, Theme, Renderer>
where
    C: Component<'a, Message, Theme, Renderer> + 'a,
{
    component: C,
    view: Element<'a, C::Event, Theme, Renderer>,
    limits: layout::Limits,
    is_outdated: Cell<bool>,
    has_overlay: bool,
}

struct Internal<State, Event> {
    state: State,
    events: shell::Bus<Event>,
}

impl<'a, C, Message, Theme, Renderer> widget::Meta for Instance<'a, C, Message, Theme, Renderer> where
    C: Component<'a, Message, Theme, Renderer> + 'a
{
}

impl<'a, C, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Instance<'a, C, Message, Theme, Renderer>
where
    C: Component<'a, Message, Theme, Renderer> + 'a,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<RefCell<Internal<C::State, C::Event>>>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(RefCell::new(Internal {
            state: C::State::default(),
            events: shell::Bus::<C::Event>::new(),
        }))
    }

    fn diff(&mut self, tree: &mut Tree) {
        let mut internal = tree
            .state
            .downcast_mut::<RefCell<Internal<C::State, C::Event>>>()
            .borrow_mut();

        self.component.diff(&mut internal.state);

        if self.is_outdated.get() {
            self.view = self.component.view(&internal.state)._boxed();
            drop(internal);

            tree.diff_children(std::slice::from_mut(&mut self.view));

            self.is_outdated.set(false);
            self.limits = layout::Limits::new(Size::ZERO, Size::INFINITE);
        }
    }

    fn size(&self) -> Size<Length> {
        self.view.size()
    }

    fn layout(&mut self, tree: &mut Tree, renderer: &Renderer, limits: &layout::Limits) {
        if &self.limits != limits {
            self.limits = *limits;
            self.view.layout(&mut tree.children[0], renderer, limits);
        }

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
        let mut internal = tree
            .state
            .downcast_mut::<RefCell<Internal<C::State, C::Event>>>()
            .borrow_mut();

        let action = self
            .component
            .listen(&internal.state, event, layout.bounds(), cursor);

        let (publish, redraw_request, event_status) = action.into_inner();

        shell.request_redraw_at(redraw_request);

        if let event::Status::Captured = event_status {
            shell.capture_event();
        }

        if let Some(event) = publish {
            let _ = internal.events.push(event);
        }

        let mut invalidation = shell::Invalidation::None;

        if !shell.is_event_captured() {
            let mut local_shell = shell.local(&mut internal.events);

            self.view.update(
                &mut tree.children[0],
                event,
                layout,
                cursor,
                renderer,
                &mut local_shell,
                viewport,
            );

            if local_shell.is_event_captured() {
                shell.capture_event();
            }

            shell.request_redraw_at(local_shell.redraw_request());
            shell.request_input_method(local_shell.input_method());
            shell.clipboard_mut().merge(local_shell.clipboard_mut());

            invalidation = local_shell.invalidation();
        }

        if internal.events.is_empty() && matches!(invalidation, shell::Invalidation::None) {
            return;
        }

        let Internal { state, events } = &mut *internal;

        for (event, receipt) in events.drain() {
            if let Some(message) = self.component.update(state, event, renderer) {
                shell.forward(message, receipt);
            }
        }

        let previous_sizing = self.view.size();

        self.view = self.component.view(state)._boxed();
        drop(internal);

        tree.diff_children(std::slice::from_mut(&mut self.view));

        let previous_size = tree.size;

        self.view
            .layout(&mut tree.children[0], renderer, &self.limits);

        tree.size = tree.children[0].size;

        let new_sizing = self.view.size();

        // We must invalidate application layout in 2 instances:
        //
        // 1. The size hint of the component changes. Other widgets
        //    may change layout behavior.
        //
        // 2. The size hint of the component is not fluid for any axis
        //    and the component has changed size. The new size may
        //    push other widgets around.
        // 在以下 2 种情况下，必须使应用程序的布局失效：
        //
        // 1. 组件的尺寸提示发生了变化。其他部件
        //    的布局行为可能随之改变。
        //
        // 2. 组件的尺寸提示在某个轴上不是流式的，
        //    并且组件的尺寸发生了变化。新的尺寸可能会
        //    把其他部件挤开。
        if new_sizing != previous_sizing {
            shell.invalidate_widgets();
        } else if (new_sizing.width.fill_factor() == 0 || new_sizing.height.fill_factor() == 0)
            && previous_size != tree.size
        {
            shell.invalidate_layout();
        } else {
            shell.invalidate_overlay();
        }

        self.is_outdated.set(false);

        if let Event::Window(window::Event::RedrawRequested(_)) = event {
            let mut internal = tree
                .state
                .downcast_mut::<RefCell<Internal<C::State, C::Event>>>()
                .borrow_mut();

            let mut local_shell = shell.local(&mut internal.events);

            self.view.update(
                &mut tree.children[0],
                event,
                layout,
                cursor,
                renderer,
                &mut local_shell,
                viewport,
            );

            if internal.events.is_empty() {
                return;
            }
        }

        shell.request_redraw();
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
        self.view.draw(
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
        let internal = tree
            .state
            .downcast_ref::<RefCell<Internal<C::State, C::Event>>>()
            .borrow();

        let interaction = self.component.mouse_interaction(&internal.state);

        if interaction != mouse::Interaction::None {
            return interaction;
        }

        self.view
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
        {
            let internal = tree
                .state
                .downcast_mut::<RefCell<Internal<C::State, C::Event>>>()
                .get_mut();

            self.component
                .operate(&mut internal.state, layout.bounds(), operation);
        }

        self.view
            .operate(&mut tree.children[0], layout, viewport, renderer, operation);
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
        window: Size,
    ) -> Vec<overlay::Element<'b, Message, Theme, Renderer>> {
        let overlays = self.view.overlay(
            &mut tree.children[0],
            layout,
            renderer,
            viewport,
            translation,
            window,
        );

        self.has_overlay = !overlays.is_empty();

        let internal = tree
            .state
            .downcast_ref::<RefCell<Internal<C::State, C::Event>>>();

        overlays
            .into_iter()
            .map(|raw| {
                overlay::Element::new(Box::new(Overlay {
                    component: &self.component,
                    internal,
                    is_outdated: &self.is_outdated,
                    raw,
                }))
            })
            .collect()
    }
}

struct Overlay<'a, 'b, C, Message, Theme, Renderer>
where
    C: Component<'a, Message, Theme, Renderer>,
{
    component: &'b C,
    internal: &'b RefCell<Internal<C::State, C::Event>>,
    is_outdated: &'b Cell<bool>,
    raw: overlay::Element<'b, C::Event, Theme, Renderer>,
}

impl<'a, 'b, C, Message, Theme, Renderer> overlay::Overlay<Message, Theme, Renderer>
    for Overlay<'a, 'b, C, Message, Theme, Renderer>
where
    C: Component<'a, Message, Theme, Renderer>,
{
    fn update(
        &mut self,
        event: &Event,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
    ) {
        let mut internal = self.internal.borrow_mut();
        let mut local_shell = shell.local(&mut internal.events);

        self.raw
            .as_overlay_mut()
            .update(event, cursor, renderer, &mut local_shell);

        if local_shell.is_event_captured() {
            shell.capture_event();
        }

        shell.invalidate(local_shell.invalidation());
        shell.request_redraw_at(local_shell.redraw_request());
        shell.request_input_method(local_shell.input_method());
        shell.clipboard_mut().merge(local_shell.clipboard_mut());

        if internal.events.is_empty() {
            return;
        }

        let Internal { state, events } = &mut *internal;

        for (event, receipt) in events.drain() {
            if let Some(message) = self.component.update(state, event, renderer) {
                shell.forward(message, receipt);
            }
        }

        self.is_outdated.set(true);

        shell.invalidate_layout_with(shell::Diff::Perform);
        shell.request_redraw();
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        cursor: mouse::Cursor,
    ) {
        self.raw.as_overlay().draw(renderer, theme, style, cursor);
    }

    fn mouse_interaction(&self, cursor: mouse::Cursor, renderer: &Renderer) -> mouse::Interaction {
        self.raw.as_overlay().mouse_interaction(cursor, renderer)
    }

    fn index(&self) -> f32 {
        self.raw.as_overlay().index()
    }

    fn operate(&mut self, renderer: &Renderer, operation: &mut dyn widget::Operation) {
        self.raw.as_overlay_mut().operate(renderer, operation);
    }

    fn overlay<'c>(
        &'c mut self,
        renderer: &Renderer,
    ) -> Vec<overlay::Element<'c, Message, Theme, Renderer>> {
        let overlays = self.raw.as_overlay_mut().overlay(renderer);

        if overlays.is_empty() {
            return Vec::new();
        }

        overlays
            .into_iter()
            .map(|raw| {
                overlay::Element::new(Box::new(Overlay {
                    component: self.component,
                    internal: self.internal,
                    is_outdated: self.is_outdated,
                    raw,
                }))
            })
            .collect()
    }
}
