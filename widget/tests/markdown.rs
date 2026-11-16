//! 针对 [`iced_widget::markdown`] 解析器的测试：一次性的
//! [`parse`] 和 [`sections`] 函数，以及增量式的
//! [`Content::push_str`] 方法——无论分块如何，它都必须收敛到一次性的
//! [`Content::parse`] 结果。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Tests for the [`iced_widget::markdown`] parser: the one-shot
//! [`parse`] and [`sections`] functions, and the incremental
//! [`Content::push_str`] method, which must converge to the one-shot
//! [`Content::parse`] result, whatever the chunks are.
//!
//! </details>
//!
//! [`parse`]: iced_widget::markdown::parse
//! [`sections`]: iced_widget::markdown::sections
//! [`Content::push_str`]: iced_widget::markdown::Content::push_str
//! [`Content::parse`]: iced_widget::markdown::Content::parse
use iced_widget::markdown::{Content, Item, parse, sections};

/// 很长的无序列表。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A long list of bullets.
///
/// </details>
#[test]
fn bullets() {
    let full = (0..50).map(|i| format!("- item {i}\n")).collect::<String>();
    assert_converges(&full, "bullets");
}

/// 很长的有序列表。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A long ordered list.
///
/// </details>
#[test]
fn ordered_list() {
    let full = (1..=30)
        .map(|i| format!("{i}. step {i}\n"))
        .collect::<String>();
    assert_converges(&full, "ordered list");
}

/// 任务列表。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A task list.
///
/// </details>
#[test]
fn task_list() {
    let full = "- [ ] a\n- [x] b\n- [ ] c\n- [x] d\n- [ ] e\n";
    assert_converges(full, "task list");
}

/// 嵌套列表。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A nested list.
///
/// </details>
#[test]
fn nested_list() {
    let full = "- a\n  - b\n  - c\n- d\n  - e\n- f\n";
    assert_converges(full, "nested list");
}

/// 松散列表（列表项之间有空行）。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A loose list (blank lines between bullets).
///
/// </details>
#[test]
fn loose_list() {
    let full = "- a\n\n- b\n\n- c\n";
    assert_converges(full, "loose list");
}

/// 包含多个段落的列表项。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Bullets with multiple paragraphs.
///
/// </details>
#[test]
fn multi_paragraph_bullets() {
    let full = "- a\n\n  more a\n\n- b\n\n  more b\n";
    assert_converges(full, "multi paragraph bullets");
}

/// 被其他条目包围的列表。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A list surrounded by other items.
///
/// </details>
#[test]
fn list_surrounded_by_other_items() {
    let full = "# Title\n\nintro\n\n- a\n- b\n\nafter\n\n- c\n";
    assert_converges(full, "list surrounded by other items");
}

/// 代码块后面跟着一个列表。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A code block followed by a list.
///
/// </details>
#[test]
fn code_block_and_list() {
    let full = "```\ncode\n```\n\n- a\n- b\n";
    assert_converges(full, "code block and list");
}

/// 如果某个引用的定义被元数据块吞掉，那么在该块
/// 仍未闭合时借助它解析出的链接必须
/// 不再解析。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A reference whose definition is swallowed by a metadata block must
/// stop resolving the links that were resolved with it while the block
/// was still open.
///
/// </details>
#[test]
fn reference_swallowed_by_metadata_block() {
    let full = "para\n\n- [x][ref]\n---\nkey: value\n| a | b |\n| - | - |\n| 1 | 2 |\n\n2. two [y][ref]\n![alt with [x][ref]](https://img.com/i.png)\n\n+++\nkey: value\n+++\n\n[ref]: https://changed.com\n\n---";
    assert_converges(full, "reference swallowed by a metadata block");
}

/// 同时定义在被吞掉的元数据块之内和
/// 之外的引用，会解析为块外的定义，与一次性
/// 解析的行为一致。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A reference defined both inside a swallowed metadata block and
/// outside it resolves to the outside definition, like the one-shot
/// parse does.
///
/// </details>
#[test]
fn reference_first_defined_in_swallowed_block() {
    let full =
        "a [x][ref]\n\n---\n[ref]: https://in_block.com\n---\n\n[ref]: https://outside.com\n";
    assert_converges(full, "reference first defined in a swallowed block");
}

/// 引用块中的列表，后面跟着另一个列表。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A list inside a quote, followed by another list.
///
/// </details>
#[test]
fn list_in_quote() {
    let full = "> - a\n> - b\n\n- c\n";
    assert_converges(full, "list in quote");
}

/// 看起来像 setext 标题下划线的列表项。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A bullet that looks like a setext heading underline.
///
/// </details>
#[test]
fn setext_like_bullet() {
    let full = "- a\n  ---\n- b\n";
    assert_converges(full, "setext like bullet");
}

/// 包含一个链接和一张图片的列表项。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A bullet containing a link and an image.
///
/// </details>
#[test]
fn bullet_with_link_and_image() {
    let full = "- [link](https://a.com) ![img](https://b.com/i.png)\n- b\n";
    assert_converges(full, "bullet with link and image");
}

/// 以竖线结尾的列表项（类似表格）。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A bullet ending with a pipe (table-ish).
///
/// </details>
#[test]
fn bullet_ending_with_pipe() {
    let full = "- foo |\n- bar\n";
    assert_converges(full, "bullet ending with pipe");
}

/// 单个非常长的列表项。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A single very long bullet.
///
/// </details>
#[test]
fn long_single_bullet() {
    let full = format!("- {}\n", "word ".repeat(500).trim_end());
    assert_converges(&full, "long single bullet");
}

/// 损坏的引用链接，在引用（定义在列表之后）
/// 到达后得到解析。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A broken reference link, resolved once the reference (defined
/// after the list) arrives.
///
/// </details>
#[test]
fn broken_reference_link_resolved_later() {
    let full = "- a [x][ref]\n- b [y][ref]\n- c\n\n[ref]: https://example.com\n";
    assert_converges(full, "broken reference link resolved later");
}

/// 损坏的引用链接，在同一个列表的两个列表项之间
/// 得到解析。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A broken reference link, resolved between two bullets of the same
/// list.
///
/// </details>
#[test]
fn broken_reference_link_resolved_between_bullets() {
    let full = "- a [x][ref]\n\n[ref]: https://example.com\n\n- b [y][ref]\n";
    assert_converges(full, "broken reference link resolved between bullets");
}

/// 表格，逐词流式推入。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A table, streamed word by word.
///
/// </details>
#[test]
fn table_word_by_word() {
    let full = "| a | b |\n| - | - |\n| 1 | 2 |\n| 3 | 4 |\n\nDone.\n";
    assert_converges(full, "table word by word");
}

/// 同一行上的一张图片和一个引用链接；随着这一行增长，
/// 图片不能重复出现。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// An image and a reference link on the same line; the image must not
/// be duplicated as the line grows.
///
/// </details>
#[test]
fn image_and_link_on_same_line() {
    let full = "prev\n\n![alt](https://img.com/i.png) [x][ref]\n\ntext\n";
    assert_converges(full, "image and link on same line");
}

/// 同一行上的多张图片和一个段落；这些图片和
/// 段落共享同一个源区域，随着这一行增长，
/// 图片不能重复出现。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Several images and a paragraph on the same line; the images and
/// the paragraph share the same source region, and the images must
/// not be duplicated as the line grows.
///
/// </details>
#[test]
fn images_on_same_line() {
    let full = "a\n\n![i1](https://u1.com/1.png) ![i2](https://u2.com/2.png) t\n\nb\n";
    assert_converges(full, "images on the same line");
}

/// 只有图片的一行，后面跟着更多内容。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A line with only images, followed by more content.
///
/// </details>
#[test]
fn image_only_line() {
    let full = "a\n\n![i1](https://u1.com/1.png) ![i2](https://u2.com/2.png)\n\nb\n";
    assert_converges(full, "image-only line");
}

/// 同一行上文本后面跟着一张图片；文本不能被
/// 吸收进图片的替代文本中。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Text followed by an image on the same line; the text must not be
/// absorbed into the image's alt.
///
/// </details>
#[test]
fn text_then_image() {
    let full = "hello ![img](https://u1.com/1.png)\n\nmore\n";
    assert_converges(full, "text then image");
}

/// 在使用之前定义的引用。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A reference defined before it is used.
///
/// </details>
#[test]
fn reference_before_use() {
    let full = "[ref]: https://example.com\n\n- [x][ref]\n- [y][ref]\n";
    assert_converges(full, "reference before use");
}

/// 内容之后的元数据块不应该打断流程。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A metadata block after content should not break the flow.
///
/// </details>
#[test]
fn metadata_after_content() {
    let full = "- a\n\n+++\nmeta: 1\n+++\n\n- b\n";
    assert_converges(full, "metadata after content");
}

/// 同时包含多种结构，逐块、逐词和
/// 逐字符流式推入。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Many constructs at once, streamed block by block, word by word and
/// char by char.
///
/// </details>
#[test]
fn mixed_document() {
    let parts = [
        "# Title\n\n",
        "intro with **bold** and *emphasis* and `code`\n\n",
        "- a\n",
        "- b [link](https://a.com)\n",
        "  - nested\n",
        "- [ ] task\n",
        "- [x] done\n\n",
        "1. one\n",
        "2. two\n\n",
        "> quote\n",
        "> - in a quote\n\n",
        "```\ncode block\n```\n\n",
        "![alt](https://img.com/i.png) [x][ref]\n\n",
        "text\n\n",
        "[ref]: https://example.com\n\n",
        "- c\n",
        "- d\n\n",
        "| a | b |\n| - | - |\n| 1 | 2 |\n",
        "\n\nafter\n",
    ];
    let full = parts.concat();

    assert_converges(&full, "mixed document");
}

/// 向一个很长的列表推入新的列表项必须保持低开销：`push_str`
/// 只重新解析最后一个列表项，因此每次推入的耗时不会随
/// 列表增长。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Pushing new bullets to a long list must stay cheap: `push_str`
/// re-parses only the last bullet, so the time per push does not grow
/// with the list.
///
/// </details>
#[test]
fn pushing_to_a_long_list_stays_fast() {
    let mut c = Content::new();
    let bullet = |i| format!("- item {i} with some **text** and a `code`\n");

    for i in 0..800 {
        c.push_str(&bullet(i));
    }

    // The last 200 pushes, as a fraction of the total, must be cheap:
    // with an O(n) re-parse per push they would take most of the time.
    // 最后 200 次推入占总耗时的比例必须很小：
    // 如果每次推入都进行 O(n) 的重新解析，它们会占去大部分时间。
    let start = std::time::Instant::now();
    for i in 800..1000 {
        c.push_str(&bullet(i));
    }
    let elapsed = start.elapsed();
    assert!(
        elapsed.as_secs_f64() < 2.0,
        "the last 200 pushes took {elapsed:?}, which suggests that the \
         whole list is re-parsed on every push"
    );
    assert_eq!(c.items().len(), 1);
}

/// 逐词推入一个包含许多代码块的大文档，
/// 必须保持快速，并收敛到一次性解析的结果。
///
/// 如果没有按块的高亮器缓存，每次推入都会重新高亮这些块
/// （或整个文档）：对于这个测试夹具，
/// 在启用 `highlighter` 特性的 debug 构建中大约需要 90 秒
/// （release 构建约 10 秒）。时间预算必须覆盖最慢的 CI 运行器
/// （debug、`highlighter`：约 6 秒）并留有余量，同时要远
/// 低于无缓存时的耗时，这样一旦退化为每次推入都重新高亮
/// 每个块，测试仍然会失败。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Pushing a large document with many code blocks, word by word,
/// must stay fast and converge to the one-shot parse.
///
/// Without a per-block highlighter cache, the blocks (or the whole
/// document) would be re-highlighted on every push: for this fixture
/// that is ~90s in a debug build with the `highlighter` feature (and
/// ~10s in release). The budget must cover the slowest CI runners
/// (debug, `highlighter`: ~6s) with headroom, while staying far
/// below the uncached times so that a regression to re-highlighting
/// every block on every push still fails it.
///
/// </details>
#[test]
#[ignore]
fn pushing_a_document_with_code_blocks_stays_fast() {
    let full = include_str!("fixtures/slow-incremental-markdown.md");
    let chunks: Vec<_> = full.split_inclusive(' ').map(str::to_owned).collect();

    let start = std::time::Instant::now();
    let mut c = Content::new();
    for chunk in &chunks {
        c.push_str(chunk);
    }
    let elapsed = start.elapsed();

    let one_shot = Content::parse(full);
    assert_eq!(
        format!("{:?}", c.items()),
        format!("{:?}", one_shot.items()),
        "the incremental parse should converge to the one-shot parse"
    );
    assert_eq!(c.images(), one_shot.images());

    let words = chunks.len();
    assert!(
        elapsed.as_secs_f64() < 15.0,
        "pushing {words} words took {elapsed:?}, which suggests that the \
         code blocks (or the whole document) are re-parsed on every push"
    );
}

#[test]
fn reference_resolved_before_next_item() {
    let full = "- a [x][ref]\n\n[ref]: https://example.com\n\npara\n";
    assert_converges(full, "reference resolved before the next item");
}

#[test]
fn duplicate_reference_definitions() {
    // The first definition wins, like in CommonMark; the second
    // definition is still growing (not terminated) when the push
    // happens.
    // 与 CommonMark 一样，第一个定义生效；推入发生时，
    // 第二个定义仍在增长（尚未
    // 结束）。
    let full = "- [x][ref]\n\n[ref]: https://one.com\n\npara\n\n[ref]: https://two.com\n";
    assert_converges(full, "duplicate reference definitions");
}

#[test]
fn broken_link_in_image_alt() {
    let full = "before\n\n![alt with [x][ref]](https://img.com/i.png)\n\n[ref]: https://example.com\n\nafter\n";
    assert_converges(full, "broken link in an image alt");
}

#[test]
fn broken_link_in_image_alt_with_trailing_text() {
    // The section is created for the image before its paragraph is
    // complete; the re-parse range must then span the whole line.
    // 在图片所在的段落完成之前，就已经为图片创建了小节；
    // 因此重新解析的范围必须覆盖整行。
    let full =
        "![alt [x][ref]](https://img.com/i.png) trailing\n\n[ref]: https://example.com\n\nafter\n";
    assert_converges(full, "broken link in an image alt, with trailing text");
}

/// 紧跟在列表项之后的分隔线，中间没有空行：
/// 分隔线结束了列表，增量解析必须同时保留
/// 列表和分隔线。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A rule directly after a list item, with no blank line in between:
/// the rule ends the list, and the incremental parse must keep both
/// the list and the rule.
///
/// </details>
#[test]
fn list_then_rule_without_blank_line() {
    let full = "[other]: https://other.com\n\n| a | b |\n| - | - |\n| 1 | 2 |\n\n\
         [ref]: https://example.com\n\n- a\n---\n\n";
    assert_converges(full, "list then rule without blank line");
}

/// 紧跟在列表行之后的引用定义，中间没有空
/// 行，会被吸收进列表项（惰性续行）；
/// 其后的列表不能丢失。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A reference definition directly after a list line, with no blank
/// line in between, is absorbed into the bullet (lazy continuation);
/// the list that follows must not be lost.
///
/// </details>
#[test]
fn reference_then_list_without_blank_line() {
    let full =
        "2. two\n[ref]: https://changed.com\n\n\n- a\n| a | b |\n| - | - |\n| 1 | 2 |\n\n---\n\n";
    assert_converges(full, "reference then list without blank line");
}

/// 被吸收进引用块内列表项（惰性
/// 续行）的引用定义，内嵌了一个指向它所定义标签的链接；这个链接
/// 必须按一次性解析的语义解析：与 CommonMark 一样，
/// 该标签的第一个定义生效。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A reference definition absorbed into a quoted bullet (lazy
/// continuation) embeds a link to the label it defines; that link
/// must resolve with the one-shot semantics: the first definition of
/// the label wins, like in CommonMark.
///
/// </details>
#[test]
fn absorbed_reference_resolves_to_first_definition() {
    let full = "[ref]: https://example.com\n\n> - in quote\n[ref]: https://changed.com\n\n\
         [ref]: https://changed.com\n\n> - in quote\n> - in quote\n- b [y][ref]\n\n";
    assert_converges(full, "absorbed reference resolves to the first definition");
}

/// 跨越重新解析窗口起点的列表：当它的
/// 最后一行正在被推入时，这一行是一个结束列表的
/// 独立段落；当它变成列表项时，列表不能被拆成
/// 两个。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A list that spans the start of the re-parse window: while its
/// last line is being pushed, it is a lone paragraph that closes the
/// list; when it becomes a list item, the list must not be split in
/// two.
///
/// </details>
#[test]
fn list_spanning_the_reparse_window_is_not_split() {
    assert_converges(
        "2. two\n\n2. two\n",
        "loose list across the re-parse window",
    );
    assert_converges(
        "2. two\n2. two\n\n2. two\n",
        "list, loose list, across the re-parse window",
    );
}

/// 流开头的元数据块一旦闭合，就会被
/// 解析器吞掉；在此之前，第一行是一条暂定的分隔线，
/// 因此随着块的增长，必须重新解析整个块。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A metadata block at the start of the stream is swallowed by the
/// parser once it is closed; the first line is a tentative rule
/// until then, so the whole block must be re-parsed as it grows.
///
/// </details>
#[test]
fn leading_metadata_block_is_swallowed() {
    assert_converges("---\nkey: value\n---\n\nmore\n", "a closed metadata block");
    assert_converges(
        "---\nkey: value\n...\n\nmore\n",
        "a metadata block closed by ellipsis",
    );
    assert_converges(
        "+++\nkey: value\n+++\n\nmore\n",
        "a plus-delimited metadata block",
    );
    // Not a metadata block: the second line is blank, so the first
    // line is a rule
    // 不是元数据块：第二行是空行，因此第一
    // 行是一条分隔线
    assert_converges("---\n\nmore\n", "a leading rule");
    // Not closed: the `---` is a rule and the block stays open
    // 未闭合：`---` 是一条分隔线，块保持未闭合状态
    assert_converges(
        "---\nkey: value\n--- x\n\nmore\n",
        "an unclosed metadata block",
    );
}

/// 流中间的元数据块一旦闭合，同样会被
/// 解析器吞掉：在此之前，它的起始行是一条暂定的
/// 分隔线，因此随着块的增长，必须从它开始的位置
/// 重新解析整个块。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A metadata block in the middle of the stream is swallowed by the
/// parser once it is closed, too: its opening line is a tentative
/// rule until then, so the whole block must be re-parsed from where
/// it starts as it grows.
///
/// </details>
#[test]
fn mid_metadata_block_is_swallowed() {
    // After a blank line, and after a list item (no blank line)
    // 在空行之后，以及在列表项之后（没有空行）
    assert_converges(
        "para\n\n---\nkey: value\n---\n\nmore\n",
        "a metadata block after a blank line",
    );
    assert_converges(
        "- x\n---\nkey: value\n---\n\nmore\n",
        "a metadata block after a list item",
    );
    assert_converges(
        "- x\n---\nkey: value\n...\n\nmore\n",
        "a metadata block closed by ellipsis",
    );
    assert_converges(
        "para\n\n+++\nkey: value\n+++\n\nmore\n",
        "a plus-delimited metadata block",
    );
    // A list follows the block: the list must not be merged with the
    // tentative rule
    // 块后面跟着一个列表：这个列表不能与
    // 暂定的分隔线合并
    assert_converges(
        "- a\n- b\n\n---\nkey: value\n---\n\n- c\n",
        "a metadata block between two lists",
    );
}

#[test]
fn empty_input_has_no_sections() {
    let items: Vec<_> = parse("").collect();
    assert_eq!(sections(&items).count(), 0);
}

#[test]
fn input_without_headings_is_a_single_section() {
    let items: Vec<_> = parse("hello\n\nworld").collect();
    let [(heading, body)] = partition(&items);

    assert!(heading.is_none());
    assert_eq!(body.len(), items.len());
}

#[test]
fn prefix_before_first_heading() {
    let items: Vec<_> = parse("prefix\n# Heading\n\nbody\n\nand more").collect();
    let [(preamble_heading, preamble), (heading, body)] = partition(&items);

    assert!(preamble_heading.is_none());
    assert_eq!(preamble.len(), 1);
    assert!(std::ptr::eq(&preamble[0], &items[0]));

    assert!(std::ptr::eq(
        heading.expect("Expected a heading"),
        &items[1]
    ));
    assert_eq!(body.len(), 2);
    assert!(std::ptr::eq(&body[0], &items[2]));
    assert!(std::ptr::eq(&body[1], &items[3]));
}

#[test]
fn consecutive_headings_yield_empty_bodies() {
    let items: Vec<_> = parse("# A\n\n# B\n\n# C\n\nbody").collect();
    let [
        (heading_a, body_a),
        (heading_b, body_b),
        (heading_c, body_c),
    ] = partition(&items);

    assert!(heading_a.is_some());
    assert!(body_a.is_empty());
    assert!(heading_b.is_some());
    assert!(body_b.is_empty());
    assert!(heading_c.is_some());
    assert_eq!(body_c.len(), 1);
}

#[test]
fn trailing_heading_yields_an_empty_body() {
    let items: Vec<_> = parse("body\n# Heading").collect();
    let [(preamble_heading, preamble), (heading, body)] = partition(&items);

    assert!(preamble_heading.is_none());
    assert_eq!(preamble.len(), 1);
    assert!(heading.is_some());
    assert!(body.is_empty());
}

#[test]
fn every_item_is_yielded_exactly_once() {
    let items: Vec<_> =
        parse("intro\n# H1\n\np1\n- item\n> quote\n## H2\n\n```\ncode\n```\np2\n# H3").collect();
    let sections = sections(&items).collect::<Vec<_>>();

    // The headings are yielded as the first element of their sections,
    // in order
    // 标题按顺序作为各自小节的第一个元素
    // 产出
    assert_same_items(
        sections.iter().filter_map(|(heading, _)| *heading),
        items
            .iter()
            .filter(|item| matches!(item, Item::Heading(..))),
    );

    // The bodies partition the non-heading items, in order
    // 正文按顺序划分所有非标题条目
    assert_same_items(
        sections.iter().flat_map(|(_, body)| body.iter()),
        items
            .iter()
            .filter(|item| !matches!(item, Item::Heading(..))),
    );
}

#[test]
fn content_sections() {
    let content = Content::parse("prefix\n# Heading\n\nbody");
    let [(preamble_heading, _), (heading, _)] = partition(content.items());

    assert!(preamble_heading.is_none());
    assert!(std::ptr::eq(
        heading.expect("Expected a heading"),
        &content.items()[1]
    ));
}

/// 在一组生成的文档上，用三种分块方案对增量解析的
/// 收敛性进行模糊测试，并在每次推入之后检查
/// 与一次性解析的等价性。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Fuzzes the convergence of the incremental parse over a corpus of
/// generated documents, with the three chunking schemes, checking the
/// one-shot equivalence after every push.
///
/// </details>
#[test]
#[ignore]
fn fuzz() {
    /// 用于生成模糊测试文档的块。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The blocks from which the fuzzed documents are generated.
    ///
    /// </details>
    const BLOCKS: &[&str] = &[
        "para\n\n",
        "text [x][ref] more\n\n",
        "- a\n",
        "- a\n- b\n",
        "- a [x][ref]\n",
        "- [x][ref]\n",
        "- b [y][ref]\n\n",
        "2. two\n",
        "2. two [y][ref]\n",
        "- [ ] task\n",
        "> - in quote\n",
        "> quote\n\n",
        "| a | b |\n| - | - |\n| 1 | 2 |\n\n",
        "| a | b |\n| - | - |\n| 1 | 2 | [x][ref] |\n\n",
        "[ref]: https://example.com\n\n",
        "[ref]: https://changed.com\n\n",
        "[other]: https://other.com\n\n",
        "[x]: https://x.com\n\n",
        "![alt](https://img.com/i.png)\n\n",
        "![alt with [x][ref]](https://img.com/i.png)\n\n",
        "before ![alt [x][ref]](https://img.com/i.png) trailing\n\n",
        "---\n\n",
        "---\nkey: value\n---\n\n",
        "+++\nkey: value\n+++\n\n",
        "---\nkey: value\n...\n\n",
        "---\nkey: value\n",
        "# heading\n\n",
        "## setext\n---\n\n",
        "```\ncode\n```\n\n",
        "\n",
    ];

    /// 确定性的伪随机数生成器。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// A deterministic pseudo-random generator.
    ///
    /// </details>
    struct Rng(u64);

    impl Rng {
        fn next(&mut self) -> usize {
            self.0 = self
                .0
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            (self.0 >> 33) as usize
        }
    }

    /// 要进行模糊测试的文档：四个收敛性问题的复现用例，以及
    /// 一组由 `BLOCKS` 生成的确定性文档。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The documents to fuzz: the four convergence repros, and a
    /// deterministic corpus of documents generated from `BLOCKS`.
    ///
    /// </details>
    fn corpus() -> impl Iterator<Item = String> {
        const REPROS: &[&str] = &[
            "[other]: https://other.com\n\n| a | b |\n| - | - |\n| 1 | 2 |\n\n\
         [ref]: https://example.com\n\n- a\n---\n\n",
            "2. two\n[ref]: https://changed.com\n\n\n- a\n| a | b |\n| - | - |\n| 1 | 2 |\n\n---\n\n",
            "[ref]: https://example.com\n\n> - in quote\n[ref]: https://changed.com\n\n\
         [ref]: https://changed.com\n\n> - in quote\n> - in quote\n- b [y][ref]\n\n",
            // A metadata block at the start of the document
            // 位于文档开头的元数据块
            "---\nkey: value\n---\n\npara\n\n- a\n",
            // A metadata block in the middle of the document, after a
            // blank line and after a list item (no blank line), closed
            // and unclosed
            // 位于文档中间的元数据块，分别在
            // 空行之后和列表项之后（没有空行），包括已闭合
            // 和未闭合的情况
            "para\n\n---\nkey: value\n---\n\nmore\n\n- a\n",
            "- x\n---\nkey\n---\npara\n",
            "- x\n---\nkey\n...\nmore\n",
        ];

        let mut rng = Rng(0x5EED);

        REPROS
            .iter()
            .copied()
            .map(str::to_owned)
            .chain((0..600).map(move |_| {
                let mut doc = String::new();

                for _ in 0..4 + rng.next() % 7 {
                    doc.push_str(BLOCKS[rng.next() % BLOCKS.len()]);
                }

                doc
            }))
    }

    for doc in corpus() {
        assert_converges(&doc, "fuzzed");
    }
}

/// 断言使用不同的分块方案增量解析 `full`，
/// 都会收敛到一次性解析的结果。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Asserts that parsing `full` incrementally, with different chunking
/// schemes, converges to the one-shot parse.
///
/// </details>
fn assert_converges(full: &str, label: &str) {
    let word_by_word = full.split_inclusive(' ').map(str::to_owned).collect();
    let char_by_char = full.chars().map(|c| c.to_string()).collect();

    let three_char_chunks: Vec<_> = {
        let mut i = 0;

        std::iter::from_fn(|| {
            if i >= full.len() {
                return None;
            }

            let end = (i + 3).min(full.len());
            let chunk = full[i..end].to_owned();
            i = end;

            Some(chunk)
        })
        .collect()
    };

    for (chunks, name) in [
        (word_by_word, "word-by-word"),
        (char_by_char, "char-by-char"),
        (three_char_chunks, "3-char chunks"),
    ] {
        let mut stream = Content::new();

        for chunk in chunks {
            stream.push_str(&chunk);

            let one_shot = Content::parse(stream.raw());

            assert_eq!(
                format!("{:?}", stream.items()),
                format!("{:?}", one_shot.items()),
                "{label}: incremental ({name}) should converge to one-shot\n\n{raw}",
                raw = stream.raw(),
            );

            assert_eq!(
                stream.images(),
                one_shot.images(),
                "{label}: images should converge ({name})\n\n{raw}",
                raw = stream.raw(),
            );
        }
    }
}

/// 断言两个迭代器以相同的顺序产出完全相同的 [`Item`]
/// 引用。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Asserts that two iterators yield the exact same [`Item`]
/// references, in the same order.
///
/// </details>
fn assert_same_items<'a>(
    left: impl IntoIterator<Item = &'a Item>,
    right: impl IntoIterator<Item = &'a Item>,
) {
    let (mut left, mut right) = (left.into_iter(), right.into_iter());

    loop {
        match (left.next(), right.next()) {
            (Some(left), Some(right)) => assert!(std::ptr::eq(left, right)),
            (None, None) => break,
            (left, right) => panic!("Length mismatch: {left:?} vs {right:?}"),
        }
    }
}

/// [`sections`] 产出的小节，预期恰好有 `N` 个。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The sections yielded by [`sections`], expecting exactly `N` of them.
///
/// </details>
fn partition<const N: usize>(items: &[Item]) -> [(Option<&Item>, &[Item]); N] {
    sections(items)
        .collect::<Vec<_>>()
        .try_into()
        .expect("Unexpected number of sections")
}
