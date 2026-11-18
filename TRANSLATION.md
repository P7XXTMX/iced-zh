# iced-zh 翻译说明

本项目 fork 自 [iced-rs/iced](https://github.com/iced-rs/iced)，把代码注释和文档翻译成了简体中文。
代码与上游一致，只有注释和 Markdown 文档不同。

- 基线提交：记录在 [`zh-tools/BASE`](zh-tools/BASE) 里，是当前译文所对应的上游提交
- 翻译规范：[`CLAUDE.md`](CLAUDE.md)
- 术语表：[`rust-glossary.md`](rust-glossary.md)（Rust 术语）、[`zh-tools/iced-glossary.md`](zh-tools/iced-glossary.md)（iced 术语）
- 译文格式与翻译规则的细节：[`zh-tools/README.md`](zh-tools/README.md)

## 1. 双语格式

英文原文全部保留。

| 位置 | 效果 |
|---|---|
| 文档注释 `///`、`//!` | 中文在前，英文折叠在 `<details>`「显示英文原文」里 |
| 文档里的标题 | `# 示例（Example）` |
| 普通注释 `//`、独占整行的 `/* */` | 英文原样保留，下面加一行中文 |
| 行尾注释（包括文档代码示例里的） | `// English（中文）` |
| 文档代码示例里的整行注释 | 英文下面加一行中文注释 |
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
├── zh.mjs               Rust 注释的提取、生成、校验、迁移工具
├── md.mjs               Markdown 的生成、校验工具
├── BASE                 基线：译文所对应的上游提交（只由 migrate 修改）
├── translations/*.txt   Rust 注释的译文（按 crate 分文件）
├── markdown.zh.txt      Markdown 的译文
├── iced-glossary.md     iced 术语表
└── README.md            译文格式与翻译规则
```

需要 Node.js（开发时用的是 v20）和 Git。所有命令都在仓库根目录运行。

命令一览：

| 命令 | 作用 |
|---|---|
| `node zh-tools/zh.mjs extract [路径...]` | 列出注释单元（英文原文） |
| `node zh-tools/zh.mjs todo [路径...]` | 只列出还没有译文、或译文已过期的单元 |
| `node zh-tools/zh.mjs apply` | 按译文重新生成全部 Rust 文件 |
| `node zh-tools/zh.mjs check` | 校验，并按 crate 统计覆盖率 |
| `node zh-tools/zh.mjs migrate [--dry-run] <提交>` | 把基线换成新的上游提交，并迁移译文 |
| `node zh-tools/md.mjs apply` / `check` | 生成、校验 Markdown 文件 |

## 3. 修改已有的翻译

1. 在 `zh-tools/translations/` 里找到对应条目。条目以 `@@ 文件路径:行号` 开头，行号是基线提交里的行号
   （不是工作区文件里的行号）。可以直接搜索中文原句：

   ```sh
   grep -rn "要找的中文" zh-tools/translations/
   ```

2. 修改条目下面的中文。
3. 重新生成并校验：

   ```sh
   node zh-tools/zh.mjs apply
   node zh-tools/zh.mjs check
   ```

Markdown 的做法相同：改 `zh-tools/markdown.zh.txt`（每个文件以 `@@@ 文件路径 原文哈希` 开头），然后：

```sh
node zh-tools/md.mjs apply
node zh-tools/md.mjs check
```

## 4. 翻译新的内容

1. 列出待翻译的单元：

   ```sh
   node zh-tools/zh.mjs todo                         # 全部
   node zh-tools/zh.mjs todo widget/src/pane_grid/   # 只看一个目录
   ```

   输出形如：

   ```
   @@ core/src/border.rs:78  [doc]  >> pub struct Radius {
   The border radii for the corners of a graphics primitive in the order:
   top-left, top-right, bottom-right, bottom-left.
   ```

   `[doc]` 是单元类型，`>>` 后面是注释所在的代码行，帮助理解上下文。
   `extract` 的输出格式相同，但会列出所有单元，包括已经翻译的。

2. 在 `zh-tools/translations/` 下的某个 `.txt` 里写译文。头部只保留 `@@ 路径:行号`：

   ```
   @@ core/src/border.rs:78
   图元各个角的边框半径，顺序为：
   左上、右上、右下、左下。
   ```

   没有可译内容的单元（纯链接、被注释掉的代码等）写一行 `=` 表示跳过。
   条目放在哪个文件、什么位置都可以，习惯上和同一源文件的其他条目放在一起。

3. 生成并校验：

   ```sh
   node zh-tools/zh.mjs apply
   node zh-tools/zh.mjs check
   ```

翻译时最容易出错的是文档链接，规则见 [`zh-tools/README.md`](zh-tools/README.md)。要点：

- ``[`Task`]``、``[`Output`](Self::Output)`` 这类链接原样保留。
- 引用式链接要翻译链接文字时，写成 `[打包][bundled]`，不要写成 `[打包]`。

## 5. 校验

```sh
node zh-tools/zh.mjs check
node zh-tools/md.mjs check
```

`zh.mjs check` 检查：

- 每个 Rust 文件都等于「基线 + 译文」的生成结果；
- 基线的每一行英文都还在；
- 没有译文的 Rust 文件、以及其他所有文件（`Cargo.toml`、着色器、CI 配置等）与上游基线完全一致；
- 按 crate 输出覆盖率，并列出含英文、但工具不处理的注释（如果有）。

```
基线：e5c350f27
crate          单元   已译   跳过   剩余
core           2117   2057     60      0
widget         2055   2051      4      0
...
校验通过：源文件与生成结果一致，英文原文完整保留，其余文件与上游一致
```

`md.mjs check` 检查 Markdown 文件与生成结果一致，并指出原文已更新、译文需要同步的文件，以及还没有翻译的 Markdown 文件。

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
| `找不到对应的注释单元` | 译文条目的 `路径:行号` 写错了（行号是基线里的行号，用 `extract` 查看） |
| `不一致（需要重新 apply）` | 改了译文但没有重新 `apply`，或手工改了源文件 |
| `与基线不同（这个文件没有译文…）` | 手工改了没有译文的文件；运行 `apply` 会把它还原为上游内容 |

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
- 夹在代码中间的 `/* */` 注释和 `/** */` 文档块注释（工具不处理，`check` 会列出来；目前上游没有这类含英文的注释）

## 8. 同步上游

仓库已添加 `upstream` 远程，指向 `iced-rs/iced`。本地 `master` 分支保持与上游一致，翻译在 `zh-translation` 分支上。

在 `zh-translation` 分支、工作区干净的状态下：

```sh
# 1. 拉取上游，先预览（不修改任何文件）
git fetch upstream
node zh-tools/zh.mjs migrate --dry-run upstream/master

# 2. 合并上游。先不提交；.rs 和 Markdown 文件有冲突是正常的，不需要手工解决
git merge --no-commit upstream/master

# 3. 把基线换成新的上游提交，译文条目的行号和路径会自动更新
node zh-tools/zh.mjs migrate upstream/master

# 4. 重新生成双语文件（冲突的文件会被整个覆盖成正确的内容）
node zh-tools/zh.mjs apply
node zh-tools/md.mjs apply
git add -A

# 5. 校验
node zh-tools/zh.mjs check
node zh-tools/md.mjs check
```

`migrate` 按英文内容把旧注释和新注释对上号，并报告四种情况：

| 情况 | 处理 |
|---|---|
| 原文没变，只是换了行号、文件名或位置 | 自动更新条目的 `路径:行号`，译文不动 |
| 原文有改动 | 旧译文留在原处，前面加上以 `!!` 开头的行（内容是旧原文），标记为过期 |
| 原文被删除 | 译文条目被移除，旧译文打印在报告里 |
| 新增的注释 | 列在报告里，等待翻译 |

过期的条目按“未翻译”处理：生成时只输出英文，`todo` 会把新原文、旧原文和旧译文一起列出来。
对照着更新译文之后，删掉所有以 `!!` 开头的行。

第 5 步的 `check` 还可能提示：

- **`基线里没有这个文件：…`**：上游删除了这个文件，但合并时它因为冲突被留在了工作区。运行 `git rm <文件>`。
- **`合并冲突还没有标记为已解决：…`**：还没有 `git add`。
- **`英文原文已更新，译文需要同步：xxx.md`**：上游改了这个 Markdown 文件。这时文件顶部会自动加一行提示，
  英文折叠区已经是新内容。按提示的 `git diff` 命令查看原文改了什么，更新 `markdown.zh.txt` 里的译文，
  再把标题行里的哈希改成提示的新值。
- **`还没有翻译的 Markdown 文件：…`**：上游新增了 Markdown 文件。

最后翻译新增和过期的内容（`node zh-tools/zh.mjs todo`），跑第 5 节的 cargo 检查，然后提交这次合并：

```sh
git commit
git fetch . upstream/master:master    # 可选：把本地 master 也快进到上游
```

如果上游没有改动任何注释，第 3 步的报告里后三项都是 0，不需要翻译，直接校验、提交即可。
