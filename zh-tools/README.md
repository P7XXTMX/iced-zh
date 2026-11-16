# zh-tools：iced 中英双语注释工具

源文件里的双语注释不是手工编辑的，而是由 `zh.mjs` 根据「基线提交的英文原文 + `translations/*.txt` 里的译文」生成的。
这样英文原文不可能被改动或丢失，代码也不可能被误改。

## 命令

```sh
node zh-tools/zh.mjs extract <路径前缀...>   # 列出待翻译单元（英文原文）
node zh-tools/zh.mjs apply [译文文件...]     # 按译文重写源文件（不带参数 = 全部译文）
node zh-tools/zh.mjs check [译文文件...]     # 校验英文完整保留 + 覆盖率统计
```

## 译文文件格式

`extract` 的输出形如：

```
@@ core/src/border.rs:78  [doc]  >> pub struct Radius {
The border radii for the corners of a graphics primitive in the order:
top-left, top-right, bottom-right, bottom-left.
```

译文文件里写同样的 `@@ 路径:行号` 头（**不要**带 `[doc]` 和 `>>` 之后的部分），下面写中文：

```
@@ core/src/border.rs:78
图元各个角的边框半径，顺序为：
左上、右上、右下、左下。
```

每个单元都必须有译文。单元类型：

| 类型 | 含义 | 译文怎么写 | 生成结果 |
|---|---|---|---|
| `doc` | `///`、`//!` 的一段散文 | 完整的 Markdown 译文，可多行、可含空行和列表 | 中文 + `<details>` 折叠英文 |
| `heading` | 文档里的 `# 标题` | 只写标题文字，不带 `#` | `# 中文（English）` |
| `plain` | 普通 `//` 注释块 | 中文，不带 `//` | 英文原样保留，下面加中文注释行 |
| `codecmt` | 文档代码示例里的 `// 注释` | 中文，不带 `//` | 英文下面加一行中文 |
| `trail` | 代码行尾的 `// 注释` | 一行中文 | `// English（中文）` |

只写一行 `=` 表示有意跳过（原样保留，不加中文）。只用于没有可译内容的单元，例如只有一个 URL、
只有公式或符号（`LMS → Oklab`）、或者被注释掉的代码。

## 翻译规则

1. 简体中文，技术文档语气，准确、简洁，不增删技术含义。
2. Rust 术语按根目录 `rust-glossary.md`，iced 术语按 `zh-tools/iced-glossary.md`。
3. 反引号里的代码、类型名、特性名一律原样保留：`` `Send` ``、`` `tokio` ``。
4. 文档链接必须原样保留，否则构建失败（仓库把 `broken_intra_doc_links` 设为 forbid）：
   - ``[`Task`]``、``[`Output`](Self::Output)``、`[text](Task::map)` 的方括号目标和圆括号目标都不能改。
   - 引用式链接如 `[bundled]`、`[the book]`、`[rows]`：链接文字要翻译时写成 `[打包][bundled]`、`[行][rows]`，
     不要写成 `[打包]`（那样链接就断了）。
   - 链接定义行（``[`winit`]: https://…``）不会出现在待译内容里，不需要处理。
5. 保留原文的 Markdown 结构：列表项数量、加粗、斜体、表格、`<https://…>` 链接。
6. 译文的换行位置大致跟随原文，每行不要过长（建议不超过 50 个汉字）。
7. 中文与英文、数字、反引号代码之间加一个空格；使用中文标点（，。：；（）“”）。
8. `TODO:` 译为 `待办：`，`FIXME` 译为 `待修复：`，`NOTE`/`Note` 译为 `注意：`，`Example` 译为 `示例`，
   `Platform-specific` 译为 `平台差异`。
9. 常用句式保持统一：
   - `Creates a new [`X`] with the given …` → `使用给定的…创建一个新的 [`X`]。`
   - `Sets the … of the [`X`].` → `设置 [`X`] 的…。`
   - `Returns the … of the [`X`].` → `返回 [`X`] 的…。`
   - `The … of the [`X`].` → `[`X`] 的…。`
   - `A …` / `An …`（类型说明）→ 省略“一个”，如 `A border.` → `边框。`
10. 如果某个标题被别处用 `#fragment` 链接引用（如 `(#example)`），在译文第一行写 `{#原锚点}` 保留锚点。

已完成的 `translations/01-small-crates.txt`、`02b-iced.txt`、`03-runtime.txt`、`04a-core.txt` 可以当作风格范例。
