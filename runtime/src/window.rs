//! 构建基于窗口的 GUI 应用程序。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Build window-based GUI applications.
//!
//! </details>
use crate::core::time::Instant;
use crate::core::window::{
    Direction, Event, Icon, Id, Level, Mode, Screenshot, Settings, UserAttention,
};
use crate::core::{Point, Size, Window};
use crate::futures::Subscription;
use crate::futures::event;
use crate::futures::futures::channel::oneshot;
use crate::task::{self, Task};

/// 要在某个窗口上执行的操作。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An operation to be performed on some window.
///
/// </details>
pub enum Action {
    /// 使用某个 [`Settings`] 打开一个新窗口。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Open a new window with some [`Settings`].
    ///
    /// </details>
    Open(Id, Settings, oneshot::Sender<Id>),

    /// 关闭窗口并退出应用程序。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Close the window and exits the application.
    ///
    /// </details>
    Close(Id),

    /// 获取最早打开的窗口的 [`Id`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Gets the [`Id`] of the oldest window.
    ///
    /// </details>
    GetOldest(oneshot::Sender<Option<Id>>),

    /// 获取最近打开的窗口的 [`Id`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Gets the [`Id`] of the latest window.
    ///
    /// </details>
    GetLatest(oneshot::Sender<Option<Id>>),

    /// 用鼠标左键移动窗口，直到松开
    /// 按键。
    ///
    /// 除非在调用这个函数之前刚刚按下了鼠标左键，
    /// 否则不保证它能生效。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Move the window with the left mouse button until the button is
    /// released.
    ///
    /// There's no guarantee that this will work unless the left mouse
    /// button was pressed immediately before this function is called.
    ///
    /// </details>
    Drag(Id),

    /// 用鼠标左键调整窗口大小，直到松开
    /// 按键。
    ///
    /// 除非在调用这个函数之前刚刚按下了鼠标左键，
    /// 否则不保证它能生效。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Resize the window with the left mouse button until the button is
    /// released.
    ///
    /// There's no guarantee that this will work unless the left mouse
    /// button was pressed immediately before this function is called.
    ///
    /// </details>
    DragResize(Id, Direction),

    /// 把窗口调整为给定的逻辑尺寸。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Resize the window to the given logical dimensions.
    ///
    /// </details>
    Resize(Id, Size),

    /// 获取窗口当前的逻辑尺寸。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Get the current logical dimensions of the window.
    ///
    /// </details>
    GetSize(Id, oneshot::Sender<Size>),

    /// 获取当前窗口是否已最大化。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Get if the current window is maximized or not.
    ///
    /// </details>
    GetMaximized(Id, oneshot::Sender<bool>),

    /// 把窗口设置为最大化或还原
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Set the window to maximized or back
    ///
    /// </details>
    Maximize(Id, bool),

    /// 获取当前窗口是否已最小化。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Get if the current window is minimized or not.
    ///
    /// </details>
    ///
    /// ## 平台差异（Platform-specific）
    /// - **Wayland：** 始终为 `None`。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// - **Wayland:** Always `None`.
    ///
    /// </details>
    GetMinimized(Id, oneshot::Sender<Option<bool>>),

    /// 把窗口设置为最小化或还原
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Set the window to minimized or back
    ///
    /// </details>
    Minimize(Id, bool),

    /// 获取窗口当前的逻辑坐标。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Get the current logical coordinates of the window.
    ///
    /// </details>
    GetPosition(Id, oneshot::Sender<Option<Point>>),

    /// 获取窗口当前的缩放因子（DPI）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Get the current scale factor (DPI) of the window.
    ///
    /// </details>
    GetScaleFactor(Id, oneshot::Sender<f32>),

    /// 把窗口移动到给定的逻辑坐标。
    ///
    /// Wayland 上不支持。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Move the window to the given logical coordinates.
    ///
    /// Unsupported on Wayland.
    ///
    /// </details>
    Move(Id, Point),

    /// 更改窗口的 [`Mode`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Change the [`Mode`] of the window.
    ///
    /// </details>
    SetMode(Id, Mode),

    /// 获取窗口当前的 [`Mode`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Get the current [`Mode`] of the window.
    ///
    /// </details>
    GetMode(Id, oneshot::Sender<Mode>),

    /// 在最大化与还原之间切换窗口
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle the window to maximized or back
    ///
    /// </details>
    ToggleMaximize(Id),

    /// 切换窗口是否带有装饰。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Toggle whether window has decorations.
    ///
    /// </details>
    ///
    /// ## 平台差异（Platform-specific）
    /// - **X11：** 未实现。
    /// - **Web：** 不支持。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// - **X11:** Not implemented.
    /// - **Web:** Unsupported.
    ///
    /// </details>
    ToggleDecorations(Id),

    /// 请求用户注意该窗口；如果应用程序已经获得焦点，则没有效果。
    /// 请求用户注意的具体表现形式取决于平台，
    /// 详见 [`UserAttention`]。
    ///
    /// 传入 `None` 会取消请求用户注意。当窗口收到输入时，
    /// 窗口管理器不一定会自动取消这个请求。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Request user attention to the window, this has no effect if the application
    /// is already focused. How requesting for user attention manifests is platform dependent,
    /// see [`UserAttention`] for details.
    ///
    /// Providing `None` will unset the request for user attention. Unsetting the request for
    /// user attention might not be done automatically by the WM when the window receives input.
    ///
    /// </details>
    ///
    /// ## 平台差异（Platform-specific）
    ///
    /// - **iOS / Android / Web：** 不支持。
    /// - **macOS：** `None` 没有效果。
    /// - **X11：** 请求用户注意必须手动清除。
    /// - **Wayland：** 需要 `xdg_activation_v1` 协议，`None` 没有效果。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// - **iOS / Android / Web:** Unsupported.
    /// - **macOS:** `None` has no effect.
    /// - **X11:** Requests for user attention must be manually cleared.
    /// - **Wayland:** Requires `xdg_activation_v1` protocol, `None` has no effect.
    ///
    /// </details>
    RequestUserAttention(Id, Option<UserAttention>),

    /// 把窗口置于最前并设置输入焦点。如果窗口已经获得焦点、
    /// 已最小化或不可见，则没有效果。
    ///
    /// 这个方法会从其他应用程序抢走输入焦点。除非你确定这正是用户想要的，
    /// 否则不要使用它。抢占焦点可能造成极具干扰性的
    /// 用户体验。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Bring the window to the front and sets input focus. Has no effect if the window is
    /// already in focus, minimized, or not visible.
    ///
    /// This method steals input focus from other applications. Do not use this method unless
    /// you are certain that's what the user wants. Focus stealing can cause an extremely disruptive
    /// user experience.
    ///
    /// </details>
    ///
    /// ## 平台差异（Platform-specific）
    ///
    /// - **Web / Wayland：** 不支持。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// - **Web / Wayland:** Unsupported.
    ///
    /// </details>
    GainFocus(Id),

    /// 更改窗口的 [`Level`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Change the window [`Level`].
    ///
    /// </details>
    SetLevel(Id, Level),

    /// 在光标位置显示系统菜单。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Show the system menu at cursor position.
    ///
    /// </details>
    ///
    /// ## 平台差异（Platform-specific）
    /// Android / iOS / macOS / Orbital / Web / X11：不支持。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Android / iOS / macOS / Orbital / Web / X11: Unsupported.
    ///
    /// </details>
    ShowSystemMenu(Id),

    /// 获取窗口唯一的原始标识符。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Get the raw identifier unique to the window.
    ///
    /// </details>
    GetRawId(Id, oneshot::Sender<u64>),

    /// 更改窗口的 [`Icon`]。
    ///
    /// 在 Windows 和 X11 上，它通常是标题栏左上角的
    /// 小图标。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Change the window [`Icon`].
    ///
    /// On Windows and X11, this is typically the small icon in the top-left
    /// corner of the titlebar.
    ///
    /// </details>
    ///
    /// ## 平台差异（Platform-specific）
    ///
    /// - **Web / Wayland / macOS：** 不支持。
    ///
    /// - **Windows：** 设置 `ICON_SMALL`。窗口图标的基准尺寸是 16x16，但建议
    ///   考虑屏幕缩放并选用它的倍数，例如 32x32。
    ///
    /// - **X11：** 没有通用的图标尺寸规范，因此取决于窗口管理器。不过，
    ///   通常与 Windows 上的尺寸相近。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// - **Web / Wayland / macOS:** Unsupported.
    ///
    /// - **Windows:** Sets `ICON_SMALL`. The base size for a window icon is 16x16, but it's
    ///   recommended to account for screen scaling and pick a multiple of that, i.e. 32x32.
    ///
    /// - **X11:** Has no universal guidelines for icon sizes, so you're at the whims of the WM. That
    ///   said, it's usually in the same ballpark as on Windows.
    ///
    /// </details>
    SetIcon(Id, Icon),

    /// 使用具有给定 [`Id`] 的 [`Window`] 的引用来运行闭包。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Runs the closure with a reference to the [`Window`] with the given [`Id`].
    ///
    /// </details>
    Run(Id, Box<dyn FnOnce(&dyn Window) + Send>),

    /// 对窗口的视口截图。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Screenshot the viewport of the window.
    ///
    /// </details>
    Screenshot(Id, oneshot::Sender<Screenshot>),

    /// 为给定的窗口启用鼠标穿透。
    ///
    /// 这会禁用该窗口的鼠标事件，并把鼠标事件
    /// 传递给位于其下方的窗口。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Enable mouse passthrough for the given window.
    ///
    /// This disables mouse events for the window and passes mouse events
    /// through to whatever window is underneath.
    ///
    /// </details>
    EnableMousePassthrough(Id),

    /// 为给定的窗口禁用鼠标穿透。
    ///
    /// 这会启用该窗口的鼠标事件，并且不再把鼠标事件
    /// 传递给位于其下方的内容。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Disable mouse passthrough for the given window.
    ///
    /// This enables mouse events for the window and stops mouse events
    /// from being passed to whatever is underneath.
    ///
    /// </details>
    DisableMousePassthrough(Id),

    /// 设置窗口内部区域的最小尺寸。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Set the minimum inner window size.
    ///
    /// </details>
    SetMinSize(Id, Option<Size>),

    /// 设置窗口内部区域的最大尺寸。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Set the maximum inner window size.
    ///
    /// </details>
    SetMaxSize(Id, Option<Size>),

    /// 设置窗口是否可以调整大小。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Set the window to be resizable or not.
    ///
    /// </details>
    SetResizable(Id, bool),

    /// 设置窗口尺寸的调整增量。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Set the window size increment.
    ///
    /// </details>
    SetResizeIncrements(Id, Option<Size>),

    /// 获取具有给定 [`Id`] 的窗口所在显示器的逻辑尺寸。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Get the logical dimensions of the monitor containing the window with the given [`Id`].
    ///
    /// </details>
    GetMonitorSize(Id, oneshot::Sender<Option<Size>>),

    /// 设置系统是否可以自动把窗口组织成标签页。
    ///
    /// 参见 <https://developer.apple.com/documentation/appkit/nswindow/1646657-allowsautomaticwindowtabbing>
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Set whether the system can automatically organize windows into tabs.
    ///
    /// See <https://developer.apple.com/documentation/appkit/nswindow/1646657-allowsautomaticwindowtabbing>
    ///
    /// </details>
    SetAllowAutomaticTabbing(bool),

    /// 重绘所有窗口。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Redraw all the windows.
    ///
    /// </details>
    RedrawAll,

    /// 重新计算所有窗口的布局。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Recompute the layouts of all the windows.
    ///
    /// </details>
    RelayoutAll,
}

/// 订阅正在运行的应用程序的窗口的帧。
///
/// 得到的 [`Subscription`] 产生项的速率等于应用程序第一个窗口的
/// 刷新率。注意这个速率可能是可变的，因为它通常由图形驱动
/// 和/或操作系统管理。
///
/// 无论如何，这个 [`Subscription`] 适合用来流畅地绘制由应用程序驱动的
/// 动画，而不会丢帧。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Subscribes to the frames of the window of the running application.
///
/// The resulting [`Subscription`] will produce items at a rate equal to the
/// refresh rate of the first application window. Note that this rate may be variable, as it is
/// normally managed by the graphics driver and/or the OS.
///
/// In any case, this [`Subscription`] is useful to smoothly draw application-driven
/// animations without missing any frames.
///
/// </details>
pub fn frames() -> Subscription<Instant> {
    event::listen_raw(|event, _status, _window| match event {
        crate::core::Event::Window(Event::RedrawRequested(at)) => Some(at),
        _ => None,
    })
}

/// 订阅正在运行的应用程序的所有窗口事件。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Subscribes to all window events of the running application.
///
/// </details>
pub fn events() -> Subscription<(Id, Event)> {
    event::listen_with(|event, _status, id| {
        if let crate::core::Event::Window(event) = event {
            Some((id, event))
        } else {
            None
        }
    })
}

/// 订阅正在运行的应用程序中发生的所有 [`Event::Opened`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Subscribes to all [`Event::Opened`] occurrences in the running application.
///
/// </details>
pub fn open_events() -> Subscription<Id> {
    event::listen_with(|event, _status, id| {
        if let crate::core::Event::Window(Event::Opened { .. }) = event {
            Some(id)
        } else {
            None
        }
    })
}

/// 订阅正在运行的应用程序中发生的所有 [`Event::Closed`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Subscribes to all [`Event::Closed`] occurrences in the running application.
///
/// </details>
pub fn close_events() -> Subscription<Id> {
    event::listen_with(|event, _status, id| {
        if let crate::core::Event::Window(Event::Closed) = event {
            Some(id)
        } else {
            None
        }
    })
}

/// 订阅正在运行的应用程序中发生的所有 [`Event::Resized`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Subscribes to all [`Event::Resized`] occurrences in the running application.
///
/// </details>
pub fn resize_events() -> Subscription<(Id, Size)> {
    event::listen_with(|event, _status, id| {
        if let crate::core::Event::Window(Event::Resized(size)) = event {
            Some((id, size))
        } else {
            None
        }
    })
}

/// 订阅正在运行的应用程序中发生的所有 [`Event::CloseRequested`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Subscribes to all [`Event::CloseRequested`] occurrences in the running application.
///
/// </details>
pub fn close_requests() -> Subscription<Id> {
    event::listen_with(|event, _status, id| {
        if let crate::core::Event::Window(Event::CloseRequested) = event {
            Some(id)
        } else {
            None
        }
    })
}

/// 使用给定的 [`Settings`] 打开一个新窗口；完成时产生新窗口的
/// [`Id`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Opens a new window with the given [`Settings`]; producing the [`Id`]
/// of the new window on completion.
///
/// </details>
pub fn open(settings: Settings) -> (Id, Task<Id>) {
    let id = Id::unique();

    (
        id,
        task::oneshot(|channel| crate::Action::Window(Action::Open(id, settings, channel))),
    )
}

/// 关闭具有 `id` 的窗口。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Closes the window with `id`.
///
/// </details>
pub fn close<T>(id: Id) -> Task<T> {
    task::effect(crate::Action::Window(Action::Close(id)))
}

/// 获取最早打开的窗口的窗口 [`Id`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Gets the window [`Id`] of the oldest window.
///
/// </details>
pub fn oldest() -> Task<Option<Id>> {
    task::oneshot(|channel| crate::Action::Window(Action::GetOldest(channel)))
}

/// 获取最近打开的窗口的窗口 [`Id`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Gets the window [`Id`] of the latest window.
///
/// </details>
pub fn latest() -> Task<Option<Id>> {
    task::oneshot(|channel| crate::Action::Window(Action::GetLatest(channel)))
}

/// 在按住鼠标左键期间开始拖动窗口。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Begins dragging the window while the left mouse button is held.
///
/// </details>
pub fn drag<T>(id: Id) -> Task<T> {
    task::effect(crate::Action::Window(Action::Drag(id)))
}

/// 在按住鼠标左键期间开始调整窗口大小。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Begins resizing the window while the left mouse button is held.
///
/// </details>
pub fn drag_resize<T>(id: Id, direction: Direction) -> Task<T> {
    task::effect(crate::Action::Window(Action::DragResize(id, direction)))
}

/// 把窗口调整为给定的逻辑尺寸。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Resizes the window to the given logical dimensions.
///
/// </details>
pub fn resize<T>(id: Id, new_size: Size) -> Task<T> {
    task::effect(crate::Action::Window(Action::Resize(id, new_size)))
}

/// 设置窗口是否可以调整大小。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Set the window to be resizable or not.
///
/// </details>
pub fn set_resizable<T>(id: Id, resizable: bool) -> Task<T> {
    task::effect(crate::Action::Window(Action::SetResizable(id, resizable)))
}

/// 设置窗口内部区域的最大尺寸。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Set the inner maximum size of the window.
///
/// </details>
pub fn set_max_size<T>(id: Id, size: Option<Size>) -> Task<T> {
    task::effect(crate::Action::Window(Action::SetMaxSize(id, size)))
}

/// 设置窗口内部区域的最小尺寸。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Set the inner minimum size of the window.
///
/// </details>
pub fn set_min_size<T>(id: Id, size: Option<Size>) -> Task<T> {
    task::effect(crate::Action::Window(Action::SetMinSize(id, size)))
}

/// 设置窗口尺寸的调整增量。
///
/// 通常用于终端模拟器这类需要“按格”调整大小的应用。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Set the window size increment.
///
/// This is usually used by apps such as terminal emulators that need "blocky" resizing.
///
/// </details>
pub fn set_resize_increments<T>(id: Id, increments: Option<Size>) -> Task<T> {
    task::effect(crate::Action::Window(Action::SetResizeIncrements(
        id, increments,
    )))
}

/// 获取以逻辑尺寸表示的窗口大小。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Gets the window size in logical dimensions.
///
/// </details>
pub fn size(id: Id) -> Task<Size> {
    task::oneshot(move |channel| crate::Action::Window(Action::GetSize(id, channel)))
}

/// 获取具有给定 [`Id`] 的窗口的最大化状态。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Gets the maximized state of the window with the given [`Id`].
///
/// </details>
pub fn is_maximized(id: Id) -> Task<bool> {
    task::oneshot(move |channel| crate::Action::Window(Action::GetMaximized(id, channel)))
}

/// 最大化窗口。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Maximizes the window.
///
/// </details>
pub fn maximize<T>(id: Id, maximized: bool) -> Task<T> {
    task::effect(crate::Action::Window(Action::Maximize(id, maximized)))
}

/// 获取具有给定 [`Id`] 的窗口的最小化状态。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Gets the minimized state of the window with the given [`Id`].
///
/// </details>
pub fn is_minimized(id: Id) -> Task<Option<bool>> {
    task::oneshot(move |channel| crate::Action::Window(Action::GetMinimized(id, channel)))
}

/// 最小化窗口。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Minimizes the window.
///
/// </details>
pub fn minimize<T>(id: Id, minimized: bool) -> Task<T> {
    task::effect(crate::Action::Window(Action::Minimize(id, minimized)))
}

/// 获取具有给定 [`Id`] 的窗口以逻辑坐标表示的位置。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Gets the position in logical coordinates of the window with the given [`Id`].
///
/// </details>
pub fn position(id: Id) -> Task<Option<Point>> {
    task::oneshot(move |channel| crate::Action::Window(Action::GetPosition(id, channel)))
}

/// 获取具有给定 [`Id`] 的窗口的缩放因子。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Gets the scale factor of the window with the given [`Id`].
///
/// </details>
pub fn scale_factor(id: Id) -> Task<f32> {
    task::oneshot(move |channel| crate::Action::Window(Action::GetScaleFactor(id, channel)))
}

/// 把窗口移动到给定的逻辑坐标。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Moves the window to the given logical coordinates.
///
/// </details>
pub fn move_to<T>(id: Id, position: Point) -> Task<T> {
    task::effect(crate::Action::Window(Action::Move(id, position)))
}

/// 获取窗口当前的 [`Mode`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Gets the current [`Mode`] of the window.
///
/// </details>
pub fn mode(id: Id) -> Task<Mode> {
    task::oneshot(move |channel| crate::Action::Window(Action::GetMode(id, channel)))
}

/// 更改窗口的 [`Mode`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Changes the [`Mode`] of the window.
///
/// </details>
pub fn set_mode<T>(id: Id, mode: Mode) -> Task<T> {
    task::effect(crate::Action::Window(Action::SetMode(id, mode)))
}

/// 在最大化与还原之间切换窗口。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Toggles the window to maximized or back.
///
/// </details>
pub fn toggle_maximize<T>(id: Id) -> Task<T> {
    task::effect(crate::Action::Window(Action::ToggleMaximize(id)))
}

/// 切换窗口装饰。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Toggles the window decorations.
///
/// </details>
pub fn toggle_decorations<T>(id: Id) -> Task<T> {
    task::effect(crate::Action::Window(Action::ToggleDecorations(id)))
}

/// 请求用户注意该窗口。如果应用程序已经获得焦点，则没有效果。
/// 请求用户注意的具体表现形式取决于平台，
/// 详见 [`UserAttention`]。
///
/// 传入 `None` 会取消请求用户注意。当窗口收到输入时，
/// 窗口管理器不一定会自动取消这个请求。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Requests user attention to the window. This has no effect if the application
/// is already focused. How requesting for user attention manifests is platform dependent,
/// see [`UserAttention`] for details.
///
/// Providing `None` will unset the request for user attention. Unsetting the request for
/// user attention might not be done automatically by the WM when the window receives input.
///
/// </details>
pub fn request_user_attention<T>(id: Id, user_attention: Option<UserAttention>) -> Task<T> {
    task::effect(crate::Action::Window(Action::RequestUserAttention(
        id,
        user_attention,
    )))
}

/// 把窗口置于最前并设置输入焦点。如果窗口已经获得焦点、
/// 已最小化或不可见，则没有效果。
///
/// 这个 [`Task`] 会从其他应用程序抢走输入焦点。除非你确定这正是用户想要的，
/// 否则不要使用它。抢占焦点可能造成极具干扰性的
/// 用户体验。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Brings the window to the front and sets input focus. Has no effect if the window is
/// already in focus, minimized, or not visible.
///
/// This [`Task`] steals input focus from other applications. Do not use this method unless
/// you are certain that's what the user wants. Focus stealing can cause an extremely disruptive
/// user experience.
///
/// </details>
pub fn gain_focus<T>(id: Id) -> Task<T> {
    task::effect(crate::Action::Window(Action::GainFocus(id)))
}

/// 更改窗口的 [`Level`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Changes the window [`Level`].
///
/// </details>
pub fn set_level<T>(id: Id, level: Level) -> Task<T> {
    task::effect(crate::Action::Window(Action::SetLevel(id, level)))
}

/// 在光标位置显示[系统菜单][system menu]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Shows the [system menu] at cursor position.
///
/// </details>
///
/// [system menu]: https://en.wikipedia.org/wiki/Common_menus_in_Microsoft_Windows#System_menu
pub fn show_system_menu<T>(id: Id) -> Task<T> {
    task::effect(crate::Action::Window(Action::ShowSystemMenu(id)))
}

/// 获取由底层窗口系统提供的、窗口唯一的标识符。
/// 不要把它与 [`Id`] 混淆。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Gets an identifier unique to the window, provided by the underlying windowing system. This is
/// not to be confused with [`Id`].
///
/// </details>
pub fn raw_id<Message>(id: Id) -> Task<u64> {
    task::oneshot(|channel| crate::Action::Window(Action::GetRawId(id, channel)))
}

/// 更改窗口的 [`Icon`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Changes the [`Icon`] of the window.
///
/// </details>
pub fn set_icon<T>(id: Id, icon: Icon) -> Task<T> {
    task::effect(crate::Action::Window(Action::SetIcon(id, icon)))
}

/// 使用具有给定 [`Id`] 的 [`Window`] 的引用来运行给定的回调。
///
/// 注意，如果窗口在这个调用被处理之前关闭，回调将不会运行。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Runs the given callback with a reference to the [`Window`] with the given [`Id`].
///
/// Note that if the window closes before this call is processed the callback will not be run.
///
/// </details>
pub fn run<T>(id: Id, f: impl FnOnce(&dyn Window) -> T + Send + 'static) -> Task<T>
where
    T: Send + 'static,
{
    task::oneshot(move |channel| {
        crate::Action::Window(Action::Run(
            id,
            Box::new(move |handle| {
                let _ = channel.send(f(handle));
            }),
        ))
    })
}

/// 从窗口捕获一张 [`Screenshot`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Captures a [`Screenshot`] from the window.
///
/// </details>
pub fn screenshot(id: Id) -> Task<Screenshot> {
    task::oneshot(move |channel| crate::Action::Window(Action::Screenshot(id, channel)))
}

/// 为给定的窗口启用鼠标穿透。
///
/// 这会禁用该窗口的鼠标事件，并把鼠标事件
/// 传递给位于其下方的窗口。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Enables mouse passthrough for the given window.
///
/// This disables mouse events for the window and passes mouse events
/// through to whatever window is underneath.
///
/// </details>
pub fn enable_mouse_passthrough<Message>(id: Id) -> Task<Message> {
    task::effect(crate::Action::Window(Action::EnableMousePassthrough(id)))
}

/// 为给定的窗口禁用鼠标穿透。
///
/// 这会启用该窗口的鼠标事件，并且不再把鼠标事件
/// 传递给位于其下方的内容。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Disables mouse passthrough for the given window.
///
/// This enables mouse events for the window and stops mouse events
/// from being passed to whatever is underneath.
///
/// </details>
pub fn disable_mouse_passthrough<Message>(id: Id) -> Task<Message> {
    task::effect(crate::Action::Window(Action::DisableMousePassthrough(id)))
}

/// 获取具有给定 [`Id`] 的窗口所在显示器的逻辑尺寸。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Gets the logical dimensions of the monitor containing the window with the given [`Id`].
///
/// </details>
pub fn monitor_size(id: Id) -> Task<Option<Size>> {
    task::oneshot(move |channel| crate::Action::Window(Action::GetMonitorSize(id, channel)))
}

/// 设置系统是否可以自动把窗口组织成标签页。
///
/// 参见 <https://developer.apple.com/documentation/appkit/nswindow/1646657-allowsautomaticwindowtabbing>
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Sets whether the system can automatically organize windows into tabs.
///
/// See <https://developer.apple.com/documentation/appkit/nswindow/1646657-allowsautomaticwindowtabbing>
///
/// </details>
pub fn allow_automatic_tabbing<T>(enabled: bool) -> Task<T> {
    task::effect(crate::Action::Window(Action::SetAllowAutomaticTabbing(
        enabled,
    )))
}
