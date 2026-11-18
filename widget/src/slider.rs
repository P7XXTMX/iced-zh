//! 滑块让用户通过移动指示器来设置一个值。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Sliders let users set a value by moving an indicator.
//!
//! </details>
//!
//! # 示例（Example）
//! ```no_run
//! # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
//! # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
//! #
//! use iced::widget::slider;
//!
//! struct State {
//!    value: f32,
//! }
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     ValueChanged(f32),
//! }
//!
//! fn view(state: &State) -> Element<'_, Message> {
//!     slider(0.0..=100.0, state.value, Message::ValueChanged).into()
//! }
//!
//! fn update(state: &mut State, message: Message) {
//!     match message {
//!         Message::ValueChanged(value) => {
//!             state.value = value;
//!         }
//!     }
//! }
//! ```
use crate::core::border::{self, Border};
use crate::core::keyboard;
use crate::core::keyboard::key::{self, Key};
use crate::core::layout;
use crate::core::mouse;
use crate::core::renderer;
use crate::core::touch;
use crate::core::widget::tree::{self, Tree};
use crate::core::window;
use crate::core::{
    self, Background, Clipboard, Color, Element, Event, Layout, Length, Pixels,
    Point, Rectangle, Shell, Size, Theme, Widget,
};

use std::ops::RangeInclusive;

/// 一个水平条和一个手柄，用于从一个取值范围中
/// 选出单个值。
///
/// [`Slider`] 会尝试填满其容器的水平空间。
///
/// [`Slider`] 的数值范围是泛型的，步长默认为
/// 1 个单位。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An horizontal bar and a handle that selects a single value from a range of
/// values.
///
/// A [`Slider`] will try to fill the horizontal space of its container.
///
/// The [`Slider`] range of numeric values is generic and its step size defaults
/// to 1 unit.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # pub type Element<'a, Message> = iced_widget::core::Element<'a, Message, iced_widget::Theme, iced_widget::Renderer>;
/// #
/// use iced::widget::slider;
///
/// struct State {
///    value: f32,
/// }
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     ValueChanged(f32),
/// }
///
/// fn view(state: &State) -> Element<'_, Message> {
///     slider(0.0..=100.0, state.value, Message::ValueChanged).into()
/// }
///
/// fn update(state: &mut State, message: Message) {
///     match message {
///         Message::ValueChanged(value) => {
///             state.value = value;
///         }
///     }
/// }
/// ```
pub struct Slider<'a, T, Message, Theme = crate::Theme>
where
    Theme: Catalog,
{
    range: RangeInclusive<T>,
    step: T,
    shift_step: Option<T>,
    value: T,
    default: Option<T>,
    on_change: Box<dyn Fn(T) -> Message + 'a>,
    on_release: Option<Message>,
    width: Length,
    height: f32,
    class: Theme::Class<'a>,
    status: Option<Status>,
}

impl<'a, T, Message, Theme> Slider<'a, T, Message, Theme>
where
    T: Copy + From<u8> + PartialOrd,
    Message: Clone,
    Theme: Catalog,
{
    /// [`Slider`] 的默认高度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The default height of a [`Slider`].
    ///
    /// </details>
    pub const DEFAULT_HEIGHT: f32 = 16.0;

    /// 创建一个新的 [`Slider`]。
    ///
    /// 它需要：
    ///   * 一个由可能取值构成的闭区间
    ///   * [`Slider`] 的当前值
    ///   * 一个在 [`Slider`] 被拖动时调用的函数。
    ///     它接收 [`Slider`] 的新值，并且必须产生一个
    ///     `Message`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Slider`].
    ///
    /// It expects:
    ///   * an inclusive range of possible values
    ///   * the current value of the [`Slider`]
    ///   * a function that will be called when the [`Slider`] is dragged.
    ///     It receives the new value of the [`Slider`] and must produce a
    ///     `Message`.
    ///
    /// </details>
    pub fn new<F>(range: RangeInclusive<T>, value: T, on_change: F) -> Self
    where
        F: 'a + Fn(T) -> Message,
    {
        let value = if value >= *range.start() {
            value
        } else {
            *range.start()
        };

        let value = if value <= *range.end() {
            value
        } else {
            *range.end()
        };

        Slider {
            value,
            default: None,
            range,
            step: T::from(1),
            shift_step: None,
            on_change: Box::new(on_change),
            on_release: None,
            width: Length::Fill,
            height: Self::DEFAULT_HEIGHT,
            class: Theme::default(),
            status: None,
        }
    }

    /// 设置 [`Slider`] 可选的默认值。
    ///
    /// 如果设置了，按住 ctrl 或 command 点击时 [`Slider`] 会重置为这个值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the optional default value for the [`Slider`].
    ///
    /// If set, the [`Slider`] will reset to this value when ctrl-clicked or command-clicked.
    ///
    /// </details>
    pub fn default(mut self, default: impl Into<T>) -> Self {
        self.default = Some(default.into());
        self
    }

    /// 设置 [`Slider`] 的释放消息。
    /// 它在鼠标从滑块上松开时被调用。
    ///
    /// 通常，这条消息产生时，用户与滑块的交互已经结束。
    /// 如果你需要根据滑块的结果启动一个长时间运行的任务，而
    /// 默认的 on_change 消息可能产生过多事件，这会很有用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the release message of the [`Slider`].
    /// This is called when the mouse is released from the slider.
    ///
    /// Typically, the user's interaction with the slider is finished when this message is produced.
    /// This is useful if you need to spawn a long-running task from the slider's result, where
    /// the default on_change message could create too many events.
    ///
    /// </details>
    pub fn on_release(mut self, on_release: Message) -> Self {
        self.on_release = Some(on_release);
        self
    }

    /// 设置 [`Slider`] 的宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the width of the [`Slider`].
    ///
    /// </details>
    pub fn width(mut self, width: impl Into<Length>) -> Self {
        self.width = width.into();
        self
    }

    /// 设置 [`Slider`] 的高度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the height of the [`Slider`].
    ///
    /// </details>
    pub fn height(mut self, height: impl Into<Pixels>) -> Self {
        self.height = height.into().0;
        self
    }

    /// 设置 [`Slider`] 的步长。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the step size of the [`Slider`].
    ///
    /// </details>
    pub fn step(mut self, step: impl Into<T>) -> Self {
        self.step = step.into();
        self
    }

    /// 设置 [`Slider`] 可选的“shift”步长。
    ///
    /// 如果设置了，按住 shift 键时会使用这个值作为步长。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the optional "shift" step for the [`Slider`].
    ///
    /// If set, this value is used as the step while the shift key is pressed.
    ///
    /// </details>
    pub fn shift_step(mut self, shift_step: impl Into<T>) -> Self {
        self.shift_step = Some(shift_step.into());
        self
    }

    /// 设置 [`Slider`] 的样式。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the style of the [`Slider`].
    ///
    /// </details>
    #[must_use]
    pub fn style(mut self, style: impl Fn(&Theme, Status) -> Style + 'a) -> Self
    where
        Theme::Class<'a>: From<StyleFn<'a, Theme>>,
    {
        self.class = (Box::new(style) as StyleFn<'a, Theme>).into();
        self
    }

    /// 设置 [`Slider`] 的样式类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the style class of the [`Slider`].
    ///
    /// </details>
    #[cfg(feature = "advanced")]
    #[must_use]
    pub fn class(mut self, class: impl Into<Theme::Class<'a>>) -> Self {
        self.class = class.into();
        self
    }
}

impl<T, Message, Theme, Renderer> Widget<Message, Theme, Renderer>
    for Slider<'_, T, Message, Theme>
where
    T: Copy + Into<f64> + num_traits::FromPrimitive,
    Message: Clone,
    Theme: Catalog,
    Renderer: core::Renderer,
{
    fn tag(&self) -> tree::Tag {
        tree::Tag::of::<State>()
    }

    fn state(&self) -> tree::State {
        tree::State::new(State::default())
    }

    fn size(&self) -> Size<Length> {
        Size {
            width: self.width,
            height: Length::Shrink,
        }
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        _renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout::atomic(limits, self.width, self.height)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        let state = tree.state.downcast_mut::<State>();

        let mut update = || {
            let current_value = self.value;

            let locate = |cursor_position: Point| -> Option<T> {
                let bounds = layout.bounds();

                if cursor_position.x <= bounds.x {
                    Some(*self.range.start())
                } else if cursor_position.x >= bounds.x + bounds.width {
                    Some(*self.range.end())
                } else {
                    let step = if state.keyboard_modifiers.shift() {
                        self.shift_step.unwrap_or(self.step)
                    } else {
                        self.step
                    }
                    .into();

                    let start = (*self.range.start()).into();
                    let end = (*self.range.end()).into();

                    let percent = f64::from(cursor_position.x - bounds.x)
                        / f64::from(bounds.width);

                    let steps = (percent * (end - start) / step).round();
                    let value = steps * step + start;

                    T::from_f64(value.min(end))
                }
            };

            let increment = |value: T| -> Option<T> {
                let step = if state.keyboard_modifiers.shift() {
                    self.shift_step.unwrap_or(self.step)
                } else {
                    self.step
                }
                .into();

                let steps = (value.into() / step).round();
                let new_value = step * (steps + 1.0);

                if new_value > (*self.range.end()).into() {
                    return Some(*self.range.end());
                }

                T::from_f64(new_value)
            };

            let decrement = |value: T| -> Option<T> {
                let step = if state.keyboard_modifiers.shift() {
                    self.shift_step.unwrap_or(self.step)
                } else {
                    self.step
                }
                .into();

                let steps = (value.into() / step).round();
                let new_value = step * (steps - 1.0);

                if new_value < (*self.range.start()).into() {
                    return Some(*self.range.start());
                }

                T::from_f64(new_value)
            };

            let change = |new_value: T| {
                if (self.value.into() - new_value.into()).abs() > f64::EPSILON {
                    shell.publish((self.on_change)(new_value));

                    self.value = new_value;
                }
            };

            match &event {
                Event::Mouse(mouse::Event::ButtonPressed(
                    mouse::Button::Left,
                ))
                | Event::Touch(touch::Event::FingerPressed { .. }) => {
                    if let Some(cursor_position) =
                        cursor.position_over(layout.bounds())
                    {
                        if state.keyboard_modifiers.command() {
                            let _ = self.default.map(change);
                            state.is_dragging = false;
                        } else {
                            let _ = locate(cursor_position).map(change);
                            state.is_dragging = true;
                        }

                        shell.capture_event();
                    }
                }
                Event::Mouse(mouse::Event::ButtonReleased(
                    mouse::Button::Left,
                ))
                | Event::Touch(touch::Event::FingerLifted { .. })
                | Event::Touch(touch::Event::FingerLost { .. }) => {
                    if state.is_dragging {
                        if let Some(on_release) = self.on_release.clone() {
                            shell.publish(on_release);
                        }
                        state.is_dragging = false;
                    }
                }
                Event::Mouse(mouse::Event::CursorMoved { .. })
                | Event::Touch(touch::Event::FingerMoved { .. }) => {
                    if state.is_dragging {
                        let _ = cursor
                            .land()
                            .position()
                            .and_then(locate)
                            .map(change);

                        shell.capture_event();
                    }
                }
                Event::Mouse(mouse::Event::WheelScrolled { delta })
                    if state.keyboard_modifiers.control() =>
                {
                    if cursor.is_over(layout.bounds()) {
                        let delta = match delta {
                            mouse::ScrollDelta::Lines { x: _, y } => y,
                            mouse::ScrollDelta::Pixels { x: _, y } => y,
                        };

                        if *delta < 0.0 {
                            let _ = decrement(current_value).map(change);
                        } else {
                            let _ = increment(current_value).map(change);
                        }

                        shell.capture_event();
                    }
                }
                Event::Keyboard(keyboard::Event::KeyPressed {
                    key, ..
                }) => {
                    if cursor.is_over(layout.bounds()) {
                        match key {
                            Key::Named(key::Named::ArrowUp) => {
                                let _ = increment(current_value).map(change);
                                shell.capture_event();
                            }
                            Key::Named(key::Named::ArrowDown) => {
                                let _ = decrement(current_value).map(change);
                                shell.capture_event();
                            }
                            _ => (),
                        }
                    }
                }
                Event::Keyboard(keyboard::Event::ModifiersChanged(
                    modifiers,
                )) => {
                    state.keyboard_modifiers = *modifiers;
                }
                _ => {}
            }
        };

        update();

        let current_status = if state.is_dragging {
            Status::Dragged
        } else if cursor.is_over(layout.bounds()) {
            Status::Hovered
        } else {
            Status::Active
        };

        if let Event::Window(window::Event::RedrawRequested(_now)) = event {
            self.status = Some(current_status);
        } else if self.status.is_some_and(|status| status != current_status) {
            shell.request_redraw();
        }
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();

        let style =
            theme.style(&self.class, self.status.unwrap_or(Status::Active));

        let (handle_width, handle_height, handle_border_radius) =
            match style.handle.shape {
                HandleShape::Circle { radius } => {
                    (radius * 2.0, radius * 2.0, radius.into())
                }
                HandleShape::Rectangle {
                    width,
                    border_radius,
                } => (f32::from(width), bounds.height, border_radius),
            };

        let value = self.value.into() as f32;
        let (range_start, range_end) = {
            let (start, end) = self.range.clone().into_inner();

            (start.into() as f32, end.into() as f32)
        };

        let offset = if range_start >= range_end {
            0.0
        } else {
            (bounds.width - handle_width) * (value - range_start)
                / (range_end - range_start)
        };

        let rail_y = bounds.y + bounds.height / 2.0;

        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle {
                    x: bounds.x,
                    y: rail_y - style.rail.width / 2.0,
                    width: offset + handle_width / 2.0,
                    height: style.rail.width,
                },
                border: style.rail.border,
                ..renderer::Quad::default()
            },
            style.rail.backgrounds.0,
        );

        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle {
                    x: bounds.x + offset + handle_width / 2.0,
                    y: rail_y - style.rail.width / 2.0,
                    width: bounds.width - offset - handle_width / 2.0,
                    height: style.rail.width,
                },
                border: style.rail.border,
                ..renderer::Quad::default()
            },
            style.rail.backgrounds.1,
        );

        renderer.fill_quad(
            renderer::Quad {
                bounds: Rectangle {
                    x: bounds.x + offset,
                    y: rail_y - handle_height / 2.0,
                    width: handle_width,
                    height: handle_height,
                },
                border: Border {
                    radius: handle_border_radius,
                    width: style.handle.border_width,
                    color: style.handle.border_color,
                },
                ..renderer::Quad::default()
            },
            style.handle.background,
        );
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        let state = tree.state.downcast_ref::<State>();

        if state.is_dragging {
            // FIXME: Fall back to `Pointer` on Windows
            // See https://github.com/rust-windowing/winit/issues/1043
            // 待修复：在 Windows 上回退到 `Pointer`
            // 参见 https://github.com/rust-windowing/winit/issues/1043
            if cfg!(target_os = "windows") {
                mouse::Interaction::Pointer
            } else {
                mouse::Interaction::Grabbing
            }
        } else if cursor.is_over(layout.bounds()) {
            if cfg!(target_os = "windows") {
                mouse::Interaction::Pointer
            } else {
                mouse::Interaction::Grab
            }
        } else {
            mouse::Interaction::default()
        }
    }
}

impl<'a, T, Message, Theme, Renderer> From<Slider<'a, T, Message, Theme>>
    for Element<'a, Message, Theme, Renderer>
where
    T: Copy + Into<f64> + num_traits::FromPrimitive + 'a,
    Message: Clone + 'a,
    Theme: Catalog + 'a,
    Renderer: core::Renderer + 'a,
{
    fn from(
        slider: Slider<'a, T, Message, Theme>,
    ) -> Element<'a, Message, Theme, Renderer> {
        Element::new(slider)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
struct State {
    is_dragging: bool,
    keyboard_modifiers: keyboard::Modifiers,
}

/// [`Slider`] 可能的交互状态。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The possible status of a [`Slider`].
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// [`Slider`] 可以交互。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Slider`] can be interacted with.
    ///
    /// </details>
    Active,
    /// [`Slider`] 正处于悬停状态。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Slider`] is being hovered.
    ///
    /// </details>
    Hovered,
    /// [`Slider`] 正在被拖动。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Slider`] is being dragged.
    ///
    /// </details>
    Dragged,
}

/// 滑块的外观。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The appearance of a slider.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Style {
    /// 滑块轨道的颜色。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The colors of the rail of the slider.
    ///
    /// </details>
    pub rail: Rail,
    /// 滑块的 [`Handle`] 的外观。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The appearance of the [`Handle`] of the slider.
    ///
    /// </details>
    pub handle: Handle,
}

impl Style {
    /// 把 [`Style`] 的 [`HandleShape`] 改为具有给定半径的
    /// 圆形。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Changes the [`HandleShape`] of the [`Style`] to a circle
    /// with the given radius.
    ///
    /// </details>
    pub fn with_circular_handle(mut self, radius: impl Into<Pixels>) -> Self {
        self.handle.shape = HandleShape::Circle {
            radius: radius.into().0,
        };
        self
    }
}

/// 滑块轨道的外观
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The appearance of a slider rail
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rail {
    /// 滑块轨道的背景。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The backgrounds of the rail of the slider.
    ///
    /// </details>
    pub backgrounds: (Background, Background),
    /// 滑块轨道的线条宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The width of the stroke of a slider rail.
    ///
    /// </details>
    pub width: f32,
    /// 轨道的边框。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The border of the rail.
    ///
    /// </details>
    pub border: Border,
}

/// 滑块手柄的外观。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The appearance of the handle of a slider.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Handle {
    /// 手柄的形状。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The shape of the handle.
    ///
    /// </details>
    pub shape: HandleShape,
    /// 手柄的 [`Background`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Background`] of the handle.
    ///
    /// </details>
    pub background: Background,
    /// 手柄的边框宽度。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The border width of the handle.
    ///
    /// </details>
    pub border_width: f32,
    /// 手柄的边框 [`Color`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The border [`Color`] of the handle.
    ///
    /// </details>
    pub border_color: Color,
}

/// 滑块手柄的形状。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The shape of the handle of a slider.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HandleShape {
    /// 圆形手柄。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A circular handle.
    ///
    /// </details>
    Circle {
        /// 圆的半径。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The radius of the circle.
        ///
        /// </details>
        radius: f32,
    },
    /// 矩形形状。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A rectangular shape.
    ///
    /// </details>
    Rectangle {
        /// 矩形的宽度。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The width of the rectangle.
        ///
        /// </details>
        width: u16,
        /// 矩形各个角的边框半径。
        ///
        /// <details>
        /// <summary>显示英文原文</summary>
        ///
        /// The border radius of the corners of the rectangle.
        ///
        /// </details>
        border_radius: border::Radius,
    },
}

/// [`Slider`] 的主题样式目录。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The theme catalog of a [`Slider`].
///
/// </details>
pub trait Catalog: Sized {
    /// [`Catalog`] 的样式类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The item class of the [`Catalog`].
    ///
    /// </details>
    type Class<'a>;

    /// [`Catalog`] 产生的默认样式类。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The default class produced by the [`Catalog`].
    ///
    /// </details>
    fn default<'a>() -> Self::Class<'a>;

    /// 某个样式类在给定交互状态下的 [`Style`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The [`Style`] of a class with the given status.
    ///
    /// </details>
    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style;
}

/// [`Slider`] 的样式函数。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A styling function for a [`Slider`].
///
/// </details>
pub type StyleFn<'a, Theme> = Box<dyn Fn(&Theme, Status) -> Style + 'a>;

impl Catalog for Theme {
    type Class<'a> = StyleFn<'a, Self>;

    fn default<'a>() -> Self::Class<'a> {
        Box::new(default)
    }

    fn style(&self, class: &Self::Class<'_>, status: Status) -> Style {
        class(self, status)
    }
}

/// [`Slider`] 的默认样式。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The default style of a [`Slider`].
///
/// </details>
pub fn default(theme: &Theme, status: Status) -> Style {
    let palette = theme.extended_palette();

    let color = match status {
        Status::Active => palette.primary.base.color,
        Status::Hovered => palette.primary.strong.color,
        Status::Dragged => palette.primary.weak.color,
    };

    Style {
        rail: Rail {
            backgrounds: (color.into(), palette.background.strong.color.into()),
            width: 4.0,
            border: Border {
                radius: 2.0.into(),
                width: 0.0,
                color: Color::TRANSPARENT,
            },
        },
        handle: Handle {
            shape: HandleShape::Circle { radius: 7.0 },
            background: color.into(),
            border_color: Color::TRANSPARENT,
            border_width: 0.0,
        },
    }
}
