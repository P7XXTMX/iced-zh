//! 使用基于 flex 的布局来分布元素。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Distribute elements using a flex-based layout.
//!
//! </details>
// This code is heavily inspired by the [`druid`] codebase.
//
// [`druid`]: https://github.com/xi-editor/druid
//
// Copyright 2018 The xi-editor Authors, Héctor Ramón
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
// 这段代码在很大程度上受到了 [`druid`] 代码库的启发。
//
// 版权所有 2018 The xi-editor Authors, Héctor Ramón
//
// 根据 Apache 许可证 2.0 版（“许可证”）授权；
// 除非遵守许可证，否则你不得使用此文件。
// 你可以在以下网址获取许可证的副本：
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// 除非适用法律要求或经书面同意，
// 根据许可证分发的软件均按“原样”分发，
// 不附带任何明示或暗示的保证或条件。
// 有关许可证下的权限和限制的具体规定，
// 请参阅许可证。
use crate::layout::{Limits, Vector};
use crate::length;
use crate::widget;
use crate::{Alignment, Length, Padding, Size, Widget};

/// flex 布局的主轴。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The main axis of a flex layout.
///
/// </details>
#[derive(Debug)]
pub enum Axis {
    /// 水平轴
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The horizontal axis
    ///
    /// </details>
    Horizontal,

    /// 垂直轴
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// The vertical axis
    ///
    /// </details>
    Vertical,
}

impl Axis {
    fn main<T>(&self, size: Size<T>) -> T {
        match self {
            Axis::Horizontal => size.width,
            Axis::Vertical => size.height,
        }
    }

    fn cross<T>(&self, size: Size<T>) -> T {
        match self {
            Axis::Horizontal => size.height,
            Axis::Vertical => size.width,
        }
    }

    fn pack<T>(&self, main: T, cross: T) -> (T, T) {
        match self {
            Axis::Horizontal => (main, cross),
            Axis::Vertical => (cross, main),
        }
    }
}

/// [`resolve`] 所用的、每个容器各自保留的状态。
///
/// [`resolve`] 会在它的多个阶段之间保留每个子级的簿记数据。为了
/// 避免在每次调用 `layout` 时都分配这些簿记数据，每个 flex
/// 容器都在它的 [`widget::Tree`] 状态里存储一个 [`Cache`]，
/// [`resolve`] 会在多次布局调用之间复用存储在那里的缓冲区。
///
/// 该类型是不透明的：其内容完全由 [`resolve`] 管理。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Per-container retained state for [`resolve`].
///
/// [`resolve`] keeps per-child bookkeeping across its multiple passes. To
/// avoid allocating that bookkeeping on every `layout` call, each flex
/// container stores a [`Cache`] in its [`widget::Tree`] state, and
/// [`resolve`] reuses the buffer stored there between layout calls.
///
/// The type is opaque: its contents are managed entirely by [`resolve`].
///
/// </details>
#[derive(Debug, Default)]
pub struct Cache {
    metas: Vec<Meta>,
}

#[derive(Debug, Clone, Copy)]
struct Meta {
    main: Length,
    cross: Length,
    category: Category,
    resolved: bool,
}

#[derive(Debug, Clone, Copy)]
enum Category {
    Static,
    CrossFluid,
    CrossFluidDeferred(f32),
    MainFluid,
}

/// 使用给定的轴和限制计算 flex 布局，并根据需要
/// 对各个条目应用间距、内边距和对齐。
///
/// 布局会存储在给定的各个子级 [`widget::Tree`] 中，并返回
/// 容器解析后的尺寸。
///
/// 给定的 [`Cache`] 保存着算法中每个子级的簿记数据；
/// 它应该在多次调用之间复用（参见 [`Cache`]）。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Computes the flex layout with the given axis and limits, applying spacing,
/// padding and alignment to the items as needed.
///
/// The layout is stored in the provided children [`widget::Tree`]s, and the
/// resolved size of the container is returned.
///
/// The provided [`Cache`] holds the per-child bookkeeping of the algorithm;
/// it is expected to be reused between calls (see [`Cache`]).
///
/// </details>
pub fn resolve<Message, Theme, Renderer>(
    axis: Axis,
    renderer: &Renderer,
    limits: &Limits,
    width: Length,
    height: Length,
    padding: Padding,
    spacing: f32,
    align_items: Alignment,
    children: &mut [widget::Tree],
    items: &mut [impl Widget<Message, Theme, Renderer>],
    cache: &mut Cache,
) -> Size {
    let limits = limits.width(width).height(height).shrink(padding);
    let total_spacing = spacing * items.len().saturating_sub(1) as f32;
    let max_cross = axis.cross(limits.max);

    let compression = limits.compression;
    let infinite = limits.infinite;

    let (main_compress, cross_compress) = axis.pack(compression.width, compression.height);

    let compression = {
        let (compress_x, compress_y) = axis.pack(main_compress, false);
        Size::new(compress_x, compress_y)
    };

    let cross_infinite = {
        let (infinite_x, infinite_y) = axis.pack(axis.main(infinite), false);
        Size::new(infinite_x, infinite_y)
    };

    let cross_dynamic = cross_compress || axis.cross(infinite);

    let mut fill_main_sum = 0;
    let mut some_fill_cross = false;
    let mut some_fill_max = false;
    let mut some_fill_min = false;
    let mut min_total = 0.0;
    let mut min_factors = 0;
    let mut cross = 0.0f32;
    let mut available = axis.main(limits.max) - total_spacing;

    cache.metas.clear();
    cache.metas.reserve(items.len());

    // STATIC PASS
    // We lay out non-fluid elements in the main axis.
    // If we need to compress the cross axis, then we skip any of these elements
    // that are also fluid in the cross axis.
    // 静态阶段
    // 我们在主轴上布局非流动（non-fluid）元素。
    // 如果需要压缩交叉轴，就跳过其中那些
    // 在交叉轴上同样是流动的元素。
    for (i, child) in items.iter_mut().enumerate() {
        let size = child.size();
        let (size_main, size_cross) = axis.pack(size.width, size.height);

        let fill_main_factor = size_main.fill_factor();
        let fill_cross_factor = size_cross.fill_factor();
        let main_is_static = main_compress || fill_main_factor == 0;

        let category = match (main_is_static, cross_dynamic, fill_cross_factor == 0) {
            (true, false, _) | (true, _, true) => Category::Static,
            (true, true, false) => {
                if let Length::Fixed(main) = size_main {
                    available -= main;
                    Category::CrossFluidDeferred(main)
                } else {
                    Category::CrossFluid
                }
            }
            (false, _, _) => Category::MainFluid,
        };

        let meta = Meta {
            main: size_main,
            cross: size_cross,
            resolved: false,
            category,
        };

        cache.metas.push(meta);

        match meta.main {
            Length::Bounded {
                sizing: length::Sizing::Fill(_),
                bounds: length::Bounds::Min(min),
            }
            | Length::Fluid(length::Constraint::Min(min)) => {
                min_total += min;
                min_factors += fill_main_factor;
                some_fill_min = true;
            }
            Length::Bounded {
                sizing: length::Sizing::Fill(_),
                bounds: length::Bounds::Max(_),
            }
            | Length::Fluid(length::Constraint::Max) => {
                some_fill_max = true;
            }
            Length::Bounded {
                sizing: length::Sizing::Fill(_),
                bounds: length::Bounds::Both { .. },
            } => {
                some_fill_max = true;
                some_fill_min = true;
            }
            _ => {}
        }

        let Category::Static = meta.category else {
            fill_main_sum += fill_main_factor;
            some_fill_cross = some_fill_cross || fill_cross_factor != 0;
            continue;
        };

        let (max_width, max_height) = axis.pack(available, max_cross);

        let child_limits = Limits::with_flags(
            Size::ZERO,
            Size::new(max_width, max_height),
            compression,
            infinite,
        );

        child.layout(&mut children[i], renderer, &child_limits);

        let size = children[i].size;

        available -= axis.main(size);
        cross = cross.max(axis.cross(size));
    }

    // CROSS FLUID PASS
    // If we must compress the cross axis and there are fluid elements in the
    // cross axis, we lay out any of these elements that are also non-fluid in
    // the main axis (i.e. the ones we deliberately skipped in the first pass).
    //
    // We use the maximum cross length obtained in the first pass as the maximum
    // cross limit.
    //
    // We can defer the layout of any elements that have a fixed size in the main axis,
    // allowing them to use the cross calculations of the next pass.
    // 交叉轴流动阶段
    // 如果必须压缩交叉轴，并且存在在交叉轴上流动的元素，
    // 我们就布局其中那些在主轴上非流动的元素
    // （即我们在第一阶段特意跳过的那些）。
    //
    // 我们把第一阶段得到的最大交叉轴长度用作
    // 交叉轴的最大限制。
    //
    // 对于在主轴上具有固定尺寸的元素，我们可以推迟它们的布局，
    // 让它们使用下一阶段的交叉轴计算结果。
    if cross_dynamic && some_fill_cross {
        for (i, child) in items.iter_mut().enumerate() {
            let meta = cache.metas[i];

            let Category::CrossFluid = meta.category else {
                continue;
            };

            let (max_width, max_height) =
                axis.pack(available, if cross_dynamic { cross } else { max_cross });

            let child_limits = Limits::with_flags(
                Size::ZERO,
                Size::new(max_width, max_height),
                compression,
                cross_infinite,
            );

            child.layout(&mut children[i], renderer, &child_limits);

            let size = children[i].size;

            available -= axis.main(size);
            cross = cross.max(axis.cross(size));
        }
    }

    let mut remaining = available.max(0.0);

    // MIN / MAX PASSES
    // We lay out any elements that are either bounded or fluid with some min / max constraints.
    //
    // First, we lay out any elements with `Max` bounds to potentially free up space; then
    // another pass lays out elements with `Min` bounds.
    //
    // Since the remaining space may change mid-iteration, we need to revisit previous elements
    // until they stabilize. Still, `layout` is only called once per element.
    // 最小 / 最大阶段
    // 我们布局那些有边界的元素，或者带有最小 / 最大约束的流动元素。
    //
    // 首先，我们布局带有 `Max` 边界的元素，以尽可能腾出空间；然后
    // 再用一个阶段布局带有 `Min` 边界的元素。
    //
    // 由于剩余空间可能在迭代中途发生变化，我们需要重新访问之前的元素，
    // 直到它们稳定下来。不过，每个元素的 `layout` 仍然只会被调用一次。
    #[derive(Debug, Clone, Copy)]
    enum Stage {
        Max,
        Min,
    }

    let mut step = if main_compress {
        None
    } else if some_fill_max {
        Some(Stage::Max)
    } else if some_fill_min {
        Some(Stage::Min)
    } else {
        None
    };

    while let Some(stage) = step {
        let current = remaining;
        let (reserved_space, reserved_factors) = match stage {
            Stage::Max => (min_total, min_factors),
            Stage::Min => (0.0, 0),
        };

        for (i, child) in items.iter_mut().enumerate() {
            let meta = &mut cache.metas[i];

            if meta.resolved {
                continue;
            }

            let fill_main_factor = meta.main.fill_factor();

            if fill_main_factor == 0 {
                continue;
            }

            let bounds = match stage {
                Stage::Max => match meta.main {
                    Length::Bounded {
                        bounds: bounds @ (length::Bounds::Max(_) | length::Bounds::Both { .. }),
                        ..
                    } => bounds,
                    Length::Fluid(length::Constraint::Max) => length::Bounds::Min(0.0),
                    _ => continue,
                },
                Stage::Min => match meta.main {
                    Length::Bounded {
                        bounds: bounds @ (length::Bounds::Min(_) | length::Bounds::Both { .. }),
                        ..
                    } => bounds,
                    Length::Fluid(length::Constraint::Min(min)) => length::Bounds::Min(min),
                    _ => continue,
                },
            };

            let max_available = (remaining - reserved_space) * fill_main_factor as f32
                / (fill_main_sum - reserved_factors) as f32;

            let max_available = if max_available.is_nan() {
                f32::INFINITY
            } else {
                max_available
            };

            let (min, max) = match bounds {
                length::Bounds::Max(max) => (0.0, max),
                length::Bounds::Both { min, max } => (min, max),
                length::Bounds::Min(min) => (min, max_available),
            };

            match stage {
                Stage::Max if max > max_available => continue,
                Stage::Min if min < max_available => continue,
                _ => {}
            }

            let min = min.min(remaining);
            let max = max.min(max_available).max(min);

            let (max_cross, infinite) = if !cross_dynamic || meta.cross.fill_factor() == 0 {
                (max_cross, infinite)
            } else {
                (cross, cross_infinite)
            };

            let (min_width, min_height) = axis.pack(min, 0.0);
            let (max_width, max_height) = axis.pack(max, max_cross);

            let child_limits = Limits::with_flags(
                Size::new(min_width, min_height),
                Size::new(max_width, max_height),
                compression,
                infinite,
            );

            child.layout(&mut children[i], renderer, &child_limits);

            let size = children[i].size;

            cross = cross.max(axis.cross(size));
            remaining -= axis.main(size);
            fill_main_sum -= fill_main_factor;
            meta.resolved = true;
        }

        if remaining == current {
            step = match stage {
                Stage::Max if some_fill_min => Some(Stage::Min),
                _ => None,
            };
        }
    }

    // MAIN FLUID PASS (conditional)
    // We lay out the elements that are fluid in the main axis.
    // We use the remaining space to evenly allocate space based on fill factors.
    // 主轴流动阶段（有条件执行）
    // 我们布局在主轴上流动的元素。
    // 我们根据填充因子，用剩余空间均匀地分配空间。
    if !main_compress {
        for (i, child) in items.iter_mut().enumerate() {
            let meta = cache.metas[i];

            if meta.resolved {
                continue;
            }

            let Category::MainFluid = meta.category else {
                continue;
            };

            let max_main = remaining * meta.main.fill_factor() as f32 / fill_main_sum as f32;

            let max_main = if max_main.is_nan() {
                f32::INFINITY
            } else {
                max_main
            };

            let min_main = if max_main.is_infinite() {
                0.0
            } else {
                max_main
            };

            let (max_cross, infinite) = if !cross_dynamic || meta.cross.fill_factor() == 0 {
                (max_cross, infinite)
            } else {
                (cross, cross_infinite)
            };

            let (min_width, min_height) = axis.pack(min_main, 0.0);
            let (max_width, max_height) = axis.pack(max_main, max_cross);

            let child_limits = Limits::with_flags(
                Size::new(min_width, min_height),
                Size::new(max_width, max_height),
                compression,
                infinite,
            );

            child.layout(&mut children[i], renderer, &child_limits);

            let size = children[i].size;
            cross = cross.max(axis.cross(size));
        }
    }

    // CROSS FLUID DEFERRED PASS (conditional)
    // We lay out any elements that were deferred in the second pass.
    // These are elements that must be compressed in their cross axis and have
    // a fixed length in the main axis.
    // 交叉轴流动的延迟阶段（有条件执行）
    // 我们布局在第二阶段被推迟的元素。
    // 这些元素必须在交叉轴上被压缩，并且
    // 在主轴上具有固定长度。
    if cross_dynamic && some_fill_cross {
        for (i, child) in items.iter_mut().enumerate() {
            let meta = cache.metas[i];

            let Category::CrossFluidDeferred(main) = meta.category else {
                continue;
            };

            let (max_width, max_height) = axis.pack(main, cross);

            let child_limits = Limits::with_flags(
                Size::ZERO,
                Size::new(max_width, max_height),
                Size::new(false, false),
                cross_infinite,
            );

            child.layout(&mut children[i], renderer, &child_limits);

            let size = children[i].size;
            cross = cross.max(axis.cross(size));
        }
    }

    let pad = axis.pack(padding.left, padding.top);
    let mut main = pad.0;

    let cross = match axis {
        Axis::Horizontal => limits.resolve_height(height, cross),
        Axis::Vertical => limits.resolve_width(width, cross),
    };

    // ALIGNMENT PASS
    // We align all the laid out nodes in the cross axis, if needed.
    // 对齐阶段
    // 如有需要，我们在交叉轴上对齐所有已布局的节点。

    for (i, tree) in children.iter_mut().enumerate() {
        if i > 0 {
            main += spacing;
        }

        let position = {
            let (x, y) = axis.pack(main, pad.1);

            Vector::new(x, y)
        };

        let alignment = match axis {
            Axis::Horizontal => {
                tree.size
                    .align(Size::new(0.0, cross), Alignment::Start, align_items)
            }
            Axis::Vertical => tree
                .size
                .align(Size::new(cross, 0.0), align_items, Alignment::Start),
        };

        tree.translation = position + alignment;
        main += axis.main(tree.size);
    }

    let main = match axis {
        Axis::Horizontal => limits.resolve_width(width, main - pad.0),
        Axis::Vertical => limits.resolve_height(height, main - pad.0),
    };

    Size::from(axis.pack(main, cross)).expand(padding)
}
