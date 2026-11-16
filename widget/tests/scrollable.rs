//! 针对 `scrollable` 部件的集成测试。
//!
//! 这些测试通过 `Simulator` 驱动部件，并通过
//! `on_scroll` 通知观察它的行为：不会发布多余的视口，
//! 因此消息的数量也告诉我们平滑滚动动画
//! 用了多少帧。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Integration tests for the `scrollable` widget.
//!
//! These tests drive the widget through the `Simulator` and observe its
//! behavior through the `on_scroll` notifications: no redundant viewports
//! are published, so the number of messages also tells us how many frames
//! the smooth scrolling animation took.
//!
//! </details>
use iced_test::Simulator;
use iced_widget::Renderer;
use iced_widget::core::keyboard::{self, Modifiers};
use iced_widget::core::layout::{self, Layout};
use iced_widget::core::mouse::{self, ScrollDelta};
use iced_widget::core::widget::{Id, Meta, Tree, operation};
use iced_widget::core::window;
use iced_widget::core::{self, Event, Length, Point, Rectangle, Size, Theme, Widget};
use iced_widget::scrollable::{
    AbsoluteOffset, Action, Anchor, Direction, RelativeOffset, Scroll, Scrollable, Scrollbar,
    Source, Viewport,
};
use iced_widget::{Space, space};

use std::cell::RefCell;
use std::rc::Rc;
use std::time::{Duration, Instant};

/// 模拟动画的一帧。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// One frame of the simulated animation.
///
/// </details>
const FRAME: Duration = Duration::from_millis(16);

/// 滚轮每滚动一行的距离（以像素为单位）。
///
/// 使用默认渲染器创建一个 [`Simulator`]，并根据提供的元素
/// 推断消息类型。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The distance (in pixels) scrolled per wheel line.
///
/// Creates a [`Simulator`] with the default renderer, inferring the message
/// type from the provided element.
///
/// </details>
fn simulate<M>(
    element: impl Widget<M, Theme, Renderer> + 'static,
) -> Simulator<'static, M, Theme, Renderer>
where
    M: 'static,
{
    Simulator::new(element)
}

/// 需要与 `scrollable` 模块中私有的 `WHEEL_PX_PER_LINE` 常量
/// 保持同步。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Keep in sync with the private `WHEEL_PX_PER_LINE` constant in the
/// `scrollable` module.
///
/// </details>
const PX_PER_LINE: f32 = 120.0;

/// 给定行数的滚轮事件所滚动的距离
/// （以像素为单位）。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The distance (in pixels) scrolled by a wheel event of the given number
/// of lines.
///
/// </details>
fn px(lines: f32) -> f32 {
    lines * PX_PER_LINE
}

/// 具有给定内容高度和视口高度的 `Scrollable`，每次滚动时
/// 发布它的视口。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A `Scrollable` with the given content and viewport heights, publishing
/// its viewport on every scroll.
///
/// </details>
fn element(content_height: u32, viewport_height: u32) -> Scrollable<'static, Scroll, Space, Theme> {
    Scrollable::new(space().height(content_height))
        .id("scrollable")
        .width(Length::Fill)
        .height(viewport_height)
        .on_scroll(Some)
}

/// 被测 [`element`] 的 [`Id`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The [`Id`] of the [`element`] under test.
///
/// </details>
const ELEMENT_ID: Id = Id::new("scrollable");

/// 步进给定数量的帧，并让实时时钟始终领先于
/// 合成的帧时间线。
///
/// `Simulator` 使用实时时钟为滚轮事件打时间戳，因此如果
/// 合成的帧时刻跑到了它前面，滚轮事件就会被
/// 观察为发生在动画时间线的“过去”。每一帧都休眠
/// 可以保证两条时间线保持一致。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Steps the given number of frames, keeping the real-time clock ahead of
/// the synthetic frame timeline.
///
/// The `Simulator` timestamps wheel events with the real-time clock, so if
/// the synthetic frame instants run ahead of it, a wheel event would be
/// observed "in the past" of the animation's timeline. Sleeping for each
/// frame guarantees the two timelines stay consistent.
///
/// </details>
fn step_frames(simulator: &mut Simulator<'_, Scroll>, instant: &mut Instant, frames: usize) {
    for _ in 0..frames {
        *instant += FRAME;
        std::thread::sleep(FRAME);

        let _ = simulator.simulate([Event::Window(window::Event::RedrawRequested(*instant))]);
    }
}

#[test]
fn wheel_scrolling_is_smooth() {
    let mut simulator = simulate(element(3000, 200));
    simulator.point_at(Point::new(500.0, 100.0));

    // Scroll down two lines
    // 向下滚动两行
    let _ = simulator.scroll(ScrollDelta::Lines { x: 0.0, y: -2.0 });

    // Settle the animation, frame by frame
    // 逐帧让动画稳定下来
    let mut instant = Instant::now();

    for _ in 0..60 {
        instant += FRAME;

        let _ = simulator.simulate([Event::Window(window::Event::RedrawRequested(instant))]);
    }

    let (offsets, animating): (Vec<f32>, Vec<bool>) = simulator
        .into_messages()
        .map(|scroll| (scroll.viewport.absolute_offset().y, scroll.target.is_some()))
        .unzip();

    // The wheel event must not have moved the offset: the first
    // notification still reports the original position
    // 滚轮事件不能已经移动了偏移量：第一条
    // 通知仍然报告原始位置
    assert!(
        !offsets.is_empty(),
        "expected on_scroll notifications, got: {offsets:?}"
    );
    assert_eq!(
        offsets.first(),
        Some(&0.0),
        "offset moved before the first frame: {offsets:?}"
    );

    // The scroll must have been gradual: it took several frames to settle
    // 滚动必须是渐进的：它用了好几帧才稳定下来
    assert!(
        offsets.len() > 2,
        "expected multiple intermediate offsets, got: {offsets:?}"
    );
    assert!(
        *offsets.get(1).unwrap() < px(2.0) / 2.0,
        "scrolling should not jump instantly: {offsets:?}"
    );

    // ... and it must have settled exactly on the target (two lines)
    // ……并且它必须恰好稳定在目标位置（两行）
    assert_eq!(offsets.last(), Some(&px(2.0)));

    // The notifications report the animation as in progress until the
    // scroll settles
    // 在滚动稳定之前，通知都把动画报告为
    // 进行中
    assert!(
        animating[..animating.len() - 1]
            .iter()
            .all(|&is_animating| is_animating),
        "expected the scroll to be animating until it settles: {offsets:?} {animating:?}"
    );
    assert!(
        !animating.last().unwrap(),
        "expected the scroll to have settled: {offsets:?} {animating:?}"
    );
}

#[test]
fn wheel_scrolling_is_immediate_when_smooth_scroll_disabled() {
    let mut simulator = simulate(element(3000, 200).smooth_scroll(false));
    simulator.point_at(Point::new(500.0, 100.0));

    // Scroll down two lines
    // 向下滚动两行
    let _ = simulator.scroll(ScrollDelta::Lines { x: 0.0, y: -2.0 });

    // No frames needed: a single notification, fully scrolled,
    // on the wheel event itself — and no animation
    // 不需要任何帧：只有一条通知，已完全滚动到位，
    // 就在滚轮事件本身上——并且没有动画
    let notifications: Vec<(f32, bool)> = simulator
        .into_messages()
        .map(|scroll| (scroll.viewport.absolute_offset().y, scroll.target.is_some()))
        .collect();

    assert_eq!(notifications, [(px(2.0), false)]);
}

#[test]
fn pixel_scrolling_is_immediate_even_when_smooth_scroll_enabled() {
    let mut simulator = simulate(element(3000, 200));
    simulator.point_at(Point::new(500.0, 100.0));

    // Scroll down 120 pixels
    // 向下滚动 120 像素
    let _ = simulator.scroll(ScrollDelta::Pixels { x: 0.0, y: -120.0 });

    // High-precision scrolls are applied immediately, even though
    // smooth scrolling is enabled by default — and there is no animation
    // 高精度滚动会立即应用，即使
    // 平滑滚动默认是启用的——并且没有动画
    let notifications: Vec<(f32, bool)> = simulator
        .into_messages()
        .map(|scroll| (scroll.viewport.absolute_offset().y, scroll.target.is_some()))
        .collect();

    assert_eq!(notifications, [(120.0, false)]);
}

#[test]
fn smooth_scroll_settles_on_target() {
    let mut simulator = simulate(element(3000, 200));
    simulator.point_at(Point::new(500.0, 100.0));

    // Scroll down one line
    // 向下滚动一行
    let _ = simulator.scroll(ScrollDelta::Lines { x: 0.0, y: -1.0 });

    // Settle the animation, frame by frame
    // 逐帧让动画稳定下来
    let mut instant = Instant::now();

    for _ in 0..60 {
        instant += FRAME;

        let _ = simulator.simulate([Event::Window(window::Event::RedrawRequested(instant))]);
    }

    let offsets: Vec<f32> = simulator
        .into_messages()
        .map(|scroll| scroll.viewport.absolute_offset().y)
        .collect();

    // The offset must approach the target monotonically
    // 偏移量必须单调地接近目标
    let mut previous = 0.0;

    for offset in &offsets {
        assert!(
            *offset >= previous,
            "offset must not scroll backwards: {offsets:?}"
        );

        previous = *offset;
    }

    assert!(
        offsets.len() > 2,
        "smooth scroll must take more than one frame: {offsets:?}"
    );

    // ... and it must settle exactly on the target (one line)
    // ……并且它必须恰好稳定在目标位置（一行）
    assert_eq!(offsets.last(), Some(&px(1.0)));
}

#[test]
fn smooth_scroll_accumulates_pending_delta() {
    let mut simulator = simulate(element(3000, 200));
    simulator.point_at(Point::new(500.0, 100.0));

    // Scroll down one line...
    // 向下滚动一行……
    let _ = simulator.scroll(ScrollDelta::Lines { x: 0.0, y: -1.0 });

    // ...a few frames in, keeping real time consistent with the animation's
    // timeline...
    // ……经过几帧之后（保持实际时间与动画的
    // 时间线一致）……
    let mut instant = Instant::now();
    step_frames(&mut simulator, &mut instant, 3);

    // ...a new wheel movement must add to the pending target, not replace it
    // ……新的滚轮移动必须累加到待处理的目标上，而不是替换它
    let _ = simulator.scroll(ScrollDelta::Lines { x: 0.0, y: -0.5 });

    // Settle the animation, frame by frame
    // 逐帧让动画稳定下来
    for _ in 0..60 {
        instant += FRAME;

        let _ = simulator.simulate([Event::Window(window::Event::RedrawRequested(instant))]);
    }

    let offsets: Vec<f32> = simulator
        .into_messages()
        .map(|scroll| scroll.viewport.absolute_offset().y)
        .collect();

    // The offset must not scroll backwards
    // 偏移量不能向后滚动
    let mut previous = 0.0;

    for offset in &offsets {
        assert!(
            *offset >= previous,
            "offset must not scroll backwards: {offsets:?}"
        );

        previous = *offset;
    }

    // The full distance of both movements was scrolled
    // 两次移动的全部距离都已滚动完毕
    assert_eq!(offsets.last(), Some(&px(1.5)));
}

#[test]
fn smooth_scroll_duration_scales_inversely_with_distance() {
    /// 给定行数的滚轮移动稳定下来所需的
    /// 消息数量（即滚动的帧数）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The number of messages (i.e. scrolled frames) a wheel movement of the
    /// given number of lines takes to settle.
    ///
    /// </details>
    fn settle_frames(lines: f32) -> Vec<f32> {
        let mut simulator = simulate(element(3000, 200));
        simulator.point_at(Point::new(500.0, 100.0));

        let _ = simulator.scroll(ScrollDelta::Lines { x: 0.0, y: -lines });

        let mut instant = Instant::now();

        for _ in 0..60 {
            instant += FRAME;

            let _ = simulator.simulate([Event::Window(window::Event::RedrawRequested(instant))]);
        }

        simulator
            .into_messages()
            .map(|scroll| scroll.viewport.absolute_offset().y)
            .collect()
    }

    // No redundant viewports are published, so the number of messages tells
    // us how many frames the animation took
    // 不会发布多余的视口，因此消息的数量告诉
    // 我们动画用了多少帧

    let short = settle_frames(1.0);
    let medium = settle_frames(2.5);
    let long = settle_frames(10.0);

    assert_eq!(short.last(), Some(&px(1.0)));
    assert_eq!(medium.last(), Some(&px(2.5)));
    assert_eq!(long.last(), Some(&px(10.0)));

    // The longer the distance, the shorter (snappier) the animation
    // 距离越长，动画越短（越干脆）
    assert!(
        short.len() > medium.len(),
        "a short scroll ({:?}) must take more frames than a medium one ({:?})",
        short,
        medium
    );
    assert!(
        medium.len() > long.len(),
        "a medium scroll ({:?}) must take more frames than a long one ({:?})",
        medium,
        long
    );
}

#[test]
fn smooth_scroll_retarget_preserves_velocity() {
    let mut simulator = simulate(element(3000, 200));
    simulator.point_at(Point::new(500.0, 100.0));

    // Scrolling down...
    // 向下滚动……
    let _ = simulator.scroll(ScrollDelta::Lines { x: 0.0, y: -1.0 });

    // ...a few frames in, keeping real time consistent with the animation's
    // timeline...
    // ……经过几帧之后（保持实际时间与动画的
    // 时间线一致）……
    let mut instant = Instant::now();
    step_frames(&mut simulator, &mut instant, 3);

    // ...a new wheel movement retargets the animation
    // ……新的滚轮移动会重新设定动画的目标
    let _ = simulator.scroll(ScrollDelta::Lines { x: 0.0, y: -0.5 });

    // ...and the animation continues, frame by frame
    // ……然后动画逐帧继续
    for _ in 0..60 {
        instant += FRAME;

        let _ = simulator.simulate([Event::Window(window::Event::RedrawRequested(instant))]);
    }

    let offsets: Vec<f32> = simulator
        .into_messages()
        .map(|scroll| scroll.viewport.absolute_offset().y)
        .collect();

    // The wheel event itself does not publish a new viewport (the offset
    // has not moved yet), so:
    // - `offsets[2]` and `offsets[3]` are the last two frames of the
    //   original animation, and
    // - `offsets[4]` is the first frame of the retargeted one
    // 滚轮事件本身不会发布新的视口（偏移量
    // 还没有移动），因此：
    // - `offsets[2]` 和 `offsets[3]` 是原动画的
    //   最后两帧，而
    // - `offsets[4]` 是重新设定目标后的动画的第一帧
    let before = offsets[3] - offsets[2];
    let after = offsets[4] - offsets[3];

    // The retargeted animation must keep the velocity of the original one:
    // it must not restart from rest
    // 重新设定目标后的动画必须保持原动画的速度：
    // 它不能从静止状态重新开始
    assert!(
        after >= before / 2.0,
        "expected the retarget to preserve most of the previous velocity \
         ({before}px/frame), got {after}px/frame: {offsets:?}"
    );

    // ...and it settles exactly on the target (one and a half lines)
    // ……并且它恰好稳定在目标位置（一行半）
    assert_eq!(offsets.last(), Some(&px(1.5)));
}

#[test]
fn smooth_scroll_reversal_carries_momentum() {
    let mut simulator = simulate(element(3000, 200));
    simulator.point_at(Point::new(500.0, 100.0));

    // Scrolling down...
    // 向下滚动……
    let _ = simulator.scroll(ScrollDelta::Lines { x: 0.0, y: -1.0 });

    // ...a few frames in, keeping real time consistent with the animation's
    // timeline...
    // ……经过几帧之后（保持实际时间与动画的
    // 时间线一致）……
    let mut instant = Instant::now();
    step_frames(&mut simulator, &mut instant, 3);

    // ...and then the user scrolls back up past the current position
    // ……然后用户向上滚动，越过了当前位置
    let _ = simulator.scroll(ScrollDelta::Lines { x: 0.0, y: 1.0 });

    // ...and the animation continues, frame by frame
    // ……然后动画逐帧继续
    for _ in 0..60 {
        instant += FRAME;

        let _ = simulator.simulate([Event::Window(window::Event::RedrawRequested(instant))]);
    }

    let offsets: Vec<f32> = simulator
        .into_messages()
        .map(|scroll| scroll.viewport.absolute_offset().y)
        .collect();

    // The wheel event itself does not publish a new viewport (the offset
    // has not moved yet), so:
    // - `offsets[3]` is the last frame of the original animation, and
    // - `offsets[4]` is the first frame of the retargeted one
    // 滚轮事件本身不会发布新的视口（偏移量
    // 还没有移动），因此：
    // - `offsets[3]` 是原动画的最后一帧，而
    // - `offsets[4]` 是重新设定目标后的动画的第一帧

    // The retargeted animation must keep moving down (carrying the previous
    // momentum) before it turns up
    // 重新设定目标后的动画必须先继续向下移动（带着之前的
    // 动量），然后才掉头向上
    assert!(
        offsets[4] > offsets[3],
        "expected the reversal to carry the previous momentum: {offsets:?}"
    );

    // ...and it settles exactly back on the original position
    // ……并且它恰好稳定回到原始位置
    assert_eq!(offsets.last(), Some(&0.0));
}

#[test]
fn high_precision_scrolling_cancels_smooth_scroll() {
    let mut simulator = simulate(element(3000, 200));
    simulator.point_at(Point::new(500.0, 100.0));

    // Start a smooth scroll...
    // 开始一次平滑滚动……
    let _ = simulator.scroll(ScrollDelta::Lines { x: 0.0, y: -1.0 });

    let mut instant = Instant::now();

    for _ in 0..2 {
        instant += FRAME;

        let _ = simulator.simulate([Event::Window(window::Event::RedrawRequested(instant))]);
    }

    // ...and then a high-precision scroll arrives: it must be applied
    // immediately, and cancel the pending smooth scroll
    // ……然后一次高精度滚动到达：它必须立即
    // 应用，并取消待处理的平滑滚动
    let _ = simulator.scroll(ScrollDelta::Pixels { x: 0.0, y: -10.0 });

    // Settle for a while longer
    // 再多等待一段时间让它稳定
    for _ in 0..30 {
        instant += FRAME;

        let _ = simulator.simulate([Event::Window(window::Event::RedrawRequested(instant))]);
    }

    let (offsets, animating): (Vec<f32>, Vec<bool>) = simulator
        .into_messages()
        .map(|scroll| (scroll.viewport.absolute_offset().y, scroll.target.is_some()))
        .unzip();

    // Exactly four notifications: the wheel event (at the original
    // position), the two frames of the smooth scroll, and the immediate
    // high-precision scroll — and nothing after that
    // 恰好四条通知：滚轮事件（位于原始
    // 位置）、平滑滚动的两帧，以及立即应用的
    // 高精度滚动——此后再没有其他通知
    assert_eq!(offsets.len(), 4, "unexpected notifications: {offsets:?}");

    // The high-precision scroll moved the offset by exactly its delta
    // 高精度滚动把偏移量恰好移动了它的增量
    assert!((offsets[3] - (offsets[2] + 10.0)).abs() < 1e-3);

    // The notifications report the smooth scroll as in progress until the
    // high-precision scroll cancels it
    // 在高精度滚动取消平滑滚动之前，通知都把
    // 平滑滚动报告为进行中
    assert_eq!(&animating[..], &[true, true, true, false]);
}

#[test]
fn smooth_scroll_is_noop_without_overflow() {
    // The content fits: there is nothing to scroll
    // 内容可以完全容纳：没有可滚动的内容
    let mut simulator = simulate(element(100, 200));
    simulator.point_at(Point::new(500.0, 100.0));

    let _ = simulator.scroll(ScrollDelta::Lines { x: 0.0, y: -1.0 });

    let mut instant = Instant::now();

    for _ in 0..30 {
        instant += FRAME;

        let _ = simulator.simulate([Event::Window(window::Event::RedrawRequested(instant))]);
    }

    // Nothing is published: no offset ever moved
    // 没有发布任何消息：偏移量从未移动过
    let offsets: Vec<f32> = simulator
        .into_messages()
        .map(|scroll| scroll.viewport.absolute_offset().y)
        .collect();

    assert!(offsets.is_empty(), "unexpected notifications: {offsets:?}");
}

#[test]
fn scroll_to_with_smooth_behavior_is_smooth() {
    let mut simulator = simulate(element(3000, 200));

    let mut operation = operation::scrollable::scroll_to(
        ELEMENT_ID,
        AbsoluteOffset {
            x: None,
            y: Some(px(2.0)),
        },
        operation::Animation::Smooth,
    );

    simulator.operate(&mut operation);

    // Settle the animation, frame by frame
    // 逐帧让动画稳定下来
    let mut instant = Instant::now();
    step_frames(&mut simulator, &mut instant, 60);

    let (offsets, animating): (Vec<f32>, Vec<bool>) = simulator
        .into_messages()
        .map(|scroll| (scroll.viewport.absolute_offset().y, scroll.target.is_some()))
        .unzip();

    // The operation is backdated by one nominal frame: the first drawn
    // frame must already show progress
    // 操作的时间被回拨了一个标称帧：绘制的第一
    // 帧必须已经显示出进度
    assert!(
        offsets.first() > Some(&0.0) && offsets.first() < Some(&px(2.0)),
        "the first frame must already show progress: {offsets:?}"
    );

    // The scroll must have been gradual: it took several frames to settle
    // 滚动必须是渐进的：它用了好几帧才稳定下来
    assert!(
        offsets.len() > 2,
        "expected multiple intermediate offsets, got: {offsets:?}"
    );

    // ... and it must have settled exactly on the target (two lines)
    // ……并且它必须恰好稳定在目标位置（两行）
    assert_eq!(offsets.last(), Some(&px(2.0)));

    // The notifications report the animation as in progress until the
    // scroll settles
    // 在滚动稳定之前，通知都把动画报告为
    // 进行中
    assert!(
        animating[..animating.len() - 1]
            .iter()
            .all(|&is_animating| is_animating),
        "expected the scroll to be animating until it settles: {offsets:?} {animating:?}"
    );
    assert!(
        !animating.last().unwrap(),
        "expected the scroll to have settled: {offsets:?} {animating:?}"
    );
}

#[test]
fn scroll_to_with_instant_behavior_is_immediate() {
    let mut simulator = simulate(element(3000, 200));

    let mut operation = operation::scrollable::scroll_to(
        ELEMENT_ID,
        AbsoluteOffset {
            x: None,
            y: Some(px(2.0)),
        },
        operation::Animation::Instant,
    );

    simulator.operate(&mut operation);

    let mut instant = Instant::now();
    step_frames(&mut simulator, &mut instant, 2);

    let (offsets, animating): (Vec<f32>, Vec<bool>) = simulator
        .into_messages()
        .map(|scroll| (scroll.viewport.absolute_offset().y, scroll.target.is_some()))
        .unzip();

    // The offset must be at the target on the very first frame, with no
    // animation in between
    // 偏移量必须在第一帧就位于目标位置，中间
    // 没有动画
    assert_eq!(
        offsets.len(),
        1,
        "expected a single notification, got: {offsets:?}"
    );
    assert_eq!(offsets.first(), Some(&px(2.0)));
    assert!(!animating[0]);
}

#[test]
fn scroll_to_with_auto_behavior_follows_smooth_scroll() {
    // With smooth scrolling enabled (the default), `Auto` resolves to a
    // smooth scroll
    // 启用平滑滚动（默认）时，`Auto` 解析为
    // 平滑滚动
    {
        let mut simulator = simulate(element(3000, 200));

        let mut operation = operation::scrollable::scroll_to(
            ELEMENT_ID,
            AbsoluteOffset {
                x: None,
                y: Some(px(2.0)),
            },
            operation::Animation::Auto,
        );

        simulator.operate(&mut operation);

        let mut instant = Instant::now();
        step_frames(&mut simulator, &mut instant, 60);

        let offsets: Vec<f32> = simulator
            .into_messages()
            .map(|scroll| scroll.viewport.absolute_offset().y)
            .collect();

        assert!(
            offsets.len() > 2,
            "`Auto` must follow the enabled smooth scrolling: {offsets:?}"
        );
        assert_eq!(offsets.last(), Some(&px(2.0)));
    }

    // ... while with smooth scrolling disabled, it resolves to an immediate
    // scroll
    // ……而禁用平滑滚动时，它解析为立即
    // 滚动
    {
        let mut simulator = simulate(element(3000, 200).smooth_scroll(false));

        let mut operation = operation::scrollable::scroll_to(
            ELEMENT_ID,
            AbsoluteOffset {
                x: None,
                y: Some(px(2.0)),
            },
            operation::Animation::Auto,
        );

        simulator.operate(&mut operation);

        let mut instant = Instant::now();
        step_frames(&mut simulator, &mut instant, 2);

        let offsets: Vec<f32> = simulator
            .into_messages()
            .map(|scroll| scroll.viewport.absolute_offset().y)
            .collect();

        assert_eq!(
            offsets.len(),
            1,
            "`Auto` must follow the disabled smooth scrolling: {offsets:?}"
        );
        assert_eq!(offsets.first(), Some(&px(2.0)));
    }
}

#[test]
fn snap_to_with_smooth_behavior_settles_at_percentage() {
    let mut simulator = simulate(element(3000, 200));

    let mut operation = operation::scrollable::snap_to(
        ELEMENT_ID,
        RelativeOffset {
            x: None,
            y: Some(1.0),
        },
        operation::Animation::Smooth,
    );

    simulator.operate(&mut operation);

    let mut instant = Instant::now();
    step_frames(&mut simulator, &mut instant, 60);

    let offsets: Vec<f32> = simulator
        .into_messages()
        .map(|scroll| scroll.viewport.absolute_offset().y)
        .collect();

    // The 100% snap must settle exactly at the end of the content
    // 100% 的吸附必须恰好稳定在内容的末尾
    assert_eq!(offsets.last(), Some(&(3000.0 - 200.0)));
    assert!(
        offsets.len() > 2,
        "expected a smooth animation: {offsets:?}"
    );
}

#[test]
fn scroll_by_with_smooth_behavior_accumulates() {
    let mut simulator = simulate(element(3000, 200));

    // Start a smooth scroll of one line...
    // 开始一次一行的平滑滚动……
    let mut operation = operation::scrollable::scroll_by(
        ELEMENT_ID,
        AbsoluteOffset { x: 0.0, y: px(1.0) },
        operation::Animation::Smooth,
    );

    simulator.operate(&mut operation);

    let mut instant = Instant::now();
    step_frames(&mut simulator, &mut instant, 2);

    // ... and add another line while it is still running
    // ……并在它仍在运行时再加一行
    let mut operation = operation::scrollable::scroll_by(
        ELEMENT_ID,
        AbsoluteOffset { x: 0.0, y: px(1.0) },
        operation::Animation::Smooth,
    );

    simulator.operate(&mut operation);
    step_frames(&mut simulator, &mut instant, 60);

    let offsets: Vec<f32> = simulator
        .into_messages()
        .map(|scroll| scroll.viewport.absolute_offset().y)
        .collect();

    // The delta must accumulate onto the pending target
    // 增量必须累加到待处理的目标上
    assert_eq!(offsets.last(), Some(&px(2.0)));
}

#[test]
fn scroll_to_with_smooth_behavior_replaces_pending_target() {
    let mut simulator = simulate(element(3000, 200));

    // Start a smooth scroll of one line...
    // 开始一次一行的平滑滚动……
    let mut operation = operation::scrollable::scroll_by(
        ELEMENT_ID,
        AbsoluteOffset { x: 0.0, y: px(1.0) },
        operation::Animation::Smooth,
    );

    simulator.operate(&mut operation);

    let mut instant = Instant::now();
    step_frames(&mut simulator, &mut instant, 2);

    // ... then scroll to an absolute target while it is still running
    // ……然后在它仍在运行时滚动到一个绝对目标
    let mut operation = operation::scrollable::scroll_to(
        ELEMENT_ID,
        AbsoluteOffset {
            x: None,
            y: Some(px(3.0)),
        },
        operation::Animation::Smooth,
    );

    simulator.operate(&mut operation);
    step_frames(&mut simulator, &mut instant, 60);

    let offsets: Vec<f32> = simulator
        .into_messages()
        .map(|scroll| scroll.viewport.absolute_offset().y)
        .collect();

    // The absolute target must replace the pending one, not accumulate
    // 绝对目标必须替换待处理的目标，而不是累加
    assert_eq!(offsets.last(), Some(&px(3.0)));
}

/// 对内容坐标的一次观察，由一帧之内的某一遍
/// （`update` 或 `draw`）所见。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A single observation of the content's coordinates, as seen by one pass
/// (`update` or `draw`) within a frame.
///
/// </details>
#[derive(Debug)]
struct Observation {
    drawn: bool,
    cursor: f32,
    viewport: f32,
}

/// 一个子部件，记录它在 `update` 和 `draw` 中
/// 收到的 `cursor` 和 `viewport`，以便我们断言这两遍
/// 在一帧之内是一致的。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A child widget that records the `cursor` and `viewport` it receives in
/// both `update` and `draw`, so we can assert that the two passes agree
/// within a frame.
///
/// </details>
struct Recorder {
    observations: Rc<RefCell<Vec<Observation>>>,
}

impl Recorder {
    fn observe(&self, drawn: bool, cursor: mouse::Cursor, viewport: &Rectangle) {
        let mut observations = self.observations.borrow_mut();

        observations.push(Observation {
            drawn,
            cursor: cursor
                .position()
                .map(|position| position.y)
                .unwrap_or(f32::NAN),
            viewport: viewport.y,
        });
    }
}

impl Meta for Recorder {}

impl<Message, Theme, Renderer> Widget<Message, Theme, Renderer> for Recorder {
    fn size(&self) -> Size<Length> {
        Size::new(Length::Fill, Length::Fill)
    }

    fn layout(&mut self, tree: &mut Tree, _renderer: &Renderer, limits: &layout::Limits) {
        tree.size = layout::atomic(limits, Length::Fill, Length::Fixed(1000.0));
    }

    fn update(
        &mut self,
        _tree: &mut Tree,
        _event: &Event,
        _layout: Layout,
        cursor: mouse::Cursor,
        _renderer: &Renderer,
        _shell: &mut core::Shell<'_, Message>,
        viewport: &Rectangle,
    ) {
        self.observe(false, cursor, viewport);
    }

    fn draw(
        &self,
        _tree: &Tree,
        _renderer: &mut Renderer,
        _theme: &Theme,
        _style: &core::renderer::Style,
        _layout: Layout,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        self.observe(true, cursor, viewport);
    }
}

#[test]
fn content_update_and_draw_see_the_same_translation() {
    let observations = Rc::new(RefCell::new(Vec::new()));

    let element: Scrollable<'static, Viewport, Recorder, Theme> = Scrollable::new(Recorder {
        observations: observations.clone(),
    })
    .width(Length::Fill)
    .height(200);

    let mut simulator = simulate(element);
    simulator.point_at(Point::new(500.0, 100.0));

    // A smooth scroll moves the content on every redraw frame; within
    // each frame the content's `update` and `draw` must observe the same
    // translation
    // 平滑滚动在每个重绘帧都会移动内容；在
    // 每一帧之内，内容的 `update` 和 `draw` 必须观察到相同的
    // 平移量
    let _ = simulator.scroll(ScrollDelta::Lines { x: 0.0, y: -5.0 });

    for _ in 0..30 {
        let before = observations.borrow().len();

        // A frame: the `RedrawRequested` update pass and the draw pass
        // 一帧：`RedrawRequested` 更新遍和绘制遍
        simulator.draw(&Theme::Dark);

        let current = observations.borrow_mut().split_off(before);

        let (Some(update), Some(draw)) = (
            current.iter().find(|observation| !observation.drawn),
            current.iter().find(|observation| observation.drawn),
        ) else {
            panic!("expected an `update` and a `draw` observation: {current:?}");
        };

        assert_eq!(
            update.viewport, draw.viewport,
            "content `update` and `draw` disagree on the viewport within a frame"
        );

        assert!(
            update.cursor == draw.cursor || (update.cursor.is_nan() && draw.cursor.is_nan()),
            "content `update` and `draw` disagree on the cursor within a frame: \
             {update:?} vs {draw:?}"
        );

        // Give the animation a frame's worth of time before the next redraw
        // 在下一次重绘之前，给动画一帧的时间
        std::thread::sleep(FRAME);
    }
}

#[test]
fn snap_to_end_with_smooth_behavior_stays_snapped_when_content_grows() {
    let mut simulator = simulate(element(3000, 200));

    let mut operation = operation::scrollable::snap_to(
        ELEMENT_ID,
        RelativeOffset {
            x: None,
            y: Some(1.0),
        },
        operation::Animation::Smooth,
    );

    simulator.operate(&mut operation);

    // Settle the animation, frame by frame
    // 逐帧让动画稳定下来
    let mut instant = Instant::now();
    step_frames(&mut simulator, &mut instant, 60);

    // Grow the content while keeping the state, like an app would on a new
    // message
    // 在保留状态的同时让内容增长，就像应用程序收到新
    // 消息时那样
    simulator = simulator.rebuild(element(6000, 200));

    // The snapped (relative) offset must re-resolve to the new end
    // 吸附的（相对）偏移量必须重新解析到新的末尾
    step_frames(&mut simulator, &mut instant, 2);

    let offsets: Vec<f32> = simulator
        .into_messages()
        .map(|scroll| scroll.viewport.absolute_offset().y)
        .collect();

    assert_eq!(
        offsets.last(),
        Some(&5800.0),
        "the smooth snap must stay snapped to the end after the content grows: {offsets:?}"
    );
}

#[test]
fn snap_to_end_with_smooth_behavior_tracks_content_growth_mid_scroll() {
    let mut simulator = simulate(element(3000, 200));

    let mut operation = operation::scrollable::snap_to(
        ELEMENT_ID,
        RelativeOffset {
            x: None,
            y: Some(1.0),
        },
        operation::Animation::Smooth,
    );

    simulator.operate(&mut operation);

    // A few frames into the animation, grow the content
    // 动画进行几帧之后，让内容增长
    let mut instant = Instant::now();
    step_frames(&mut simulator, &mut instant, 5);
    simulator = simulator.rebuild(element(6000, 200));

    // Settle the retargeted animation
    // 让重新设定目标后的动画稳定下来
    step_frames(&mut simulator, &mut instant, 60);

    let offsets: Vec<f32> = simulator
        .into_messages()
        .map(|scroll| scroll.viewport.absolute_offset().y)
        .collect();

    // The animation must retarget to the new end, not settle on the stale one
    // 动画必须把目标重新设定为新的末尾，而不是稳定在过期的末尾
    assert_eq!(
        offsets.last(),
        Some(&5800.0),
        "the scroll must settle on the grown end: {offsets:?}"
    );

    // ... and the offset must never move backwards along the way
    // ……并且偏移量在此过程中绝不能向后移动
    assert!(
        offsets.windows(2).all(|pair| pair[1] >= pair[0]),
        "the offset must not jump: {offsets:?}"
    );
}

#[test]
fn wheel_during_smooth_snap_to_end_drops_the_snap() {
    let mut simulator = simulate(element(3000, 200));
    simulator.point_at(Point::new(500.0, 100.0));

    let mut operation = operation::scrollable::snap_to(
        ELEMENT_ID,
        RelativeOffset {
            x: None,
            y: Some(1.0),
        },
        operation::Animation::Smooth,
    );

    simulator.operate(&mut operation);

    // A few frames into the animation, scroll up one line with the wheel
    // 动画进行几帧之后，用滚轮向上滚动一行
    let mut instant = Instant::now();
    step_frames(&mut simulator, &mut instant, 5);

    let _ = simulator.scroll(ScrollDelta::Lines { x: 0.0, y: 1.0 });

    // Settle the retargeted animation: the pending destination (the end,
    // 2800 px) minus one line (120 px)
    // 让重新设定目标后的动画稳定下来：待处理的终点（末尾，
    // 2800 px）减去一行（120 px）
    step_frames(&mut simulator, &mut instant, 60);

    // The snap must have been replaced by the wheel's absolute target:
    // growing the content no longer moves the offset
    // 吸附必须已被滚轮的绝对目标替换：
    // 内容增长不再移动偏移量
    simulator = simulator.rebuild(element(6000, 200));
    step_frames(&mut simulator, &mut instant, 2);

    let offsets: Vec<f32> = simulator
        .into_messages()
        .map(|scroll| scroll.viewport.absolute_offset().y)
        .collect();

    assert_eq!(
        offsets.last(),
        Some(&2680.0),
        "the wheel must replace the snap with an absolute target: {offsets:?}"
    );
}

#[test]
fn snap_to_end_with_smooth_behavior_stays_snapped_when_already_at_destination() {
    let mut simulator = simulate(element(3000, 200));

    // Scroll to the end with an immediate (absolute) offset
    // 使用立即（绝对）偏移量滚动到末尾
    let mut operation = operation::scrollable::scroll_to(
        ELEMENT_ID,
        AbsoluteOffset {
            x: None,
            y: Some(2800.0),
        },
        operation::Animation::Instant,
    );

    simulator.operate(&mut operation);

    // Let the immediate scroll notify
    // 让立即滚动发出通知
    let mut instant = Instant::now();
    step_frames(&mut simulator, &mut instant, 2);

    // Snap to the end smoothly: there is nothing to animate, but the offset
    // must still become relative (snapped)
    // 平滑地吸附到末尾：没有什么可以做动画的，但偏移量
    // 仍然必须变为相对的（吸附的）
    let mut operation = operation::scrollable::snap_to(
        ELEMENT_ID,
        RelativeOffset {
            x: None,
            y: Some(1.0),
        },
        operation::Animation::Smooth,
    );

    simulator.operate(&mut operation);

    // Grow the content: the snapped offset must pin the scroll to the end
    // 让内容增长：吸附的偏移量必须把滚动固定在末尾
    simulator = simulator.rebuild(element(6000, 200));
    step_frames(&mut simulator, &mut instant, 2);

    let offsets: Vec<f32> = simulator
        .into_messages()
        .map(|scroll| scroll.viewport.absolute_offset().y)
        .collect();

    assert_eq!(
        offsets.last(),
        Some(&5800.0),
        "the snap must apply even when already at the destination: {offsets:?}"
    );
}

#[test]
fn snap_to_end_with_smooth_behavior_stays_snapped_without_overflow() {
    let mut simulator = simulate(element(100, 200));

    let mut operation = operation::scrollable::snap_to(
        ELEMENT_ID,
        RelativeOffset {
            x: None,
            y: Some(1.0),
        },
        operation::Animation::Smooth,
    );

    simulator.operate(&mut operation);

    // The content does not overflow, so there is nothing to animate; the
    // snapped (relative) offset must still be applied
    // 内容没有溢出，因此没有什么可以做动画的；
    // 吸附的（相对）偏移量仍然必须被应用
    let mut instant = Instant::now();
    step_frames(&mut simulator, &mut instant, 2);

    // Once the content overflows, the snapped offset must pin the scroll
    // to the end
    // 一旦内容溢出，吸附的偏移量必须把滚动固定
    // 在末尾
    simulator = simulator.rebuild(element(3000, 200));
    step_frames(&mut simulator, &mut instant, 2);

    let offsets: Vec<f32> = simulator
        .into_messages()
        .map(|scroll| scroll.viewport.absolute_offset().y)
        .collect();

    assert_eq!(
        offsets.last(),
        Some(&2800.0),
        "the snap must apply even without overflow: {offsets:?}"
    );
}

/// 具有给定大小的内容和 1024 × 200 视口的 `Scrollable`，
/// 因此内容在两个轴上都会溢出。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A `Scrollable` with content of the given size and a 1024 × 200 viewport,
/// so that the content overflows on both axes.
///
/// </details>
fn element_xy(
    content_width: u32,
    content_height: u32,
) -> Scrollable<'static, Scroll, Space, Theme> {
    Scrollable::new(
        space()
            .width(Length::Fixed(content_width as f32))
            .height(Length::Fixed(content_height as f32)),
    )
    .id("scrollable")
    .width(Length::Fill)
    .height(200)
    .direction(Direction::Both {
        vertical: Scrollbar::default(),
        horizontal: Scrollbar::default(),
    })
    .on_scroll(Some)
}

#[test]
fn snap_to_end_with_smooth_behavior_preserves_other_axis_snappedness() {
    let mut simulator = simulate(element_xy(2000, 3000));

    // Pin the horizontal scroll to the end (instantly)
    // 把水平滚动固定在末尾（立即）
    let mut operation = operation::scrollable::snap_to(
        ELEMENT_ID,
        RelativeOffset {
            x: Some(1.0),
            y: None,
        },
        operation::Animation::Instant,
    );

    simulator.operate(&mut operation);

    let mut instant = Instant::now();
    step_frames(&mut simulator, &mut instant, 2);

    // Snap the vertical scroll to the end smoothly: the horizontal offset
    // must keep its snappedness
    // 平滑地把垂直滚动吸附到末尾：水平偏移量
    // 必须保持它的吸附状态
    let mut operation = operation::scrollable::snap_to(
        ELEMENT_ID,
        RelativeOffset {
            x: None,
            y: Some(1.0),
        },
        operation::Animation::Smooth,
    );

    simulator.operate(&mut operation);
    step_frames(&mut simulator, &mut instant, 60);

    // Grow the content wider: the horizontally-snapped offset must follow
    // the new right edge (3000 − 1024), not freeze at the old one (976)
    // 让内容变宽：水平吸附的偏移量必须跟随
    // 新的右边缘（3000 − 1024），而不是停留在旧的右边缘（976）
    simulator = simulator.rebuild(element_xy(3000, 3000));
    step_frames(&mut simulator, &mut instant, 2);

    let offsets: Vec<f32> = simulator
        .into_messages()
        .map(|scroll| scroll.viewport.absolute_offset().x)
        .collect();

    assert_eq!(
        offsets.last(),
        Some(&1976.0),
        "the horizontal snappedness must survive the vertical snap: {offsets:?}"
    );
}

#[test]
fn scrollbar_drag_unsnaps_only_the_dragged_axis() {
    let mut simulator = simulate(element_xy(2000, 3000));

    // Pin the horizontal scroll to the end (instantly)
    // 把水平滚动固定在末尾（立即）
    let mut operation = operation::scrollable::snap_to(
        ELEMENT_ID,
        RelativeOffset {
            x: Some(1.0),
            y: None,
        },
        operation::Animation::Instant,
    );

    simulator.operate(&mut operation);

    let mut instant = Instant::now();
    step_frames(&mut simulator, &mut instant, 2);

    // Drag the vertical scroller (right edge, 10 px wide) partway down.
    // The scroller sits at the top of the track at the current offset, so
    // grab it there
    // 把垂直滚动块（右边缘，10 px 宽）向下拖动一段距离。
    // 在当前偏移量下，滚动块位于轨道的顶部，因此
    // 在那里抓住它
    simulator.point_at(Point::new(1019.0, 6.0));
    let _ = simulator.simulate([Event::Mouse(mouse::Event::ButtonPressed(
        mouse::Button::Left,
    ))]);

    simulator.point_at(Point::new(1019.0, 150.0));
    let _ = simulator.simulate([Event::Mouse(mouse::Event::CursorMoved {
        position: Point::new(1019.0, 150.0),
    })]);

    let _ = simulator.simulate([Event::Mouse(mouse::Event::ButtonReleased(
        mouse::Button::Left,
    ))]);

    step_frames(&mut simulator, &mut instant, 2);

    // Grow the content wider
    // 让内容变宽
    simulator = simulator.rebuild(element_xy(3000, 3000));
    step_frames(&mut simulator, &mut instant, 2);

    let viewports: Vec<Scroll> = simulator.into_messages().collect();

    let (dragged, grown) = (
        viewports[viewports.len() - 2].viewport.absolute_offset(),
        viewports.last().unwrap().viewport.absolute_offset(),
    );

    // The drag must have registered, moving the vertical scroll
    // 拖动必须已经生效，移动了垂直滚动
    assert!(
        grown.y > 1000.0,
        "the drag must move the vertical scroll: {viewports:?}"
    );

    // The drag must have materialized the vertical offset: growing the
    // content leaves it where the drag left it
    // 拖动必须已经把垂直偏移量固化下来：内容
    // 增长之后，它仍然停在拖动结束时的位置
    assert_eq!(
        dragged.y, grown.y,
        "the dragged axis must not keep its snappedness: {viewports:?}"
    );

    // ... while the horizontal snappedness follows the new right edge
    // ……而水平方向的吸附状态跟随新的右边缘
    assert_eq!(
        grown.x, 1976.0,
        "the horizontal snappedness must survive the vertical drag: {viewports:?}"
    );
}

/// 填满模拟器窗口的 `Scrollable`，其内容具有
/// 给定的高度。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A `Scrollable` that fills the simulator's window, with content of the
/// given height.
///
/// </details>
fn fill_element(content_height: u32) -> Scrollable<'static, Scroll, Space, Theme> {
    Scrollable::new(space().height(content_height))
        .id("scrollable")
        .width(Length::Fill)
        .height(Length::Fill)
        .on_scroll(Some)
}

#[test]
fn notifications_report_the_wheel_source() {
    let mut simulator = simulate(element(3000, 200));
    simulator.point_at(Point::new(500.0, 100.0));

    let _ = simulator.scroll(ScrollDelta::Lines { x: 0.0, y: -1.0 });

    let mut instant = Instant::now();
    step_frames(&mut simulator, &mut instant, 60);

    let sources: Vec<Source> = simulator
        .into_messages()
        .map(|scroll| scroll.source)
        .collect();

    assert!(
        !sources.is_empty(),
        "expected wheel notifications: {sources:?}"
    );
    assert!(
        sources.iter().all(|source| matches!(source, Source::Wheel)),
        "all wheel notifications must report `Wheel`: {sources:?}"
    );
}

#[test]
fn notifications_report_the_operation_source() {
    let mut simulator = simulate(element(3000, 200));

    let mut instant = Instant::now();

    // A smooth scroll operation
    // 一次平滑滚动操作
    let mut operation = operation::scrollable::scroll_by(
        ELEMENT_ID,
        AbsoluteOffset { x: 0.0, y: 1000.0 },
        operation::Animation::Smooth,
    );

    simulator.operate(&mut operation);

    step_frames(&mut simulator, &mut instant, 60);

    // An immediate scroll operation
    // 一次立即滚动操作
    let mut operation = operation::scrollable::scroll_to(
        ELEMENT_ID,
        AbsoluteOffset {
            x: None,
            y: Some(600.0),
        },
        operation::Animation::Instant,
    );

    simulator.operate(&mut operation);

    step_frames(&mut simulator, &mut instant, 2);

    // A smooth snap operation
    // 一次平滑吸附操作
    let mut operation = operation::scrollable::snap_to(
        ELEMENT_ID,
        RelativeOffset {
            x: None,
            y: Some(1.0),
        },
        operation::Animation::Smooth,
    );

    simulator.operate(&mut operation);
    step_frames(&mut simulator, &mut instant, 60);

    let sources: Vec<Source> = simulator
        .into_messages()
        .map(|scroll| scroll.source)
        .collect();

    assert!(
        !sources.is_empty(),
        "expected operation notifications: {sources:?}"
    );
    assert!(
        sources
            .iter()
            .all(|source| matches!(source, Source::Operation)),
        "all operation notifications must report `Operation`: {sources:?}"
    );
}

#[test]
fn notifications_report_the_scrollbar_source() {
    let mut simulator = simulate(element_xy(2000, 3000));

    let mut instant = Instant::now();
    step_frames(&mut simulator, &mut instant, 2);

    // Drag the vertical scroller (right edge, 10 px wide) partway down.
    // The scroller sits at the top of the track at the current offset, so
    // grab it there
    // 把垂直滚动块（右边缘，10 px 宽）向下拖动一段距离。
    // 在当前偏移量下，滚动块位于轨道的顶部，因此
    // 在那里抓住它
    simulator.point_at(Point::new(1019.0, 6.0));
    let _ = simulator.simulate([Event::Mouse(mouse::Event::ButtonPressed(
        mouse::Button::Left,
    ))]);

    simulator.point_at(Point::new(1019.0, 100.0));
    let _ = simulator.simulate([Event::Mouse(mouse::Event::CursorMoved {
        position: Point::new(1019.0, 100.0),
    })]);

    simulator.point_at(Point::new(1019.0, 150.0));
    let _ = simulator.simulate([Event::Mouse(mouse::Event::CursorMoved {
        position: Point::new(1019.0, 150.0),
    })]);

    let _ = simulator.simulate([Event::Mouse(mouse::Event::ButtonReleased(
        mouse::Button::Left,
    ))]);

    let sources: Vec<Source> = simulator
        .into_messages()
        .map(|scroll| scroll.source)
        .collect();

    assert_eq!(
        sources,
        vec![Source::Content, Source::Scrollbar, Source::Scrollbar],
        "the drag must report `Scrollbar`: {sources:?}"
    );
}

#[test]
fn notifications_report_the_auto_scroll_source() {
    let element = Scrollable::new(space().height(Length::Fixed(3000.0)))
        .id("scrollable")
        .width(Length::Fill)
        .height(200)
        .auto_scroll(true)
        .on_scroll(Some);

    let mut simulator = simulate(element);

    let mut instant = Instant::now();
    step_frames(&mut simulator, &mut instant, 2);

    // Press the middle button over the content, then move the cursor below
    // the scrollable to auto-scroll down
    // 在内容上按下鼠标中键，然后把光标移到
    // 可滚动区域下方，以自动向下滚动
    simulator.point_at(Point::new(500.0, 100.0));
    let _ = simulator.simulate([Event::Mouse(mouse::Event::ButtonPressed(
        mouse::Button::Middle,
    ))]);

    simulator.point_at(Point::new(500.0, 400.0));
    let _ = simulator.simulate([Event::Mouse(mouse::Event::CursorMoved {
        position: Point::new(500.0, 400.0),
    })]);

    step_frames(&mut simulator, &mut instant, 2);

    let sources: Vec<Source> = simulator
        .into_messages()
        .map(|scroll| scroll.source)
        .collect();

    assert!(
        sources
            .iter()
            .any(|source| matches!(source, Source::AutoScroll)),
        "the auto-scroll must report `AutoScroll`: {sources:?}"
    );
}

#[test]
fn notifications_report_the_layout_source() {
    let mut simulator = simulate(fill_element(3000));

    // The first notification reports the initial layout
    // 第一条通知报告初始布局
    let mut instant = Instant::now();
    step_frames(&mut simulator, &mut instant, 1);

    let sources: Vec<Source> = simulator.drain().map(|scroll| scroll.source).collect();
    assert_eq!(
        sources,
        vec![Source::Content],
        "the initial notification must report `Content`: {sources:?}"
    );

    // Growing the content reports `Content`
    // 内容增长时报告 `Content`
    simulator = simulator.rebuild(fill_element(6000));
    step_frames(&mut simulator, &mut instant, 1);

    let sources: Vec<Source> = simulator.drain().map(|scroll| scroll.source).collect();
    assert_eq!(
        sources,
        vec![Source::Content],
        "the content growth must report `Content`: {sources:?}"
    );

    // Resizing the window reports `Resize`
    // 调整窗口大小时报告 `Resize`
    simulator = simulator.resize(Size::new(1024.0, 600.0));
    step_frames(&mut simulator, &mut instant, 1);

    let sources: Vec<Source> = simulator.drain().map(|scroll| scroll.source).collect();
    assert_eq!(
        sources,
        vec![Source::Resize],
        "the resize must report `Resize`: {sources:?}"
    );
}

#[test]
fn destination_is_snapped_during_animation() {
    let mut simulator = simulate(element(3000, 200));

    // Smoothly snap to the end: the animation targets a relative offset,
    // so the Y axis is snapped while it is still in flight
    // 平滑地吸附到末尾：动画的目标是一个相对偏移量，
    // 因此 Y 轴在动画仍在进行时就已处于吸附状态
    let mut operation = operation::scrollable::snap_to(
        ELEMENT_ID,
        RelativeOffset {
            x: None,
            y: Some(1.0),
        },
        operation::Animation::Smooth,
    );

    simulator.operate(&mut operation);

    let mut instant = Instant::now();
    step_frames(&mut simulator, &mut instant, 1);

    let scroll = simulator.drain().last().unwrap();
    assert!(
        scroll.target.is_some(),
        "expected an in-flight animation: {scroll:?}"
    );
    assert!(
        scroll.destination().y.is_snapped(),
        "the Y axis must be snapped mid-flight, towards its relative target: {scroll:?}"
    );

    // Settled: the offset itself is relative now
    // 已稳定：偏移量本身现在是相对的
    step_frames(&mut simulator, &mut instant, 60);

    let scroll = simulator.drain().last().unwrap();
    assert!(
        scroll.target.is_none(),
        "expected a settled animation: {scroll:?}"
    );
    assert!(
        scroll.viewport.y.is_snapped(),
        "the Y offset must be relative once settled: {scroll:?}"
    );
}

#[test]
fn destination_is_unsnapped_for_absolute_targets() {
    let mut simulator = simulate(element(3000, 200));
    simulator.point_at(Point::new(500.0, 100.0));

    // A wheel targets an absolute offset
    // 滚轮的目标是一个绝对偏移量
    let _ = simulator.scroll(ScrollDelta::Lines { x: 0.0, y: -1.0 });

    let mut instant = Instant::now();
    step_frames(&mut simulator, &mut instant, 1);

    let scroll = simulator.drain().last().unwrap();
    assert!(
        scroll.target.is_some(),
        "expected an in-flight animation: {scroll:?}"
    );
    assert!(
        !scroll.destination().y.is_snapped(),
        "a wheel targets an absolute offset: {scroll:?}"
    );

    step_frames(&mut simulator, &mut instant, 60);

    let scroll = simulator.drain().last().unwrap();
    assert!(
        !scroll.viewport.y.is_snapped(),
        "the settled offset must be absolute: {scroll:?}"
    );
}

// ─── Rail (track) interaction ────────────────────────────────────────────────
//
// The vertical scrollbar's rail occupies x ∈ [1014, 1024) for an `element`
// with a 1024-wide viewport. At the top of the content the scroller spans
// y ∈ [0, 13.33), so a click at y = 60 lands on the rail, below it.
//
// With a 200-tall viewport and 3000-tall content the rail page step is
// `0.875 * 200 = 175` px, the autoscroll velocity is `175 * 20 = 3500` px/s,
// and the maximum offset is `3000 − 200 = 2800` px. A rail click at y = 60
// scrolls until the scroller's bottom edge reaches the pointer, i.e. to the
// offset `700`; at y = 150, to the offset `2050`.
// ─── 轨道（track）交互 ───
//
// 对于视口宽度为 1024 的 `element`，垂直滚动条的轨道占据
// x ∈ [1014, 1024)。在内容顶部时，滚动块占据
// y ∈ [0, 13.33)，因此在 y = 60 处的点击落在轨道上，位于滚动块下方。
//
// 视口高 200、内容高 3000 时，轨道的翻页步长为
// `0.875 * 200 = 175` px，自动滚动速度为 `175 * 20 = 3500` px/s，
// 最大偏移量为 `3000 − 200 = 2800` px。在 y = 60 处点击轨道，
// 会一直滚动到滚动块的下边缘到达指针位置，即滚动到
// 偏移量 `700`；在 y = 150 处，则滚动到偏移量 `2050`。

/// 快速点击轨道（按下并松开）恰好滚动一页，并带有
/// 与滚轮滚动相同的渐进缓动。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A quick rail click (press and release) scrolls exactly one page, with the
/// same gradual easing as a wheel scroll.
///
/// </details>
#[test]
fn rail_click_scrolls_a_page_smoothly() {
    let mut simulator = simulate(element(3000, 200));

    // Render a frame so that the scroller is at its position for the
    // current offset before the press
    // 渲染一帧，让滚动块在按下之前位于
    // 当前偏移量对应的位置
    let mut instant = Instant::now();
    step_frames(&mut simulator, &mut instant, 1);

    simulator.point_at(Point::new(1019.0, 60.0));

    let _ = simulator.simulate([Event::Mouse(mouse::Event::ButtonPressed(
        mouse::Button::Left,
    ))]);
    let _ = simulator.simulate([Event::Mouse(mouse::Event::ButtonReleased(
        mouse::Button::Left,
    ))]);

    step_frames(&mut simulator, &mut instant, 60);

    let (offsets, animating): (Vec<f32>, Vec<bool>) = simulator
        .into_messages()
        .map(|scroll| (scroll.viewport.absolute_offset().y, scroll.target.is_some()))
        .unzip();

    assert!(
        !offsets.is_empty(),
        "expected on_scroll notifications, got: {offsets:?}"
    );
    assert_eq!(
        offsets.first(),
        Some(&0.0),
        "the offset must not move before the first frame: {offsets:?}"
    );
    assert!(
        offsets.len() > 2,
        "the page step must be gradual, not an instant jump: {offsets:?}"
    );
    assert_eq!(
        offsets.last(),
        Some(&175.0),
        "a quick rail click must settle exactly one page (175 px): {offsets:?}"
    );
    // The first notification is the pre-press content (not animating); the
    // scroll notifications animate until they settle
    // 第一条通知是按下之前的内容（没有动画）；
    // 滚动通知在稳定之前都处于动画中
    assert!(
        !animating.first().unwrap(),
        "the pre-press notification must not be animating: {animating:?}"
    );
    assert!(
        animating[1..animating.len() - 1]
            .iter()
            .all(|&is_animating| is_animating),
        "expected the scroll to be animating until it settles: {offsets:?} {animating:?}"
    );
    assert!(
        !animating.last().unwrap(),
        "expected the scroll to have settled: {offsets:?} {animating:?}"
    );
}

/// 禁用平滑滚动时，快速点击轨道会立即滚动一页，
/// 没有进行中的动画。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A quick rail click scrolls one page instantly when smooth scrolling is
/// disabled, with no in-flight animation.
///
/// </details>
#[test]
fn rail_click_is_immediate_when_smooth_scroll_disabled() {
    let mut simulator = simulate(element(3000, 200).smooth_scroll(false));

    // Render a frame so that the scroller is at its position for the
    // current offset before the press
    // 渲染一帧，让滚动块在按下之前位于
    // 当前偏移量对应的位置
    let mut instant = Instant::now();
    step_frames(&mut simulator, &mut instant, 1);

    simulator.point_at(Point::new(1019.0, 60.0));

    let _ = simulator.simulate([Event::Mouse(mouse::Event::ButtonPressed(
        mouse::Button::Left,
    ))]);
    let _ = simulator.simulate([Event::Mouse(mouse::Event::ButtonReleased(
        mouse::Button::Left,
    ))]);

    let notifications: Vec<(f32, bool)> = simulator
        .into_messages()
        .map(|scroll| (scroll.viewport.absolute_offset().y, scroll.target.is_some()))
        .collect();

    let (offset, animating) = *notifications.last().unwrap();
    assert_eq!(
        offset, 175.0,
        "the page step must apply instantly: {notifications:?}"
    );
    assert!(
        !animating,
        "there must be no in-flight animation: {notifications:?}"
    );
}

/// 按住轨道会在按压延迟之后让滚动块朝指针自动滚动，
/// 并在滚动块的前缘到达指针时停止。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Holding a rail press autoscrolls the scroller toward the pointer after the
/// press delay, stopping when the scroller's leading edge reaches it.
///
/// </details>
#[test]
fn rail_hold_autoscrolls_to_pointer() {
    let mut simulator = simulate(element(3000, 200));

    // Render a frame so that the scroller is at its position for the
    // current offset before the press
    // 渲染一帧，让滚动块在按下之前位于
    // 当前偏移量对应的位置
    let mut instant = Instant::now();
    step_frames(&mut simulator, &mut instant, 1);

    simulator.point_at(Point::new(1019.0, 60.0));
    let _ = simulator.simulate([Event::Mouse(mouse::Event::ButtonPressed(
        mouse::Button::Left,
    ))]);

    // Hold the button down: past the 250 ms press delay the autoscroll
    // slides the scroller toward the pointer and stops with its bottom edge
    // at y = 60 (offset 700)
    // 按住按键不放：超过 250 ms 的按压延迟之后，自动滚动
    // 让滚动块朝指针滑动，并在它的下边缘位于
    // y = 60（偏移量 700）时停止
    step_frames(&mut simulator, &mut instant, 120);

    let _ = simulator.simulate([Event::Mouse(mouse::Event::ButtonReleased(
        mouse::Button::Left,
    ))]);
    step_frames(&mut simulator, &mut instant, 2);

    let offsets: Vec<f32> = simulator
        .into_messages()
        .map(|scroll| scroll.viewport.absolute_offset().y)
        .collect();

    let last = *offsets.last().unwrap();
    assert!(
        (last - 700.0).abs() < 1.0,
        "the autoscroll must stop with the scroller's edge at the pointer (offset 700): {offsets:?}"
    );
    assert!(
        offsets.iter().all(|&offset| offset <= 700.0 + 1.0),
        "the autoscroll must not overshoot the pointer: {offsets:?}"
    );
}

/// 按住轨道期间，移动指针会让自动滚动的目标
/// 随之移动，因此滚动块会继续朝新位置滑动。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// While a rail is held, moving the pointer moves the autoscroll's target
/// with it, so the scroller keeps sliding toward the new position.
///
/// </details>
#[test]
fn rail_hold_follows_pointer_movement() {
    let mut simulator = simulate(element(3000, 200));

    // Render a frame so that the scroller is at its position for the
    // current offset before the press
    // 渲染一帧，让滚动块在按下之前位于
    // 当前偏移量对应的位置
    let mut instant = Instant::now();
    step_frames(&mut simulator, &mut instant, 1);

    simulator.point_at(Point::new(1019.0, 60.0));
    let _ = simulator.simulate([Event::Mouse(mouse::Event::ButtonPressed(
        mouse::Button::Left,
    ))]);

    // Hold until the autoscroll settles at the first stop (scroller at y = 60,
    // offset 700)
    // 一直按住，直到自动滚动稳定在第一个停止点（滚动块位于 y = 60，
    // 偏移量 700）
    step_frames(&mut simulator, &mut instant, 60);

    // Drag the pointer down while keeping the button pressed: the target
    // moves to y = 150 (offset 2050) and the autoscroll resumes
    // 保持按键按下的同时向下拖动指针：目标
    // 移动到 y = 150（偏移量 2050），自动滚动恢复
    simulator.point_at(Point::new(1019.0, 150.0));
    let _ = simulator.simulate([Event::Mouse(mouse::Event::CursorMoved {
        position: Point::new(1019.0, 150.0),
    })]);
    step_frames(&mut simulator, &mut instant, 60);

    let _ = simulator.simulate([Event::Mouse(mouse::Event::ButtonReleased(
        mouse::Button::Left,
    ))]);
    step_frames(&mut simulator, &mut instant, 2);

    let offsets: Vec<f32> = simulator
        .into_messages()
        .map(|scroll| scroll.viewport.absolute_offset().y)
        .collect();

    let last = *offsets.last().unwrap();
    assert!(
        (last - 2050.0).abs() < 1.0,
        "the autoscroll must follow the pointer to its new stop (offset 2050): {offsets:?}"
    );
    assert!(
        offsets.iter().all(|&offset| offset <= 2050.0 + 1.0),
        "the autoscroll must not overshoot the pointer: {offsets:?}"
    );
}

/// 镜像的滚动条（`Anchor::End`）同样会让滚动块朝
/// 指针滚动：滚动块位于轨道底部时，按下
/// 它上方的轨道会让它向上滑动，并在其上边缘到达指针时停止。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A mirrored scrollbar (`Anchor::End`) scrolls the scroller toward the
/// pointer as well: with the scroller at the bottom of the track, pressing
/// the rail above it slides it up, stopping with its top edge at the pointer.
///
/// </details>
#[test]
fn rail_hold_autoscrolls_to_pointer_with_end_anchor() {
    let mut simulator = simulate(element(3000, 200).anchor_y(Anchor::End));

    // Render a frame so that the scroller is at its position for the
    // current offset before the press: with `Anchor::End`, at the top of the
    // content the scroller sits at the bottom of the track
    // (y ∈ [186.67, 200)), so y = 60 is on the rail above it
    // 渲染一帧，让滚动块在按下之前位于
    // 当前偏移量对应的位置：使用 `Anchor::End` 时，在
    // 内容顶部，滚动块位于轨道的底部
    // （y ∈ [186.67, 200)），因此 y = 60 位于它上方的轨道上
    let mut instant = Instant::now();
    step_frames(&mut simulator, &mut instant, 1);

    simulator.point_at(Point::new(1019.0, 60.0));
    let _ = simulator.simulate([Event::Mouse(mouse::Event::ButtonPressed(
        mouse::Button::Left,
    ))]);

    // Holding slides the scroller up toward the pointer; it stops with its
    // top edge at y = 60, i.e. at offset 1900
    // 按住会让滚动块朝指针向上滑动；它在
    // 上边缘位于 y = 60 时停止，即偏移量 1900
    step_frames(&mut simulator, &mut instant, 120);

    let _ = simulator.simulate([Event::Mouse(mouse::Event::ButtonReleased(
        mouse::Button::Left,
    ))]);
    step_frames(&mut simulator, &mut instant, 2);

    let offsets: Vec<f32> = simulator
        .into_messages()
        .map(|scroll| scroll.viewport.absolute_offset().y)
        .collect();

    let last = *offsets.last().unwrap();
    assert!(
        (last - 1900.0).abs() < 1.0,
        "the autoscroll must slide the mirrored scroller toward the pointer (offset 1900): {offsets:?}"
    );
    assert!(
        offsets.iter().all(|&offset| offset <= 1900.0 + 1.0),
        "the autoscroll must not overshoot the pointer: {offsets:?}"
    );
}

/// 按住轨道的自动滚动在水平轴上同样有效，让
/// 滚动块朝指针滑动，直到它的边缘到达指针。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The held-rail autoscroll works along the horizontal axis too, sliding the
/// scroller toward the pointer until its edge reaches it.
///
/// </details>
#[test]
fn rail_hold_autoscrolls_to_pointer_on_horizontal_rail() {
    let mut simulator = simulate(element_xy(3000, 200));

    // Render a frame so that the scroller is at its position for the
    // current offset before the press
    // 渲染一帧，让滚动块在按下之前位于
    // 当前偏移量对应的位置
    let mut instant = Instant::now();
    step_frames(&mut simulator, &mut instant, 1);

    // The horizontal rail spans y ∈ [190, 200); at the start of the content
    // the scroller spans x ∈ [0, 349.5), so x = 768 is on the rail, to the
    // right of it
    // 水平轨道占据 y ∈ [190, 200)；在内容的起始位置，
    // 滚动块占据 x ∈ [0, 349.5)，因此 x = 768 位于轨道上，在
    // 滚动块的右侧
    simulator.point_at(Point::new(768.0, 195.0));
    let _ = simulator.simulate([Event::Mouse(mouse::Event::ButtonPressed(
        mouse::Button::Left,
    ))]);

    // Holding slides the scroller right toward the pointer; it stops with
    // its right edge at x = 768, i.e. at offset 1226
    // 按住会让滚动块朝指针向右滑动；它在
    // 右边缘位于 x = 768 时停止，即偏移量 1226
    step_frames(&mut simulator, &mut instant, 120);

    let _ = simulator.simulate([Event::Mouse(mouse::Event::ButtonReleased(
        mouse::Button::Left,
    ))]);
    step_frames(&mut simulator, &mut instant, 2);

    let offsets: Vec<f32> = simulator
        .into_messages()
        .map(|scroll| scroll.viewport.absolute_offset().x)
        .collect();

    let last = *offsets.last().unwrap();
    assert!(
        (last - 1226.0).abs() < 1.0,
        "the autoscroll must stop with the scroller's right edge at the pointer (offset 1226): {offsets:?}"
    );
    assert!(
        offsets.iter().all(|&offset| offset <= 1226.0 + 1.0),
        "the autoscroll must not overshoot the pointer: {offsets:?}"
    );
}

/// 按住 `Shift` 点击轨道是一次“跳转点击”：滚动块会跳转
/// （没有动画），使其中心位于指针下方，然后
/// 表现得像一个被抓住的滚动块。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// `Shift`-clicking the rail is a "jump click": the scroller jumps
/// (without animation) so that its center is under the pointer, and then
/// behaves like a grabbed scroller.
///
/// </details>
#[test]
fn shift_rail_click_jumps_to_pointer() {
    let mut simulator = simulate(element(3000, 200));

    // Render a frame so that the scroller is at its position for the
    // current offset before the press
    // 渲染一帧，让滚动块在按下之前位于
    // 当前偏移量对应的位置
    let mut instant = Instant::now();
    step_frames(&mut simulator, &mut instant, 1);

    simulator.point_at(Point::new(1019.0, 60.0));

    let _ = simulator.simulate([Event::Keyboard(keyboard::Event::ModifiersChanged(
        Modifiers::SHIFT,
    ))]);
    let _ = simulator.simulate([Event::Mouse(mouse::Event::ButtonPressed(
        mouse::Button::Left,
    ))]);

    // The jump is immediate: no further frames are needed, the scroller is
    // centered on the pointer (offset 800), with no in-flight animation
    // 跳转是立即的：不需要更多的帧，滚动块
    // 以指针为中心（偏移量 800），没有进行中的动画
    let (offsets, animating): (Vec<f32>, Vec<bool>) = simulator
        .into_messages()
        .map(|scroll| (scroll.viewport.absolute_offset().y, scroll.target.is_some()))
        .unzip();

    let last = *offsets.last().unwrap();
    assert!(
        (last - 800.0).abs() < 1.0,
        "a `Shift` rail click must center the scroller on the pointer (offset 800): {offsets:?}"
    );
    assert!(
        !animating.last().unwrap(),
        "the jump must not animate: {offsets:?} {animating:?}"
    );
}

/// 启用 `click_to_scroll` 时，普通的轨道点击是一次“跳转点击”：
/// 滚动块会跳转（没有动画），使其中心位于
/// 指针下方，然后表现得像一个被抓住的滚动块。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// With `click_to_scroll` enabled, a plain rail click is a "jump click":
/// the scroller jumps (without animation) so that its center is under the
/// pointer, and then behaves like a grabbed scroller.
///
/// </details>
#[test]
fn click_to_scroll_rail_click_jumps_to_pointer() {
    let mut simulator = simulate(element(3000, 200).click_to_scroll(true));

    // Render a frame so that the scroller is at its position for the
    // current offset before the press
    // 渲染一帧，让滚动块在按下之前位于
    // 当前偏移量对应的位置
    let mut instant = Instant::now();
    step_frames(&mut simulator, &mut instant, 1);

    simulator.point_at(Point::new(1019.0, 60.0));

    let _ = simulator.simulate([Event::Mouse(mouse::Event::ButtonPressed(
        mouse::Button::Left,
    ))]);

    // The jump is immediate: no further frames are needed, the scroller is
    // centered on the pointer (offset 800), with no in-flight animation
    // 跳转是立即的：不需要更多的帧，滚动块
    // 以指针为中心（偏移量 800），没有进行中的动画
    let (offsets, animating): (Vec<f32>, Vec<bool>) = simulator
        .into_messages()
        .map(|scroll| (scroll.viewport.absolute_offset().y, scroll.target.is_some()))
        .unzip();

    let last = *offsets.last().unwrap();
    assert!(
        (last - 800.0).abs() < 1.0,
        "a rail click must center the scroller on the pointer (offset 800): {offsets:?}"
    );
    assert!(
        !animating.last().unwrap(),
        "the jump must not animate: {offsets:?} {animating:?}"
    );
}

/// 启用 `click_to_scroll` 时，按住 `Shift` 点击轨道是一次翻页
/// 按压：快速按下并松开恰好滚动一页，并带有
/// 与滚轮滚动相同的渐进缓动。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// With `click_to_scroll` enabled, `Shift`-clicking the rail is a page
/// press: a quick press and release scrolls exactly one page, with the
/// same gradual easing as a wheel scroll.
///
/// </details>
#[test]
fn click_to_scroll_shift_rail_click_pages() {
    let mut simulator = simulate(element(3000, 200).click_to_scroll(true));

    // Render a frame so that the scroller is at its position for the
    // current offset before the press
    // 渲染一帧，让滚动块在按下之前位于
    // 当前偏移量对应的位置
    let mut instant = Instant::now();
    step_frames(&mut simulator, &mut instant, 1);

    simulator.point_at(Point::new(1019.0, 60.0));

    let _ = simulator.simulate([Event::Keyboard(keyboard::Event::ModifiersChanged(
        Modifiers::SHIFT,
    ))]);
    let _ = simulator.simulate([Event::Mouse(mouse::Event::ButtonPressed(
        mouse::Button::Left,
    ))]);
    let _ = simulator.simulate([Event::Mouse(mouse::Event::ButtonReleased(
        mouse::Button::Left,
    ))]);

    step_frames(&mut simulator, &mut instant, 60);

    let (offsets, animating): (Vec<f32>, Vec<bool>) = simulator
        .into_messages()
        .map(|scroll| (scroll.viewport.absolute_offset().y, scroll.target.is_some()))
        .unzip();

    assert!(
        !offsets.is_empty(),
        "expected on_scroll notifications, got: {offsets:?}"
    );
    assert_eq!(
        offsets.first(),
        Some(&0.0),
        "the offset must not move before the first frame: {offsets:?}"
    );
    assert!(
        offsets.len() > 2,
        "the page step must be gradual, not an instant jump: {offsets:?}"
    );
    assert_eq!(
        offsets.last(),
        Some(&175.0),
        "a quick `Shift` rail click must settle exactly one page (175 px): {offsets:?}"
    );
    assert!(
        !animating.first().unwrap(),
        "the pre-press notification must not be animating: {animating:?}"
    );
    assert!(
        animating[1..animating.len() - 1]
            .iter()
            .all(|&is_animating| is_animating),
        "expected the scroll to be animating until it settles: {offsets:?} {animating:?}"
    );
    assert!(
        !animating.last().unwrap(),
        "expected the scroll to have settled: {offsets:?} {animating:?}"
    );
}

#[test]
fn on_scroll_action_can_scroll() {
    let element = Scrollable::new(space().height(3000))
        .id("scrollable")
        .width(Length::Fill)
        .height(200)
        .smooth_scroll(false)
        .on_scroll(move |scroll| {
            if scroll.origin.is_none() {
                Action::ScrollTo(
                    AbsoluteOffset {
                        x: None,
                        y: Some(1000.0),
                    },
                    operation::Animation::Instant,
                )
            } else {
                Action::Custom(scroll)
            }
        });

    let mut simulator = simulate(element);
    simulator.point_at(Point::new(500.0, 100.0));

    // Scroll down two lines (240 px); the handler consumes this notification
    // and instead scrolls the widget to 1000 px
    // 向下滚动两行（240 px）；处理函数消费了这条通知，
    // 转而把部件滚动到 1000 px
    let _ = simulator.scroll(ScrollDelta::Lines { x: 0.0, y: -2.0 });

    // Let the `Action::ScrollTo` take effect on the following frames
    // 让 `Action::ScrollTo` 在接下来的几帧中生效
    let mut instant = Instant::now();
    step_frames(&mut simulator, &mut instant, 5);

    let (offsets, sources): (Vec<f32>, Vec<Source>) = simulator
        .into_messages()
        .map(|scroll| (scroll.viewport.absolute_offset().y, scroll.source))
        .collect();

    // The wheel's own notification (240 px) was not published, and the only
    // message is the move to 1000 px requested by the handler
    // 滚轮自身的通知（240 px）没有被发布，唯一的
    // 消息是处理函数请求的移动到 1000 px
    assert_eq!(offsets, vec![1000.0]);
    assert!(sources.into_iter().all(|source| source == Source::Action));
}
