# 示例（Examples）
__Iced 发展很快，`master` 分支可能包含破坏性变更！__ 如果你想浏览与最新发布版本兼容的示例，
请[切换到 `latest` 分支](https://github.com/iced-rs/iced/tree/latest/examples#examples)。

## [Tour](tour)
一个简单的 UI 导览，既可以在原生平台上运行，也可以在 Web 上运行！它展示了可以用 Iced 构建的各种部件。

**[`main`](tour/src/main.rs)** 文件包含了这个示例的全部代码！所有跨平台 GUI 都是用**状态**、**消息**、**更新逻辑**和**视图逻辑**来定义的。

<div align="center">
  <a href="https://iced.rs/examples/tour.mp4">
    <img src="https://iced.rs/examples/tour.gif">
  </a>
</div>

你可以用 `cargo run` 运行原生版本：
```
cargo run --package tour
```

## [Todos](todos)
一个受 [TodoMVC] 启发的待办事项管理器。它展示了动态布局、文本输入、复选框、可滚动区域、图标和异步操作！它会在后台自动保存你的任务，即使你还没有输入完。

示例代码位于 __[`main`](todos/src/main.rs)__ 文件中。

<div align="center">
  <a href="https://iced.rs/examples/todos.mp4">
    <img src="https://iced.rs/examples/todos.gif" height="400px">
  </a>
</div>

你可以用 `cargo run` 运行原生版本：
```
cargo run --package todos
```

## [Game of Life](game_of_life)
[生命游戏][Game of Life]的交互式版本，生命游戏由 [John Horton Conway] 发明。

它在后台线程中运行模拟，同时允许与一个 `Canvas` 交互；这个 `Canvas` 显示一个无限网格，支持缩放、平移和绘制。

相关代码位于 __[`main`](game_of_life/src/main.rs)__ 文件中。

<div align="center">
  <img src="https://iced.rs/examples/game_of_life.gif">
</div>

你可以用 `cargo run` 运行它：
```
cargo run --package game_of_life
```

## [Styling](styling)
一个展示自定义样式的示例，带有亮色和暗色主题。

示例代码位于 __[`main`](styling/src/main.rs)__ 文件中。

<div align="center">
  <img src="https://iced.rs/examples/styling.gif">
</div>

你可以用 `cargo run` 运行它：
```
cargo run --package styling
```

## 其他示例（Extras）
还有许多更简单的示例：

- [`bezier_tool`](bezier_tool)：一个类似画图的工具，使用 `Canvas` 部件绘制贝塞尔曲线。
- [`clock`](clock)：一个使用 `Canvas` 部件绘制时钟及其指针来显示当前时间的应用程序。
- [`color_palette`](color_palette)：一个基于用户自定义根颜色的调色板生成器。
- [`counter`](counter)：[`README`](../README.md) 中讲解的经典计数器示例。
- [`custom_widget`](custom_widget)：演示如何构建一个绘制圆形的自定义部件。
- [`download_progress`](download_progress)：一个基本的应用程序，异步下载一个 100 MB 的虚拟文件并跟踪下载进度。
- [`events`](events)：使用条件 `Subscription` 显示的原生事件日志。
- [`geometry`](geometry)：一个自定义部件，展示如何用 [`iced_wgpu`](../wgpu) 中的 `Mesh2D` 图元绘制几何图形。
- [`integration`](integration)：演示如何把 Iced 集成到一个已有的 [`wgpu`] 应用程序中。
- [`pane_grid`](pane_grid)：一个由窗格组成的网格，窗格可以分割、调整大小和重新排列。
- [`pick_list`](pick_list)：一个由可选选项组成的下拉列表。
- [`pokedex`](pokedex)：一个使用 [PokéAPI] 显示随机宝可梦图鉴条目（包括精灵图！）的应用程序。
- [`progress_bar`](progress_bar)：一个简单的进度条，可以用滑块来填充。
- [`scrollable`](scrollable)：展示各种可滚动内容的配置。
- [`sierpinski_triangle`](sierpinski_triangle)：一个[谢尔宾斯基三角形](https://en.wikipedia.org/wiki/Sierpi%C5%84ski_triangle)模拟器，使用 `Canvas` 和 `Slider`。
- [`solar_system`](solar_system)：一个用 `Canvas` 部件绘制的动画太阳系，展示如何组合不同的变换。
- [`stopwatch`](stopwatch)：一个带有开始/停止和重置按钮的秒表，展示如何监听时间。
- [`svg`](svg)：一个利用 `Svg` 部件渲染 [Ghostscript Tiger] 的应用程序。

它们都被打包在各自的 crate 中，因此可以用 `cargo` 运行：
```
cargo run --package <example>
```

## [Coffee]
自从 [Iced 在 2019 年 5 月诞生][Iced was born in May 2019]以来，它一直为
实验性 2D 游戏引擎 [Coffee] 提供用户界面。

<div align="center">
  <img src="https://iced.rs/examples/coffee.gif">
</div>

<details>
<summary>显示英文原文</summary>

# Examples
__Iced moves fast and the `master` branch can contain breaking changes!__ If you want to browse examples that are compatible with the latest release,
then [switch to the `latest` branch](https://github.com/iced-rs/iced/tree/latest/examples#examples).

## [Tour](tour)
A simple UI tour that can run both on native platforms and the web! It showcases different widgets that can be built using Iced.

The __[`main`](tour/src/main.rs)__ file contains all the code of the example! All the cross-platform GUI is defined in terms of __state__, __messages__, __update logic__ and __view logic__.

<div align="center">
  <a href="https://iced.rs/examples/tour.mp4">
    <img src="https://iced.rs/examples/tour.gif">
  </a>
</div>

[`iced_winit`]: ../winit
[`iced_native`]: ../native
[`iced_wgpu`]: ../wgpu
[`iced_web`]: https://github.com/iced-rs/iced_web
[`winit`]: https://github.com/rust-windowing/winit
[`wgpu`]: https://github.com/gfx-rs/wgpu

You can run the native version with `cargo run`:
```
cargo run --package tour
```

## [Todos](todos)
A todos tracker inspired by [TodoMVC]. It showcases dynamic layout, text input, checkboxes, scrollables, icons, and async actions! It automatically saves your tasks in the background, even if you did not finish typing them.

The example code is located in the __[`main`](todos/src/main.rs)__ file.

<div align="center">
  <a href="https://iced.rs/examples/todos.mp4">
    <img src="https://iced.rs/examples/todos.gif" height="400px">
  </a>
</div>

You can run the native version with `cargo run`:
```
cargo run --package todos
```

[TodoMVC]: http://todomvc.com/

## [Game of Life](game_of_life)
An interactive version of the [Game of Life], invented by [John Horton Conway].

It runs a simulation in a background thread while allowing interaction with a `Canvas` that displays an infinite grid with zooming, panning, and drawing support.

The relevant code is located in the __[`main`](game_of_life/src/main.rs)__ file.

<div align="center">
  <img src="https://iced.rs/examples/game_of_life.gif">
</div>

You can run it with `cargo run`:
```
cargo run --package game_of_life
```

[Game of Life]: https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life
[John Horton Conway]: https://en.wikipedia.org/wiki/John_Horton_Conway

## [Styling](styling)
An example showcasing custom styling with a light and dark theme.

The example code is located in the __[`main`](styling/src/main.rs)__ file.

<div align="center">
  <img src="https://iced.rs/examples/styling.gif">
</div>

You can run it with `cargo run`:
```
cargo run --package styling
```

## Extras
A bunch of simpler examples exist:

- [`bezier_tool`](bezier_tool), a Paint-like tool for drawing Bézier curves using the `Canvas` widget.
- [`clock`](clock), an application that uses the `Canvas` widget to draw a clock and its hands to display the current time.
- [`color_palette`](color_palette), a color palette generator based on a user-defined root color.
- [`counter`](counter), the classic counter example explained in the [`README`](../README.md).
- [`custom_widget`](custom_widget), a demonstration of how to build a custom widget that draws a circle.
- [`download_progress`](download_progress), a basic application that asynchronously downloads a dummy file of 100 MB and tracks the download progress.
- [`events`](events), a log of native events displayed using a conditional `Subscription`.
- [`geometry`](geometry), a custom widget showcasing how to draw geometry with the `Mesh2D` primitive in [`iced_wgpu`](../wgpu).
- [`integration`](integration), a demonstration of how to integrate Iced in an existing [`wgpu`] application.
- [`pane_grid`](pane_grid), a grid of panes that can be split, resized, and reorganized.
- [`pick_list`](pick_list), a dropdown list of selectable options.
- [`pokedex`](pokedex), an application that displays a random Pokédex entry (sprite included!) by using the [PokéAPI].
- [`progress_bar`](progress_bar), a simple progress bar that can be filled by using a slider.
- [`scrollable`](scrollable), a showcase of various scrollable content configurations.
- [`sierpinski_triangle`](sierpinski_triangle), a [sierpiński triangle](https://en.wikipedia.org/wiki/Sierpi%C5%84ski_triangle) Emulator, use `Canvas` and `Slider`.
- [`solar_system`](solar_system), an animated solar system drawn using the `Canvas` widget and showcasing how to compose different transforms.
- [`stopwatch`](stopwatch), a watch with start/stop and reset buttons showcasing how to listen to time.
- [`svg`](svg), an application that renders the [Ghostscript Tiger] by leveraging the `Svg` widget.

All of them are packaged in their own crate and, therefore, can be run using `cargo`:
```
cargo run --package <example>
```

[`lyon`]: https://github.com/nical/lyon
[PokéAPI]: https://pokeapi.co/
[Ghostscript Tiger]: https://commons.wikimedia.org/wiki/File:Ghostscript_Tiger.svg
[`wgpu`]: https://github.com/gfx-rs/wgpu

## [Coffee]
Since [Iced was born in May 2019], it has been powering the user interfaces in
[Coffee], an experimental 2D game engine.


<div align="center">
  <img src="https://iced.rs/examples/coffee.gif">
</div>

[Iced was born in May 2019]: https://github.com/hecrj/coffee/pull/35
[`ui` module]: https://docs.rs/coffee/0.3.2/coffee/ui/index.html
[Coffee]: https://github.com/hecrj/coffee

</details>
