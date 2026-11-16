## Pane grid（窗格网格）

一个由窗格组成的网格，窗格可以分割、调整大小和重新排列。

这个示例展示了 `PaneGrid` 部件，它的特性包括：

* 垂直和水平分割
* 跟踪最后一个活动窗格
* 用鼠标调整大小
* 通过拖放重新排列窗格
* 支持快捷键
* 可配置的修饰键
* 以编程方式执行操作的 API（`split`、`swap`、`resize` 等）

__[`main`]__ 文件包含了这个示例的全部代码。

<div align="center">
  <img src="https://iced.rs/examples/pane_grid.gif">
</div>

你可以用 `cargo run` 运行它：
```
cargo run --package pane_grid
```

<details>
<summary>显示英文原文</summary>

## Pane grid

A grid of panes that can be split, resized, and reorganized.

This example showcases the `PaneGrid` widget, which features:

* Vertical and horizontal splits
* Tracking of the last active pane
* Mouse-based resizing
* Drag and drop to reorganize panes
* Hotkey support
* Configurable modifier keys
* API to perform actions programmatically (`split`, `swap`, `resize`, etc.)

The __[`main`]__ file contains all the code of the example.

<div align="center">
  <img src="https://iced.rs/examples/pane_grid.gif">
</div>

You can run it with `cargo run`:
```
cargo run --package pane_grid
```

[`main`]: src/main.rs

</details>
