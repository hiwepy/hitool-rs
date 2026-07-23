//! 对齐: `cn.hutool.core.annotation.Hierarchical`

use std::cmp::Ordering;

/// 对齐 Java interface: `cn.hutool.core.annotation.Hierarchical`
pub trait Hierarchical: Send + Sync {
    /// 参照物。
    fn get_root(&self) -> Option<&dyn std::any::Any>;

    /// 与参照物的垂直距离。
    fn get_vertical_distance(&self) -> i32;

    /// 与参照物的水平距离。
    fn get_horizontal_distance(&self) -> i32;

    /// 默认层级比较器。
    fn compare_hierarchical(&self, other: &dyn Hierarchical) -> Ordering {
        self.get_vertical_distance()
            .cmp(&other.get_vertical_distance())
            .then_with(|| {
                self.get_horizontal_distance()
                    .cmp(&other.get_horizontal_distance())
            })
    }
}

/// 层级选择器，对齐 `Hierarchical.Selector`。
pub trait HierarchicalSelector: Send + Sync {
    /// 从两个层级对象中选择一个。
    fn choose(&self, prev: &dyn Hierarchical, next: &dyn Hierarchical) -> ChooseSide;
}

/// 选择结果侧。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ChooseSide {
    Prev,
    Next,
}

/// 更近且更旧优先。
#[derive(Debug, Clone, Copy, Default)]
pub struct NearestAndOldestPrioritySelector;

impl HierarchicalSelector for NearestAndOldestPrioritySelector {
    fn choose(&self, prev: &dyn Hierarchical, next: &dyn Hierarchical) -> ChooseSide {
        if next.get_vertical_distance() < prev.get_vertical_distance() {
            ChooseSide::Next
        } else {
            ChooseSide::Prev
        }
    }
}

/// 更近且更新优先。
#[derive(Debug, Clone, Copy, Default)]
pub struct NearestAndNewestPrioritySelector;

impl HierarchicalSelector for NearestAndNewestPrioritySelector {
    fn choose(&self, prev: &dyn Hierarchical, next: &dyn Hierarchical) -> ChooseSide {
        if next.get_vertical_distance() <= prev.get_vertical_distance() {
            ChooseSide::Next
        } else {
            ChooseSide::Prev
        }
    }
}

/// 更远且更旧优先。
#[derive(Debug, Clone, Copy, Default)]
pub struct FarthestAndOldestPrioritySelector;

impl HierarchicalSelector for FarthestAndOldestPrioritySelector {
    fn choose(&self, prev: &dyn Hierarchical, next: &dyn Hierarchical) -> ChooseSide {
        if next.get_vertical_distance() > prev.get_vertical_distance() {
            ChooseSide::Next
        } else {
            ChooseSide::Prev
        }
    }
}

/// 更远且更新优先。
#[derive(Debug, Clone, Copy, Default)]
pub struct FarthestAndNewestPrioritySelector;

impl HierarchicalSelector for FarthestAndNewestPrioritySelector {
    fn choose(&self, prev: &dyn Hierarchical, next: &dyn Hierarchical) -> ChooseSide {
        if next.get_vertical_distance() >= prev.get_vertical_distance() {
            ChooseSide::Next
        } else {
            ChooseSide::Prev
        }
    }
}

/// 默认比较器函数。
pub fn default_hierarchical_cmp(a: &dyn Hierarchical, b: &dyn Hierarchical) -> Ordering {
    a.compare_hierarchical(b)
}
