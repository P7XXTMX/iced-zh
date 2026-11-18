## Download progress（下载进度）

一个基本的应用程序，异步下载多个 100 MB 的虚拟文件并跟踪下载进度。

这个示例在 __[`download`](src/download.rs)__ 模块中实现了一个自定义的 `Subscription`。这个订阅执行下载，并产生可用于跟踪其进度的消息。

<div align="center">
  <img src="https://iced.rs/examples/download_progress.gif">
</div>

你可以用 `cargo run` 运行它：

```
cargo run --package download_progress
```

<details>
<summary>显示英文原文</summary>

## Download progress

A basic application that asynchronously downloads multiple dummy files of 100 MB and tracks the download progress.

The example implements a custom `Subscription` in the __[`download`](src/download.rs)__ module. This subscription downloads and produces messages that can be used to keep track of its progress.

<div align="center">
  <img src="https://iced.rs/examples/download_progress.gif">
</div>

You can run it with `cargo run`:

```
cargo run --package download_progress
```

</details>
