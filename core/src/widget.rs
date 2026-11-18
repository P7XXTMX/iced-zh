//! 创建自定义部件并对它们进行操作。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Create custom widgets and operate on them.
//!
//! </details>
pub mod operation;
pub mod text;
pub mod tree;

mod id;

pub use id::Id;
pub use operation::Operation;
pub use text::Text;
pub use tree::Tree;

use crate::layout::{self, Layout};
use crate::mouse;
use crate::overlay;
use crate::renderer;
use crate::{Clipboard, Event, Length, Rectangle, Shell, Size, Vector};

/// 显示信息并允许交互的组件。
///
/// 如果你想构建自己的部件，就需要实现这个
/// 特质。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A component that displays information and allows interaction.
///
/// If you want to build your own widgets, you will need to implement this
/// trait.
///
/// </details>
///
/// # 示例（Examples）
/// 仓库中有一些[示例][examples]，展示了如何实现自定义
/// 部件：
///
/// - [`custom_widget`]：演示如何构建一个绘制圆形的
///   自定义部件。
/// - [`geometry`]：一个自定义部件，展示如何使用 [`iced_wgpu`] 中的
///   `Mesh2D` 图元绘制几何图形。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The repository has some [examples] showcasing how to implement a custom
/// widget:
///
/// - [`custom_widget`], a demonstration of how to build a custom widget that
///   draws a circle.
/// - [`geometry`], a custom widget showcasing how to draw geometry with the
///   `Mesh2D` primitive in [`iced_wgpu`].
///
/// </details>
///
/// [examples]: https://github.com/iced-rs/iced/tree/0.14/examples
/// [`custom_widget`]: https://github.com/iced-rs/iced/tree/0.14/examples/custom_widget
/// [`geometry`]: https://github.com/iced-rs/iced/tree/0.14/examples/geometry
/// [`iced_wgpu`]: https://github.com/iced-rs/iced/tree/0.14/wgpu
pub trait Widget<Message, Theme, Renderer>
where
    Renderer: crate::Renderer,
{
    /// 返回 [`Widget`] 以长度表示的 [`Size`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`Size`] of the [`Widget`] in lengths.
    ///
    /// </details>
    fn size(&self) -> Size<Length>;

    /// 返回用于布局 [`Widget`] 的 [`Size`] 提示。
    ///
    /// 某些部件容器可能会在构造期间用这个提示来调整
    /// 它们的尺寸策略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns a [`Size`] hint for laying out the [`Widget`].
    ///
    /// This hint may be used by some widget containers to adjust their sizing strategy
    /// during construction.
    ///
    /// </details>
    fn size_hint(&self) -> Size<Length> {
        self.size()
    }

    /// 返回 [`Widget`] 的 [`layout::Node`]。
    ///
    /// 运行时用这个 [`layout::Node`] 来计算用户界面的
    /// [`Layout`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`layout::Node`] of the [`Widget`].
    ///
    /// This [`layout::Node`] is used by the runtime to compute the [`Layout`] of the
    /// user interface.
    ///
    /// </details>
    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node;

    /// 使用关联的 `Renderer` 绘制 [`Widget`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Draws the [`Widget`] using the associated `Renderer`.
    ///
    /// </details>
    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Theme,
        style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    );

    /// 返回 [`Widget`] 的 [`Tag`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`Tag`] of the [`Widget`].
    ///
    /// </details>
    ///
    /// [`Tag`]: tree::Tag
    fn tag(&self) -> tree::Tag {
        tree::Tag::stateless()
    }

    /// 返回 [`Widget`] 的 [`State`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`State`] of the [`Widget`].
    ///
    /// </details>
    ///
    /// [`State`]: tree::State
    fn state(&self) -> tree::State {
        tree::State::None
    }

    /// 返回 [`Widget`] 各子级的状态 [`Tree`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the state [`Tree`] of the children of the [`Widget`].
    ///
    /// </details>
    fn children(&self) -> Vec<Tree> {
        Vec::new()
    }

    /// 把 [`Widget`] 与给定的 [`Tree`] 进行协调。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Reconciles the [`Widget`] with the provided [`Tree`].
    ///
    /// </details>
    fn diff(&self, tree: &mut Tree) {
        tree.children.clear();
    }

    /// 对 [`Widget`] 应用一个 [`Operation`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Applies an [`Operation`] to the [`Widget`].
    ///
    /// </details>
    fn operate(
        &mut self,
        _tree: &mut Tree,
        _layout: Layout<'_>,
        _renderer: &Renderer,
        _operation: &mut dyn Operation,
    ) {
    }

    /// 处理运行时的 [`Event`]。
    ///
    /// 默认情况下，它什么都不做。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Processes a runtime [`Event`].
    ///
    /// By default, it does nothing.
    ///
    /// </details>
    fn update(
        &mut self,
        _tree: &mut Tree,
        _event: &Event,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _renderer: &Renderer,
        _clipboard: &mut dyn Clipboard,
        _shell: &mut Shell<'_, Message>,
        _viewport: &Rectangle,
    ) {
    }

    /// 返回 [`Widget`] 当前的 [`mouse::Interaction`]。
    ///
    /// 默认情况下，它返回 [`mouse::Interaction::None`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current [`mouse::Interaction`] of the [`Widget`].
    ///
    /// By default, it returns [`mouse::Interaction::None`].
    ///
    /// </details>
    fn mouse_interaction(
        &self,
        _tree: &Tree,
        _layout: Layout<'_>,
        _cursor: mouse::Cursor,
        _viewport: &Rectangle,
        _renderer: &Renderer,
    ) -> mouse::Interaction {
        mouse::Interaction::None
    }

    /// 返回 [`Widget`] 的覆盖层（如果有）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the overlay of the [`Widget`], if there is any.
    ///
    /// </details>
    fn overlay<'a>(
        &'a mut self,
        _tree: &'a mut Tree,
        _layout: Layout<'a>,
        _renderer: &Renderer,
        _viewport: &Rectangle,
        _translation: Vector,
    ) -> Option<overlay::Element<'a, Message, Theme, Renderer>> {
        None
    }
}
