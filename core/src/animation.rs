//! 为你的应用程序添加动画。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Animate your applications.
//!
//! </details>
use crate::time::{Duration, Instant};

pub use lilt::{Easing, FloatRepresentable as Float, Interpolable};

/// 某个特定状态的动画。
///
/// 它跟踪状态的变化，并可以随时间
/// 投影出插值后的值。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// The animation of some particular state.
///
/// It tracks state changes and allows projecting interpolated values
/// through time.
///
/// </details>
#[derive(Debug, Clone)]
pub struct Animation<T>
where
    T: Clone + Copy + PartialEq + Float,
{
    raw: lilt::Animated<T, Instant>,
    duration: Duration, // TODO: Expose duration getter in `lilt`（待办：在 `lilt` 中公开时长的 getter）
}

impl<T> Animation<T>
where
    T: Clone + Copy + PartialEq + Float,
{
    /// 使用给定的初始状态创建一个新的 [`Animation`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Creates a new [`Animation`] with the given initial state.
    ///
    /// </details>
    pub fn new(state: T) -> Self {
        Self {
            raw: lilt::Animated::new(state),
            duration: Duration::from_millis(100),
        }
    }

    /// 设置 [`Animation`] 的 [`Easing`] 函数。
    ///
    /// 详见[缓动函数速查表](https://easings.net)！
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the [`Easing`] function of the [`Animation`].
    ///
    /// See the [Easing Functions Cheat Sheet](https://easings.net) for
    /// details!
    ///
    /// </details>
    pub fn easing(mut self, easing: Easing) -> Self {
        self.raw = self.raw.easing(easing);
        self
    }

    /// 把 [`Animation`] 的时长设置为 100 毫秒。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the duration of the [`Animation`] to 100ms.
    ///
    /// </details>
    pub fn very_quick(self) -> Self {
        self.duration(Duration::from_millis(100))
    }

    /// 把 [`Animation`] 的时长设置为 200 毫秒。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the duration of the [`Animation`] to 200ms.
    ///
    /// </details>
    pub fn quick(self) -> Self {
        self.duration(Duration::from_millis(200))
    }

    /// 把 [`Animation`] 的时长设置为 400 毫秒。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the duration of the [`Animation`] to 400ms.
    ///
    /// </details>
    pub fn slow(self) -> Self {
        self.duration(Duration::from_millis(400))
    }

    /// 把 [`Animation`] 的时长设置为 500 毫秒。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the duration of the [`Animation`] to 500ms.
    ///
    /// </details>
    pub fn very_slow(self) -> Self {
        self.duration(Duration::from_millis(500))
    }

    /// 把 [`Animation`] 的时长设置为给定的值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets the duration of the [`Animation`] to the given value.
    ///
    /// </details>
    pub fn duration(mut self, duration: Duration) -> Self {
        self.raw = self.raw.duration(duration.as_secs_f32() * 1_000.0);
        self.duration = duration;
        self
    }

    /// 为 [`Animation`] 设置延迟。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Sets a delay for the [`Animation`].
    ///
    /// </details>
    pub fn delay(mut self, duration: Duration) -> Self {
        self.raw = self.raw.delay(duration.as_secs_f64() as f32 * 1000.0);
        self
    }

    /// 让 [`Animation`] 重复给定的次数。
    ///
    /// 重复 1 次表示动画总共播放两遍。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Makes the [`Animation`] repeat a given amount of times.
    ///
    /// Providing 1 repetition plays the animation twice in total.
    ///
    /// </details>
    pub fn repeat(mut self, repetitions: u32) -> Self {
        self.raw = self.raw.repeat(repetitions);
        self
    }

    /// 让 [`Animation`] 无限重复。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Makes the [`Animation`] repeat forever.
    ///
    /// </details>
    pub fn repeat_forever(mut self) -> Self {
        self.raw = self.raw.repeat_forever();
        self
    }

    /// 让 [`Animation`] 在重复时自动反向播放。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Makes the [`Animation`] automatically reverse when repeating.
    ///
    /// </details>
    pub fn auto_reverse(mut self) -> Self {
        self.raw = self.raw.auto_reverse();
        self
    }

    /// 在给定的时刻，让 [`Animation`] 从当前状态过渡到
    /// 给定的新状态。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Transitions the [`Animation`] from its current state to the given new state
    /// at the given time.
    ///
    /// </details>
    pub fn go(mut self, new_state: T, at: Instant) -> Self {
        self.go_mut(new_state, at);
        self
    }

    /// 在给定的时刻，让 [`Animation`] 从当前状态过渡到
    /// 给定的新状态（通过引用）。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Transitions the [`Animation`] from its current state to the given new state
    /// at the given time, by reference.
    ///
    /// </details>
    pub fn go_mut(&mut self, new_state: T, at: Instant) {
        self.raw.transition(new_state, at);
    }

    /// 如果 [`Animation`] 当前正在进行中，返回 true。
    ///
    /// [`Animation`] 正在向另一个状态过渡时，即为进行中。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns true if the [`Animation`] is currently in progress.
    ///
    /// An [`Animation`] is in progress when it is transitioning to a different state.
    ///
    /// </details>
    pub fn is_animating(&self, at: Instant) -> bool {
        self.raw.in_progress(at)
    }

    /// 把 [`Animation`] 投影为给定 [`Instant`] 时刻的插值；使用给定的
    /// 闭包计算插值的各个关键帧。
    ///
    /// 如果 [`Animation`] 的状态是 `bool`，可以使用更简单的 [`interpolate`] 方法。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Projects the [`Animation`] into an interpolated value at the given [`Instant`]; using the
    /// closure provided to calculate the different keyframes of interpolated values.
    ///
    /// If the [`Animation`] state is a `bool`, you can use the simpler [`interpolate`] method.
    ///
    /// </details>
    ///
    /// [`interpolate`]: Animation::interpolate
    pub fn interpolate_with<I>(&self, f: impl Fn(T) -> I, at: Instant) -> I
    where
        I: Interpolable,
    {
        self.raw.animate(f, at)
    }

    /// 返回 [`Animation`] 的当前状态。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Retuns the current state of the [`Animation`].
    ///
    /// </details>
    pub fn value(&self) -> T {
        self.raw.value
    }
}

impl Animation<f32> {
    /// 把 [`Animation`] 投影为给定 [`Instant`] 时刻的插值。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Projects the [`Animation`] into an interpolated value at the given [`Instant`].
    ///
    /// </details>
    pub fn interpolate(&self, at: Instant) -> f32 {
        self.interpolate_with(std::convert::identity, at)
    }
}

impl Animation<bool> {
    /// 把 [`Animation`] 投影为给定 [`Instant`] 时刻的插值；以
    /// `start` 和 `end` 作为起始和目标关键帧。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Projects the [`Animation`] into an interpolated value at the given [`Instant`]; using the
    /// `start` and `end` values as the origin and destination keyframes.
    ///
    /// </details>
    pub fn interpolate<I>(&self, start: I, end: I, at: Instant) -> I
    where
        I: Interpolable + Clone,
    {
        self.raw.animate_bool(start, end, at)
    }

    /// 返回 [`Animation`] 剩余的 [`Duration`]。
    ///
    /// <details>
    /// <summary>显示英文原文</summary>
    ///
    /// Returns the remaining [`Duration`] of the [`Animation`].
    ///
    /// </details>
    pub fn remaining(&self, at: Instant) -> Duration {
        Duration::from_secs_f32(self.interpolate(self.duration.as_secs_f32(), 0.0, at))
    }
}
