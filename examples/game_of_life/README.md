## Game of Life（生命游戏）

[生命游戏][Game of Life]的交互式版本，生命游戏由 [John Horton Conway] 发明。

它在后台线程中运行模拟，同时允许与一个 `Canvas` 交互；这个 `Canvas` 显示一个无限网格，支持缩放、平移和绘制。

__[`main`]__ 文件包含了这个示例的相关代码。

<div align="center">
  <img src="https://iced.rs/examples/game_of_life.gif">
</div>

你可以用 `cargo run` 运行它：
```
cargo run --package game_of_life
```

<details>
<summary>显示英文原文</summary>

## Game of Life

An interactive version of the [Game of Life], invented by [John Horton Conway].

It runs a simulation in a background thread while allowing interaction with a `Canvas` that displays an infinite grid with zooming, panning, and drawing support.

The __[`main`]__ file contains the relevant code of the example.

<div align="center">
  <img src="https://iced.rs/examples/game_of_life.gif">
</div>

You can run it with `cargo run`:
```
cargo run --package game_of_life
```

[`main`]: src/main.rs
[Game of Life]: https://en.wikipedia.org/wiki/Conway%27s_Game_of_Life
[John Horton Conway]: https://en.wikipedia.org/wiki/John_Horton_Conway

</details>
