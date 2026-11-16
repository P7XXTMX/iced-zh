//! 带键的部件可以提供提示，以确保连续性。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Keyed widgets can provide hints to ensure continuity.
//!
//! </details>
//!
//! # 什么是连续性？（What is continuity?）
//! 连续性是指状态持续存在的感觉。
//!
//! 在图形用户界面中，用户期望部件具有
//! 一定程度的连续状态。例如，一个获得焦点的
//! 文本输入框，即使部件树发生了轻微的变化，
//! 也应该保持焦点。
//!
//! 在 `iced` 和 Elm 架构中，连续性比较棘手，因为
//! 每次调用 `view` 都会重建整棵部件树。从开发者的
//! 角度看这非常方便，因为你可以构建极其动态的
//! 界面，而不必担心状态的变化。
//!
//! 然而，代价是 `iced` 很难判断到底是什么
//! 发生了变化。如果你有一个列表，在顶部添加一个元素
//! 就可能导致列表中每个元素都失去连续性！
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Continuity is the feeling of persistence of state.
//!
//! In a graphical user interface, users expect widgets to have a
//! certain degree of continuous state. For instance, a text input
//! that is focused should stay focused even if the widget tree
//! changes slightly.
//!
//! Continuity is tricky in `iced` and the Elm Architecture because
//! the whole widget tree is rebuilt during every `view` call. This is
//! very convenient from a developer perspective because you can build
//! extremely dynamic interfaces without worrying about changing state.
//!
//! However, the tradeoff is that determining what changed becomes hard
//! for `iced`. If you have a list of things, adding an element at the
//! top may cause a loss of continuity on every element on the list!
//!
//! </details>
//!
//! # 如何保持连续性？（How can we keep continuity?）
//! 好消息是，用户界面的部件结构通常是
//! 静态的。可以依靠这种结构来确保一定程度的
//! 连续性。`iced` 已经这样做了。
//!
//! 然而，有时界面中的某一部分会
//! 相当动态。例如，一个可以在任意位置添加
//! 或删除条目的列表。
//!
//! 在协调阶段有多种方式可以缓解这个问题，
//! 但它们都需要在一定深度上比较树并进行
//! 回溯……计算开销相当大。
//!
//! 一种开销更低的做法是让用户提供一些关于
//! 各个部件身份的提示，这样就可以直接比较它们，
//! 而不必深入下去。
//!
//! 本模块中的部件都会要求提供某种“提示”。为了
//! 帮助它们保持连续性，你需要确保在多次 `view` 调用之间，
//! 用户界面中相同条目的提示保持不变。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! The good news is that user interfaces generally have a static widget
//! structure. This structure can be relied on to ensure some degree of
//! continuity. `iced` already does this.
//!
//! However, sometimes you have a certain part of your interface that is
//! quite dynamic. For instance, a list of things where items may be added
//! or removed at any place.
//!
//! There are different ways to mitigate this during the reconciliation
//! stage, but they involve comparing trees at certain depths and
//! backtracking... Quite computationally expensive.
//!
//! One approach that is cheaper consists in letting the user provide some hints
//! about the identities of the different widgets so that they can be compared
//! directly without going deeper.
//!
//! The widgets in this module will all ask for a "hint" of some sort. In order
//! to help them keep continuity, you need to make sure the hint stays the same
//! for the same items in your user interface between `view` calls.
//!
//! </details>
pub mod column;

pub use column::Column;

/// 使用给定的子元素创建一个带键的 [`Column`]。
///
/// 带键的列在垂直方向上分布内容，同时保持连续性。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates a keyed [`Column`] with the given children.
///
/// Keyed columns distribute content vertically while keeping continuity.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run
/// # mod iced { pub mod widget { pub use iced_widget::*; } pub use iced_widget::Renderer; pub use iced_widget::core::*; }
/// # use iced::widget::Widget;
/// # pub type State = ();
/// use iced::widget::keyed_column;
///
/// enum Message {
///     // ...
/// }
///
/// fn view(state: &State) -> impl Widget<Message> {
///     keyed_column![
///         (0, "Item 0"),
///         (1, "Item 1"),
///         (2, "Item 2"),
///     ]
/// }
/// ```
#[macro_export]
macro_rules! keyed_column {
    () => (
        $crate::keyed::Column::new()
    );
    ($(($key:expr, $x:expr)),+ $(,)?) => (
        $crate::keyed::Column::with_children(vec![$(($key, $crate::core::Widget::_boxed($x))),+])
    );
}
