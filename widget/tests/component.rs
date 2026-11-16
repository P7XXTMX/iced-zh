//! 针对带有覆盖层的 [`Component`] 的测试。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Tests for [`Component`]s with overlays.
//!
//! </details>
use iced_test::Simulator;
use iced_test::simulator;
use iced_widget::core::{self, Point, Size, mouse};
use iced_widget::{Component, Renderer, Widget, button, component, tooltip};

use std::time::Duration;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Message {
    Pressed,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Event {
    Pressed,
}

/// 视图为按钮上的 [`tooltip`](tooltip) 的 [`Component`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A [`Component`] whose view is a [`tooltip`](tooltip) over a button.
///
/// </details>
struct WithTooltip;

impl<'a> Component<'a, Message> for WithTooltip {
    type State = ();
    type Event = Event;

    fn update(&self, _state: &mut (), event: Event, _renderer: &Renderer) -> Option<Message> {
        (event == Self::Event::Pressed).then_some(Message::Pressed)
    }

    fn view(&self, _state: &()) -> impl Widget<Event> + 'a {
        tooltip(button("Press").on_press(Event::Pressed), "Hover me")
            .position(tooltip::Position::Top)
            .delay(Duration::ZERO)
    }
}

#[test]
fn component_with_overlay() {
    let component = component(WithTooltip);
    let mut simulator = Simulator::new(component);

    // Move the cursor over the button: the tooltip opens
    // 把光标移到按钮上：工具提示打开
    simulator.point_at(Point::new(10.0, 10.0));

    let _ = simulator.simulate([core::Event::Mouse(mouse::Event::CursorMoved {
        position: Point::new(10.0, 10.0),
    })]);

    // The tooltip overlay is part of the user interface now
    // 工具提示的覆盖层现在是用户界面的一部分
    let found = simulator
        .find("Hover me")
        .expect("tooltip overlay should be present");
    assert_ne!(found.bounds().size(), Size::ZERO);

    // The tooltip does not block the button: clicking it still forwards the
    // component's message
    // 工具提示不会挡住按钮：点击按钮仍然会转发
    // 组件的消息
    let _ = simulator.simulate(simulator::click());

    assert_eq!(
        simulator.into_messages().collect::<Vec<_>>(),
        [Message::Pressed]
    );
}
