//! 针对 `popover` 部件的行为测试。
//!
//! 这些测试直接驱动 `Widget` 和 `Overlay`特质（使用空
//! 渲染器 `()`），以验证弹出层只在
//! 其 `popover` 参数为 `Some`（打开）时显示覆盖层，为 `None`
//! （关闭）时则不显示；验证基底是一个普通元素；并验证当用户
//! 在其边界之外点击时，弹出层会通过 `on_close` 处理函数
//! 通知应用程序。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Behavioral tests for the `popover` widget.
//!
//! These drive the `Widget` and `Overlay` traits directly (using the null
//! renderer `()`) to verify that the popover displays its overlay only when
//! its `popover` argument is `Some` (open) and not when it is `None`
//! (closed), that the base is a plain element, and that the popover notifies
//! the application through its `on_close` handler when the user clicks
//! outside of its bounds.
//!
//! </details>
use iced_widget::core::event;
use iced_widget::core::layout::{Layout, Limits};
use iced_widget::core::mouse::{self, Button, Cursor};
use iced_widget::core::shell;
use iced_widget::core::widget::{Tree, Widget};
use iced_widget::core::window::Headless;
use iced_widget::core::{Event, Point, Rectangle, Size, Theme, Vector};
use iced_widget::{Opaque, Popover, Space, button, popover, space};

type Message = u8;

/// 请求关闭时弹出层产生的消息。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The message produced by the popover when a close is requested.
///
/// </details>
const ON_CLOSE: Message = 0x42;

/// 测试视口的大小。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The size of the test viewport.
///
/// </details>
const VIEWPORT: Size = Size::new(1000.0, 1000.0);

/// 测试窗口的大小。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The size of the test window.
///
/// </details>
const WINDOW: Size = Size::new(1000.0, 1000.0);

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

/// 带有 50x50 基底和 80x80 弹出内容（当 `open` 时）的弹出层，并带有
/// `on_close` 处理函数。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A popover with a 50x50 base and an 80x80 popup (when `open`), with an
/// `on_close` handler.
///
/// </details>
fn new_popover(open: bool) -> impl Widget<Message, Theme, ()> {
    new_popover_element(open)
}

/// 同一个弹出层，但作为具体类型，这样它的构建器方法
/// （例如 `capture_scroll`）仍然可以访问。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The same popover as a concrete type, so that its builder methods
/// (e.g. `capture_scroll`) remain accessible.
///
/// </details>
fn new_popover_element(open: bool) -> Popover<Space, Opaque<Space>, Message> {
    popover(
        space().width(50).height(50),
        open.then(|| space().width(80).height(80)),
    )
    .position(popover::Position::Bottom)
    .on_close(ON_CLOSE)
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

/// 返回部件当前是否产生覆盖层。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Returns whether the widget currently produces an overlay.
///
/// </details>
fn has_overlay<W>(widget: &mut W, tree: &mut Tree, layout: Layout) -> bool
where
    W: Widget<Message, Theme, ()>,
{
    !widget
        .overlay(
            tree,
            layout,
            &(),
            &Rectangle::with_size(VIEWPORT),
            Vector::ZERO,
            WINDOW,
        )
        .is_empty()
}

/// 让单个事件经过（打开的）弹出层覆盖层，并返回
/// 产生的事件状态和已发布消息的总线。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Drives a single event through the (open) popover overlay and returns the
/// resulting event status and the bus of published messages.
///
/// </details>
fn drive_overlay<W>(
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

    let mut overlay = widget
        .overlay(tree, layout, &(), &viewport, Vector::ZERO, WINDOW)
        .into_iter()
        .next()
        .expect("popover overlay should exist");

    let mut bus = shell::Bus::new();
    let mut shell = shell::Shell::new(&Headless, shell::Waker::noop(), &mut bus);

    overlay
        .as_overlay_mut()
        .update(event, cursor, &(), &mut shell);

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

#[test]
fn popover_layout_is_plain_base() {
    let mut element = new_popover(true);
    let (_, layout) = setup(&mut element);

    let bounds = layout.bounds();

    // The base is a plain 50x50 element: the popover does not wrap it in
    // padding or any other styling.
    // 基底是一个普通的 50x50 元素：弹出层不会给它加上
    // 内边距或任何其他样式。
    assert_eq!(
        bounds.size(),
        Size::new(50.0, 50.0),
        "popover layout {bounds:?} should be the plain base, with no padding"
    );
}

#[test]
fn base_mouse_interaction_is_delegated() {
    // A `button` base reports a `Pointer` interaction when hovered, and the
    // popover delegates it to the base (which is a plain element).
    // `button` 基底在悬停时报告 `Pointer` 交互，
    // 弹出层把它委托给基底（基底是一个普通元素）。
    let mut element = popover(
        button("Open").width(50).height(50).on_press(0x01),
        Some(space().width(80).height(80)),
    )
    .on_close(ON_CLOSE);

    let (tree, layout) = setup(&mut element);
    let viewport = Rectangle::with_size(VIEWPORT);

    // Hovering the base reports the base's interaction (a `Pointer`, as a
    // `button` would).
    // 悬停在基底上会报告基底的交互（`Pointer`，和
    // `button` 一样）。
    let hovered = element.mouse_interaction(
        &tree,
        layout,
        Cursor::Available(Point::new(25.0, 25.0)),
        &viewport,
        &(),
    );

    assert_eq!(
        hovered,
        mouse::Interaction::Pointer,
        "hovering the base should report the base's interaction"
    );
}

#[test]
fn overlay_present_when_open() {
    let mut element = new_popover(true);
    let (mut tree, layout) = setup(&mut element);

    assert!(
        has_overlay(&mut element, &mut tree, layout),
        "popover should show its overlay when the popover argument is `Some`"
    );
}

#[test]
fn overlay_absent_when_closed() {
    let mut element = new_popover(false);
    let (mut tree, layout) = setup(&mut element);

    assert!(
        !has_overlay(&mut element, &mut tree, layout),
        "popover should not show its overlay when the popover argument is `None`"
    );
}

#[test]
fn click_outside_requests_close() {
    let mut element = new_popover(true);
    let (mut tree, layout) = setup(&mut element);

    // Press somewhere far away: outside both the base and the popover.
    // 在很远的地方按下：位于基底和弹出层之外。
    let (event, cursor) = press(Point::new(900.0, 900.0));
    let (_, bus) = drive_overlay(&mut element, &mut tree, layout, &event, cursor);

    assert_eq!(
        messages(bus),
        vec![ON_CLOSE],
        "clicking outside should request a close"
    );
}

#[test]
fn click_on_base_does_not_request_close() {
    let mut element = new_popover(true);
    let (mut tree, layout) = setup(&mut element);

    // The base occupies (0, 0) .. (50, 50). Pressing it must not request a
    // close: the base is responsible for its own behavior.
    // 基底占据 (0, 0) .. (50, 50)。按下它不能请求
    // 关闭：基底对自己的行为负责。
    let (event, cursor) = press(Point::new(25.0, 25.0));
    let (_, bus) = drive_overlay(&mut element, &mut tree, layout, &event, cursor);

    assert!(
        messages(bus).is_empty(),
        "clicking the base should not request a close"
    );
}

#[test]
fn click_inside_does_not_request_close() {
    let mut element = new_popover(true);
    let (mut tree, layout) = setup(&mut element);

    // The popover appears below the base (Position::Bottom), so it occupies
    // roughly (0, 50) .. (80, 130). Click a point that lies inside the popover.
    // 弹出层出现在基底下方（Position::Bottom），因此它大致
    // 占据 (0, 50) .. (80, 130)。点击位于弹出层内部的一点。
    let (event, cursor) = press(Point::new(40.0, 95.0));
    let (_, bus) = drive_overlay(&mut element, &mut tree, layout, &event, cursor);

    assert!(
        messages(bus).is_empty(),
        "clicking inside the popover should not request a close"
    );
}

#[test]
fn no_on_close_publishes_nothing() {
    // A popover without an `on_close` handler simply does not publish anything
    // when clicked outside.
    // 没有 `on_close` 处理函数的弹出层在外部被点击时，
    // 不会发布任何消息。
    let mut element = popover(
        space().width(50).height(50),
        Some(space().width(80).height(80)),
    );

    let (mut tree, layout) = setup(&mut element);

    let (event, cursor) = press(Point::new(900.0, 900.0));
    let (_, bus) = drive_overlay(&mut element, &mut tree, layout, &event, cursor);

    assert!(
        messages(bus).is_empty(),
        "a popover without an on_close handler should publish nothing"
    );
}

#[test]
fn popup_button_press_is_captured_by_default() {
    let mut element = new_popover_element(true);
    let (mut tree, layout) = setup(&mut element);

    // The popup occupies roughly (0, 50) .. (80, 130); press inside it.
    // 弹出内容大致占据 (0, 50) .. (80, 130)；在其内部按下。
    let (event, cursor) = press(Point::new(40.0, 95.0));
    let (status, _) = drive_overlay(&mut element, &mut tree, layout, &event, cursor);

    assert_eq!(
        status,
        event::Status::Captured,
        "button presses inside the popup should be captured by default"
    );
}

#[test]
fn popup_wheel_scroll_is_not_captured_by_default() {
    let mut element = new_popover_element(true);
    let (mut tree, layout) = setup(&mut element);

    let (event, cursor) = scroll(Point::new(40.0, 95.0));
    let (status, _) = drive_overlay(&mut element, &mut tree, layout, &event, cursor);

    assert_eq!(
        status,
        event::Status::Ignored,
        "scroll wheel events over the popup should not be captured by default"
    );
}

#[test]
fn popup_wheel_scroll_is_captured_when_capture_scroll_enabled() {
    let mut element = new_popover_element(true).capture_scroll(true);
    let (mut tree, layout) = setup(&mut element);

    let (event, cursor) = scroll(Point::new(40.0, 95.0));
    let (status, _) = drive_overlay(&mut element, &mut tree, layout, &event, cursor);

    assert_eq!(
        status,
        event::Status::Captured,
        "scroll wheel events over the popup should be captured when enabled"
    );
}
