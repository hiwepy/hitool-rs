//! 对齐: `cn.hutool.core.lang.Chain`
//! 来源: hutool-core/src/main/java/cn/hutool/core/lang/Chain.java

/// 对齐 Java: `Chain<E, T>` — 可迭代责任链，`add_chain` 返回 `Self` 以链式调用。
pub trait Chain<E>: IntoIterator {
    /// 对齐 Java: `addChain(E)` — 追加环节并返回自身。
    fn add_chain(self, element: E) -> Self
    where
        Self: Sized;
}

/// 基于 `Vec` 的简易责任链实现。
#[derive(Debug, Clone, Default)]
pub struct VecChain<E> {
    items: Vec<E>,
}

impl<E> VecChain<E> {
    /// 创建空链。
    #[must_use]
    pub fn new() -> Self {
        Self { items: Vec::new() }
    }

    /// 当前环节列表。
    #[must_use]
    pub fn as_slice(&self) -> &[E] {
        &self.items
    }
}

impl<E> Chain<E> for VecChain<E> {
    fn add_chain(mut self, element: E) -> Self {
        self.items.push(element);
        self
    }
}

impl<E> IntoIterator for VecChain<E> {
    type Item = E;
    type IntoIter = std::vec::IntoIter<E>;

    fn into_iter(self) -> Self::IntoIter {
        self.items.into_iter()
    }
}

#[cfg(test)]
mod chain_idiomatic_parity {
    use super::*;

    #[test]
    fn vec_chain_add_and_iterate() {
        let chain = VecChain::new().add_chain("a").add_chain("b");
        assert_eq!(chain.as_slice(), &["a", "b"]);
        let collected: Vec<_> = chain.into_iter().collect();
        assert_eq!(collected, vec!["a", "b"]);
    }
}
