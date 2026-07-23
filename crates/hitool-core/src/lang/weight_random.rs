//! 对齐: `cn.hutool.core.lang.WeightRandom` / 内部类 `WeightObj`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/WeightRandom.java

use rand::Rng;
use std::hash::{Hash, Hasher};

/// 对齐 Java: `WeightRandom.WeightObj<T>`
#[derive(Debug, Clone)]
pub struct WeightObj<T> {
    obj: T,
    weight: f64,
}

impl<T> WeightObj<T> {
    /// 对齐 Java: `WeightObj(T, double)`
    pub fn new(obj: T, weight: f64) -> Self {
        Self { obj, weight }
    }

    /// 对齐 Java: `getObj`
    pub fn get_obj(&self) -> &T {
        &self.obj
    }

    /// 对齐 Java: `setObj`
    pub fn set_obj(&mut self, obj: T) {
        self.obj = obj;
    }

    /// 对齐 Java: `getWeight`
    #[must_use]
    pub fn get_weight(&self) -> f64 {
        self.weight
    }
}

impl<T: PartialEq> PartialEq for WeightObj<T> {
    fn eq(&self, other: &Self) -> bool {
        self.obj == other.obj
            && self.weight.to_bits() == other.weight.to_bits()
    }
}
impl<T: Eq> Eq for WeightObj<T> {}

impl<T: Hash> Hash for WeightObj<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.obj.hash(state);
        self.weight.to_bits().hash(state);
    }
}

/// 对齐 Java: `WeightRandom<T>`
pub struct WeightRandom<T> {
    items: Vec<WeightObj<T>>,
}

impl<T: Clone> WeightRandom<T> {
    /// 对齐 `WeightRandom.create()` / 无参构造
    pub fn create() -> Self {
        Self { items: Vec::new() }
    }

    /// 对齐 `WeightRandom(WeightObj)`
    pub fn from_obj(weight_obj: WeightObj<T>) -> Self {
        let mut w = Self::create();
        w.add_obj(weight_obj);
        w
    }

    /// 对齐 `WeightRandom(Iterable)` / 数组构造
    pub fn from_iter(weight_objs: impl IntoIterator<Item = WeightObj<T>>) -> Self {
        let mut w = Self::create();
        for o in weight_objs {
            w.add_obj(o);
        }
        w
    }

    /// 对齐 `add(T, double)`
    pub fn add(&mut self, item: T, weight: f64) -> &mut Self {
        self.add_obj(WeightObj::new(item, weight))
    }

    /// 对齐 `add(WeightObj)`
    pub fn add_obj(&mut self, weight_obj: WeightObj<T>) -> &mut Self {
        self.items.push(weight_obj);
        self
    }

    /// 对齐 `clear`
    pub fn clear(&mut self) -> &mut Self {
        self.items.clear();
        self
    }

    /// 对齐 `next()`
    pub fn next(&self) -> Option<T> {
        if self.items.is_empty() {
            return None;
        }
        let total: f64 = self.items.iter().map(|w| w.weight).sum();
        if total <= 0.0 {
            return self.items.last().map(|w| w.obj.clone());
        }
        let mut r = rand::thread_rng().gen_range(0.0..total);
        for w in &self.items {
            if r < w.weight {
                return Some(w.obj.clone());
            }
            r -= w.weight;
        }
        self.items.last().map(|w| w.obj.clone())
    }

    /// 当前权重项数量（测试辅助）。
    #[must_use]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// 是否为空。
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

impl<T: Clone> Default for WeightRandom<T> {
    fn default() -> Self {
        Self::create()
    }
}

#[cfg(test)]
mod weight_random_idiomatic_parity {
    use super::*;

    /// 对齐 Java WeightRandom/WeightObj 可执行证据。
    #[test]
    fn weight_random_add_next_clear_and_weight_obj() {
        let mut wr = WeightRandom::create();
        wr.add("a", 1.0).add("b", 0.0);
        assert_eq!(wr.len(), 2);
        let picked = wr.next().expect("pick");
        assert!(picked == "a" || picked == "b");
        let obj = WeightObj::new("x", 2.5);
        assert_eq!(obj.get_obj(), &"x");
        assert_eq!(obj.get_weight(), 2.5);
        wr.clear();
        assert!(wr.is_empty());
        assert!(wr.next().is_none());
    }
}
