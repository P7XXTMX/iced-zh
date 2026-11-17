//! 实现你自己的事件循环来驱动用户界面。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Implement your own event loop to drive a user interface.
//!
//! </details>
use crate::core::event::{self, Event};
use crate::core::keyboard;
use crate::core::layout;
use crate::core::mouse;
use crate::core::overlay;
use crate::core::renderer;
use crate::core::shell;
use crate::core::text;
use crate::core::widget;
use crate::core::widget::operation::{self, Operation};
use crate::core::window;
use crate::core::{
    Clipboard, InputMethod, Layout, Point, Rectangle, Shell, Size, Vector, Widget, Window,
};

/// 一组具有特定 [`Layout`] 的可交互图形元素。
///
/// 它可以被更新和绘制。
///
/// Iced 尽量不规定你该如何编写事件循环。如何在你的系统中
/// 使用这个类型，完全由你决定。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A set of interactive graphical elements with a specific [`Layout`].
///
/// It can be updated and drawn.
///
/// Iced tries to avoid dictating how to write your event loop. You are in
/// charge of using this type in your system in any way you want.
///
/// </details>
///
/// <a id="example"></a>
///
/// # 示例（Example）
/// [`integration`] 示例使用 [`UserInterface`] 把 Iced 集成到一个
/// 已有的图形应用程序中。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The [`integration`] example uses a [`UserInterface`] to integrate Iced in an
/// existing graphical application.
///
/// </details>
///
/// [`integration`]: https://github.com/iced-rs/iced/tree/master/examples/integration
pub struct UserInterface<'a, Message, Theme, Renderer> {
    root: widget::Element<'a, Message, Theme, Renderer>,
    state: widget::Tree,
    overlay: Option<mouse::Interaction>,
    bounds: Size,
    last_click: Option<mouse::Click>,
    selection: Option<(Point, Point)>,
}

impl<'a, Message, Theme, Renderer> UserInterface<'a, Message, Theme, Renderer>
where
    Message: 'a,
    Theme: 'a,
    Renderer: crate::core::Renderer + 'a,
{
    /// 为一个 [`Widget`] 构建用户界面。
    ///
    /// 如果使用从上一个 [`UserInterface`] 实例取得的 [`Cache`]，
    /// 它可以避免昂贵的计算。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Builds a user interface for a [`Widget`].
    ///
    /// It is able to avoid expensive computations when using a [`Cache`]
    /// obtained from a previous instance of a [`UserInterface`].
    ///
    /// </details>
    ///
    /// <a id="example-1"></a>
    ///
    /// # 示例（Example）
    /// 假设我们想为[之前编写的计数器示例](index.html#usage)
    /// 构建一个 [`UserInterface`]。下面是搭建应用程序循环的
    /// 一种朴素做法：
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Imagine we want to build a [`UserInterface`] for
    /// [the counter example that we previously wrote](index.html#usage). Here
    /// is naive way to set up our application loop:
    ///
    /// </details>
    ///
    /// ```no_run
    /// # mod iced_wgpu {
    /// #     pub type Renderer = ();
    /// # }
    /// #
    /// # pub struct Counter;
    /// #
    /// # impl Counter {
    /// #     pub fn new() -> Self { Counter }
    /// #     pub fn view(&self) -> iced_core::Element<(), (), Renderer> { unimplemented!() }
    /// #     pub fn update(&mut self, _: ()) {}
    /// # }
    /// use iced_runtime::core::shell;
    /// use iced_runtime::core::window;
    /// use iced_runtime::core::Size;
    /// use iced_runtime::user_interface::{self, UserInterface};
    /// use iced_wgpu::Renderer;
    ///
    /// // Initialization
    /// // 初始化
    /// let mut counter = Counter::new();
    /// let mut cache = user_interface::Cache::new();
    /// let mut renderer = Renderer::default();
    /// let mut window = window::Headless; // This should be a proper window, like a `winit` one
    /// let mut waker = shell::Waker::noop();
    /// let mut window_size = Size::new(1024.0, 768.0);
    ///
    /// // Application loop
    /// // 应用程序循环
    /// loop {
    ///     // Process system events here...
    ///     // 在这里处理系统事件……
    ///
    ///     // Build the user interface
    ///     // 构建用户界面
    ///     let user_interface = UserInterface::build(
    ///         counter.view(),
    ///         window_size,
    ///         cache,
    ///         &mut renderer,
    ///     );
    ///
    ///     // Update and draw the user interface here...
    ///     // 在这里更新并绘制用户界面……
    ///     // ...
    ///
    ///     // Obtain the cache for the next iteration
    ///     // 取得用于下一次迭代的缓存
    ///     cache = user_interface.into_cache();
    /// }
    /// ```
    pub fn build(
        root: impl Widget<Message, Theme, Renderer> + 'a,
        bounds: Size,
        cache: Cache,
        renderer: &mut Renderer,
    ) -> Self {
        let Cache {
            mut state,
            last_click,
            selection,
        } = cache;

        let mut root = root._boxed();
        state.diff(&mut root);

        root.layout(
            &mut state,
            renderer,
            &layout::Limits::new(Size::ZERO, bounds),
        );

        UserInterface {
            root,
            state,
            overlay: None,
            bounds,
            last_click,
            selection,
        }
    }

    /// 处理给定的每一个 [`Event`]，以此更新 [`UserInterface`]。
    ///
    /// 它返回因用户交互而可能产生的__消息__。
    /// 你应该把这些消息交给你的__更新逻辑__。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Updates the [`UserInterface`] by processing each provided [`Event`].
    ///
    /// It returns __messages__ that may have been produced as a result of user
    /// interactions. You should feed these to your __update logic__.
    ///
    /// </details>
    ///
    /// # 示例（Example）
    /// 让我们补全[上一个示例](#example)，使我们的
    /// [计数器](index.html#usage)能够改变状态：
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Let's allow our [counter](index.html#usage) to change state by
    /// completing [the previous example](#example):
    ///
    /// </details>
    ///
    /// ```no_run
    /// # mod iced_wgpu {
    /// #     pub type Renderer = ();
    /// # }
    /// #
    /// # pub struct Counter;
    /// #
    /// # impl Counter {
    /// #     pub fn new() -> Self { Counter }
    /// #     pub fn view(&self) -> iced_core::Element<(), (), Renderer> { unimplemented!() }
    /// #     pub fn update(&mut self, _: ()) {}
    /// # }
    /// use iced_runtime::core::mouse;
    /// use iced_runtime::core::shell;
    /// use iced_runtime::core::window;
    /// use iced_runtime::core::Size;
    /// use iced_runtime::user_interface::{self, UserInterface};
    /// use iced_wgpu::Renderer;
    ///
    /// let mut counter = Counter::new();
    /// let mut cache = user_interface::Cache::new();
    /// let mut renderer = Renderer::default();
    /// let mut window = window::Headless; // This should be a proper window, like a `winit` one
    /// let mut waker = shell::Waker::noop();
    /// let mut window_size = Size::new(1024.0, 768.0);
    /// let mut cursor = mouse::Cursor::default();
    ///
    /// // Initialize our event storage
    /// // 初始化我们的事件存储
    /// let mut events = Vec::new();
    /// let mut messages = shell::Bus::new();
    ///
    /// loop {
    ///     // Obtain system events...
    ///     // 获取系统事件……
    ///
    ///     let mut user_interface = UserInterface::build(
    ///         counter.view(),
    ///         window_size,
    ///         cache,
    ///         &mut renderer,
    ///     );
    ///
    ///     // Update the user interface
    ///     // 更新用户界面
    ///     let (state, event_statuses) = user_interface.update(
    ///         &window,
    ///         &waker,
    ///         &events,
    ///         cursor,
    ///         &mut renderer,
    ///         &mut messages
    ///     );
    ///
    ///     cache = user_interface.into_cache();
    ///
    ///     // Process the produced messages
    ///     // 处理产生的消息
    ///     for (message, _receipt) in messages.drain() {
    ///         counter.update(message);
    ///     }
    /// }
    /// ```
    pub fn update(
        &mut self,
        window: &dyn Window,
        waker: &shell::Waker,
        events: &[Event],
        cursor: mouse::Cursor,
        renderer: &mut Renderer,
        messages: &mut shell::Bus<Message>,
    ) -> (State, Vec<event::Status>) {
        let mut status = shell::Invalidation::None;
        let mut redraw_request = window::RedrawRequest::Wait;
        let mut input_method = InputMethod::Disabled;
        let mut clipboard = Clipboard::new();
        let mut layout = Layout::new(self.state.size);
        let viewport = Rectangle::with_size(self.bounds);

        let overlay = self.root.overlay(
            &mut self.state,
            layout,
            renderer,
            &viewport,
            Vector::ZERO,
            self.bounds,
        );

        let mut maybe_overlay = (!overlay.is_empty()).then(|| overlay::Group::new(overlay));

        let (base_cursor, overlay_statuses, overlay_interaction) = if maybe_overlay.is_some() {
            let mut overlay = maybe_overlay.as_mut().unwrap();
            let mut event_statuses = Vec::new();

            for event in events {
                let mut shell = Shell::new(window, waker.clone(), messages);

                overlay.update(event, cursor, renderer, &mut shell);

                let invalidation = shell.invalidation();
                status = status.max(invalidation);
                redraw_request = redraw_request.min(shell.redraw_request());

                event_statuses.push(shell.event_status());
                input_method.merge(shell.input_method());
                clipboard.merge(shell.clipboard_mut());

                if let shell::Invalidation::Layout(_) | shell::Invalidation::Overlay = invalidation
                {
                    drop(maybe_overlay);

                    if let shell::Invalidation::Layout(diff) = invalidation {
                        match diff {
                            shell::Diff::Perform => {
                                self.root.diff(&mut self.state);
                            }
                            shell::Diff::Skip => {}
                        }

                        self.root.layout(
                            &mut self.state,
                            renderer,
                            &layout::Limits::new(Size::ZERO, self.bounds),
                        );

                        layout = Layout::new(self.state.size);
                    }

                    maybe_overlay = {
                        let overlay = self.root.overlay(
                            &mut self.state,
                            layout,
                            renderer,
                            &viewport,
                            Vector::ZERO,
                            self.bounds,
                        );

                        (!overlay.is_empty()).then(|| overlay::Group::new(overlay))
                    };

                    if maybe_overlay.is_none() {
                        event_statuses.resize(events.len(), event::Status::Ignored);
                        break;
                    }

                    overlay = maybe_overlay.as_mut().unwrap();
                }
            }

            let (base_cursor, interaction) = if let Some(overlay) = maybe_overlay.as_mut() {
                let interaction = cursor
                    .position()
                    .map(|cursor_position| {
                        overlay
                            .mouse_interaction(mouse::Cursor::Available(cursor_position), renderer)
                    })
                    .unwrap_or_default();

                if interaction == mouse::Interaction::None {
                    (cursor, mouse::Interaction::None)
                } else {
                    (cursor.levitate(), interaction)
                }
            } else {
                (cursor, mouse::Interaction::None)
            };

            self.overlay = maybe_overlay.as_ref().map(|_| interaction);

            (base_cursor, event_statuses, interaction)
        } else {
            self.overlay = None;

            (
                cursor,
                vec![event::Status::Ignored; events.len()],
                mouse::Interaction::None,
            )
        };

        drop(maybe_overlay);

        let event_statuses = events
            .iter()
            .zip(overlay_statuses)
            .map(|(event, overlay_status)| {
                if matches!(overlay_status, event::Status::Captured) {
                    return overlay_status;
                }

                let mut shell = Shell::new(window, waker.clone(), messages);

                self.root.update(
                    &mut self.state,
                    event,
                    layout,
                    base_cursor,
                    renderer,
                    &mut shell,
                    &viewport,
                );

                if shell.event_status() == event::Status::Captured {
                    self.overlay = None;
                }

                let invalidation = shell.invalidation();

                if let shell::Invalidation::Layout(_) | shell::Invalidation::Overlay = invalidation
                {
                    if let shell::Invalidation::Layout(diff) = invalidation {
                        match diff {
                            shell::Diff::Perform => {
                                self.root.diff(&mut self.state);
                            }
                            shell::Diff::Skip => {}
                        }

                        self.root.layout(
                            &mut self.state,
                            renderer,
                            &layout::Limits::new(Size::ZERO, self.bounds),
                        );

                        layout = Layout::new(self.state.size);
                    }

                    let overlay = self.root.overlay(
                        &mut self.state,
                        layout,
                        renderer,
                        &viewport,
                        Vector::ZERO,
                        self.bounds,
                    );

                    if !overlay.is_empty() {
                        let mut overlay = overlay::Group::new(overlay);

                        self.overlay = Some(overlay.mouse_interaction(cursor, renderer));
                    }
                }

                status = status.max(invalidation);
                redraw_request = redraw_request.min(shell.redraw_request());
                input_method.merge(shell.input_method());
                clipboard.merge(shell.clipboard_mut());

                shell.event_status()
            })
            .collect();

        for (event, &status) in events.iter().zip(&event_statuses) {
            match (event, status) {
                (
                    Event::Mouse(mouse::Event::ButtonPressed(mouse::Button::Left)),
                    event::Status::Ignored,
                ) => {
                    if let Some(position) = cursor.position() {
                        self.last_click = Some(mouse::Click::new(
                            position,
                            mouse::Button::Left,
                            self.last_click,
                        ));

                        self.selection = Some((position, Point::ORIGIN));
                    }
                }
                (Event::Mouse(mouse::Event::ButtonReleased(mouse::Button::Left)), _) => {
                    self.selection = None;
                }
                (
                    Event::Keyboard(keyboard::Event::KeyPressed {
                        key,
                        modifiers,
                        repeat: false,
                        ..
                    }),
                    event::Status::Ignored,
                ) if matches!(key.as_ref(), keyboard::Key::Character("c"))
                    && modifiers.command() =>
                {
                    let mut copy = operation::text::copy();
                    self.operate(renderer, &mut operation::black_box(&mut copy));

                    if let operation::Outcome::Some(text) = copy.finish()
                        && !text.is_empty()
                    {
                        clipboard.write = Some(text.into());
                    }
                }
                _ => {}
            }
        }

        if let Some((start, end)) = self.selection.as_mut()
            && let Some(click) = self.last_click
            && let Some(position) = cursor.position()
            && (position != *end
                || events.iter().any(|event| {
                    matches!(event, Event::Window(window::Event::RedrawRequested { .. }))
                }))
        {
            *end = position;

            let mut select = operation::text::select(
                *start,
                *end,
                match click.kind() {
                    mouse::click::Kind::Single => text::Target::Character,
                    mouse::click::Kind::Double => text::Target::Word,
                    mouse::click::Kind::Triple => text::Target::Line,
                },
            );

            self.operate(renderer, &mut select);
            redraw_request = window::RedrawRequest::NextFrame;
        }

        let mouse_interaction = if overlay_interaction == mouse::Interaction::None {
            self.root
                .mouse_interaction(&self.state, layout, base_cursor, &viewport, renderer)
        } else {
            overlay_interaction
        };

        let change = match status {
            shell::Invalidation::None => Change::None,
            shell::Invalidation::Overlay => Change::Overlay,
            shell::Invalidation::Layout(_) => Change::Layout,
            shell::Invalidation::Widgets => return (State::Outdated, event_statuses),
        };

        (
            State::Updated {
                mouse_interaction,
                redraw_request,
                input_method,
                clipboard,
                change,
            },
            event_statuses,
        )
    }

    /// 使用给定的 [`Renderer`] 绘制 [`UserInterface`]。
    ///
    /// 它返回当前的 [`mouse::Interaction`]。你应该据此在你的系统中
    /// 更新鼠标光标的图标。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Draws the [`UserInterface`] with the provided [`Renderer`].
    ///
    /// It returns the current [`mouse::Interaction`]. You should update the
    /// icon of the mouse cursor accordingly in your system.
    ///
    /// </details>
    ///
    /// [`Renderer`]: crate::core::Renderer
    ///
    /// # 示例（Example）
    /// [补全上一个示例](#example-1)之后，我们终于可以绘制
    /// 我们的[计数器](index.html#usage)了：
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// We can finally draw our [counter](index.html#usage) by
    /// [completing the last example](#example-1):
    ///
    /// </details>
    ///
    /// ```no_run
    /// # mod iced_wgpu {
    /// #     pub type Renderer = ();
    /// #     pub type Theme = ();
    /// # }
    /// #
    /// # pub struct Counter;
    /// #
    /// # impl Counter {
    /// #     pub fn new() -> Self { Counter }
    /// #     pub fn view(&self) -> Element<(), (), Renderer> { unimplemented!() }
    /// #     pub fn update(&mut self, _: ()) {}
    /// # }
    /// use iced_runtime::core::mouse;
    /// use iced_runtime::core::renderer;
    /// use iced_runtime::core::shell;
    /// use iced_runtime::core::window;
    /// use iced_runtime::core::{Element, Size};
    /// use iced_runtime::user_interface::{self, UserInterface};
    /// use iced_wgpu::{Renderer, Theme};
    ///
    /// let mut counter = Counter::new();
    /// let mut cache = user_interface::Cache::new();
    /// let mut renderer = Renderer::default();
    /// let mut window = window::Headless; // This should be a proper window, like a `winit` one
    /// let mut waker = shell::Waker::noop();
    /// let mut window_size = Size::new(1024.0, 768.0);
    /// let mut cursor = mouse::Cursor::default();
    /// let mut events = Vec::new();
    /// let mut messages = shell::Bus::new();
    /// let mut theme = Theme::default();
    ///
    /// loop {
    ///     // Obtain system events...
    ///     // 获取系统事件……
    ///
    ///     let mut user_interface = UserInterface::build(
    ///         counter.view(),
    ///         window_size,
    ///         cache,
    ///         &mut renderer,
    ///     );
    ///
    ///     // Update the user interface
    ///     // 更新用户界面
    ///     let event_statuses = user_interface.update(
    ///         &window,
    ///         &waker,
    ///         &events,
    ///         cursor,
    ///         &mut renderer,
    ///         &mut messages
    ///     );
    ///
    ///     // Draw the user interface
    ///     // 绘制用户界面
    ///     let mouse_interaction = user_interface.draw(&mut renderer, &theme, &renderer::Style::default(), cursor);
    ///
    ///     cache = user_interface.into_cache();
    ///
    ///     for (message, _receipt) in messages.drain() {
    ///         counter.update(message);
    ///     }
    ///
    ///     // Update mouse cursor icon...
    ///     // 更新鼠标光标图标……
    ///     // Flush rendering operations...
    ///     // 刷新渲染操作……
    /// }
    /// ```
    pub fn draw(
        &mut self,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        cursor: mouse::Cursor,
    ) {
        let viewport = Rectangle::with_size(self.bounds);
        renderer.reset(viewport);

        let layout = Layout::new(self.state.size);
        let base_cursor = match &self.overlay {
            None | Some(mouse::Interaction::None) => cursor,
            _ => cursor.levitate(),
        };

        self.root.draw(
            &self.state,
            renderer,
            theme,
            style,
            layout,
            base_cursor,
            &viewport,
        );

        let Self { overlay, root, .. } = self;

        if overlay.is_none() {
            return;
        }

        let overlay = root.overlay(
            &mut self.state,
            layout,
            renderer,
            &viewport,
            Vector::ZERO,
            self.bounds,
        );

        if !overlay.is_empty() {
            let mut overlay = overlay::Group::new(overlay);

            overlay.draw(renderer, theme, style, cursor);
        }
    }

    /// 对 [`UserInterface`] 应用一个 [`widget::Operation`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Applies a [`widget::Operation`] to the [`UserInterface`].
    ///
    /// </details>
    pub fn operate(&mut self, renderer: &Renderer, operation: &mut dyn widget::Operation) {
        let viewport = Rectangle::with_size(self.bounds);
        let layout = Layout::new(self.state.size);

        operation.traverse(&mut |operation| {
            self.root
                .operate(&mut self.state, layout, &viewport, renderer, operation);
        });

        let overlay = self.root.overlay(
            &mut self.state,
            layout,
            renderer,
            &viewport,
            Vector::ZERO,
            self.bounds,
        );

        if !overlay.is_empty() {
            let mut overlay = overlay::Group::new(overlay);

            overlay.operate(renderer, operation);
        }
    }

    /// 使用给定的边界重新布局，并返回一个新的
    /// [`UserInterface`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Relayouts and returns a new  [`UserInterface`] using the provided
    /// bounds.
    ///
    /// </details>
    pub fn relayout(self, bounds: Size, renderer: &mut Renderer) -> Self {
        Self::build(
            self.root,
            bounds,
            Cache {
                state: self.state,
                last_click: self.last_click,
                selection: self.selection,
            },
            renderer,
        )
    }

    /// 取出 [`UserInterface`] 的 [`Cache`]，这个过程会
    /// 消耗该用户界面。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Extract the [`Cache`] of the [`UserInterface`], consuming it in the
    /// process.
    ///
    /// </details>
    pub fn into_cache(self) -> Cache {
        Cache {
            state: self.state,
            last_click: self.last_click,
            selection: self.selection,
        }
    }
}

/// 某个特定 [`UserInterface`] 的可复用数据。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Reusable data of a specific [`UserInterface`].
///
/// </details>
#[derive(Debug)]
pub struct Cache {
    state: widget::Tree,
    last_click: Option<mouse::Click>,
    selection: Option<(Point, Point)>,
}

impl Cache {
    /// 创建一个空的 [`Cache`]。
    ///
    /// 在构建第一个 [`UserInterface`] 之前，你应该用它来
    /// 初始化 [`Cache`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates an empty [`Cache`].
    ///
    /// You should use this to initialize a [`Cache`] before building your first
    /// [`UserInterface`].
    ///
    /// </details>
    pub fn new() -> Cache {
        Cache {
            state: widget::Tree::empty(),
            last_click: None,
            selection: None,
        }
    }
}

impl Default for Cache {
    fn default() -> Cache {
        Cache::new()
    }
}

/// 更新 [`UserInterface`] 之后得到的状态。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The resulting state after updating a [`UserInterface`].
///
/// </details>
#[derive(Debug)]
pub enum State {
    /// [`UserInterface`] 已过时，需要重新构建。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`UserInterface`] is outdated and needs to be rebuilt.
    ///
    /// </details>
    Outdated,

    /// [`UserInterface`] 是最新的，无需重新构建即可
    /// 复用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`UserInterface`] is up-to-date and can be reused without
    /// rebuilding.
    ///
    /// </details>
    Updated {
        /// 用户界面当前的 [`mouse::Interaction`]。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The current [`mouse::Interaction`] of the user interface.
        ///
        /// </details>
        mouse_interaction: mouse::Interaction,
        /// 描述何时应当重绘的 [`window::RedrawRequest`]。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The [`window::RedrawRequest`] describing when a redraw should be performed.
        ///
        /// </details>
        redraw_request: window::RedrawRequest,
        /// 用户界面当前的 [`InputMethod`] 策略。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The current [`InputMethod`] strategy of the user interface.
        ///
        /// </details>
        input_method: InputMethod,
        /// 用户界面产生的一组 [`Clipboard`] 请求。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The set of [`Clipboard`] requests that the user interface has produced.
        ///
        /// </details>
        clipboard: Clipboard,
        /// [`UserInterface`] 经历的 [`Change`]。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The [`Change`] that the [`UserInterface`] experienced.
        ///
        /// </details>
        change: Change,
    },
}

/// [`UserInterface`] 在更新过程中经历的
/// 变化。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The change experienced by a [`UserInterface`] while being
/// updated.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Change {
    /// 没有任何内容失效：[`UserInterface`] 没有变化。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Nothing was invalidated: the [`UserInterface`] is unchanged.
    ///
    /// </details>
    None,
    /// 只有应用程序的覆盖层被重新创建。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Only the application overlays were recreated.
    ///
    /// </details>
    Overlay,
    /// 应用程序的布局被重新计算。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The application layout was recomputed.
    ///
    /// </details>
    Layout,
}
