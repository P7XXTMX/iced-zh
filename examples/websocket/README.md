## Websocket

一个简单的示例，它与一个回显服务器保持 WebSocket 连接。

这个示例由 3 个模块组成：
- [`main`] 包含 `Application` 逻辑。
- [`echo`] 使用 `async-tungstenite` 实现了一个连接 [`echo::server`] 的 WebSocket 客户端。
- [`echo::server`] 使用 `warp` 实现了一个简单的 WebSocket 回显服务器。

你可以用 `cargo run` 运行它：
```
cargo run --package websocket
```

<details>
<summary>显示英文原文</summary>

## Websocket

A simple example that keeps a WebSocket connection open to an echo server.

The example consists of 3 modules:
- [`main`] contains the `Application` logic.
- [`echo`] implements a WebSocket client for the [`echo::server`] with `async-tungstenite`.
- [`echo::server`] implements a simple WebSocket echo server with `warp`.

You can run it with `cargo run`:
```
cargo run --package websocket
```

[`main`]: src/main.rs
[`echo`]: src/echo.rs
[`echo::server`]: src/echo/server.rs

</details>
