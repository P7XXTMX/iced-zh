//! 针对 `Opaque` 部件的行为测试。
//!
//! 这些测试直接驱动 `Widget`特质（使用空渲染器 `()`），以
//! 验证 `Opaque` 默认会捕获鼠标按键的按下，并且
//! 只有在启用了 `capture_scroll` 标志且光标位于其边界之内时，
//! 才会捕获滚轮事件。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Behavioral tests for the `Opaque` widget.
//!
//! These drive the `Widget` trait directly (using the null renderer `()`) to
//! verify that an `Opaque` captures mouse button presses by default, and that
//! it captures scroll wheel events only when its `capture_scroll` flag
//! is enabled and the cursor is over its bounds.
//!
//! </details>
use iced_widget::core::event;
use iced_widget::core::layout::{self, Layout, Limits};
use iced_widget::core::mouse::{self, Button, Cursor};
use iced_widget::core::renderer;
use iced_widget::core::shell;
use iced_widget::core::widget::{self, Tree, Widget};
use iced_widget::core::window::Headless;
use iced_widget::core::{Event, Length, Point, Rectangle, Size, Theme};
use iced_widget::{Opaque, Space, opaque, space};

type Message = u8;

/// [`WheelSpy`] 收到滚轮事件时发布的
/// 消息。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The message published by the [`WheelSpy`] when it receives a scroll wheel
/// event.
///
/// </details>
const WHEEL_SEEN: Message = 0x42;

/// 测试视口的大小。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The size of the test viewport.
///
/// </details>
const VIEWPORT: Size = Size::new(1000.0, 1000.0);

/// 在给定位置按下左键。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A left-button press at the given position.
///
/// </details>
fn press(position: Point) -> (Event, Cursor) {
    (
        Event::Mouse(mouse::Event::ButtonPressed(Button::Left)),
        Cursor::Available(position),
    )
}

/// 在给定位置的滚轮事件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A scroll wheel event at the given position.
///
/// </details>
fn scroll(position: Point) -> (Event, Cursor) {
    (
        Event::Mouse(mouse::Event::WheelScrolled {
            delta: mouse::ScrollDelta::Lines { x: 0.0, y: 1.0 },
        }),
        Cursor::Available(position),
    )
}

/// 包裹一个 50x50 `space` 的 `Opaque`，可以选择是否捕获滚轮。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An `Opaque` wrapping a 50x50 `space`, optionally capturing the scroll wheel.
///
/// </details>
fn new_opaque(capture_scroll: bool) -> Opaque<Space> {
    let space = space().width(50).height(50);

    if capture_scroll {
        opaque(space).capture_scroll(true)
    } else {
        opaque(space)
    }
}

/// 构建状态树并为部件计算（初始）布局。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Builds the state tree and computes the (initial) layout for the widget.
///
/// </details>
fn setup<W>(widget: &mut W) -> (Tree, Layout)
where
    W: Widget<Message, Theme, ()>,
{
    let mut tree = Tree::new(widget);

    widget.diff(&mut tree);
    widget.layout(&mut tree, &(), &Limits::new(Size::ZERO, VIEWPORT));

    let layout = Layout::new(tree.size);

    (tree, layout)
}

/// 让单个事件经过部件，并返回产生的事件
/// 状态和已发布消息的总线。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Drives a single event through the widget and returns the resulting event
/// status and the bus of published messages.
///
/// </details>
fn drive<W>(
    widget: &mut W,
    tree: &mut Tree,
    layout: Layout,
    event: &Event,
    cursor: Cursor,
) -> (event::Status, shell::Bus<Message>)
where
    W: Widget<Message, Theme, ()>,
{
    let viewport = Rectangle::with_size(VIEWPORT);

    let mut bus = shell::Bus::new();
    let mut shell = shell::Shell::new(&Headless, shell::Waker::noop(), &mut bus);

    widget.update(tree, event, layout, cursor, &(), &mut shell, &viewport);

    (shell.event_status(), bus)
}

/// 收集总线上发布的消息。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Collects the messages published on the bus.
///
/// </details>
fn messages(bus: shell::Bus<Message>) -> Vec<Message> {
    bus.into_iter().collect()
}

/// 一个 50x50 的部件，收到滚轮事件时
/// 发布 [`WHEEL_SEEN`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A 50x50 widget that publishes [`WHEEL_SEEN`] when it receives a scroll
/// wheel event.
///
/// </details>
struct WheelSpy;

impl widget::Meta for WheelSpy {}

impl Widget<Message, Theme, ()> for WheelSpy {
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fixed(50.0), Length::Fixed(50.0))
    }

    fn layout(&mut self, tree: &mut Tree, _renderer: &(), limits: &layout::Limits) {
        tree.size = layout::atomic(limits, 50, 50);
    }

    fn draw(
        &self,
        _tree: &Tree,
        _renderer: &mut (),
        _theme: &Theme,
        _style: &renderer::Style,
        _layout: Layout,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
    ) {
    }

    fn update(
        &mut self,
        _tree: &mut Tree,
        event: &Event,
        _layout: Layout,
        _cursor: mouse::Cursor,
        _renderer: &(),
        shell: &mut shell::Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
        if matches!(event, Event::Mouse(mouse::Event::WheelScrolled { .. })) {
            shell.publish(WHEEL_SEEN);
        }
    }
}

#[test]
fn mouse_press_is_captured_by_default() {
    let mut element = new_opaque(false);
    let (mut tree, layout) = setup(&mut element);

    // The 50x50 content occupies (0, 0) .. (50, 50); press inside it.
    // 50x50 的内容占据 (0, 0) .. (50, 50)；在其内部按下。
    let (event, cursor) = press(Point::new(25.0, 25.0));
    let (status, _) = drive(&mut element, &mut tree, layout, &event, cursor);

    assert_eq!(
        status,
        event::Status::Captured,
        "an opaque layer should capture mouse button presses by default"
    );
}

#[test]
fn wheel_scroll_is_not_captured_by_default() {
    let mut element = new_opaque(false);
    let (mut tree, layout) = setup(&mut element);

    let (event, cursor) = scroll(Point::new(25.0, 25.0));
    let (status, _) = drive(&mut element, &mut tree, layout, &event, cursor);

    assert_eq!(
        status,
        event::Status::Ignored,
        "scroll wheel events should not be captured unless the flag is enabled"
    );
}

#[test]
fn wheel_scroll_is_captured_when_flag_enabled() {
    let mut element = new_opaque(true);
    let (mut tree, layout) = setup(&mut element);

    let (event, cursor) = scroll(Point::new(25.0, 25.0));
    let (status, _) = drive(&mut element, &mut tree, layout, &event, cursor);

    assert_eq!(
        status,
        event::Status::Captured,
        "scroll wheel events should be captured when the flag is enabled and hovered"
    );
}

#[test]
fn wheel_scroll_is_not_captured_when_not_hovered() {
    let mut element = new_opaque(true);
    let (mut tree, layout) = setup(&mut element);

    // Scroll somewhere far away: outside the opaque bounds.
    // 在很远的地方滚动：位于 opaque 的边界之外。
    let (event, cursor) = scroll(Point::new(900.0, 900.0));
    let (status, _) = drive(&mut element, &mut tree, layout, &event, cursor);

    assert_eq!(
        status,
        event::Status::Ignored,
        "scroll wheel events should only be captured when the cursor is hovered"
    );
}

#[test]
fn mouse_press_is_still_captured_when_flag_enabled() {
    let mut element = new_opaque(true);
    let (mut tree, layout) = setup(&mut element);

    // The flag must not interfere with the default mouse press capture.
    // 这个标志不能干扰默认的鼠标按下捕获。
    let (event, cursor) = press(Point::new(25.0, 25.0));
    let (status, _) = drive(&mut element, &mut tree, layout, &event, cursor);

    assert_eq!(
        status,
        event::Status::Captured,
        "mouse button presses should still be captured when the flag is enabled"
    );
}

#[test]
fn wrapped_content_still_receives_wheel_scroll() {
    // The flag must not swallow events from the wrapped content: the spy
    // still receives the scroll, even though the event is then captured.
    // 这个标志不能吞掉被包裹内容的事件：即使事件随后被捕获，
    // 侦测部件仍然会收到滚动。
    let mut element = opaque(WheelSpy).capture_scroll(true);
    let (mut tree, layout) = setup(&mut element);

    let (event, cursor) = scroll(Point::new(25.0, 25.0));
    let (status, bus) = drive(&mut element, &mut tree, layout, &event, cursor);

    assert_eq!(
        messages(bus),
        vec![WHEEL_SEEN],
        "the wrapped content should still receive the scroll wheel event"
    );

    assert_eq!(
        status,
        event::Status::Captured,
        "the event should be captured after being delegated to the content"
    );
}
