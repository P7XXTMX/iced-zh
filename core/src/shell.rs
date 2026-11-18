use crate::InputMethod;
use crate::event;
use crate::window;

/// 与外壳状态的连接。
///
/// [`Widget`] 可以利用 [`Shell`] 来触发应用程序中的变化，
/// 比如发布消息，或者使当前布局失效。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A connection to the state of a shell.
///
/// A [`Widget`] can leverage a [`Shell`] to trigger changes in an application,
/// like publishing messages or invalidating the current layout.
///
/// </details>
///
/// [`Widget`]: crate::Widget
#[derive(Debug)]
pub struct Shell<'a, Message> {
    messages: &'a mut Vec<Message>,
    event_status: event::Status,
    redraw_request: window::RedrawRequest,
    input_method: InputMethod,
    is_layout_invalid: bool,
    are_widgets_invalid: bool,
}

impl<'a, Message> Shell<'a, Message> {
    /// 使用给定的消息缓冲区创建一个新的 [`Shell`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Shell`] with the provided buffer of messages.
    ///
    /// </details>
    pub fn new(messages: &'a mut Vec<Message>) -> Self {
        Self {
            messages,
            event_status: event::Status::Ignored,
            redraw_request: window::RedrawRequest::Wait,
            is_layout_invalid: false,
            are_widgets_invalid: false,
            input_method: InputMethod::Disabled,
        }
    }

    /// 如果 [`Shell`] 不包含任何已发布的消息，返回 true
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns true if the [`Shell`] contains no published messages
    ///
    /// </details>
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    /// 发布给定的 `Message`，交给应用程序处理。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Publish the given `Message` for an application to process it.
    ///
    /// </details>
    pub fn publish(&mut self, message: Message) {
        self.messages.push(message);
    }

    /// 把当前事件标记为已捕获。阻止“事件冒泡”。
    ///
    /// 当任何祖先都不应该处理某个事件时，
    /// 部件应该捕获它。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Marks the current event as captured. Prevents "event bubbling".
    ///
    /// A widget should capture an event when no ancestor should
    /// handle it.
    ///
    /// </details>
    pub fn capture_event(&mut self) {
        self.event_status = event::Status::Captured;
    }

    /// 返回 [`Shell`] 当前的 [`event::Status`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current [`event::Status`] of the [`Shell`].
    ///
    /// </details>
    #[must_use]
    pub fn event_status(&self) -> event::Status {
        self.event_status
    }

    /// 返回当前事件是否已被捕获。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns whether the current event has been captured.
    ///
    /// </details>
    #[must_use]
    pub fn is_event_captured(&self) -> bool {
        self.event_status == event::Status::Captured
    }

    /// 请求尽快绘制新的一帧。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Requests a new frame to be drawn as soon as possible.
    ///
    /// </details>
    pub fn request_redraw(&mut self) {
        self.redraw_request = window::RedrawRequest::NextFrame;
    }

    /// 请求按照给定的 [`window::RedrawRequest`] 绘制新的一帧。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Requests a new frame to be drawn at the given [`window::RedrawRequest`].
    ///
    /// </details>
    pub fn request_redraw_at(
        &mut self,
        redraw_request: impl Into<window::RedrawRequest>,
    ) {
        self.redraw_request = self.redraw_request.min(redraw_request.into());
    }

    /// 返回重绘应该何时发生的请求（如果有）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the request a redraw should happen, if any.
    ///
    /// </details>
    #[must_use]
    pub fn redraw_request(&self) -> window::RedrawRequest {
        self.redraw_request
    }

    /// 替换 [`Shell`] 的重绘请求；不做冲突处理。
    ///
    /// 如果你想把重绘请求覆盖为之前的某个值，这会很有用。
    /// 由于这是相当高级的用例，应该很少用到，所以它是一个静态
    /// 方法。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Replaces the redraw request of the [`Shell`]; without conflict resolution.
    ///
    /// This is useful if you want to overwrite the redraw request to a previous value.
    /// Since it's a fairly advanced use case and should rarely be used, it is a static
    /// method.
    ///
    /// </details>
    pub fn replace_redraw_request(
        shell: &mut Self,
        redraw_request: window::RedrawRequest,
    ) {
        shell.redraw_request = redraw_request;
    }

    /// 请求当前的 [`InputMethod`] 策略。
    ///
    /// **重要**：[`Shell`] 只会在
    /// [`window::Event::RedrawRequested`] 期间响应这个请求。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Requests the current [`InputMethod`] strategy.
    ///
    /// __Important__: This request will only be honored by the
    /// [`Shell`] only during a [`window::Event::RedrawRequested`].
    ///
    /// </details>
    pub fn request_input_method<T: AsRef<str>>(
        &mut self,
        ime: &InputMethod<T>,
    ) {
        self.input_method.merge(ime);
    }

    /// 返回当前的 [`InputMethod`] 策略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current [`InputMethod`] strategy.
    ///
    /// </details>
    #[must_use]
    pub fn input_method(&self) -> &InputMethod {
        &self.input_method
    }

    /// 返回当前的 [`InputMethod`] 策略。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current [`InputMethod`] strategy.
    ///
    /// </details>
    #[must_use]
    pub fn input_method_mut(&mut self) -> &mut InputMethod {
        &mut self.input_method
    }

    /// 返回当前布局是否已失效。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns whether the current layout is invalid or not.
    ///
    /// </details>
    #[must_use]
    pub fn is_layout_invalid(&self) -> bool {
        self.is_layout_invalid
    }

    /// 使当前的应用程序布局失效。
    ///
    /// 外壳会重新布局应用程序的部件。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Invalidates the current application layout.
    ///
    /// The shell will relayout the application widgets.
    ///
    /// </details>
    pub fn invalidate_layout(&mut self) {
        self.is_layout_invalid = true;
    }

    /// 如果布局已失效，就触发给定的函数，并在此过程中
    /// 清除失效标记。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Triggers the given function if the layout is invalid, cleaning it in the
    /// process.
    ///
    /// </details>
    pub fn revalidate_layout(&mut self, f: impl FnOnce()) {
        if self.is_layout_invalid {
            self.is_layout_invalid = false;

            f();
        }
    }

    /// 返回当前应用程序的部件是否已经
    /// 失效。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns whether the widgets of the current application have been
    /// invalidated.
    ///
    /// </details>
    #[must_use]
    pub fn are_widgets_invalid(&self) -> bool {
        self.are_widgets_invalid
    }

    /// 使当前的应用程序部件失效。
    ///
    /// 外壳会重建部件树并重新布局。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Invalidates the current application widgets.
    ///
    /// The shell will rebuild and relayout the widget tree.
    ///
    /// </details>
    pub fn invalidate_widgets(&mut self) {
        self.are_widgets_invalid = true;
    }

    /// 把给定的函数应用到另一个 [`Shell`] 的消息上，
    /// 从而把当前的 [`Shell`] 与它合并。
    ///
    /// 此方法对组合很有用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Merges the current [`Shell`] with another one by applying the given
    /// function to the messages of the latter.
    ///
    /// This method is useful for composition.
    ///
    /// </details>
    pub fn merge<B>(&mut self, other: Shell<'_, B>, f: impl Fn(B) -> Message) {
        self.messages.extend(other.messages.drain(..).map(f));

        self.is_layout_invalid =
            self.is_layout_invalid || other.is_layout_invalid;

        self.are_widgets_invalid =
            self.are_widgets_invalid || other.are_widgets_invalid;

        self.redraw_request = self.redraw_request.min(other.redraw_request);
        self.event_status = self.event_status.merge(other.event_status);
        self.input_method.merge(&other.input_method);
    }
}
