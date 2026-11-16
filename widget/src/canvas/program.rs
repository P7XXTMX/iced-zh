use crate::Action;
use crate::canvas::mouse;
use crate::canvas::{Event, Geometry};
use crate::core::Rectangle;
use crate::graphics::geometry;

/// [`Canvas`] 的状态和逻辑。
///
/// [`Program`] 可以修改内部状态，并为应用程序
/// 产生消息。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The state and logic of a [`Canvas`].
///
/// A [`Program`] can mutate internal state and produce messages for an
/// application.
///
/// </details>
///
/// [`Canvas`]: crate::Canvas
pub trait Program<Message, Theme = crate::Theme, Renderer = crate::Renderer>
where
    Renderer: geometry::Renderer,
{
    /// 由 [`Program`] 修改的内部状态。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The internal state mutated by the [`Program`].
    ///
    /// </details>
    type State: Default + 'static;

    /// 更新 [`Program`] 的 [`State`](Self::State)。
    ///
    /// 当 [`Program`] 用在 [`Canvas`] 中时，运行时会为每个 [`Event`]
    /// 调用此方法。
    ///
    /// 此方法可以选择返回一个 [`Action`]，用来把有意义的交互
    /// 通知给应用程序、捕获该事件，或者
    /// 请求重绘。
    ///
    /// 默认情况下，此方法什么都不做，也不返回任何东西。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Updates the [`State`](Self::State) of the [`Program`].
    ///
    /// When a [`Program`] is used in a [`Canvas`], the runtime will call this
    /// method for each [`Event`].
    ///
    /// This method can optionally return an [`Action`] to either notify an
    /// application of any meaningful interactions, capture the event, or
    /// request a redraw.
    ///
    /// By default, this method does and returns nothing.
    ///
    /// </details>
    ///
    /// [`Canvas`]: crate::Canvas
    fn update(
        &self,
        _state: &mut Self::State,
        _event: &Event,
        _bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> Option<Action<Message>> {
        None
    }

    /// 绘制 [`Program`] 的状态，产生一组 [`Geometry`]。
    ///
    /// [`Geometry`] 可以很方便地用 [`Frame`] 生成，或者存储在
    /// [`Cache`] 中。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Draws the state of the [`Program`], producing a bunch of [`Geometry`].
    ///
    /// [`Geometry`] can be easily generated with a [`Frame`] or stored in a
    /// [`Cache`].
    ///
    /// </details>
    ///
    /// [`Geometry`]: crate::canvas::Geometry
    /// [`Frame`]: crate::canvas::Frame
    /// [`Cache`]: crate::canvas::Cache
    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<Geometry<Renderer>>;

    /// 返回 [`Program`] 当前的鼠标交互。
    ///
    /// 即使光标位置超出了该程序所在 [`Canvas`] 的边界，
    /// 返回的交互依然有效。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current mouse interaction of the [`Program`].
    ///
    /// The interaction returned will be in effect even if the cursor position
    /// is out of bounds of the program's [`Canvas`].
    ///
    /// </details>
    ///
    /// [`Canvas`]: crate::Canvas
    fn mouse_interaction(
        &self,
        _state: &Self::State,
        _bounds: Rectangle,
        _cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        mouse::Interaction::default()
    }
}

impl<Message, Theme, Renderer, T> Program<Message, Theme, Renderer> for &T
where
    Renderer: geometry::Renderer,
    T: Program<Message, Theme, Renderer>,
{
    type State = T::State;

    fn update(
        &self,
        state: &mut Self::State,
        event: &Event,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Option<Action<Message>> {
        T::update(self, state, event, bounds, cursor)
    }

    fn draw(
        &self,
        state: &Self::State,
        renderer: &Renderer,
        theme: &Theme,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> Vec<Geometry<Renderer>> {
        T::draw(self, state, renderer, theme, bounds, cursor)
    }

    fn mouse_interaction(
        &self,
        state: &Self::State,
        bounds: Rectangle,
        cursor: mouse::Cursor,
    ) -> mouse::Interaction {
        T::mouse_interaction(self, state, bounds, cursor)
    }
}
