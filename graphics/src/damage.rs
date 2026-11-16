//! 计算帧与帧之间的损伤（damage）。
//!
//! <details>
//! <summary>显示英文原文</summary>
//!
//! Compute the damage between frames.
//!
//! </details>
use crate::core::{Point, Rectangle};

/// 根据先前和当前的图元，求出二者差异对应的损伤区域。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Diffs the damage regions given some previous and current primitives.
///
/// </details>
pub fn diff<T>(
    previous: &[T],
    current: &[T],
    bounds: impl Fn(&T) -> Vec<Rectangle>,
    diff: impl Fn(&T, &T) -> Vec<Rectangle>,
) -> Vec<Rectangle> {
    let damage = previous.iter().zip(current).flat_map(|(a, b)| diff(a, b));

    if previous.len() == current.len() {
        damage.collect()
    } else {
        let (smaller, bigger) = if previous.len() < current.len() {
            (previous, current)
        } else {
            (current, previous)
        };

        // Extend damage by the added/removed primitives
        // 把新增/移除的图元计入损伤区域
        damage
            .chain(bigger[smaller.len()..].iter().flat_map(bounds))
            .collect()
    }
}

/// 根据先前和当前的图元计算损伤区域。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Computes the damage regions given some previous and current primitives.
///
/// </details>
pub fn list<T>(
    previous: &[T],
    current: &[T],
    bounds: impl Fn(&T) -> Vec<Rectangle>,
    are_equal: impl Fn(&T, &T) -> bool,
) -> Vec<Rectangle> {
    diff(previous, current, &bounds, |a, b| {
        if are_equal(a, b) {
            vec![]
        } else {
            bounds(a).into_iter().chain(bounds(b)).collect()
        }
    })
}

/// 在给定的边界内，把彼此靠近的损伤区域
/// 合并成组。
///
/// <details>
/// <summary>显示英文原文</summary>
///
/// Groups the given damage regions that are close together inside the given
/// bounds.
///
/// </details>
pub fn group(mut damage: Vec<Rectangle>, bounds: Rectangle) -> Vec<Rectangle> {
    const AREA_THRESHOLD: f32 = 20_000.0;

    damage.sort_by(|a, b| {
        a.center()
            .distance(Point::ORIGIN)
            .total_cmp(&b.center().distance(Point::ORIGIN))
    });

    let mut output = Vec::new();
    let mut scaled = damage
        .into_iter()
        .filter_map(|region| region.intersection(&bounds))
        .filter(|region| region.width >= 1.0 && region.height >= 1.0);

    if let Some(mut current) = scaled.next() {
        for region in scaled {
            let union = current.union(&region);

            if union.area() - current.area() - region.area() <= AREA_THRESHOLD {
                current = union;
            } else {
                output.push(current);
                current = region;
            }
        }

        output.push(current);
    }

    output
}
