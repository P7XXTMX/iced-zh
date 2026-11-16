<div align="center">

<img src="docs/logo.svg" width="140px" />

# Iced

[![Documentation](https://docs.rs/iced/badge.svg)][documentation]
[![Crates.io](https://img.shields.io/crates/v/iced.svg)](https://crates.io/crates/iced)
[![License](https://img.shields.io/crates/l/iced.svg)](https://github.com/iced-rs/iced/blob/master/LICENSE)
[![Downloads](https://img.shields.io/crates/d/iced.svg)](https://crates.io/crates/iced)
[![Test Status](https://img.shields.io/github/actions/workflow/status/iced-rs/iced/test.yml?branch=master&event=push&label=test)](https://github.com/iced-rs/iced/actions)
[![Zulip Chat](https://img.shields.io/badge/chat-on%20Zulip-5e7ce2?logo=zulip&logoColor=white)](https://iced.zulipchat.com/)
[![Discord Server](https://img.shields.io/discord/628993209984614400?label=&labelColor=6A7EC2&logo=discord&logoColor=ffffff&color=7389D8)](https://discord.gg/3xZJ65GAhd)

一个专注于简洁和类型安全的 Rust 跨平台 GUI 库。
灵感来自 [Elm]。

<a href="https://github.com/squidowl/halloy">
  <img src="https://iced.rs/showcase/halloy.gif" width="460px">
</a>
<a href="https://github.com/hecrj/icebreaker">
  <img src="https://iced.rs/showcase/icebreaker.gif" width="360px">
</a>

</div>

> 本项目 fork 自 [iced](https://github.com/iced-rs/iced)，将注释和文档翻译成了中文。使用方法见[翻译说明](TRANSLATION.md)。

## 特性

* 简单易用、开箱即用的 API
* 类型安全的响应式编程模型
* [跨平台支持][Cross-platform support]（Windows、macOS、Linux 和 Web）
* 响应式布局
* 内置部件（包括[文本输入框][text inputs]、[可滚动区域][scrollables]等等！）
* 支持自定义部件（创建你自己的部件！）
* [带有性能指标和时间旅行的调试工具][Debug tooling with performance metrics and time traveling]
* 对异步操作的一等支持（使用 future！）
* 模块化的生态系统，拆分为多个可复用的部分：
  * 一个[与渲染器无关的原生运行时][renderer-agnostic native runtime]，可以集成到已有的系统中
  * 两个内置渲染器，分别基于 [`wgpu`] 和 [`tiny-skia`]
    * [`iced_wgpu`] 支持 Vulkan、Metal 和 DX12
    * [`iced_tiny_skia`] 提供软件渲染作为回退方案
  * 一个[窗口外壳][windowing shell]

__Iced 目前是实验性软件。__ 可以[看看路线图][Take a look at the roadmap]，
也可以[查看 issue 列表][check out the issues]。

## 概览

受 [Elm 架构][The Elm Architecture]的启发，Iced 期望你把用户界面
拆分为四个不同的概念：

* __状态（State）__ —— 应用程序的状态
* __消息（Messages）__ —— 你关心的用户交互或有意义的事件
* __视图逻辑（View logic）__ —— 把__状态__显示为部件的方式，
  这些部件在用户交互时可能产生__消息__
* __更新逻辑（Update logic）__ —— 响应__消息__并更新__状态__的方式

我们可以动手做个东西来看看它是如何工作的！假设我们想要一个简单的计数器，
可以用两个按钮来增加和减少。

我们先为应用程序的__状态__建模：

```rust
#[derive(Default)]
struct Counter {
    value: i32,
}
```

接下来，我们需要定义计数器可能的用户交互：
按钮的按下。这些交互就是我们的__消息__：

```rust
#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}
```

现在，让我们在__视图逻辑__中把这些组合起来，
显示出真正的计数器：

```rust
use iced::widget::{button, column, text, Widget};

impl Counter {
    pub fn view(&self) -> impl Widget<Message> {
        // 我们使用一个列：一种简单的垂直布局
        column![
            // 增加按钮。我们让它在被按下时产生一条
            // `Increment` 消息
            button("+").on_press(Message::Increment),

            // 我们在这里显示计数器的值
            text(self.value).size(50),

            // 减少按钮。我们让它在被按下时产生一条
            // `Decrement` 消息
            button("-").on_press(Message::Decrement),
        ]
    }
}
```

最后，我们需要能够在__更新逻辑__中响应产生的__消息__，
并相应地改变__状态__：

```rust
impl Counter {
    // ...

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }
}
```

这就是全部了！我们刚刚写出了一个完整的用户界面。让我们运行它：

```rust
fn main() -> iced::Result {
    iced::run(Counter::update, Counter::view)
}
```

Iced 会自动：

  1. 取得__视图逻辑__的结果，并对其中的部件进行布局。
  1. 处理来自系统的事件，并为__更新逻辑__产生__消息__。
  1. 绘制得到的用户界面。

阅读[这本书][book]、[文档][documentation]和[示例][examples]以了解更多！

## 实现细节

Iced 最初诞生于一次尝试：把 [Elm] 和 [Elm 架构][The Elm Architecture]的简洁
带入 [Coffee]——我正在开发的一个 2D 游戏库。

这个库的核心是在 2019 年 5 月通过[这个 pull request][this pull request] 实现的。
[第一个 alpha 版本][The first alpha version]最终作为
[一个与渲染器无关的 GUI 库][a renderer-agnostic GUI library]发布。当时这个库不提供渲染器，
并在游戏库 [`ggez`] 之上实现了现在的 [tour 示例][tour example]。

从那以后，重心转向提供一个开箱即用、面向最终用户的 GUI 库，
同时保持生态系统的模块化。

## 贡献 / 反馈

如果你想参与贡献，请阅读我们的[贡献指南][contributing guidelines]了解详情。

也欢迎反馈！你可以在[我们的 Zulip 论坛][our Zulip forum]发起新话题，
或者来[我们的 Discord 服务器][our Discord server]聊天。

## 赞助者

Iced 的开发由 [Kraken.com] 的 [Cryptowatch] 团队赞助

<details>
<summary>显示英文原文</summary>

<div align="center">

<img src="docs/logo.svg" width="140px" />

# Iced

[![Documentation](https://docs.rs/iced/badge.svg)][documentation]
[![Crates.io](https://img.shields.io/crates/v/iced.svg)](https://crates.io/crates/iced)
[![License](https://img.shields.io/crates/l/iced.svg)](https://github.com/iced-rs/iced/blob/master/LICENSE)
[![Downloads](https://img.shields.io/crates/d/iced.svg)](https://crates.io/crates/iced)
[![Test Status](https://img.shields.io/github/actions/workflow/status/iced-rs/iced/test.yml?branch=master&event=push&label=test)](https://github.com/iced-rs/iced/actions)
[![Zulip Chat](https://img.shields.io/badge/chat-on%20Zulip-5e7ce2?logo=zulip&logoColor=white)](https://iced.zulipchat.com/)
[![Discord Server](https://img.shields.io/discord/628993209984614400?label=&labelColor=6A7EC2&logo=discord&logoColor=ffffff&color=7389D8)](https://discord.gg/3xZJ65GAhd)

A cross-platform GUI library for Rust focused on simplicity and type-safety.
Inspired by [Elm].

<a href="https://github.com/squidowl/halloy">
  <img src="https://iced.rs/showcase/halloy.gif" width="460px">
</a>
<a href="https://github.com/hecrj/icebreaker">
  <img src="https://iced.rs/showcase/icebreaker.gif" width="360px">
</a>

</div>

## Features

* Simple, easy-to-use, batteries-included API
* Type-safe, reactive programming model
* [Cross-platform support] (Windows, macOS, Linux, and the Web)
* Responsive layout
* Built-in widgets (including [text inputs], [scrollables], and more!)
* Custom widget support (create your own!)
* [Debug tooling with performance metrics and time traveling]
* First-class support for async actions (use futures!)
* Modular ecosystem split into reusable parts:
  * A [renderer-agnostic native runtime] enabling integration with existing systems
  * Two built-in renderers leveraging [`wgpu`] and [`tiny-skia`]
    * [`iced_wgpu`] supporting Vulkan, Metal and DX12
    * [`iced_tiny_skia`] offering a software alternative as a fallback
  * A [windowing shell]

__Iced is currently experimental software.__ [Take a look at the roadmap] and
[check out the issues].

[Cross-platform support]: https://raw.githubusercontent.com/iced-rs/iced/master/docs/images/todos_desktop.jpg
[text inputs]: https://iced.rs/examples/text_input.mp4
[scrollables]: https://iced.rs/examples/scrollable.mp4
[Debug tooling with performance metrics and time traveling]: https://github.com/user-attachments/assets/2e49695c-0261-4b43-ac2e-8d7da5454c4b
[renderer-agnostic native runtime]: runtime/
[`wgpu`]: https://github.com/gfx-rs/wgpu
[`tiny-skia`]: https://github.com/RazrFalcon/tiny-skia
[`iced_wgpu`]: wgpu/
[`iced_tiny_skia`]: tiny_skia/
[windowing shell]: winit/
[Take a look at the roadmap]: ROADMAP.md
[check out the issues]: https://github.com/iced-rs/iced/issues

## Overview

Inspired by [The Elm Architecture], Iced expects you to split user interfaces
into four different concepts:

* __State__ — the state of your application
* __Messages__ — user interactions or meaningful events that you care
  about
* __View logic__ — a way to display your __state__ as widgets that
  may produce __messages__ on user interaction
* __Update logic__ — a way to react to __messages__ and update your
  __state__

We can build something to see how this works! Let's say we want a simple counter
that can be incremented and decremented using two buttons.

We start by modelling the __state__ of our application:

```rust
#[derive(Default)]
struct Counter {
    value: i32,
}
```

Next, we need to define the possible user interactions of our counter:
the button presses. These interactions are our __messages__:

```rust
#[derive(Debug, Clone, Copy)]
pub enum Message {
    Increment,
    Decrement,
}
```

Now, let's show the actual counter by putting it all together in our
__view logic__:

```rust
use iced::widget::{button, column, text, Widget};

impl Counter {
    pub fn view(&self) -> impl Widget<Message> {
        // We use a column: a simple vertical layout
        column![
            // The increment button. We tell it to produce an
            // `Increment` message when pressed
            button("+").on_press(Message::Increment),

            // We show the value of the counter here
            text(self.value).size(50),

            // The decrement button. We tell it to produce a
            // `Decrement` message when pressed
            button("-").on_press(Message::Decrement),
        ]
    }
}
```

Finally, we need to be able to react to any produced __messages__ and change our
__state__ accordingly in our __update logic__:

```rust
impl Counter {
    // ...

    pub fn update(&mut self, message: Message) {
        match message {
            Message::Increment => {
                self.value += 1;
            }
            Message::Decrement => {
                self.value -= 1;
            }
        }
    }
}
```

And that's everything! We just wrote a whole user interface. Let's run it:

```rust
fn main() -> iced::Result {
    iced::run(Counter::update, Counter::view)
}
```

Iced will automatically:

  1. Take the result of our __view logic__ and layout its widgets.
  1. Process events from our system and produce __messages__ for our
     __update logic__.
  1. Draw the resulting user interface.

Read the [book], the [documentation], and the [examples] to learn more!

## Implementation details

Iced was originally born as an attempt at bringing the simplicity of [Elm] and
[The Elm Architecture] into [Coffee], a 2D game library I am working on.

The core of the library was implemented during May 2019 in [this pull request].
[The first alpha version] was eventually released as
[a renderer-agnostic GUI library]. The library did not provide a renderer and
implemented the current [tour example] on top of [`ggez`], a game library.

Since then, the focus has shifted towards providing a batteries-included,
end-user-oriented GUI library, while keeping the ecosystem modular.

[this pull request]: https://github.com/hecrj/coffee/pull/35
[The first alpha version]: https://github.com/iced-rs/iced/tree/0.1.0-alpha
[a renderer-agnostic GUI library]: https://www.reddit.com/r/rust/comments/czzjnv/iced_a_rendereragnostic_gui_library_focused_on/
[tour example]: examples/README.md#tour
[`ggez`]: https://github.com/ggez/ggez

## Contributing / Feedback

If you want to contribute, please read our [contributing guidelines] for more details.

Feedback is also welcome! You can create a new topic in [our Zulip forum] or
come chat to [our Discord server].

## Sponsors

The development of Iced is sponsored by the [Cryptowatch] team at [Kraken.com]

[book]: https://book.iced.rs/
[documentation]: https://docs.rs/iced/
[examples]: https://github.com/iced-rs/iced/tree/master/examples#examples
[Coffee]: https://github.com/hecrj/coffee
[Elm]: https://elm-lang.org/
[The Elm Architecture]: https://guide.elm-lang.org/architecture/
[the current issues]: https://github.com/iced-rs/iced/issues
[contributing guidelines]: https://github.com/iced-rs/iced/blob/master/CONTRIBUTING.md
[our Zulip forum]: https://iced.zulipchat.com/
[our Discord server]: https://discord.gg/3xZJ65GAhd
[Cryptowatch]: https://cryptowat.ch/charts
[Kraken.com]: https://kraken.com/

</details>
