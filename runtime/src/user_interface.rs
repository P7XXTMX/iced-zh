//! 实现你自己的事件循环来驱动用户界面。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Implement your own event loop to drive a user interface.
//!
//! </details>
use crate::core::event::{self, Event};
use crate::core::layout;
use crate::core::mouse;
use crate::core::overlay;
use crate::core::renderer;
use crate::core::widget;
use crate::core::window;
use crate::core::{
    Clipboard, Element, InputMethod, Layout, Rectangle, Shell, Size, Vector,
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
/// [`integration`]: https://github.com/iced-rs/iced/tree/0.14/examples/integration
pub struct UserInterface<'a, Message, Theme, Renderer> {
    root: Element<'a, Message, Theme, Renderer>,
    base: layout::Node,
    state: widget::Tree,
    overlay: Option<Overlay>,
    bounds: Size,
}

struct Overlay {
    layout: layout::Node,
    interaction: mouse::Interaction,
}

impl<'a, Message, Theme, Renderer> UserInterface<'a, Message, Theme, Renderer>
where
    Renderer: crate::core::Renderer,
{
    /// 为一个 [`Element`] 构建用户界面。
    ///
    /// 如果使用从上一个 [`UserInterface`] 实例得到的 [`Cache`]，
    /// 它可以避免昂贵的计算。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Builds a user interface for an [`Element`].
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
    /// use iced_runtime::core::Size;
    /// use iced_runtime::user_interface::{self, UserInterface};
    /// use iced_wgpu::Renderer;
    ///
    /// // Initialization
    /// // 初始化
    /// let mut counter = Counter::new();
    /// let mut cache = user_interface::Cache::new();
    /// let mut renderer = Renderer::default();
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
    pub fn build<E: Into<Element<'a, Message, Theme, Renderer>>>(
        root: E,
        bounds: Size,
        cache: Cache,
        renderer: &mut Renderer,
    ) -> Self {
        let mut root = root.into();

        let Cache { mut state } = cache;
        state.diff(root.as_widget());

        let base = root.as_widget_mut().layout(
            &mut state,
            renderer,
            &layout::Limits::new(Size::ZERO, bounds),
        );

        UserInterface {
            root,
            base,
            state,
            overlay: None,
            bounds,
        }
    }

    /// 处理给定的每一个 [`Event`]，以此更新 [`UserInterface`]。
    ///
    /// 它返回因用户交互而可能产生的**消息**。
    /// 你应该把这些消息交给你的**更新逻辑**。
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
    /// use iced_runtime::core::clipboard;
    /// use iced_runtime::core::mouse;
    /// use iced_runtime::core::Size;
    /// use iced_runtime::user_interface::{self, UserInterface};
    /// use iced_wgpu::Renderer;
    ///
    /// let mut counter = Counter::new();
    /// let mut cache = user_interface::Cache::new();
    /// let mut renderer = Renderer::default();
    /// let mut window_size = Size::new(1024.0, 768.0);
    /// let mut cursor = mouse::Cursor::default();
    /// let mut clipboard = clipboard::Null;
    ///
    /// // Initialize our event storage
    /// // 初始化我们的事件存储
    /// let mut events = Vec::new();
    /// let mut messages = Vec::new();
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
    ///         &events,
    ///         cursor,
    ///         &mut renderer,
    ///         &mut clipboard,
    ///         &mut messages
    ///     );
    ///
    ///     cache = user_interface.into_cache();
    ///
    ///     // Process the produced messages
    ///     // 处理产生的消息
    ///     for message in messages.drain(..) {
    ///         counter.update(message);
    ///     }
    /// }
    /// ```
    pub fn update(
        &mut self,
        events: &[Event],
        cursor: mouse::Cursor,
        renderer: &mut Renderer,
        clipboard: &mut dyn Clipboard,
        messages: &mut Vec<Message>,
    ) -> (State, Vec<event::Status>) {
        let mut outdated = false;
        let mut redraw_request = window::RedrawRequest::Wait;
        let mut input_method = InputMethod::Disabled;
        let mut has_layout_changed = false;
        let viewport = Rectangle::with_size(self.bounds);

        let mut maybe_overlay = self
            .root
            .as_widget_mut()
            .overlay(
                &mut self.state,
                Layout::new(&self.base),
                renderer,
                &viewport,
                Vector::ZERO,
            )
            .map(overlay::Nested::new);

        let (base_cursor, overlay_statuses, overlay_interaction) =
            if maybe_overlay.is_some() {
                let bounds = self.bounds;

                let mut overlay = maybe_overlay.as_mut().unwrap();
                let mut layout = overlay.layout(renderer, bounds);
                let mut event_statuses = Vec::new();

                for event in events {
                    let mut shell = Shell::new(messages);

                    overlay.update(
                        event,
                        Layout::new(&layout),
                        cursor,
                        renderer,
                        clipboard,
                        &mut shell,
                    );

                    event_statuses.push(shell.event_status());
                    redraw_request = redraw_request.min(shell.redraw_request());
                    input_method.merge(shell.input_method());

                    if shell.is_layout_invalid() {
                        drop(maybe_overlay);

                        self.base = self.root.as_widget_mut().layout(
                            &mut self.state,
                            renderer,
                            &layout::Limits::new(Size::ZERO, self.bounds),
                        );

                        maybe_overlay = self
                            .root
                            .as_widget_mut()
                            .overlay(
                                &mut self.state,
                                Layout::new(&self.base),
                                renderer,
                                &viewport,
                                Vector::ZERO,
                            )
                            .map(overlay::Nested::new);

                        if maybe_overlay.is_none() {
                            break;
                        }

                        overlay = maybe_overlay.as_mut().unwrap();

                        shell.revalidate_layout(|| {
                            layout = overlay.layout(renderer, bounds);
                            has_layout_changed = true;
                        });
                    }

                    if shell.are_widgets_invalid() {
                        outdated = true;
                    }
                }

                let (base_cursor, interaction) =
                    if let Some(overlay) = maybe_overlay.as_mut() {
                        let interaction = cursor
                            .position()
                            .map(|cursor_position| {
                                overlay.mouse_interaction(
                                    Layout::new(&layout),
                                    mouse::Cursor::Available(cursor_position),
                                    renderer,
                                )
                            })
                            .unwrap_or_default();

                        if interaction == mouse::Interaction::None {
                            (cursor, mouse::Interaction::None)
                        } else {
                            (mouse::Cursor::Unavailable, interaction)
                        }
                    } else {
                        (cursor, mouse::Interaction::None)
                    };

                self.overlay = Some(Overlay {
                    layout,
                    interaction,
                });

                (base_cursor, event_statuses, interaction)
            } else {
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

                let mut shell = Shell::new(messages);

                self.root.as_widget_mut().update(
                    &mut self.state,
                    event,
                    Layout::new(&self.base),
                    base_cursor,
                    renderer,
                    clipboard,
                    &mut shell,
                    &viewport,
                );

                if shell.event_status() == event::Status::Captured {
                    self.overlay = None;
                }

                redraw_request = redraw_request.min(shell.redraw_request());
                input_method.merge(shell.input_method());

                shell.revalidate_layout(|| {
                    has_layout_changed = true;

                    self.base = self.root.as_widget_mut().layout(
                        &mut self.state,
                        renderer,
                        &layout::Limits::new(Size::ZERO, self.bounds),
                    );

                    if let Some(mut overlay) = self
                        .root
                        .as_widget_mut()
                        .overlay(
                            &mut self.state,
                            Layout::new(&self.base),
                            renderer,
                            &viewport,
                            Vector::ZERO,
                        )
                        .map(overlay::Nested::new)
                    {
                        let layout = overlay.layout(renderer, self.bounds);
                        let interaction = overlay.mouse_interaction(
                            Layout::new(&layout),
                            cursor,
                            renderer,
                        );

                        self.overlay = Some(Overlay {
                            layout,
                            interaction,
                        });
                    }
                });

                if shell.are_widgets_invalid() {
                    outdated = true;
                }

                shell.event_status().merge(overlay_status)
            })
            .collect();

        let mouse_interaction =
            if overlay_interaction == mouse::Interaction::None {
                self.root.as_widget().mouse_interaction(
                    &self.state,
                    Layout::new(&self.base),
                    base_cursor,
                    &viewport,
                    renderer,
                )
            } else {
                overlay_interaction
            };

        (
            if outdated {
                State::Outdated
            } else {
                State::Updated {
                    mouse_interaction,
                    redraw_request,
                    input_method,
                    has_layout_changed,
                }
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
    /// use iced_runtime::core::clipboard;
    /// use iced_runtime::core::mouse;
    /// use iced_runtime::core::renderer;
    /// use iced_runtime::core::{Element, Size};
    /// use iced_runtime::user_interface::{self, UserInterface};
    /// use iced_wgpu::{Renderer, Theme};
    ///
    /// let mut counter = Counter::new();
    /// let mut cache = user_interface::Cache::new();
    /// let mut renderer = Renderer::default();
    /// let mut window_size = Size::new(1024.0, 768.0);
    /// let mut cursor = mouse::Cursor::default();
    /// let mut clipboard = clipboard::Null;
    /// let mut events = Vec::new();
    /// let mut messages = Vec::new();
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
    ///         &events,
    ///         cursor,
    ///         &mut renderer,
    ///         &mut clipboard,
    ///         &mut messages
    ///     );
    ///
    ///     // Draw the user interface
    ///     // 绘制用户界面
    ///     let mouse_interaction = user_interface.draw(&mut renderer, &theme, &renderer::Style::default(), cursor);
    ///
    ///     cache = user_interface.into_cache();
    ///
    ///     for message in messages.drain(..) {
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

        let base_cursor = match &self.overlay {
            None
            | Some(Overlay {
                interaction: mouse::Interaction::None,
                ..
            }) => cursor,
            _ => mouse::Cursor::Unavailable,
        };

        self.root.as_widget().draw(
            &self.state,
            renderer,
            theme,
            style,
            Layout::new(&self.base),
            base_cursor,
            &viewport,
        );

        let Self {
            overlay,
            root,
            base,
            ..
        } = self;

        let Some(Overlay { layout, .. }) = overlay.as_ref() else {
            return;
        };

        let overlay = root
            .as_widget_mut()
            .overlay(
                &mut self.state,
                Layout::new(base),
                renderer,
                &viewport,
                Vector::ZERO,
            )
            .map(overlay::Nested::new);

        if let Some(mut overlay) = overlay {
            overlay.draw(renderer, theme, style, Layout::new(layout), cursor);
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
    pub fn operate(
        &mut self,
        renderer: &Renderer,
        operation: &mut dyn widget::Operation,
    ) {
        let viewport = Rectangle::with_size(self.bounds);

        self.root.as_widget_mut().operate(
            &mut self.state,
            Layout::new(&self.base),
            renderer,
            operation,
        );

        if let Some(mut overlay) = self
            .root
            .as_widget_mut()
            .overlay(
                &mut self.state,
                Layout::new(&self.base),
                renderer,
                &viewport,
                Vector::ZERO,
            )
            .map(overlay::Nested::new)
        {
            if self.overlay.is_none() {
                self.overlay = Some(Overlay {
                    layout: overlay.layout(renderer, self.bounds),
                    interaction: mouse::Interaction::None,
                });
            }

            overlay.operate(
                Layout::new(&self.overlay.as_ref().unwrap().layout),
                renderer,
                operation,
            );
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
        Self::build(self.root, bounds, Cache { state: self.state }, renderer)
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
        Cache { state: self.state }
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
#[derive(Debug, Clone)]
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
        /// [`UserInterface`] 的布局是否发生了变化。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// Whether the layout of the [`UserInterface`] has changed.
        ///
        /// </details>
        has_layout_changed: bool,
    },
}

impl State {
    /// 返回 [`UserInterface`] 的布局是否发生了变化。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns whether the layout of the [`UserInterface`] has changed.
    ///
    /// </details>
    pub fn has_layout_changed(&self) -> bool {
        match self {
            State::Outdated => true,
            State::Updated {
                has_layout_changed, ..
            } => *has_layout_changed,
        }
    }
}
