//! 针对 [`lazy`] 部件的测试。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Tests for the [`lazy`] widget.
//!
//! </details>
//!
//! [`lazy`]: iced_widget::lazy
use iced_test::Simulator;
use iced_widget::core::Length;
use iced_widget::{Widget, lazy, row, text};

/// 一个行，包含一个缓存内容填满主轴的 [`lazy`] 子元素，
/// 后面跟着一段静态文本。
///
/// `lazy` 子元素必须在每一帧都报告它的 `Fill` 大小提示，这样
/// 行才会把它作为流式条目布局，静态文本才能保持其固有
/// 宽度。如果提示过期（即新创建的 [`Lazy`] 值的
/// 默认 `Fit` 提示），缓存的内容会作为静态条目
/// 以整个可用宽度布局，静态文本则被压缩为零。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A row with a [`lazy`] child whose cached content fills the main axis,
/// followed by a static text.
///
/// The `lazy` child must report its `Fill` size hint on every frame, so the
/// row lays it out as a fluid item and the static text keeps its intrinsic
/// width. If the hint goes stale (i.e. the default `Fit` hint of a freshly
/// created [`Lazy`] value), the cached content is laid out as a static item
/// with the whole available width and the static text is collapsed to zero.
///
/// </details>
///
/// [`Lazy`]: iced_widget::Lazy
fn view(dependency: u8) -> impl Widget<()> {
    row![
        lazy(dependency, |_| text("fill").width(Length::Fill)),
        text("anchor"),
    ]
}

/// 断言静态文本保持非零的固有宽度，这只有在
/// `lazy` 内容的 `Fill` 提示被报告给行时才会发生。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Asserts that the static text keeps a non-zero intrinsic width, which only
/// happens if the `Fill` hint of the `lazy` content was reported to the row.
///
/// </details>
fn assert_hints_respected(
    simulator: &mut Simulator<'_, (), iced_widget::Theme, iced_widget::Renderer>,
) {
    let fill = simulator.find("fill").unwrap().bounds().width;
    let anchor = simulator.find("anchor").unwrap().bounds().width;

    assert!(
        anchor > 0.0,
        "the static text should keep its intrinsic width, got {anchor}"
    );

    // The row is the root of the UI: the two children should share the whole
    // window width (1024, the default simulator size).
    // 行是 UI 的根：两个子元素应该共享整个
    // 窗口宽度（1024，模拟器的默认大小）。
    assert!(
        (fill + anchor - 1024.0).abs() < 1.0,
        "the row should distribute the available width between the children: {fill} + {anchor}"
    );
}

#[test]
fn size_hint_is_respected_on_first_frame() {
    let mut simulator = Simulator::new(view(0));

    assert_hints_respected(&mut simulator);
}

#[test]
fn size_hint_is_respected_when_dependency_is_unchanged() {
    let simulator = Simulator::new(view(0));

    // Rebuild the UI with the same dependency: the cached element is reused,
    // but the `Lazy` widget value is recreated from scratch.
    // 使用相同的依赖重建 UI：缓存的元素被复用，
    // 但 `Lazy` 部件的值是从头重新创建的。
    let mut simulator = simulator.rebuild(view(0));

    assert_hints_respected(&mut simulator);
}

#[test]
fn size_hint_is_respected_when_dependency_changes() {
    let simulator = Simulator::new(view(0));

    // Rebuild the UI with a new dependency: the cached element is replaced.
    // 使用新的依赖重建 UI：缓存的元素被替换。
    let mut simulator = simulator.rebuild(view(1));

    assert_hints_respected(&mut simulator);
}
