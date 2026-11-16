## Todos（待办事项）

一个受 [TodoMVC] 启发的待办事项管理器。它展示了动态布局、文本输入、复选框、可滚动区域、图标和异步操作！它会在后台自动保存你的任务，即使你还没有输入完。

全部示例代码位于 __[`main`]__ 文件中。

<div align="center">
  <a href="https://iced.rs/examples/todos.mp4">
    <img src="https://iced.rs/examples/todos.gif">
  </a>
</div>

你可以用 `cargo run` 运行原生版本：
```
cargo run --package todos
```

Web 版本可以用 [`trunk`] 运行：

```
cd examples/todos
trunk serve
```

<details>
<summary>显示英文原文</summary>

## Todos

A todos tracker inspired by [TodoMVC]. It showcases dynamic layout, text input, checkboxes, scrollables, icons, and async actions! It automatically saves your tasks in the background, even if you did not finish typing them.

All the example code is located in the __[`main`]__ file.

<div align="center">
  <a href="https://iced.rs/examples/todos.mp4">
    <img src="https://iced.rs/examples/todos.gif">
  </a>
</div>

You can run the native version with `cargo run`:
```
cargo run --package todos
```

The web version can be run with [`trunk`]:

```
cd examples/todos
trunk serve
```

[`main`]: src/main.rs
[TodoMVC]: http://todomvc.com/
[`trunk`]: https://trunk-rs.github.io/trunk/

</details>
