//! 针对从容器中过滤掉“空”（即 `None`）子元素的测试。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Tests for filtering "void" (i.e. `None`) children out of containers.
//!
//! </details>
use iced_widget::button::Button;
use iced_widget::core::widget;
use iced_widget::core::{Never, Widget};
use iced_widget::{Theme, button, column, row, space};

fn build(element: &mut impl Widget<Never, Theme, ()>) -> widget::Tree {
    let mut tree = widget::Tree::new(&*element);
    element.diff(&mut tree);

    tree
}

/// `column!["Hello", Some(button)]` 保留两个子元素。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// `column!["Hello", Some(button)]` keeps both children.
///
/// </details>
#[test]
fn column_macro_accepts_mixed_plain_and_optional_widgets() {
    let maybe: Option<Button<'static, Never, &'static str, Theme>> = Some(button("Click"));

    let mut col = column!["Hello", maybe];
    let tree = build(&mut col);

    // Both `"Hello"` and the button survive.
    // `"Hello"` 和按钮都保留了下来。
    assert_eq!(tree.children.len(), 2);
}

/// `None` 子元素会从列中完全过滤掉：它不占用
/// 布局槽位，也不贡献间距。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A `None` child is filtered out of the column entirely: it takes no
/// layout slot and contributes no spacing.
///
/// </details>
#[test]
fn column_macro_filters_none_children() {
    let maybe: Option<Button<'static, Never, &'static str, Theme>> = None;

    let mut col = column!["Hello", maybe];
    let tree = build(&mut col);

    // Only `"Hello"` survives.
    // 只有 `"Hello"` 保留了下来。
    assert_eq!(tree.children.len(), 1);
}

/// `row!` 的行为相同。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Same behaviour for `row!`.
///
/// </details>
#[test]
fn row_macro_filters_none_children() {
    let maybe: Option<space::Space> = None;

    let mut row = row!["Hello", space(), maybe];
    let tree = build(&mut row);

    // Only `"Hello"` and the plain `space()` survive.
    // 只有 `"Hello"` 和普通的 `space()` 保留了下来。
    assert_eq!(tree.children.len(), 2);
}
