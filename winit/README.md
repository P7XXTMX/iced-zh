# `iced_winit`
[![Documentation](https://docs.rs/iced_winit/badge.svg)][documentation]
[![Crates.io](https://img.shields.io/crates/v/iced_winit.svg)](https://crates.io/crates/iced_winit)
[![License](https://img.shields.io/crates/l/iced_winit.svg)](https://github.com/iced-rs/iced/blob/master/LICENSE)
[![Discord Server](https://img.shields.io/discord/628993209984614400?label=&labelColor=6A7EC2&logo=discord&logoColor=ffffff&color=7389D8)](https://discord.gg/3xZJ65GAhd)

`iced_winit` 在 [`iced_native`] 之上提供了一些便捷的抽象，帮助你在使用 [`winit`] 时快速开始开发。

它公开了一个与渲染器无关的 `Application` trait，实现它之后只需一次简单的调用即可运行。这个 trait 的使用是可选的。对于决定自己实现事件循环的用户，还提供了一个 `conversion` 模块。

<p align="center">
  <img alt="原生目标" src="../docs/graphs/native.png" width="80%">
</p>

<details>
<summary>显示英文原文</summary>

# `iced_winit`
[![Documentation](https://docs.rs/iced_winit/badge.svg)][documentation]
[![Crates.io](https://img.shields.io/crates/v/iced_winit.svg)](https://crates.io/crates/iced_winit)
[![License](https://img.shields.io/crates/l/iced_winit.svg)](https://github.com/iced-rs/iced/blob/master/LICENSE)
[![Discord Server](https://img.shields.io/discord/628993209984614400?label=&labelColor=6A7EC2&logo=discord&logoColor=ffffff&color=7389D8)](https://discord.gg/3xZJ65GAhd)

`iced_winit` offers some convenient abstractions on top of [`iced_native`] to quickstart development when using [`winit`].

It exposes a renderer-agnostic `Application` trait that can be implemented and then run with a simple call. The use of this trait is optional. A `conversion` module is provided for users that decide to implement a custom event loop.

<p align="center">
  <img alt="The native target" src="../docs/graphs/native.png" width="80%">
</p>

[documentation]: https://docs.rs/iced_winit
[`iced_native`]: ../native
[`winit`]: https://github.com/rust-windowing/winit

</details>
