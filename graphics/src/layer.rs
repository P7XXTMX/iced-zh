//! 绘制并堆叠由图元组成的图层。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Draw and stack layers of graphical primitives.
//!
//! </details>
use crate::core::{Rectangle, Transformation};

/// 由图元组成的图层。
///
/// 图层通常规定了一组按特定顺序
/// 渲染的图元。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A layer of graphical primitives.
///
/// Layers normally dictate a set of primitives that are
/// rendered in a specific order.
///
/// </details>
pub trait Layer: Default {
    /// 使用给定的边界创建一个新的 [`Layer`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Layer`] with the given bounds.
    ///
    /// </details>
    fn with_bounds(bounds: Rectangle) -> Self;

    /// 返回 [`Layer`] 当前的边界。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current bounds of the [`Layer`].
    ///
    /// </details>
    fn bounds(&self) -> Rectangle;

    /// 刷新并敲定 [`Layer`] 中所有待处理的图元分组。
    ///
    /// 它会在 [`Layer`] 完成时被调用。这样图层就可以高效地
    /// 把图元记录在一起，并把分组推迟到最后进行。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Flushes and settles any pending group of primitives in the [`Layer`].
    ///
    /// This will be called when a [`Layer`] is finished. It allows layers to efficiently
    /// record primitives together and defer grouping until the end.
    ///
    /// </details>
    fn flush(&mut self);

    /// 把 [`Layer`] 调整为给定的边界。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Resizes the [`Layer`] to the given bounds.
    ///
    /// </details>
    fn resize(&mut self, bounds: Rectangle);

    /// 清空图层的全部内容并重置其边界。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Clears all the layers contents and resets its bounds.
    ///
    /// </details>
    fn reset(&mut self);

    /// 返回 [`Layer`] 的起始层级。
    ///
    /// 层级是 [`Layer`] 内部的“子图层”索引。
    ///
    /// 一个 [`Layer`] 可能按一定顺序绘制多种图元类型。
    /// 层级表示它所包含的图元类型中
    /// 最低的索引。
    ///
    /// 因此，如果两个图层 A 和 B 的边界相同，
    /// 并且 A 的结束层级小于或等于 B 的起始层级，它们就可以合并。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the start level of the [`Layer`].
    ///
    /// A level is a "sublayer" index inside of a [`Layer`].
    ///
    /// A [`Layer`] may draw multiple primitive types in a certain order.
    /// The level represents the lowest index of the primitive types it
    /// contains.
    ///
    /// Two layers A and B can therefore be merged if they have the same bounds,
    /// and the end level of A is lower or equal than the start level of B.
    ///
    /// </details>
    fn start(&self) -> usize;

    /// 返回 [`Layer`] 的结束层级。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the end level of the [`Layer`].
    ///
    /// </details>
    fn end(&self) -> usize;

    /// 把一个 [`Layer`] 合并到当前图层。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Merges a [`Layer`] with the current one.
    ///
    /// </details>
    fn merge(&mut self, _layer: &mut Self);
}

/// 用于绘制的图层栈。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// A stack of layers used for drawing.
///
/// </details>
#[derive(Debug)]
pub struct Stack<T: Layer> {
    layers: Vec<T>,
    transformations: Vec<Transformation>,
    previous: Vec<usize>,
    current: usize,
    active_count: usize,
}

impl<T: Layer> Stack<T> {
    /// 创建一个新的空 [`Stack`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new empty [`Stack`].
    ///
    /// </details>
    pub fn new() -> Self {
        Self {
            layers: vec![T::default()],
            transformations: vec![Transformation::IDENTITY],
            previous: vec![],
            current: 0,
            active_count: 1,
        }
    }

    /// 返回 [`Stack`] 当前 [`Layer`] 的可变引用，
    /// 以及当前的 [`Transformation`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns a mutable reference to the current [`Layer`] of the [`Stack`], together with
    /// the current [`Transformation`].
    ///
    /// </details>
    #[inline]
    pub fn current_mut(&mut self) -> (&mut T, Transformation) {
        let transformation = self.transformation();

        (&mut self.layers[self.current], transformation)
    }

    /// 返回 [`Stack`] 当前的 [`Transformation`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the current [`Transformation`] of the [`Stack`].
    ///
    /// </details>
    #[inline]
    pub fn transformation(&self) -> Transformation {
        self.transformations.last().copied().unwrap()
    }

    /// 向 [`Stack`] 压入一个新的裁剪区域；在此过程中
    /// 会创建一个新的图层。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Pushes a new clipping region in the [`Stack`]; creating a new layer in the
    /// process.
    ///
    /// </details>
    pub fn push_clip(&mut self, bounds: Rectangle) {
        self.previous.push(self.current);

        self.current = self.active_count;
        self.active_count += 1;

        let bounds = bounds * self.transformation();

        if self.current == self.layers.len() {
            self.layers.push(T::with_bounds(bounds));
        } else {
            self.layers[self.current].resize(bounds);
        }
    }

    /// 从 [`Stack`] 弹出当前的裁剪区域，并恢复上一个裁剪区域。
    ///
    /// 当前图层会被记录下来以供绘制。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Pops the current clipping region from the [`Stack`] and restores the previous one.
    ///
    /// The current layer will be recorded for drawing.
    ///
    /// </details>
    pub fn pop_clip(&mut self) {
        self.flush();

        self.current = self.previous.pop().unwrap();
    }

    /// 向 [`Stack`] 压入一个新的 [`Transformation`]。
    ///
    /// 此后的绘制操作都会受这个新的 [`Transformation`] 影响，
    /// 直到使用 [`pop_transformation`] 把它弹出。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Pushes a new [`Transformation`] in the [`Stack`].
    ///
    /// Future drawing operations will be affected by this new [`Transformation`] until
    /// it is popped using [`pop_transformation`].
    ///
    /// </details>
    ///
    /// [`pop_transformation`]: Self::pop_transformation
    pub fn push_transformation(&mut self, transformation: Transformation) {
        self.transformations
            .push(self.transformation() * transformation);
    }

    /// 弹出 [`Stack`] 中当前的 [`Transformation`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Pops the current [`Transformation`] in the [`Stack`].
    ///
    /// </details>
    pub fn pop_transformation(&mut self) {
        let _ = self.transformations.pop();
    }

    /// 返回一个迭代器，遍历 [`Stack`] 中各图层的不可变引用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns an iterator over immutable references to the layers in the [`Stack`].
    ///
    /// </details>
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.layers[..self.active_count].iter()
    }

    /// 返回 [`Stack`] 中图层的切片。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the slice of layers in the [`Stack`].
    ///
    /// </details>
    pub fn as_slice(&self) -> &[T] {
        &self.layers[..self.active_count]
    }

    /// 刷新并敲定 [`Stack`] 中的所有图元。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Flushes and settles any primitives in the [`Stack`].
    ///
    /// </details>
    pub fn flush(&mut self) {
        self.layers[self.current].flush();
    }

    /// 尽可能地合并图层。
    ///
    /// 刷新并敲定 [`Stack`] 中的所有图元。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Performs layer merging wherever possible.
    ///
    /// Flushes and settles any primitives in the [`Stack`].
    ///
    /// </details>
    pub fn merge(&mut self) {
        self.flush();

        // These are the layers left to process
        // 这些是尚待处理的图层
        let mut left = self.active_count;

        // There must be at least 2 or more layers to merge
        // 至少要有 2 个图层才能合并
        while left > 1 {
            // We set our target as the topmost layer left to process
            // 把尚待处理的最顶层图层设为目标
            let mut current = left - 1;
            let mut target = &self.layers[current];
            let mut target_start = target.start();
            let mut target_index = current;

            // We scan downwards for a contiguous block of mergeable layer candidates
            // 向下扫描，寻找一段连续的可合并候选图层
            while current > 0 {
                let candidate = &self.layers[current - 1];
                let start = candidate.start();
                let end = candidate.end();

                // We skip empty layers
                // 跳过空图层
                if end == 0 {
                    current -= 1;
                    continue;
                }

                // Candidate can be merged if primitive sublayers do not overlap with
                // previous targets and the clipping bounds match
                // 如果候选图层的图元子图层不与先前的目标重叠，
                // 并且裁剪边界一致，就可以合并
                if end > target_start || candidate.bounds() != target.bounds() {
                    break;
                }

                // Candidate is not empty and can be merged into
                // 候选图层非空，并且可以作为合并的目标
                target = candidate;
                target_start = start;
                target_index = current;
                current -= 1;
            }

            // We merge all the layers scanned into the target
            //
            // Since we use `target_index` instead of `current`, we
            // deliberately avoid merging into empty layers.
            //
            // If no candidates were mergeable, this is a no-op.
            // 把扫描到的所有图层合并到目标中
            //
            // 由于使用的是 `target_index` 而不是 `current`，
            // 我们有意避免合并到空图层中。
            //
            // 如果没有可合并的候选图层，这一步什么也不做。
            let (head, tail) = self.layers.split_at_mut(target_index + 1);
            let layer = &mut head[target_index];

            for middle in &mut tail[0..left - target_index - 1] {
                layer.merge(middle);
            }

            // Empty layers found after the target can be skipped
            // 目标之后发现的空图层可以跳过
            left = current;
        }
    }

    /// 清空 [`Stack`] 的图层，以便复用。
    ///
    /// 它会把基础图层的边界调整为 `new_bounds`。
    ///
    /// 这通常会保留图层的内存分配，供以后的绘制操作使用。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Clears the layers of the [`Stack`], allowing reuse.
    ///
    /// It resizes the base layer bounds to the `new_bounds`.
    ///
    /// This will normally keep layer allocations for future drawing operations.
    ///
    /// </details>
    pub fn reset(&mut self, new_bounds: Rectangle) {
        for layer in self.layers[..self.active_count].iter_mut() {
            layer.reset();
        }

        self.layers[0].resize(new_bounds);
        self.current = 0;
        self.active_count = 1;
        self.previous.clear();
    }
}

impl<T: Layer> Default for Stack<T> {
    fn default() -> Self {
        Self::new()
    }
}
