//! 对齐: `cn.hutool.core.map.TableMap`
//! 来源: hutool-core/src/main/java/cn/hutool/core/map/TableMap.java

/// 对齐 Java 类: `cn.hutool.core.map.TableMap`
///
/// 可重复键/值的 Map：键值分列存储，支持正向与反向多值查找。
#[derive(Debug, Clone)]
pub struct TableMap<K, V> {
    keys: Vec<K>,
    values: Vec<V>,
}

impl<K: PartialEq, V: PartialEq> TableMap<K, V> {
    /// 对齐 Java: `TableMap()`
    pub fn new() -> Self {
        Self::with_capacity(10)
    }

    /// 对齐 Java: `TableMap(int size)`
    pub fn with_capacity(size: usize) -> Self {
        Self {
            keys: Vec::with_capacity(size),
            values: Vec::with_capacity(size),
        }
    }

    /// 对齐 Java: `TableMap(K[], V[])`
    pub fn from_pairs(keys: Vec<K>, values: Vec<V>) -> Self {
        Self { keys, values }
    }

    /// 对齐 Java: `size()`
    pub fn len(&self) -> usize {
        self.keys.len()
    }

    /// 对齐 Java: `isEmpty()`
    pub fn is_empty(&self) -> bool {
        self.keys.is_empty()
    }

    /// 对齐 Java: `containsKey`
    pub fn contains_key(&self, key: &K) -> bool {
        self.keys.contains(key)
    }

    /// 对齐 Java: `containsValue`
    pub fn contains_value(&self, value: &V) -> bool {
        self.values.contains(value)
    }

    /// 对齐 Java: `get` —— 返回第一个匹配键的值。
    pub fn get(&self, key: &K) -> Option<&V> {
        self.keys
            .iter()
            .position(|k| k == key)
            .map(|i| &self.values[i])
    }

    /// 对齐 Java: `getKey`
    pub fn get_key(&self, value: &V) -> Option<&K> {
        self.values
            .iter()
            .position(|v| v == value)
            .map(|i| &self.keys[i])
    }

    /// 对齐 Java: `getValues`
    pub fn get_values(&self, key: &K) -> Vec<&V> {
        self.keys
            .iter()
            .enumerate()
            .filter(|(_, k)| *k == key)
            .map(|(i, _)| &self.values[i])
            .collect()
    }

    /// 对齐 Java: `getKeys`
    pub fn get_keys(&self, value: &V) -> Vec<&K> {
        self.values
            .iter()
            .enumerate()
            .filter(|(_, v)| *v == value)
            .map(|(i, _)| &self.keys[i])
            .collect()
    }

    /// 对齐 Java: `put` —— 始终追加，允许重复键。
    pub fn put(&mut self, key: K, value: V) -> Option<V> {
        self.keys.push(key);
        self.values.push(value);
        None
    }

    /// 对齐 Java: `remove` —— 移除所有匹配键，返回最后移除的值。
    pub fn remove(&mut self, key: &K) -> Option<V>
    where
        V: Clone,
    {
        let mut last = None;
        while let Some(idx) = self.keys.iter().position(|k| k == key) {
            last = Some(self.remove_by_index(idx));
        }
        last
    }

    /// 对齐 Java: `removeByIndex`
    pub fn remove_by_index(&mut self, index: usize) -> V {
        self.keys.remove(index);
        self.values.remove(index)
    }

    /// 对齐 Java: `putAll`
    pub fn put_all(&mut self, entries: impl IntoIterator<Item = (K, V)>) {
        for (k, v) in entries {
            self.put(k, v);
        }
    }

    /// 对齐 Java: `clear`
    pub fn clear(&mut self) {
        self.keys.clear();
        self.values.clear();
    }

    /// 对齐 Java: `keys()` —— 可重复键列表。
    pub fn keys(&self) -> &[K] {
        &self.keys
    }

    /// 对齐 Java: `values()`
    pub fn values(&self) -> &[V] {
        &self.values
    }

    /// 对齐 Java: `keySet` —— 去重键。
    pub fn key_set(&self) -> Vec<&K> {
        let mut seen = Vec::new();
        for k in &self.keys {
            if !seen.contains(&k) {
                seen.push(k);
            }
        }
        seen
    }

    /// 对齐 Java: `entrySet` / 迭代
    pub fn iter(&self) -> impl Iterator<Item = (&K, &V)> {
        self.keys.iter().zip(self.values.iter())
    }

    /// 对齐 Java: `forEach`
    pub fn for_each<F>(&self, mut action: F)
    where
        F: FnMut(&K, &V),
    {
        for (k, v) in self.iter() {
            action(k, v);
        }
    }

    /// 对齐 Java: `remove(K, V)`
    pub fn remove_entry(&mut self, key: &K, value: &V) -> bool {
        let mut removed = false;
        let mut i = 0;
        while i < self.len() {
            if &self.keys[i] == key && &self.values[i] == value {
                self.remove_by_index(i);
                removed = true;
            } else {
                i += 1;
            }
        }
        removed
    }

    /// 对齐 Java: `replaceAll`
    pub fn replace_all<F>(&mut self, mut function: F)
    where
        F: FnMut(&K, &V) -> V,
    {
        for i in 0..self.len() {
            let new_v = function(&self.keys[i], &self.values[i]);
            self.values[i] = new_v;
        }
    }

    /// 对齐 Java: `replace(K, V, V)`
    pub fn replace_old(&mut self, key: &K, old_value: &V, new_value: V) -> bool {
        for i in 0..self.len() {
            if &self.keys[i] == key && &self.values[i] == old_value {
                self.values[i] = new_value;
                return true;
            }
        }
        false
    }

    /// 对齐 Java: `replace(K, V)` —— 替换所有匹配键。
    pub fn replace(&mut self, key: &K, value: V) -> Option<V>
    where
        V: Clone,
    {
        let mut last = None;
        for i in 0..self.len() {
            if &self.keys[i] == key {
                last = Some(std::mem::replace(&mut self.values[i], value.clone()));
            }
        }
        last
    }

    /// 对齐 Java: `computeIfPresent`
    pub fn compute_if_present<F>(&mut self, key: &K, mut remapping: F) -> Option<V>
    where
        F: FnMut(&K, &V) -> Option<V>,
        V: Clone,
    {
        let mut last = None;
        let mut i = 0;
        while i < self.len() {
            if &self.keys[i] == key {
                match remapping(key, &self.values[i]) {
                    Some(new_v) => {
                        last = Some(std::mem::replace(&mut self.values[i], new_v));
                        i += 1;
                    }
                    None => {
                        last = Some(self.remove_by_index(i));
                    }
                }
            } else {
                i += 1;
            }
        }
        last
    }
}

impl<K: PartialEq, V: PartialEq> Default for TableMap<K, V> {
    fn default() -> Self {
        Self::new()
    }
}
