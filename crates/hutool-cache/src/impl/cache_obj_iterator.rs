//! `CacheObjIterator` — 对齐 `cn.hutool.cache.CacheObjIterator`。
use crate::compat::CacheObj;
use std::vec::IntoIter;

pub struct CacheObjIterator<K, V> { inner: IntoIter<CacheObj<K, V>> }
impl<K, V> CacheObjIterator<K, V> {
    pub fn new(items: Vec<CacheObj<K, V>>) -> Self { Self { inner: items.into_iter() } }
}
impl<K, V> Iterator for CacheObjIterator<K, V> {
    type Item = CacheObj<K, V>;
    fn next(&mut self) -> Option<Self::Item> { self.inner.next() }
}
