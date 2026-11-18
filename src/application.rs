//! 一步一步地创建并运行 iced 应用程序。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Create and run iced applications step by step.
//!
//! </details>
//!
//! # 示例（Example）
//! ```no_run,standalone_crate
//! use iced::widget::{button, column, text, Column};
//! use iced::Theme;
//!
//! pub fn main() -> iced::Result {
//!     iced::application(u64::default, update, view)
//!         .theme(Theme::Dark)
//!         .centered()
//!         .run()
//! }
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Increment,
//! }
//!
//! fn update(value: &mut u64, message: Message) {
//!     match message {
//!         Message::Increment => *value += 1,
//!     }
//! }
//!
//! fn view(value: &u64) -> Column<Message> {
//!     column![
//!         text(value),
//!         button("+").on_press(Message::Increment),
//!     ]
//! }
//! ```
use crate::message;
use crate::program::{self, Program};
use crate::shell;
use crate::theme;
use crate::window;
use crate::{
    Element, Executor, Font, Never, Preset, Result, Settings, Size,
    Subscription, Task, Theme,
};

use iced_debug as debug;

use std::borrow::Cow;

pub mod timed;

pub use timed::timed;

/// 根据启动、更新和视图逻辑创建一个 iced [`Application`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates an iced [`Application`] given its boot, update, and view logic.
///
/// </details>
///
/// # 示例（Example）
/// ```no_run,standalone_crate
/// use iced::widget::{button, column, text, Column};
///
/// pub fn main() -> iced::Result {
///     iced::application(u64::default, update, view).run()
/// }
///
/// #[derive(Debug, Clone)]
/// enum Message {
///     Increment,
/// }
///
/// fn update(value: &mut u64, message: Message) {
///     match message {
///         Message::Increment => *value += 1,
///     }
/// }
///
/// fn view(value: &u64) -> Column<Message> {
///     column![
///         text(value),
///         button("+").on_press(Message::Increment),
///     ]
/// }
/// ```
pub fn application<State, Message, Theme, Renderer>(
    boot: impl BootFn<State, Message>,
    update: impl UpdateFn<State, Message>,
    view: impl for<'a> ViewFn<'a, State, Message, Theme, Renderer>,
) -> Application<impl Program<State = State, Message = Message, Theme = Theme>>
where
    State: 'static,
    Message: Send + 'static,
    Theme: theme::Base,
    Renderer: program::Renderer,
{
    use std::marker::PhantomData;

    struct Instance<State, Message, Theme, Renderer, Boot, Update, View> {
        boot: Boot,
        update: Update,
        view: View,
        _state: PhantomData<State>,
        _message: PhantomData<Message>,
        _theme: PhantomData<Theme>,
        _renderer: PhantomData<Renderer>,
    }

    impl<State, Message, Theme, Renderer, Boot, Update, View> Program
        for Instance<State, Message, Theme, Renderer, Boot, Update, View>
    where
        Message: Send + 'static,
        Theme: theme::Base,
        Renderer: program::Renderer,
        Boot: self::BootFn<State, Message>,
        Update: self::UpdateFn<State, Message>,
        View: for<'a> self::ViewFn<'a, State, Message, Theme, Renderer>,
    {
        type State = State;
        type Message = Message;
        type Theme = Theme;
        type Renderer = Renderer;
        type Executor = iced_futures::backend::default::Executor;

        fn name() -> &'static str {
            let name = std::any::type_name::<State>();

            name.split("::").next().unwrap_or("a_cool_application")
        }

        fn boot(&self) -> (State, Task<Message>) {
            self.boot.boot()
        }

        fn update(
            &self,
            state: &mut Self::State,
            message: Self::Message,
        ) -> Task<Self::Message> {
            self.update.update(state, message)
        }

        fn view<'a>(
            &self,
            state: &'a Self::State,
            _window: window::Id,
        ) -> Element<'a, Self::Message, Self::Theme, Self::Renderer> {
            self.view.view(state)
        }

        fn settings(&self) -> Settings {
            Settings::default()
        }

        fn window(&self) -> Option<iced_core::window::Settings> {
            Some(window::Settings::default())
        }
    }

    Application {
        raw: Instance {
            boot,
            update,
            view,
            _state: PhantomData,
            _message: PhantomData,
            _theme: PhantomData,
            _renderer: PhantomData,
        },
        settings: Settings::default(),
        window: window::Settings::default(),
        presets: Vec::new(),
    }
}

/// iced 应用程序的底层定义与配置。
///
/// 你可以用这套 API 一步一步地创建并运行 iced 应用程序，
/// 而不必把你的逻辑与某个特质或某个特定类型
/// 耦合在一起。
///
/// 你可以用 [`application`] 辅助函数创建一个 [`Application`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The underlying definition and configuration of an iced application.
///
/// You can use this API to create and run iced applications
/// step by step—without coupling your logic to a trait
/// or a specific type.
///
/// You can create an [`Application`] with the [`application`] helper.
///
/// </details>
#[derive(Debug)]
pub struct Application<P: Program> {
    raw: P,
    settings: Settings,
    window: window::Settings,
    presets: Vec<Preset<P::State, P::Message>>,
}

impl<P: Program> Application<P> {
    /// 运行 [`Application`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Runs the [`Application`].
    ///
    /// </details>
    pub fn run(self) -> Result
    where
        Self: 'static,
        P::Message: message::MaybeDebug + message::MaybeClone,
    {
        #[cfg(feature = "debug")]
        iced_debug::init(iced_debug::Metadata {
            name: P::name(),
            theme: None,
            can_time_travel: cfg!(feature = "time-travel"),
        });

        #[cfg(feature = "tester")]
        let program = iced_tester::attach(self);

        #[cfg(all(
            feature = "debug",
            not(feature = "tester"),
            not(target_arch = "wasm32")
        ))]
        let program = iced_devtools::attach(self);

        #[cfg(not(any(
            feature = "tester",
            all(feature = "debug", not(target_arch = "wasm32"))
        )))]
        let program = self;

        Ok(shell::run(program)?)
    }

    /// 设置运行 [`Application`] 时使用的 [`Settings`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Settings`] that will be used to run the [`Application`].
    ///
    /// </details>
    pub fn settings(self, settings: Settings) -> Self {
        Self { settings, ..self }
    }

    /// 设置 [`Application`] 的 [`Settings::antialiasing`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Settings::antialiasing`] of the [`Application`].
    ///
    /// </details>
    pub fn antialiasing(self, antialiasing: bool) -> Self {
        Self {
            settings: Settings {
                antialiasing,
                ..self.settings
            },
            ..self
        }
    }

    /// 设置 [`Application`] 的默认 [`Font`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the default [`Font`] of the [`Application`].
    ///
    /// </details>
    pub fn default_font(self, default_font: Font) -> Self {
        Self {
            settings: Settings {
                default_font,
                ..self.settings
            },
            ..self
        }
    }

    /// 向 [`Application`] 启动时要加载的字体列表中添加一个字体。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Adds a font to the list of fonts that will be loaded at the start of the [`Application`].
    ///
    /// </details>
    pub fn font(mut self, font: impl Into<Cow<'static, [u8]>>) -> Self {
        self.settings.fonts.push(font.into());
        self
    }

    /// 设置 [`Application`] 的 [`window::Settings`]。
    ///
    /// 会覆盖之前设置的任何 [`window::Settings`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`window::Settings`] of the [`Application`].
    ///
    /// Overwrites any previous [`window::Settings`].
    ///
    /// </details>
    pub fn window(self, window: window::Settings) -> Self {
        Self { window, ..self }
    }

    /// 把 [`Application`] 的 [`window::Settings::position`] 设置为 [`window::Position::Centered`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`window::Settings::position`] to [`window::Position::Centered`] in the [`Application`].
    ///
    /// </details>
    pub fn centered(self) -> Self {
        Self {
            window: window::Settings {
                position: window::Position::Centered,
                ..self.window
            },
            ..self
        }
    }

    /// 设置 [`Application`] 的 [`window::Settings::exit_on_close_request`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`window::Settings::exit_on_close_request`] of the [`Application`].
    ///
    /// </details>
    pub fn exit_on_close_request(self, exit_on_close_request: bool) -> Self {
        Self {
            window: window::Settings {
                exit_on_close_request,
                ..self.window
            },
            ..self
        }
    }

    /// 设置 [`Application`] 的 [`window::Settings::size`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`window::Settings::size`] of the [`Application`].
    ///
    /// </details>
    pub fn window_size(self, size: impl Into<Size>) -> Self {
        Self {
            window: window::Settings {
                size: size.into(),
                ..self.window
            },
            ..self
        }
    }

    /// 设置 [`Application`] 的 [`window::Settings::transparent`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`window::Settings::transparent`] of the [`Application`].
    ///
    /// </details>
    pub fn transparent(self, transparent: bool) -> Self {
        Self {
            window: window::Settings {
                transparent,
                ..self.window
            },
            ..self
        }
    }

    /// 设置 [`Application`] 的 [`window::Settings::resizable`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`window::Settings::resizable`] of the [`Application`].
    ///
    /// </details>
    pub fn resizable(self, resizable: bool) -> Self {
        Self {
            window: window::Settings {
                resizable,
                ..self.window
            },
            ..self
        }
    }

    /// 设置 [`Application`] 的 [`window::Settings::decorations`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`window::Settings::decorations`] of the [`Application`].
    ///
    /// </details>
    pub fn decorations(self, decorations: bool) -> Self {
        Self {
            window: window::Settings {
                decorations,
                ..self.window
            },
            ..self
        }
    }

    /// 设置 [`Application`] 的 [`window::Settings::position`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`window::Settings::position`] of the [`Application`].
    ///
    /// </details>
    pub fn position(self, position: window::Position) -> Self {
        Self {
            window: window::Settings {
                position,
                ..self.window
            },
            ..self
        }
    }

    /// 设置 [`Application`] 的 [`window::Settings::level`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`window::Settings::level`] of the [`Application`].
    ///
    /// </details>
    pub fn level(self, level: window::Level) -> Self {
        Self {
            window: window::Settings {
                level,
                ..self.window
            },
            ..self
        }
    }

    /// 设置 [`Application`] 的标题。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the title of the [`Application`].
    ///
    /// </details>
    pub fn title(
        self,
        title: impl TitleFn<P::State>,
    ) -> Application<
        impl Program<State = P::State, Message = P::Message, Theme = P::Theme>,
    > {
        Application {
            raw: program::with_title(self.raw, move |state, _window| {
                title.title(state)
            }),
            settings: self.settings,
            window: self.window,
            presets: self.presets,
        }
    }

    /// 设置 [`Application`] 的订阅逻辑。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the subscription logic of the [`Application`].
    ///
    /// </details>
    pub fn subscription(
        self,
        f: impl Fn(&P::State) -> Subscription<P::Message>,
    ) -> Application<
        impl Program<State = P::State, Message = P::Message, Theme = P::Theme>,
    > {
        Application {
            raw: program::with_subscription(self.raw, f),
            settings: self.settings,
            window: self.window,
            presets: self.presets,
        }
    }

    /// 设置 [`Application`] 的主题逻辑。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the theme logic of the [`Application`].
    ///
    /// </details>
    pub fn theme(
        self,
        f: impl ThemeFn<P::State, P::Theme>,
    ) -> Application<
        impl Program<State = P::State, Message = P::Message, Theme = P::Theme>,
    > {
        Application {
            raw: program::with_theme(self.raw, move |state, _window| {
                f.theme(state)
            }),
            settings: self.settings,
            window: self.window,
            presets: self.presets,
        }
    }

    /// 设置 [`Application`] 的样式逻辑。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the style logic of the [`Application`].
    ///
    /// </details>
    pub fn style(
        self,
        f: impl Fn(&P::State, &P::Theme) -> theme::Style,
    ) -> Application<
        impl Program<State = P::State, Message = P::Message, Theme = P::Theme>,
    > {
        Application {
            raw: program::with_style(self.raw, f),
            settings: self.settings,
            window: self.window,
            presets: self.presets,
        }
    }

    /// 设置 [`Application`] 的缩放因子。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the scale factor of the [`Application`].
    ///
    /// </details>
    pub fn scale_factor(
        self,
        f: impl Fn(&P::State) -> f32,
    ) -> Application<
        impl Program<State = P::State, Message = P::Message, Theme = P::Theme>,
    > {
        Application {
            raw: program::with_scale_factor(self.raw, move |state, _window| {
                f(state)
            }),
            settings: self.settings,
            window: self.window,
            presets: self.presets,
        }
    }

    /// 设置 [`Application`] 的执行器。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the executor of the [`Application`].
    ///
    /// </details>
    pub fn executor<E>(
        self,
    ) -> Application<
        impl Program<State = P::State, Message = P::Message, Theme = P::Theme>,
    >
    where
        E: Executor,
    {
        Application {
            raw: program::with_executor::<P, E>(self.raw),
            settings: self.settings,
            window: self.window,
            presets: self.presets,
        }
    }

    /// 设置 [`Application`] 的启动预设。
    ///
    /// 在测试时，可以用预设覆盖应用程序默认的
    /// 启动策略，从而创建可复现的
    /// 环境。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the boot presets of the [`Application`].
    ///
    /// Presets can be used to override the default booting strategy
    /// of your application during testing to create reproducible
    /// environments.
    ///
    /// </details>
    pub fn presets(
        self,
        presets: impl IntoIterator<Item = Preset<P::State, P::Message>>,
    ) -> Self {
        Self {
            presets: presets.into_iter().collect(),
            ..self
        }
    }
}

impl<P: Program> Program for Application<P> {
    type State = P::State;
    type Message = P::Message;
    type Theme = P::Theme;
    type Renderer = P::Renderer;
    type Executor = P::Executor;

    fn name() -> &'static str {
        P::name()
    }

    fn settings(&self) -> Settings {
        self.settings.clone()
    }

    fn window(&self) -> Option<window::Settings> {
        Some(self.window.clone())
    }

    fn boot(&self) -> (Self::State, Task<Self::Message>) {
        self.raw.boot()
    }

    fn update(
        &self,
        state: &mut Self::State,
        message: Self::Message,
    ) -> Task<Self::Message> {
        debug::hot(|| self.raw.update(state, message))
    }

    fn view<'a>(
        &self,
        state: &'a Self::State,
        window: window::Id,
    ) -> Element<'a, Self::Message, Self::Theme, Self::Renderer> {
        debug::hot(|| self.raw.view(state, window))
    }

    fn title(&self, state: &Self::State, window: window::Id) -> String {
        debug::hot(|| self.raw.title(state, window))
    }

    fn subscription(&self, state: &Self::State) -> Subscription<Self::Message> {
        debug::hot(|| self.raw.subscription(state))
    }

    fn theme(
        &self,
        state: &Self::State,
        window: iced_core::window::Id,
    ) -> Option<Self::Theme> {
        debug::hot(|| self.raw.theme(state, window))
    }

    fn style(&self, state: &Self::State, theme: &Self::Theme) -> theme::Style {
        debug::hot(|| self.raw.style(state, theme))
    }

    fn scale_factor(&self, state: &Self::State, window: window::Id) -> f32 {
        debug::hot(|| self.raw.scale_factor(state, window))
    }

    fn presets(&self) -> &[Preset<Self::State, Self::Message>] {
        &self.presets
    }
}

/// 初始化某个 [`Application`] 的 `State` 的逻辑。
///
/// `Fn() -> State` 和 `Fn() -> (State, Task<Message>)`
/// 都实现了这个特质。
///
/// 实际上，这意味着 [`application`] 既可以接受
/// `State::default` 这样的简单函数，也可以接受返回 [`Task`] 的
/// 更高级的函数。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The logic to initialize the `State` of some [`Application`].
///
/// This trait is implemented for both `Fn() -> State` and
/// `Fn() -> (State, Task<Message>)`.
///
/// In practice, this means that [`application`] can both take
/// simple functions like `State::default` and more advanced ones
/// that return a [`Task`].
///
/// </details>
pub trait BootFn<State, Message> {
    /// 初始化 [`Application`] 的状态。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Initializes the [`Application`] state.
    ///
    /// </details>
    fn boot(&self) -> (State, Task<Message>);
}

impl<T, C, State, Message> BootFn<State, Message> for T
where
    T: Fn() -> C,
    C: IntoBoot<State, Message>,
{
    fn boot(&self) -> (State, Task<Message>) {
        self().into_boot()
    }
}

/// 某个 [`Application`] 的初始状态。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The initial state of some [`Application`].
///
/// </details>
pub trait IntoBoot<State, Message> {
    /// 把某个类型转换为某个 [`Application`] 的初始状态。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Turns some type into the initial state of some [`Application`].
    ///
    /// </details>
    fn into_boot(self) -> (State, Task<Message>);
}

impl<State, Message> IntoBoot<State, Message> for State {
    fn into_boot(self) -> (State, Task<Message>) {
        (self, Task::none())
    }
}

impl<State, Message> IntoBoot<State, Message> for (State, Task<Message>) {
    fn into_boot(self) -> (State, Task<Message>) {
        self
    }
}

/// 某个 [`Application`] 的标题逻辑。
///
/// `&static str` 和任何 `Fn(&State) -> String` 闭包
/// 都实现了这个特质。
///
/// 这个特质让 [`application`] 构建器可以接受其中任何一种。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The title logic of some [`Application`].
///
/// This trait is implemented both for `&static str` and
/// any closure `Fn(&State) -> String`.
///
/// This trait allows the [`application`] builder to take any of them.
///
/// </details>
pub trait TitleFn<State> {
    /// 生成 [`Application`] 的标题。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Produces the title of the [`Application`].
    ///
    /// </details>
    fn title(&self, state: &State) -> String;
}

impl<State> TitleFn<State> for &'static str {
    fn title(&self, _state: &State) -> String {
        self.to_string()
    }
}

impl<T, State> TitleFn<State> for T
where
    T: Fn(&State) -> String,
{
    fn title(&self, state: &State) -> String {
        self(state)
    }
}

/// 某个 [`Application`] 的更新逻辑。
///
/// 这个特质让 [`application`] 构建器可以接受任何返回
/// `Into<Task<Message>>` 的闭包。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The update logic of some [`Application`].
///
/// This trait allows the [`application`] builder to take any closure that
/// returns any `Into<Task<Message>>`.
///
/// </details>
pub trait UpdateFn<State, Message> {
    /// 处理消息并更新 [`Application`] 的状态。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Processes the message and updates the state of the [`Application`].
    ///
    /// </details>
    fn update(&self, state: &mut State, message: Message) -> Task<Message>;
}

impl<State> UpdateFn<State, Never> for () {
    fn update(&self, _state: &mut State, _message: Never) -> Task<Never> {
        Task::none()
    }
}

impl<T, State, Message, C> UpdateFn<State, Message> for T
where
    T: Fn(&mut State, Message) -> C,
    C: Into<Task<Message>>,
{
    fn update(&self, state: &mut State, message: Message) -> Task<Message> {
        self(state, message).into()
    }
}

/// 某个 [`Application`] 的视图逻辑。
///
/// 这个特质让 [`application`] 构建器可以接受任何返回
/// `Into<Element<'_, Message>>` 的闭包。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The view logic of some [`Application`].
///
/// This trait allows the [`application`] builder to take any closure that
/// returns any `Into<Element<'_, Message>>`.
///
/// </details>
pub trait ViewFn<'a, State, Message, Theme, Renderer> {
    /// 生成 [`Application`] 的部件。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Produces the widget of the [`Application`].
    ///
    /// </details>
    fn view(&self, state: &'a State) -> Element<'a, Message, Theme, Renderer>;
}

impl<'a, T, State, Message, Theme, Renderer, Widget>
    ViewFn<'a, State, Message, Theme, Renderer> for T
where
    T: Fn(&'a State) -> Widget,
    State: 'static,
    Widget: Into<Element<'a, Message, Theme, Renderer>>,
{
    fn view(&self, state: &'a State) -> Element<'a, Message, Theme, Renderer> {
        self(state).into()
    }
}

/// 某个 [`Application`] 的主题逻辑。
///
/// 任何实现了这个特质的类型都可以作为参数传给
/// [`Application::theme`]。
///
/// `iced` 提供了两种实现：
/// - 内置的 [`Theme`] 本身
/// - 以及任何 `Fn(&State) -> impl Into<Option<Theme>>`。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The theme logic of some [`Application`].
///
/// Any implementors of this trait can be provided as an argument to
/// [`Application::theme`].
///
/// `iced` provides two implementors:
/// - the built-in [`Theme`] itself
/// - and any `Fn(&State) -> impl Into<Option<Theme>>`.
///
/// </details>
pub trait ThemeFn<State, Theme> {
    /// 返回 [`Application`] 在当前状态下的主题。
    ///
    /// 如果返回 `None`，`iced` 会尝试使用与系统配色方案
    /// 相匹配的主题。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the theme of the [`Application`] for the current state.
    ///
    /// If `None` is returned, `iced` will try to use a theme that
    /// matches the system color scheme.
    ///
    /// </details>
    fn theme(&self, state: &State) -> Option<Theme>;
}

impl<State> ThemeFn<State, Theme> for Theme {
    fn theme(&self, _state: &State) -> Option<Theme> {
        Some(self.clone())
    }
}

impl<F, T, State, Theme> ThemeFn<State, Theme> for F
where
    F: Fn(&State) -> T,
    T: Into<Option<Theme>>,
{
    fn theme(&self, state: &State) -> Option<Theme> {
        (self)(state).into()
    }
}
