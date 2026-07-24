//! `CacheValuesIterator` — 对齐 `cn.hutool.cache.CacheValuesIterator`。
use std::vec::IntoIter;

pub struct CacheValuesIterator<V> { inner: IntoIter<V> }
impl<V> CacheValuesIterator<V> {
    pub fn new(items: Vec<V>) -> Self { Self { inner: items.into_iter() } }
}
impl<V> Iterator for CacheValuesIterator<V> {
    type Item = V;
    fn next(&mut self) -> Option<Self::Item> { self.inner.next() }
}
