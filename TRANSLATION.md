# iced-zh 翻译说明

本项目 fork 自 [iced-rs/iced](https://github.com/iced-rs/iced)，把代码注释和文档翻译成了简体中文。
代码与上游一致，只有注释和 Markdown 文档不同。

- 基线提交：`e5c350f27`（iced `0.15.0-dev`）
- 翻译规范：[`CLAUDE.md`](CLAUDE.md)
- 术语表：[`rust-glossary.md`](rust-glossary.md)（Rust 术语）、[`zh-tools/iced-glossary.md`](zh-tools/iced-glossary.md)（iced 术语）
- 译文格式与翻译规则的细节：[`zh-tools/README.md`](zh-tools/README.md)

## 1. 双语格式

英文原文全部保留。

| 位置 | 效果 |
|---|---|
| 文档注释 `///`、`//!` | 中文在前，英文折叠在 `<details>`「显示英文原文」里 |
| 文档里的标题 | `# 示例（Example）` |
| 普通注释 `//` | 英文原样保留，下面加一行中文 |
| 行尾注释 | `// English（中文）` |
| 文档代码示例里的注释 | 英文下面加一行中文注释 |
| Markdown 文件 | 中文全文在前，英文全文折叠在文末 |

`# Safety`、`# Panics`、`# Errors` 三种标题保持英文，因为 clippy 靠标题原文识别这些小节。

## 2. 工作方式

源文件里的双语注释**不是手工编辑的**，而是由工具生成：

```
基线提交里的英文原文  +  zh-tools/translations/*.txt 里的译文  →  源文件
```

所以不要直接改 `.rs` 文件里的中文注释，下次生成会被覆盖。要改翻译，就改译文文件再重新生成。

目录结构：

```
zh-tools/
├── zh.mjs               Rust 注释的提取、生成、校验工具
├── md.mjs               Markdown 的生成、校验工具
├── translations/*.txt   Rust 注释的译文（按 crate 分文件）
├── markdown.zh.txt      Markdown 的译文
├── iced-glossary.md     iced 术语表
└── README.md            译文格式与翻译规则
```

需要 Node.js（开发时用的是 v20）和 Git。所有命令都在仓库根目录运行。

## 3. 修改已有的翻译

1. 在 `zh-tools/translations/` 里找到对应条目。条目以 `@@ 文件路径:行号` 开头，行号是基线提交里的行号。
   可以直接搜索中文原句：

   ```sh
   grep -rn "要找的中文" zh-tools/translations/
   ```

2. 修改条目下面的中文。
3. 重新生成并校验：

   ```sh
   node zh-tools/zh.mjs apply
   node zh-tools/zh.mjs check
   ```

Markdown 的做法相同：改 `zh-tools/markdown.zh.txt`（每个文件以 `@@@ 文件路径` 开头），然后：

```sh
node zh-tools/md.mjs apply
node zh-tools/md.mjs check
```

## 4. 翻译新的内容

1. 列出待翻译单元：

   ```sh
   node zh-tools/zh.mjs extract core/src/border.rs      # 一个文件
   node zh-tools/zh.mjs extract widget/src/pane_grid/   # 一个目录
   ```

   输出形如：

   ```
   @@ core/src/border.rs:78  [doc]  >> pub struct Radius {
   The border radii for the corners of a graphics primitive in the order:
   top-left, top-right, bottom-right, bottom-left.
   ```

   `[doc]` 是单元类型，`>>` 后面是注释所在的代码行，帮助理解上下文。

2. 在 `zh-tools/translations/` 下的某个 `.txt` 里写译文。头部只保留 `@@ 路径:行号`：

   ```
   @@ core/src/border.rs:78
   图元各个角的边框半径，顺序为：
   左上、右上、右下、左下。
   ```

   没有可译内容的单元（纯链接、被注释掉的代码等）写一行 `=` 表示跳过。

3. 生成并校验：

   ```sh
   node zh-tools/zh.mjs apply
   node zh-tools/zh.mjs check
   ```

翻译时最容易出错的是文档链接，规则见 [`zh-tools/README.md`](zh-tools/README.md)。要点：

- ``[`Task`]``、``[`Output`](Self::Output)`` 这类链接原样保留。
- 引用式链接要翻译链接文字时，写成 `[打包][bundled]`，不要写成 `[打包]`。

## 5. 校验

`check` 做三件事：确认源文件与生成结果一致、确认基线的每一行英文都还在、按 crate 输出覆盖率。

```sh
node zh-tools/zh.mjs check
```

```
crate          单元   已译   跳过   剩余
core           2113   2053     60      0
widget         2054   2050      4      0
...
校验通过：英文原文完整保留
```

改完翻译后，再跑一遍与上游 CI 相同的检查：

```sh
cargo fmt --all -- --check
cargo lint
cargo doc --no-deps --workspace --all-features
cargo test --doc --workspace --all-features
```

四项都应该零警告、零失败。常见问题：

| 现象 | 原因 |
|---|---|
| `unresolved link` | 译文里的文档链接被改动或漏了引用标签 |
| `unclosed HTML tag` | 译文里出现了没放进反引号的 `<T>` |
| `找不到对应的注释单元` | 译文条目的 `路径:行号` 写错了 |
| `不一致（需要重新 apply）` | 改了译文但没有重新 `apply`，或手工改了源文件 |

## 6. 生成中文 API 文档

```sh
cargo doc --no-deps --workspace --all-features --open
```

- 生成结果在 `target/doc/`，入口是 `target/doc/iced/index.html`。
- 去掉 `--open` 就只生成、不打开浏览器。
- 只生成某个 crate：`cargo doc --no-deps -p iced_widget --all-features --open`。
- 页面默认显示中文，点击「显示英文原文」展开英文，再点一次收起。

## 7. 不翻译的内容

- 代码、标识符、字符串字面量、文档里的代码示例本身
- `Cargo.toml`、`Cargo.lock`、CI 配置、`.wgsl` 着色器
- `CHANGELOG.md`、`.github/PULL_REQUEST_TEMPLATE.md`、`LICENSE`
- `widget/tests/fixtures/slow-incremental-markdown.md`（测试输入）
- `examples/markdown/overview.md`（被编译进示例程序）

## 8. 同步上游

仓库已添加 `upstream` 远程，指向 `iced-rs/iced`。

目前**没有自动迁移译文的工具**。译文条目用基线提交里的行号定位，工具里的基线写死为 `e5c350f27`
（`zh.mjs` 和 `md.mjs` 开头的 `BASE`）。上游改动注释后，对应条目的行号会变。
要跟进新的上游版本，需要：

1. 合并上游，把两个脚本里的 `BASE` 改成新的上游提交。
2. 运行 `node zh-tools/zh.mjs apply`，按报错修正行号变化的条目。
3. 用 `extract` 找出新增的注释并翻译。

合并时 `.rs` 和 Markdown 文件会有大量冲突，冲突的文件一律取上游版本，再用 `apply` 重新生成即可。
