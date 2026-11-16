//! 在部件中与 iced 运行时通信。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Communicate with the iced runtime from widgets.
//!
//! </details>
use crate::clipboard;
use crate::event;
use crate::window;
use crate::{Clipboard, InputMethod, Window};

use std::rc::{self, Rc};
use std::sync::Arc;
use std::vec;

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
    window: &'a dyn Window,
    bus: &'a mut Bus<Message>,
    waker: Waker,
    event_status: event::Status,
    redraw_request: window::RedrawRequest,
    input_method: InputMethod,
    invalidation: Invalidation,
    clipboard: Clipboard,
}

/// [`Shell`] 在处理一个事件的过程中
/// 被请求的最严重的失效级别。
///
/// 各变体按严重程度排序，[`Shell::invalidate`]
/// 会累积其中的最大值：较高的变体包含较低的变体。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The most severe invalidation requested by a [`Shell`] while
/// processing an event.
///
/// Variants are ordered by severity and [`Shell::invalidate`]
/// accumulates the maximum: a higher variant subsumes the lower ones.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Invalidation {
    /// 不需要重建任何东西。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Nothing needs to be rebuilt.
    ///
    /// </details>
    None,
    /// 需要重新创建应用程序的覆盖层。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The application overlays need to be recreated.
    ///
    /// </details>
    Overlay,
    /// 需要按照给定的 [`Diff`] 策略，
    /// 重新计算应用程序的布局。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The application layout needs to be recomputed, following the
    /// given [`Diff`] strategy.
    ///
    /// </details>
    Layout(Diff),
    /// 需要重建整个应用程序的部件树，并
    /// 重新布局。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The entire application widget tree needs to be rebuilt and
    /// relaid out.
    ///
    /// </details>
    Widgets,
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
    pub fn new(window: &'a dyn Window, waker: Waker, bus: &'a mut Bus<Message>) -> Self {
        Self {
            window,
            bus,
            waker,
            event_status: event::Status::Ignored,
            redraw_request: window::RedrawRequest::Wait,
            invalidation: Invalidation::None,
            input_method: InputMethod::Disabled,
            clipboard: Clipboard {
                reads: Vec::new(),
                write: None,
            },
        }
    }

    /// 基于当前的 [`Shell`]，使用给定的局部消息列表创建一个新的 [`Shell`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Shell`] from the current one with the given list of local messages.
    ///
    /// </details>
    pub fn local<'b, A>(&self, bus: &'b mut Bus<A>) -> Shell<'b, A>
    where
        'a: 'b,
    {
        Shell::new(self.window, self.waker.clone(), bus)
    }

    /// 返回 [`Shell`] 的 [`Window`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`Window`] of the [`Shell`].
    ///
    /// </details>
    pub fn window(&self) -> &'a dyn Window {
        self.window
    }

    /// 返回 [`Shell`] 的 [`Waker`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`Waker`] of the [`Shell`].
    ///
    /// </details>
    pub fn waker(&self) -> &Waker {
        &self.waker
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
        self.bus.messages.is_empty()
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
        let _ = self.publish_and_track(message);
    }

    /// 发布给定的 `Message`，交给应用程序处理。
    ///
    /// 返回的 [`Tracking`] 可以用来判断该消息
    /// 是否已被处理。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Publish the given `Message` for an application to process it.
    ///
    /// The returned [`Tracking`] can be used to determine if the message
    /// was processed.
    ///
    /// </details>
    pub fn publish_and_track(&mut self, message: Message) -> Tracking {
        self.bus.push(message)
    }

    /// 转发给定的 `Message`，并在它被处理之后兑现
    /// 给定的 [`Receipt`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Forwards the given `Message` and fulfills the given [`Receipt`]
    /// once processed.
    ///
    /// </details>
    pub fn forward(&mut self, message: Message, receipt: Receipt) {
        self.bus.forward(message, receipt);
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
    pub fn request_redraw_at(&mut self, redraw_request: impl Into<window::RedrawRequest>) {
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
    pub fn replace_redraw_request(shell: &mut Self, redraw_request: window::RedrawRequest) {
        shell.redraw_request = redraw_request;
    }

    /// 请求运行时读取剪贴板内容，并期望得到给定的 [`clipboard::Kind`]。
    ///
    /// 内容读取完成后，运行时会产生一个 [`clipboard::Event::Read`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Requests the runtime to read the clipboard contents expecting the given [`clipboard::Kind`].
    ///
    /// The runtime will produce a [`clipboard::Event::Read`] when the contents have been read.
    ///
    /// </details>
    pub fn read_clipboard(&mut self, kind: clipboard::Kind) {
        self.clipboard.reads.push(kind);
    }

    /// 请求运行时把给定的 [`clipboard::Content`] 写入剪贴板。
    ///
    /// 内容写入完成后，运行时会产生一个 [`clipboard::Event::Written`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Requests the runtime to write the given [`clipboard::Content`] to the clipboard.
    ///
    /// The runtime will produce a [`clipboard::Event::Written`] when the contents have been written.
    ///
    /// </details>
    pub fn write_clipboard(&mut self, content: impl Into<clipboard::Content>) {
        self.clipboard.write = Some(content.into());
    }

    /// 以可变方式返回 [`Shell`] 的 [`Clipboard`] 请求。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`Clipboard`] requests of the [`Shell`], mutably.
    ///
    /// </details>
    pub fn clipboard_mut(&mut self) -> &mut Clipboard {
        &mut self.clipboard
    }

    /// 请求当前的 [`InputMethod`] 策略。
    ///
    /// __重要__：[`Shell`] 只会在
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
    pub fn request_input_method<T: AsRef<str>>(&mut self, ime: &InputMethod<T>) {
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

    /// 使当前的应用程序覆盖层失效。
    ///
    /// 外壳会重新创建应用程序的覆盖层。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Invalidates the current application overlay.
    ///
    /// The shell will recreate the application overlays.
    ///
    /// </details>
    pub fn invalidate_overlay(&mut self) {
        self.invalidate(Invalidation::Overlay);
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
        self.invalidate_layout_with(Diff::Skip);
    }

    /// 按照给定的 [`Diff`] 策略，使当前的应用程序布局失效。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Invalidates the current application layout with the following [`Diff`] strategy.
    ///
    /// </details>
    pub fn invalidate_layout_with(&mut self, diff: Diff) {
        self.invalidate(Invalidation::Layout(diff));
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
        self.invalidate(Invalidation::Widgets);
    }

    /// 设置 [`Shell`] 的 [`Invalidation`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Invalidation`] of the [`Shell`].
    ///
    /// </details>
    pub fn invalidate(&mut self, invalidation: Invalidation) {
        self.invalidation = self.invalidation.max(invalidation);
    }

    /// 返回 [`Shell`] 的 [`Invalidation`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the [`Invalidation`] of the [`Shell`].
    ///
    /// </details>
    pub fn invalidation(&self) -> Invalidation {
        self.invalidation
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
    pub fn merge<B>(&mut self, mut other: Shell<'_, B>, f: impl Fn(B) -> Message) {
        self.bus.messages.extend(
            other
                .bus
                .messages
                .drain(..)
                .map(|(message, receipt)| (f(message), receipt)),
        );

        self.invalidation = self.invalidation.max(other.invalidation);
        self.redraw_request = self.redraw_request.min(other.redraw_request);
        self.event_status = self.event_status.merge(other.event_status);

        self.input_method.merge(&other.input_method);
        self.clipboard.merge(&mut other.clipboard);
    }
}

/// waker 可以用来唤醒 iced 运行时，从而
/// 在部件逻辑中并发地触发唤醒事件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A waker can be used to wake up the iced runtime and, consequently, trigger
/// wake events concurrently from widget logic.
///
/// </details>
#[derive(Clone)]
pub struct Waker {
    wake: Arc<dyn Fn() + Send + Sync + 'static>,
}

impl Waker {
    /// 使用给定的 `wake` 函数创建一个新的 [`Waker`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Waker`] with the given `wake` function.
    ///
    /// </details>
    pub fn new(wake: impl Fn() + Send + Sync + 'static) -> Self {
        Self {
            wake: Arc::new(wake),
        }
    }

    /// 创建一个什么都不做的新 [`Waker`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Waker`] that does nothing.
    ///
    /// </details>
    pub fn noop() -> Self {
        Self::new(|| {})
    }

    /// 尽快唤醒 iced 运行时。
    ///
    /// 通常你会并发地调用它（例如从另一个线程调用）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Wakes up the iced runtime as soon as possible.
    ///
    /// You normally want to call this concurrently (e.g. from a different thread).
    ///
    /// </details>
    pub fn wake(&self) {
        (self.wake)();
    }
}

impl std::fmt::Debug for Waker {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Waker").finish()
    }
}

/// 使某个布局失效时要遵循的差异比较（diff）策略。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The diffing strategy to follow when invalidating some layout.
///
/// </details>
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Diff {
    /// 跳过差异比较步骤。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Skips the diffing step.
    ///
    /// </details>
    Skip,
    /// 在布局之前重新执行差异比较。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Performs diffing again before layouting.
    ///
    /// </details>
    Perform,
}

/// 由 [`Shell`] 发布的消息的通道。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A channel of messages published by a [`Shell`].
///
/// </details>
#[derive(Debug)]
pub struct Bus<T> {
    messages: Vec<(T, Receipt)>,
}

impl<T> Bus<T> {
    /// 创建一个空的 [`Bus`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates an empty [`Bus`].
    ///
    /// </details>
    pub fn new() -> Self {
        Self {
            messages: Vec::new(),
        }
    }

    /// 如果 [`Bus`] 没有待处理的消息，返回 `true`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns `true` if the [`Bus`] has no messages pending.
    ///
    /// </details>
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    /// 返回 [`Bus`] 中待处理消息的数量。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the amount of messages pending in the [`Bus`].
    ///
    /// </details>
    pub fn len(&self) -> usize {
        self.messages.len()
    }

    /// 向 [`Bus`] 推入一条新消息。
    ///
    /// 返回的 [`Tracking`] 可以用来判断该消息
    /// 是否已被处理。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Pushes a new message to the [`Bus`].
    ///
    /// The returned [`Tracking`] can be used to determine if the message
    /// was processed.
    ///
    /// </details>
    pub fn push(&mut self, message: T) -> Tracking {
        let receipt = Receipt::new();
        let tracking = receipt.tracking();

        self.messages.push((message, receipt));

        tracking
    }

    /// 使用给定的 [`Receipt`] 向 [`Bus`] 转发一条新消息。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Forward a new message to the [`Bus`] with the given [`Receipt`].
    ///
    /// </details>
    pub fn forward(&mut self, message: T, receipt: Receipt) {
        self.messages.push((message, receipt));
    }

    /// 排空 [`Bus`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Drains the [`Bus`].
    ///
    /// </details>
    pub fn drain(&mut self) -> impl Iterator<Item = (T, Receipt)> {
        self.messages.drain(..)
    }
}

impl<T> Default for Bus<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> IntoIterator for Bus<T> {
    type Item = T;
    type IntoIter = IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            iter: self.messages.into_iter(),
        }
    }
}

/// [`Bus`] 的 [`IntoIterator`] 实现所返回的迭代器。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An iterator returned by the implementation of [`IntoIterator`] for [`Bus`].
///
/// </details>
pub struct IntoIter<T> {
    iter: vec::IntoIter<(T, Receipt)>,
}

impl<T> Iterator for IntoIter<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        Some(self.iter.next()?.0)
    }
}

/// 消息已被接收的凭证。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Proof that a message has been received.
///
/// </details>
#[derive(Debug)]
pub struct Receipt(Rc<()>);

impl Receipt {
    fn new() -> Self {
        Self(Rc::new(()))
    }

    fn tracking(&self) -> Tracking {
        Tracking(Rc::downgrade(&self.0))
    }
}

/// [`Shell::publish`] 返回的消息跟踪器。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A message tracking returned by [`Shell::publish`].
///
/// </details>
#[derive(Debug, Clone)]
pub struct Tracking(rc::Weak<()>);

impl Tracking {
    /// 如果此 [`Tracking`] 的消息已被 `update` 逻辑处理，
    /// 返回 `true`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns `true` if the message of this [`Tracking`] has been processed
    /// by `update` logic.
    ///
    /// </details>
    pub fn is_processed(&self) -> bool {
        self.0.strong_count() == 0
    }
}
