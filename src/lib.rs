//! iced 是一个专注于简洁和类型安全的跨平台 GUI 库。
//! 灵感来自 [Elm]。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! iced is a cross-platform GUI library focused on simplicity and type-safety.
//! Inspired by [Elm].
//!
//! </details>
//!
//! [Elm]: https://elm-lang.org/
//!
//! # 免责声明（Disclaimer）
//! iced 是**实验性**软件。如果你期望文档在入门阶段手把手地带着你，
//! 那么你的体验将会令人沮丧。
//!
//! 这个库充分利用了 Rust 的全部特性：所有权、借用、生命周期、future、
//! 流、一等函数、特质约束、闭包等等。本文档并不打算教你其中任何一项。
//! 恰恰相反，它假定你已经**精通**
//! 所有这些。
//!
//! 此外，和 Rust 一样，iced 非常不留情面。它不会让你轻易走捷径。
//! 仅凭类型签名就足以学会这个库大部分的用法。
//! 一切都是相互关联的。
//!
//! 因此，对于**高级** Rust 程序员来说，iced 很容易学；不过也有许多有耐心的
//! 初学者学会了它，并且乐在其中。由于它以类型安全的方式利用了 Rust 提供的
//! 大量特性，它也可以成为探索 Rust 本身的好途径。
//!
//! 如果你不喜欢这种说法，期望有人把知识喂到嘴边，或者在使用这个库时
//! 感到沮丧和吃力，那么我建议你耐心等待[这本书][the book]
//! 写完。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! iced is __experimental__ software. If you expect the documentation to hold your hand
//! as you learn the ropes, you are in for a frustrating experience.
//!
//! The library leverages Rust to its full extent: ownership, borrowing, lifetimes, futures,
//! streams, first-class functions, trait bounds, closures, and more. This documentation
//! is not meant to teach you any of these. Far from it, it will assume you have __mastered__
//! all of them.
//!
//! Furthermore—just like Rust—iced is very unforgiving. It will not let you easily cut corners.
//! The type signatures alone can be used to learn how to use most of the library.
//! Everything is connected.
//!
//! Therefore, iced is easy to learn for __advanced__ Rust programmers; but plenty of patient
//! beginners have learned it and had a good time with it. Since it leverages a lot of what
//! Rust has to offer in a type-safe way, it can be a great way to discover Rust itself.
//!
//! If you don't like the sound of that, you expect to be spoonfed, or you feel frustrated
//! and struggle to use the library; then I recommend you to wait patiently until [the book]
//! is finished.
//!
//! </details>
//!
//! [the book]: https://book.iced.rs
//!
//! # 袖珍指南（The Pocket Guide）
//! 从调用 [`run`] 开始：
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Start by calling [`run`]:
//!
//! </details>
//!
//! ```no_run,standalone_crate
//! pub fn main() -> iced::Result {
//!     iced::run(update, view)
//! }
//! # fn update(state: &mut (), message: ()) {}
//! # fn view(state: &()) -> iced::Element<'_, ()> { iced::widget::text("").into() }
//! ```
//!
//! 定义一个 `update` 函数来**改变**你的状态：
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Define an `update` function to __change__ your state:
//!
//! </details>
//!
//! ```standalone_crate
//! fn update(counter: &mut u64, message: Message) {
//!     match message {
//!         Message::Increment => *counter += 1,
//!     }
//! }
//! # #[derive(Clone)]
//! # enum Message { Increment }
//! ```
//!
//! 定义一个 `view` 函数来**显示**你的状态：
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Define a `view` function to __display__ your state:
//!
//! </details>
//!
//! ```standalone_crate
//! use iced::widget::{button, text};
//! use iced::Element;
//!
//! fn view(counter: &u64) -> Element<'_, Message> {
//!     button(text(counter)).on_press(Message::Increment).into()
//! }
//! # #[derive(Clone)]
//! # enum Message { Increment }
//! ```
//!
//! 再创建一个 `Message` 枚举，把 `view` 和 `update` **连接**起来：
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! And create a `Message` enum to __connect__ `view` and `update` together:
//!
//! </details>
//!
//! ```standalone_crate
//! #[derive(Debug, Clone)]
//! enum Message {
//!     Increment,
//! }
//! ```
//!
//! ## 自定义状态（Custom State）
//! 你可以为状态定义自己的结构体：
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! You can define your own struct for your state:
//!
//! </details>
//!
//! ```standalone_crate
//! #[derive(Default)]
//! struct Counter {
//!     value: u64,
//! }
//! ```
//!
//! 但你必须相应地修改 `update` 和 `view`：
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! But you have to change `update` and `view` accordingly:
//!
//! </details>
//!
//! ```standalone_crate
//! # struct Counter { value: u64 }
//! # #[derive(Clone)]
//! # enum Message { Increment }
//! # use iced::widget::{button, text};
//! # use iced::Element;
//! fn update(counter: &mut Counter, message: Message) {
//!     match message {
//!         Message::Increment => counter.value += 1,
//!     }
//! }
//!
//! fn view(counter: &Counter) -> Element<'_, Message> {
//!     button(text(counter.value)).on_press(Message::Increment).into()
//! }
//! ```
//!
//! ## 部件与元素（Widgets and Elements）
//! `view` 函数必须返回一个 [`Element`]。[`Element`] 就是一个通用的[部件][`widget`]。
//!
//! [`widget`] 模块包含许多函数，帮助你构建
//! 和使用部件。
//!
//! 部件使用构建器模式来配置：
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! The `view` function must return an [`Element`]. An [`Element`] is just a generic [`widget`].
//!
//! The [`widget`] module contains a bunch of functions to help you build
//! and use widgets.
//!
//! Widgets are configured using the builder pattern:
//!
//! </details>
//!
//! ```standalone_crate
//! # struct Counter { value: u64 }
//! # #[derive(Clone)]
//! # enum Message { Increment }
//! use iced::widget::{button, column, text};
//! use iced::Element;
//!
//! fn view(counter: &Counter) -> Element<'_, Message> {
//!     column![
//!         text(counter.value).size(20),
//!         button("Increment").on_press(Message::Increment),
//!     ]
//!     .spacing(10)
//!     .into()
//! }
//! ```
//!
//! 调用 `into` 可以把部件转换成 [`Element`]。
//!
//! 部件和元素对它们产生的消息类型是泛型的。
//! `view` 返回的 [`Element`] 必须与你的 `update`
//! 具有相同的 `Message` 类型。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! A widget can be turned into an [`Element`] by calling `into`.
//!
//! Widgets and elements are generic over the message type they produce. The
//! [`Element`] returned by `view` must have the same `Message` type as
//! your `update`.
//!
//! </details>
//!
//! ## 布局（Layout）
//! iced 中没有统一的布局系统。每个部件各自实现
//! 自己的布局策略。
//!
//! 构建布局通常就是组合使用
//! [行][rows]、[列][columns]和[容器][containers]：
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! There is no unified layout system in iced. Instead, each widget implements
//! its own layout strategy.
//!
//! Building your layout will often consist in using a combination of
//! [rows], [columns], and [containers]:
//!
//! </details>
//!
//! ```standalone_crate
//! # struct State;
//! # enum Message {}
//! use iced::widget::{column, container, row};
//! use iced::{Fill, Element};
//!
//! fn view(state: &State) -> Element<'_, Message> {
//!     container(
//!         column![
//!             "Top",
//!             row!["Left", "Right"].spacing(10),
//!             "Bottom"
//!         ]
//!         .spacing(10)
//!     )
//!     .padding(10)
//!     .center_x(Fill)
//!     .center_y(Fill)
//!     .into()
//! }
//! ```
//!
//! 行和列分别在水平和垂直方向上排列它们的子元素。
//! 可以方便地在元素之间添加[间距][Spacing]。
//!
//! 容器在自己的边界内定位或对齐单个部件。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Rows and columns lay out their children horizontally and vertically,
//! respectively. [Spacing] can be easily added between elements.
//!
//! Containers position or align a single widget inside their bounds.
//!
//! </details>
//!
//! [rows]: widget::Row
//! [columns]: widget::Column
//! [containers]: widget::Container
//! [Spacing]: widget::Column::spacing
//!
//! ## 尺寸（Sizing）
//! 部件的宽度和高度通常可以用 [`Length`] 来定义。
//!
//! - [`Fill`] 会让部件占满给定轴上的全部可用空间。
//! - [`Shrink`] 会让部件使用其固有尺寸。
//!
//! 大多数部件默认使用 [`Shrink`] 尺寸策略，但会从它们的子元素
//! 继承 [`Fill`] 策略。
//!
//! 也可以使用以 [`Pixels`] 为单位的固定数值 [`Length`]：
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! The width and height of widgets can generally be defined using a [`Length`].
//!
//! - [`Fill`] will make the widget take all the available space in a given axis.
//! - [`Shrink`] will make the widget use its intrinsic size.
//!
//! Most widgets use a [`Shrink`] sizing strategy by default, but will inherit
//! a [`Fill`] strategy from their children.
//!
//! A fixed numeric [`Length`] in [`Pixels`] can also be used:
//!
//! </details>
//!
//! ```standalone_crate
//! # struct State;
//! # enum Message {}
//! use iced::widget::container;
//! use iced::Element;
//!
//! fn view(state: &State) -> Element<'_, Message> {
//!     container("I am 300px tall!").height(300).into()
//! }
//! ```
//!
//! ## 主题（Theming）
//! 要更改应用程序的默认 [`Theme`]，可以定义一个 `theme`
//! 函数并使用 [`Application`] 构建器，而不是直接
//! 调用 [`run`]：
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! The default [`Theme`] of an application can be changed by defining a `theme`
//! function and leveraging the [`Application`] builder, instead of directly
//! calling [`run`]:
//!
//! </details>
//!
//! ```no_run,standalone_crate
//! # struct State;
//! use iced::Theme;
//!
//! pub fn main() -> iced::Result {
//!     iced::application(new, update, view)
//!         .theme(theme)
//!         .run()
//! }
//!
//! fn new() -> State {
//!     // ...
//!     # State
//! }
//!
//! fn theme(state: &State) -> Theme {
//!     Theme::TokyoNight
//! }
//! # fn update(state: &mut State, message: ()) {}
//! # fn view(state: &State) -> iced::Element<'_, ()> { iced::widget::text("").into() }
//! ```
//!
//! `theme` 函数接收应用程序的当前状态，因此返回的
//! [`Theme`] 可以完全是动态的——就像 `view` 一样。
//!
//! 有许多内置的 [`Theme`] 变体可供使用，你也可以
//! [创建自己的主题](Theme::custom)。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! The `theme` function takes the current state of the application, allowing the
//! returned [`Theme`] to be completely dynamic—just like `view`.
//!
//! There are a bunch of built-in [`Theme`] variants at your disposal, but you can
//! also [create your own](Theme::custom).
//!
//! </details>
//!
//! ## 样式（Styling）
//! 与布局一样，iced 没有统一的样式系统。不过，所有
//! 内置部件都遵循相同的样式方式。
//!
//! 调用部件的 `style` 方法可以改变它的外观：
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! As with layout, iced does not have a unified styling system. However, all
//! of the built-in widgets follow the same styling approach.
//!
//! The appearance of a widget can be changed by calling its `style` method:
//!
//! </details>
//!
//! ```standalone_crate
//! # struct State;
//! # enum Message {}
//! use iced::widget::container;
//! use iced::Element;
//!
//! fn view(state: &State) -> Element<'_, Message> {
//!     container("I am a rounded box!").style(container::rounded_box).into()
//! }
//! ```
//!
//! 部件的 `style` 方法接受一个闭包，该闭包根据当前生效的
//! [`Theme`] 返回部件的样式：
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! The `style` method of a widget takes a closure that, given the current active
//! [`Theme`], returns the widget style:
//!
//! </details>
//!
//! ```standalone_crate
//! # struct State;
//! # #[derive(Clone)]
//! # enum Message {}
//! use iced::widget::button;
//! use iced::{Element, Theme};
//!
//! fn view(state: &State) -> Element<'_, Message> {
//!     button("I am a styled button!").style(|theme: &Theme, status| {
//!         let palette = theme.extended_palette();
//!
//!         match status {
//!             button::Status::Active => {
//!                 button::Style::default()
//!                    .with_background(palette.success.strong.color)
//!             }
//!             _ => button::primary(theme, status),
//!         }
//!     })
//!     .into()
//! }
//! ```
//!
//! 可以处于多种不同状态的部件还会向闭包提供
//! 某种 [`Status`]，让你可以为每种状态使用不同的样式。
//!
//! 你可以用 [`palette`] 或 [`extended_palette`] 方法
//! 取出 [`Theme`] 的 [`Palette`] 颜色。
//!
//! 为了方便，大多数部件都在各自的模块中提供了样式函数，
//! 例如 [`container::rounded_box`]、[`button::primary`] 或 [`text::danger`]。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Widgets that can be in multiple different states will also provide the closure
//! with some [`Status`], allowing you to use a different style for each state.
//!
//! You can extract the [`Palette`] colors of a [`Theme`] with the [`palette`] or
//! [`extended_palette`] methods.
//!
//! Most widgets provide styling functions for your convenience in their respective modules;
//! like [`container::rounded_box`], [`button::primary`], or [`text::danger`].
//!
//! </details>
//!
//! [`Status`]: widget::button::Status
//! [`palette`]: Theme::palette
//! [`extended_palette`]: Theme::extended_palette
//! [`container::rounded_box`]: widget::container::rounded_box
//! [`button::primary`]: widget::button::primary
//! [`text::danger`]: widget::text::danger
//!
//! ## 并发任务（Concurrent Tasks）
//! `update` 函数可以*选择性地*返回一个 [`Task`]。
//!
//! 可以利用 [`Task`] 执行异步工作，例如运行一个
//! future 或一个流：
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! The `update` function can _optionally_ return a [`Task`].
//!
//! A [`Task`] can be leveraged to perform asynchronous work, like running a
//! future or a stream:
//!
//! </details>
//!
//! ```standalone_crate
//! # #[derive(Clone)]
//! # struct Weather;
//! use iced::Task;
//!
//! struct State {
//!     weather: Option<Weather>,
//! }
//!
//! enum Message {
//!    FetchWeather,
//!    WeatherFetched(Weather),
//! }
//!
//! fn update(state: &mut State, message: Message) -> Task<Message> {
//!     match message {
//!         Message::FetchWeather => Task::perform(
//!             fetch_weather(),
//!             Message::WeatherFetched,
//!         ),
//!         Message::WeatherFetched(weather) => {
//!             state.weather = Some(weather);
//!
//!             Task::none()
//!        }
//!     }
//! }
//!
//! async fn fetch_weather() -> Weather {
//!     // ...
//!     # unimplemented!()
//! }
//! ```
//!
//! 任务也可以用来与 iced 运行时交互。一些模块
//! 公开了创建各种用途任务的函数，例如[修改
//! 窗口设置](window#functions)、[聚焦某个部件](widget::operation::focus_next)，或者
//! [查询它的可见边界](widget::selector::find)。
//!
//! 与 future 和流一样，任务公开了[单子式接口](Task::then)；不过它们还可以被
//! [映射](Task::map)、[串联](Task::chain)、[批量合并](Task::batch)、[取消](Task::abortable)，
//! 等等。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Tasks can also be used to interact with the iced runtime. Some modules
//! expose functions that create tasks for different purposes—like [changing
//! window settings](window#functions), [focusing a widget](widget::operation::focus_next), or
//! [querying its visible bounds](widget::selector::find).
//!
//! Like futures and streams, tasks expose [a monadic interface](Task::then)—but they can also be
//! [mapped](Task::map), [chained](Task::chain), [batched](Task::batch), [canceled](Task::abortable),
//! and more.
//!
//! </details>
//!
//! ## 被动订阅（Passive Subscriptions）
//! 应用程序可以订阅被动的数据源，例如时钟滴答或运行时事件。
//!
//! 你需要定义一个 `subscription` 函数并使用 [`Application`] 构建器：
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Applications can subscribe to passive sources of data—like time ticks or runtime events.
//!
//! You will need to define a `subscription` function and use the [`Application`] builder:
//!
//! </details>
//!
//! ```no_run,standalone_crate
//! # struct State;
//! use iced::window;
//! use iced::{Size, Subscription};
//!
//! #[derive(Debug, Clone)]
//! enum Message {
//!     WindowResized(Size),
//! }
//!
//! pub fn main() -> iced::Result {
//!     iced::application(new, update, view)
//!         .subscription(subscription)
//!         .run()
//! }
//!
//! fn subscription(state: &State) -> Subscription<Message> {
//!     window::resize_events().map(|(_id, size)| Message::WindowResized(size))
//! }
//! # fn new() -> State { State }
//! # fn update(state: &mut State, message: Message) {}
//! # fn view(state: &State) -> iced::Element<'_, Message> { iced::widget::text("").into() }
//! ```
//!
//! [`Subscription`] 是[一种*声明式*的流构建器](Subscription#the-lifetime-of-a-subscription)，
//! 这些流不允许自行结束。只有 `subscription` 函数
//! 决定哪些订阅处于活动状态，就像 `view` 在每一时刻都完全决定了
//! 用户界面中可见的部件一样。
//!
//! 与任务一样，一些模块公开了方便的函数来为你构建 [`Subscription`]，例如
//! 可以用来监听时间的 [`time::every`]，或者会把所有键盘事件通知给你的
//! [`keyboard::listen`]。不过你也可以用 [`Subscription::run`] 和 [`run_with`] 创建自己的订阅。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! A [`Subscription`] is [a _declarative_ builder of streams](Subscription#the-lifetime-of-a-subscription)
//! that are not allowed to end on their own. Only the `subscription` function
//! dictates the active subscriptions—just like `view` fully dictates the
//! visible widgets of your user interface, at every moment.
//!
//! As with tasks, some modules expose convenient functions that build a [`Subscription`] for you—like
//! [`time::every`] which can be used to listen to time, or [`keyboard::listen`] which will notify you
//! of any keyboard events. But you can also create your own with [`Subscription::run`] and [`run_with`].
//!
//! </details>
//!
//! [`run_with`]: Subscription::run_with
//!
//! ## 扩展应用程序（Scaling Applications）
//! `update`、`view` 和 `Message` 这个三元组可以很好地组合。
//!
//! 一种常见的模式是利用这种可组合性，把应用程序
//! 拆分成不同的屏幕：
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! The `update`, `view`, and `Message` triplet composes very nicely.
//!
//! A common pattern is to leverage this composability to split an
//! application into different screens:
//!
//! </details>
//!
//! ```standalone_crate
//! # mod contacts {
//! #     use iced::{Element, Task};
//! #     pub struct Contacts;
//! #     impl Contacts {
//! #         pub fn update(&mut self, message: Message) -> Action { unimplemented!() }
//! #         pub fn view(&self) -> Element<Message> { unimplemented!() }
//! #     }
//! #     #[derive(Debug, Clone)]
//! #     pub enum Message {}
//! #     pub enum Action { None, Run(Task<Message>), Chat(()) }
//! # }
//! # mod conversation {
//! #     use iced::{Element, Task};
//! #     pub struct Conversation;
//! #     impl Conversation {
//! #         pub fn new(contact: ()) -> (Self, Task<Message>) { unimplemented!() }
//! #         pub fn update(&mut self, message: Message) -> Task<Message> { unimplemented!() }
//! #         pub fn view(&self) -> Element<Message> { unimplemented!() }
//! #     }
//! #     #[derive(Debug, Clone)]
//! #     pub enum Message {}
//! # }
//! use contacts::Contacts;
//! use conversation::Conversation;
//!
//! use iced::{Element, Task};
//!
//! struct State {
//!     screen: Screen,
//! }
//!
//! enum Screen {
//!     Contacts(Contacts),
//!     Conversation(Conversation),
//! }
//!
//! enum Message {
//!    Contacts(contacts::Message),
//!    Conversation(conversation::Message)
//! }
//!
//! fn update(state: &mut State, message: Message) -> Task<Message> {
//!     match message {
//!         Message::Contacts(message) => {
//!             if let Screen::Contacts(contacts) = &mut state.screen {
//!                 let action = contacts.update(message);
//!
//!                 match action {
//!                     contacts::Action::None => Task::none(),
//!                     contacts::Action::Run(task) => task.map(Message::Contacts),
//!                     contacts::Action::Chat(contact) => {
//!                         let (conversation, task) = Conversation::new(contact);
//!
//!                         state.screen = Screen::Conversation(conversation);
//!
//!                         task.map(Message::Conversation)
//!                     }
//!                  }
//!             } else {
//!                 Task::none()    
//!             }
//!         }
//!         Message::Conversation(message) => {
//!             if let Screen::Conversation(conversation) = &mut state.screen {
//!                 conversation.update(message).map(Message::Conversation)
//!             } else {
//!                 Task::none()    
//!             }
//!         }
//!     }
//! }
//!
//! fn view(state: &State) -> Element<'_, Message> {
//!     match &state.screen {
//!         Screen::Contacts(contacts) => contacts.view().map(Message::Contacts),
//!         Screen::Conversation(conversation) => conversation.view().map(Message::Conversation),
//!     }
//! }
//! ```
//!
//! 屏幕的 `update` 方法可以返回一个 `Action` 枚举，父级可以利用它来
//! 执行任务，或者干脆切换到一个完全不同的屏幕。`Action` 的变体可以
//! 带有关联数据。例如，在上面的示例中，当 `Contacts::update` 返回带有
//! 所选联系人的 `Action::Chat` 时，就会创建 `Conversation` 屏幕。
//!
//! 实际上，这种做法让你能够以类型安全的方式“讲一个故事”，
//! 把不同的屏幕连接起来。
//!
//! 此外，[`Task::map`]、[`Element::map`] 和 [`Subscription::map`] 这些函子方法
//! 让组合变得顺畅自然。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! The `update` method of a screen can return an `Action` enum that can be leveraged by the parent to
//! execute a task or transition to a completely different screen altogether. The variants of `Action` can
//! have associated data. For instance, in the example above, the `Conversation` screen is created when
//! `Contacts::update` returns an `Action::Chat` with the selected contact.
//!
//! Effectively, this approach lets you "tell a story" to connect different screens together in a type safe
//! way.
//!
//! Furthermore, functor methods like [`Task::map`], [`Element::map`], and [`Subscription::map`] make composition
//! seamless.
//!
//! </details>
#![doc(
    html_logo_url = "https://raw.githubusercontent.com/iced-rs/iced/bdf0430880f5c29443f5f0a0ae4895866dfef4c6/docs/logo.svg"
)]
#![cfg_attr(docsrs, feature(doc_cfg))]
use iced_widget::graphics;
use iced_widget::renderer;
use iced_winit as shell;
use iced_winit::core;
use iced_winit::program;
use iced_winit::runtime;

pub use iced_futures::futures;
pub use iced_futures::stream;

#[cfg(not(any(
    target_arch = "wasm32",
    feature = "thread-pool",
    feature = "tokio",
    feature = "smol"
)))]
compile_error!(
    "No futures executor has been enabled! You must enable an \
    executor feature.\n\
    Available options: thread-pool, tokio, or smol."
);

#[cfg(all(
    target_family = "unix",
    not(target_os = "macos"),
    not(feature = "wayland"),
    not(feature = "x11"),
))]
compile_error!(
    "No Unix display server backend has been enabled. You must enable a \
    display server feature.\n\
    Available options: x11, wayland."
);

#[cfg(feature = "highlighter")]
pub use iced_highlighter as highlighter;

#[cfg(feature = "wgpu")]
pub use iced_renderer::wgpu::wgpu;

mod error;

pub mod application;
pub mod daemon;
pub mod time;
pub mod window;

#[cfg(feature = "advanced")]
pub mod advanced;

pub use crate::core::alignment;
pub use crate::core::animation;
pub use crate::core::border;
pub use crate::core::color;
pub use crate::core::gradient;
pub use crate::core::padding;
pub use crate::core::theme;
pub use crate::core::{
    Alignment, Animation, Background, Border, Color, ContentFit, Degrees,
    Function, Gradient, Length, Never, Padding, Pixels, Point, Radians,
    Rectangle, Rotation, Settings, Shadow, Size, Theme, Transformation, Vector,
    never,
};
pub use crate::program::Preset;
pub use crate::program::message;
pub use crate::runtime::exit;
pub use iced_futures::Subscription;

pub use Alignment::Center;
pub use Length::{Fill, FillPortion, Shrink};
pub use alignment::Horizontal::{Left, Right};
pub use alignment::Vertical::{Bottom, Top};

pub mod debug {
    //! 调试你的应用程序。
    //!
    //! <details>
    //! <summary>显示英文原文</summary>
    //!
    //! Debug your applications.
    //!
    //! </details>
    pub use iced_debug::{Span, time, time_with};
}

pub mod task {
    //! 创建运行时任务。
    //!
    //! <details>
    //! <summary>显示英文原文</summary>
    //!
    //! Create runtime tasks.
    //!
    //! </details>
    pub use crate::runtime::task::{Handle, Task};

    #[cfg(feature = "sipper")]
    pub use crate::runtime::task::{Never, Sipper, Straw, sipper, stream};
}

pub mod clipboard {
    //! 访问剪贴板。
    //!
    //! <details>
    //! <summary>显示英文原文</summary>
    //!
    //! Access the clipboard.
    //!
    //! </details>
    pub use crate::runtime::clipboard::{
        read, read_primary, write, write_primary,
    };
}

pub mod executor {
    //! 选择你偏好的执行器来驱动应用程序。
    //!
    //! <details>
    //! <summary>显示英文原文</summary>
    //!
    //! Choose your preferred executor to power your application.
    //!
    //! </details>
    pub use iced_futures::Executor;
    pub use iced_futures::backend::default::Executor as Default;
}

pub mod font {
    //! 加载并使用字体。
    //!
    //! <details>
    //! <summary>显示英文原文</summary>
    //!
    //! Load and use fonts.
    //!
    //! </details>
    pub use crate::core::font::*;
    pub use crate::runtime::font::*;
}

pub mod event {
    //! 处理用户界面的事件。
    //!
    //! <details>
    //! <summary>显示英文原文</summary>
    //!
    //! Handle events of a user interface.
    //!
    //! </details>
    pub use crate::core::event::{Event, Status};
    pub use iced_futures::event::{listen, listen_raw, listen_with};
}

pub mod keyboard {
    //! 监听键盘事件并做出响应。
    //!
    //! <details>
    //! <summary>显示英文原文</summary>
    //!
    //! Listen and react to keyboard events.
    //!
    //! </details>
    pub use crate::core::keyboard::key;
    pub use crate::core::keyboard::{Event, Key, Location, Modifiers};
    pub use iced_futures::keyboard::listen;
}

pub mod mouse {
    //! 监听鼠标事件并做出响应。
    //!
    //! <details>
    //! <summary>显示英文原文</summary>
    //!
    //! Listen and react to mouse events.
    //!
    //! </details>
    pub use crate::core::mouse::{
        Button, Cursor, Event, Interaction, ScrollDelta,
    };
}

pub mod system {
    //! 获取系统信息。
    //!
    //! <details>
    //! <summary>显示英文原文</summary>
    //!
    //! Retrieve system information.
    //!
    //! </details>
    pub use crate::runtime::system::{theme, theme_changes};

    #[cfg(feature = "sysinfo")]
    pub use crate::runtime::system::{Information, information};
}

pub mod overlay {
    //! 在其他部件之上显示可交互的元素。
    //!
    //! <details>
    //! <summary>显示英文原文</summary>
    //!
    //! Display interactive elements on top of other widgets.
    //!
    //! </details>

    /// 一个通用的覆盖层。
    ///
    /// 它是使用默认 `Renderer` 的 [`overlay::Element`] 的别名。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A generic overlay.
    ///
    /// This is an alias of an [`overlay::Element`] with a default `Renderer`.
    ///
    /// </details>
    ///
    /// [`overlay::Element`]: crate::core::overlay::Element
    pub type Element<
        'a,
        Message,
        Theme = crate::Renderer,
        Renderer = crate::Renderer,
    > = crate::core::overlay::Element<'a, Message, Theme, Renderer>;

    pub use iced_widget::overlay::*;
}

pub mod touch {
    //! 监听触摸事件并做出响应。
    //!
    //! <details>
    //! <summary>显示英文原文</summary>
    //!
    //! Listen and react to touch events.
    //!
    //! </details>
    pub use crate::core::touch::{Event, Finger};
}

#[allow(hidden_glob_reexports)]
pub mod widget {
    //! 使用内置部件或创建你自己的部件。
    //!
    //! <details>
    //! <summary>显示英文原文</summary>
    //!
    //! Use the built-in widgets or create your own.
    //!
    //! </details>
    pub use iced_runtime::widget::*;
    pub use iced_widget::*;

    #[cfg(feature = "image")]
    pub mod image {
        //! 图像用于显示各种格式（PNG、JPG 等）的位图图形。
        //!
        //! <details>
        //! <summary>显示英文原文</summary>
        //!
        //! Images display raster graphics in different formats (PNG, JPG, etc.).
        //!
        //! </details>
        pub use iced_runtime::image::{Allocation, Error, allocate};
        pub use iced_widget::image::*;
    }

    // We hide the re-exported modules by `iced_widget`
    // 我们隐藏了由 `iced_widget` 重新导出的模块
    mod core {}
    mod graphics {}
    mod renderer {}
}

pub use application::Application;
pub use daemon::Daemon;
pub use error::Error;
pub use event::Event;
pub use executor::Executor;
pub use font::Font;
pub use program::Program;
pub use renderer::Renderer;
pub use task::Task;
pub use window::Window;

#[doc(inline)]
pub use application::application;
#[doc(inline)]
pub use daemon::daemon;

/// 一个通用的部件。
///
/// 它是使用默认 `Renderer` 的 `iced_native` 元素的别名。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A generic widget.
///
/// This is an alias of an `iced_native` element with a default `Renderer`.
///
/// </details>
pub type Element<
    'a,
    Message,
    Theme = crate::Theme,
    Renderer = crate::Renderer,
> = crate::core::Element<'a, Message, Theme, Renderer>;

/// 运行一个 iced 程序的结果。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The result of running an iced program.
///
/// </details>
pub type Result = std::result::Result<(), Error>;

/// 根据更新和视图逻辑，使用默认的 [`Settings`] 运行一个
/// 基本的 iced 应用程序。
///
/// 这等价于把 [`application()`] 与 [`Application::run`] 串联起来。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Runs a basic iced application with default [`Settings`] given its update
/// and view logic.
///
/// This is equivalent to chaining [`application()`] with [`Application::run`].
///
/// </details>
///
/// # 示例（Example）
/// ```no_run,standalone_crate
/// use iced::widget::{button, column, text, Column};
///
/// pub fn main() -> iced::Result {
///     iced::run(update, view)
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
pub fn run<State, Message, Theme, Renderer>(
    update: impl application::UpdateFn<State, Message> + 'static,
    view: impl for<'a> application::ViewFn<'a, State, Message, Theme, Renderer>
    + 'static,
) -> Result
where
    State: Default + 'static,
    Message: Send + message::MaybeDebug + message::MaybeClone + 'static,
    Theme: theme::Base + 'static,
    Renderer: program::Renderer + 'static,
{
    application(State::default, update, view).run()
}
