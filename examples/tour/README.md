## Tour（导览）

一个简单的 UI 导览，既可以在原生平台上运行，也可以在 Web 上运行！它展示了可以用 Iced 构建的各种部件。

__[`main`]__ 文件包含了这个示例的全部代码！所有跨平台 GUI 都是用__状态__、__消息__、__更新逻辑__和__视图逻辑__来定义的。

<div align="center">
  <a href="https://iced.rs/examples/tour.mp4">
    <img src="https://iced.rs/examples/tour.gif">
  </a>
</div>

你可以用 `cargo run` 运行原生版本：
```
cargo run --package tour
```

Web 版本可以用 [`trunk`] 运行：

```
cd examples/tour
trunk serve
```

<details>
<summary>显示英文原文</summary>

## Tour

A simple UI tour that can run both on native platforms and the web! It showcases different widgets that can be built using Iced.

The __[`main`]__ file contains all the code of the example! All the cross-platform GUI is defined in terms of __state__, __messages__, __update logic__ and __view logic__.

<div align="center">
  <a href="https://iced.rs/examples/tour.mp4">
    <img src="https://iced.rs/examples/tour.gif">
  </a>
</div>

[`main`]: src/main.rs
[`iced_winit`]: ../../winit
[`iced_native`]: ../../native
[`iced_wgpu`]: ../../wgpu
[`iced_web`]: https://github.com/iced-rs/iced_web
[`winit`]: https://github.com/rust-windowing/winit
[`wgpu`]: https://github.com/gfx-rs/wgpu-rs

You can run the native version with `cargo run`:
```
cargo run --package tour
```

The web version can be run with [`trunk`]:

```
cd examples/tour
trunk serve
```

[`trunk`]: https://trunk-rs.github.io/trunk/

</details>
