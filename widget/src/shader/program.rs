use crate::core::Rectangle;
use crate::core::mouse;
use crate::renderer::wgpu::Primitive;
use crate::shader::{self, Action};

/// [`Shader`] 部件的状态和逻辑。
///
/// [`Program`] 可以修改 [`Shader`] 部件的内部状态，
/// 并为应用程序产生消息。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The state and logic of a [`Shader`] widget.
///
/// A [`Program`] can mutate the internal state of a [`Shader`] widget
/// and produce messages for an application.
///
/// </details>
///
/// [`Shader`]: crate::Shader
pub trait Program<Message> {
    /// [`Program`] 的内部状态。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The internal state of the [`Program`].
    ///
    /// </details>
    type State: Default + 'static;

    /// 这个 [`Program`] 可以绘制的图元类型。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The type of primitive this [`Program`] can draw.
    ///
    /// </details>
    type Primitive: Primitive + 'static;

    /// 更新 [`Program`] 的内部 [`State`]。可以用来根据鼠标及其他事件
    /// 反映状态变化。你可以返回一个 [`Action`] 来发布消息、请求
    /// 重绘或捕获事件。
    ///
    /// 默认情况下，这个方法返回 `None`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Update the internal [`State`] of the [`Program`]. This can be used to reflect state changes
    /// based on mouse & other events. You can return an [`Action`] to publish a message, request a
    /// redraw, or capture the event.
    ///
    /// By default, this method returns `None`.
    ///
    /// </details>
    ///
    /// [`State`]: Self::State
    fn update(
        &self,
        _state: &mut Self::State,
        _event: &shader::Event,
        _bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Option<Action<Message>> {
        None
    }

    /// 绘制 [`Primitive`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Draws the [`Primitive`].
    ///
    /// </details>
    ///
    /// [`Primitive`]: Self::Primitive
    fn draw(
        &self,
        state: &Self::State,
        cursor: mouse::Cursor,
        bounds: Rectangle,
    ) -> Self::Primitive;

    /// 返回 [`Program`] 当前的鼠标交互。
    ///
    /// 即使光标位置超出了 [`Shader`] 的程序的边界，
    /// 返回的交互仍然会生效。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current mouse interaction of the [`Program`].
    ///
    /// The interaction returned will be in effect even if the cursor position is out of
    /// bounds of the [`Shader`]'s program.
    ///
    /// </details>
    ///
    /// [`Shader`]: crate::Shader
    fn mouse_interaction(
        &self,
        _state: &Self::State,
        _bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        mouse::Interaction::default()
    }
}
