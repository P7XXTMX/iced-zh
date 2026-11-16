//! 在更新逻辑中接收 [`Instant`] 的 [`Application`]。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! An [`Application`] that receives an [`Instant`] in update logic.
//!
//! </details>
use crate::application::{Application, BootFn, ViewFn};
use crate::core::Widget;
use crate::program;
use crate::theme;
use crate::time::Instant;
use crate::window;
use crate::{Program, Settings, Subscription, Task};

/// 创建一个 [`Application`]，它的 `update` 函数还会接收
/// 每条 `Message` 对应的 [`Instant`]。
///
/// 这个构造函数适合用来创建_纯_的动画应用程序
/// （即不依赖 [`Instant::now`] 这类有副作用的调用）。
///
/// 如果你希望应用程序在给定相同的消息历史时最终处于
/// 完全相同的状态，就需要纯度。有了这个性质，
/// 才能用 [`comet`] 进行正确的时间旅行调试。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Creates an [`Application`] with an `update` function that also
/// takes the [`Instant`] of each `Message`.
///
/// This constructor is useful to create animated applications that
/// are _pure_ (e.g. without relying on side-effect calls like [`Instant::now`]).
///
/// Purity is needed when you want your application to end up in the
/// same exact state given the same history of messages. This property
/// enables proper time traveling debugging with [`comet`].
///
/// </details>
///
/// [`comet`]: https://github.com/iced-rs/comet
pub fn timed<State, Message, Theme, Renderer>(
    boot: impl BootFn<State, Message>,
    update: impl UpdateFn<State, Message>,
    subscription: impl Fn(&State) -> Subscription<Message>,
    view: impl for<'a> ViewFn<'a, State, Message, Theme, Renderer>,
) -> Application<impl Program<State = State, Message = (Message, Instant), Theme = Theme>>
where
    State: 'static,
    Message: Send + 'static,
    Theme: theme::Base + 'static,
    Renderer: program::Renderer + 'static,
{
    use std::marker::PhantomData;

    struct Instance<State, Message, Theme, Renderer, Boot, Update, Subscription, View> {
        boot: Boot,
        update: Update,
        subscription: Subscription,
        view: View,
        _state: PhantomData<State>,
        _message: PhantomData<Message>,
        _theme: PhantomData<Theme>,
        _renderer: PhantomData<Renderer>,
    }

    impl<State, Message, Theme, Renderer, Boot, Update, Subscription, View> Program
        for Instance<State, Message, Theme, Renderer, Boot, Update, Subscription, View>
    where
        Message: Send + 'static,
        Theme: theme::Base + 'static,
        Renderer: program::Renderer + 'static,
        Boot: self::BootFn<State, Message>,
        Update: self::UpdateFn<State, Message>,
        Subscription: Fn(&State) -> self::Subscription<Message>,
        View: for<'a> self::ViewFn<'a, State, Message, Theme, Renderer>,
    {
        type State = State;
        type Message = (Message, Instant);
        type Theme = Theme;
        type Renderer = Renderer;
        type Executor = iced_futures::backend::default::Executor;

        fn name() -> &'static str {
            let name = std::any::type_name::<State>();

            name.split("::").next().unwrap_or("a_cool_application")
        }

        fn settings(&self) -> Settings {
            Settings::default()
        }

        fn window(&self) -> Option<iced_core::window::Settings> {
            Some(window::Settings::default())
        }

        fn boot(&self) -> (State, Task<Self::Message>) {
            let (state, task) = self.boot.boot();

            (state, task.map(|message| (message, Instant::now())))
        }

        #[inline]
        fn update(
            &self,
            state: &mut Self::State,
            (message, now): Self::Message,
        ) -> Task<Self::Message> {
            self.update
                .update(state, message, now)
                .into()
                .map(|message| (message, Instant::now()))
        }

        #[inline]
        fn view<'a>(
            &self,
            state: &'a Self::State,
            _window: window::Id,
        ) -> impl Widget<Self::Message, Self::Theme, Self::Renderer> + 'a {
            self.view
                .view(state)
                ._map(|message| (message, Instant::now()))
        }

        #[inline]
        fn subscription(&self, state: &Self::State) -> self::Subscription<Self::Message> {
            (self.subscription)(state).map(|message| (message, Instant::now()))
        }
    }

    Application {
        raw: Instance {
            boot,
            update,
            subscription,
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

/// 某个带时间的 [`Application`] 的更新逻辑。
///
/// 它与 [`application::UpdateFn`](super::UpdateFn) 类似，
/// 但还会接收一个 [`Instant`]。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The update logic of some timed [`Application`].
///
/// This is like [`application::UpdateFn`](super::UpdateFn),
/// but it also takes an [`Instant`].
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
    fn update(&self, state: &mut State, message: Message, now: Instant)
    -> impl Into<Task<Message>>;
}

impl<State, Message> UpdateFn<State, Message> for () {
    fn update(
        &self,
        _state: &mut State,
        _message: Message,
        _now: Instant,
    ) -> impl Into<Task<Message>> {
    }
}

impl<T, State, Message, C> UpdateFn<State, Message> for T
where
    T: Fn(&mut State, Message, Instant) -> C,
    C: Into<Task<Message>>,
{
    fn update(
        &self,
        state: &mut State,
        message: Message,
        now: Instant,
    ) -> impl Into<Task<Message>> {
        self(state, message, now)
    }
}
