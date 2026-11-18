# iced 专有术语对照表

`rust-glossary.md` 负责 Rust 语言术语，本表负责 iced 自己的概念。
类型名出现在反引号或文档链接里时（如 [`Task`]）保持英文不译。

| English | 简体中文 | 备注 |
|---|---|---|
| widget | 部件 | |
| widget tree | 部件树 | |
| element | 元素 | `Element` 类型 |
| application | 应用程序 | |
| program | 程序 | `Program` trait |
| daemon | 守护进程 | |
| state | 状态 | |
| message | 消息 | |
| update (logic) | 更新逻辑 | |
| view (logic) | 视图逻辑 | |
| task | 任务 | |
| subscription | 订阅 | |
| executor | 执行器 | |
| runtime | 运行时 | |
| shell | 外壳 | 窗口系统一侧的集成层 |
| event | 事件 | |
| event loop | 事件循环 | |
| layout | 布局 | |
| limits | 限制 | 布局约束 `Limits` |
| bounds | 边界 | layout bounds 布局边界；visible bounds 可见边界 |
| viewport | 视口 | |
| overlay | 覆盖层 | |
| operation | 操作 | `widget::Operation` |
| selector | 选择器 | |
| renderer | 渲染器 | |
| compositor | 合成器 | |
| surface | 表面 | |
| backend | 后端 | |
| fallback | 回退 | |
| primitive | 图元 | |
| layer | 图层 | |
| quad | 四边形 | |
| mesh | 网格 | |
| gradient | 渐变 | |
| shader | 着色器 | |
| pipeline | 管线 | |
| atlas | 图集 | |
| theme | 主题 | |
| palette | 调色板 | |
| style | 样式 | |
| catalog | 样式目录 | `Catalog` trait |
| status | 状态（交互状态） | 与 state 冲突时写「交互状态」 |
| font | 字体 | |
| glyph | 字形 | |
| text shaping | 文本整形 | |
| wrapping | 换行 | |
| alignment | 对齐 | |
| padding | 内边距 | |
| spacing | 间距 | |
| border | 边框 | |
| shadow | 阴影 | |
| scale factor | 缩放因子 | |
| focus / focused | 焦点 / 获得焦点 | |
| hover | 悬停 | |
| cursor | 光标 | 鼠标指针与文本光标都用「光标」，必要时注明 |
| scrollable | 可滚动区域 | |
| pane grid | 窗格网格 | pane 窗格；split 分割 |
| pick list | 选择列表 | |
| combo box | 组合框 | |
| toggler | 开关 | |
| slider | 滑块 | |
| tooltip | 工具提示 | |
| popover | 弹出层 | |
| canvas | 画布 | |
| frame | 帧 | |
| window | 窗口 | |
| clipboard | 剪贴板 | |
| devtools | 开发者工具 | |
| time travel | 时间旅行（调试） | |
| hot reloading / hotpatching | 热重载 / 热补丁 | |
| highlighter | 高亮器 | |
| snapshot | 快照 | |
| headless | 无头 | |
