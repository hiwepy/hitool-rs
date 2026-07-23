//! 对齐: `cn.hutool.core.util.ModifierUtil`
//! 来源: hutool-core/src/main/java/cn/hutool/core/util/ModifierUtil.java

/// 对齐 Java: `ModifierUtil.ModifierType`
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModifierType {
    /// public
    Public,
    /// private
    Private,
    /// protected
    Protected,
    /// static
    Static,
    /// final
    Final,
    /// synchronized
    Synchronized,
    /// volatile
    Volatile,
    /// transient
    Transient,
    /// native
    Native,
    /// abstract
    Abstract,
    /// strictfp
    Strict,
}

/// 对齐 Java `Method.getModifiers()` 的位标记集合。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Modifiers(u32);

impl Modifiers {
    const PUBLIC: u32 = 1 << 0;
    const PRIVATE: u32 = 1 << 1;
    const PROTECTED: u32 = 1 << 2;
    const STATIC: u32 = 1 << 3;
    const FINAL: u32 = 1 << 4;
    const SYNCHRONIZED: u32 = 1 << 5;
    const VOLATILE: u32 = 1 << 6;
    const TRANSIENT: u32 = 1 << 7;
    const NATIVE: u32 = 1 << 8;
    const ABSTRACT: u32 = 1 << 9;
    const STRICT: u32 = 1 << 10;

    /// 构造修饰符集合。
    #[must_use]
    pub fn new(flags: u32) -> Self {
        Self(flags)
    }

    /// 判断包含指定修饰符。
    #[must_use]
    pub fn has(&self, modifier_type: ModifierType) -> bool {
        self.0 & flag(modifier_type) != 0
    }
}

/// 对齐 Java `Method` 的轻量描述。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MethodDescriptor {
    /// 修饰符位标记。
    pub modifiers: Modifiers,
}

/// 对齐 Java: `cn.hutool.core.util.ModifierUtil`
#[derive(Debug, Clone, Copy, Default)]
pub struct ModifierUtil;

impl ModifierUtil {
    /// 对齐 Java: `ModifierUtil.hasModifier(Member, ModifierType...)`
    #[must_use]
    pub fn has_modifier(method: &MethodDescriptor, modifier_types: &[ModifierType]) -> bool {
        modifier_types
            .iter()
            .any(|modifier| method.modifiers.has(*modifier))
    }

    /// 对齐 Java: `ModifierUtil.hasAllModifiers(Member, ModifierType...)`
    #[must_use]
    pub fn has_all_modifiers(method: &MethodDescriptor, modifier_types: &[ModifierType]) -> bool {
        modifier_types
            .iter()
            .all(|modifier| method.modifiers.has(*modifier))
    }
}

const fn flag(modifier_type: ModifierType) -> u32 {
    match modifier_type {
        ModifierType::Public => Modifiers::PUBLIC,
        ModifierType::Private => Modifiers::PRIVATE,
        ModifierType::Protected => Modifiers::PROTECTED,
        ModifierType::Static => Modifiers::STATIC,
        ModifierType::Final => Modifiers::FINAL,
        ModifierType::Synchronized => Modifiers::SYNCHRONIZED,
        ModifierType::Volatile => Modifiers::VOLATILE,
        ModifierType::Transient => Modifiers::TRANSIENT,
        ModifierType::Native => Modifiers::NATIVE,
        ModifierType::Abstract => Modifiers::ABSTRACT,
        ModifierType::Strict => Modifiers::STRICT,
    }
}

/// parity 测试用：`private static void ddd()`
#[must_use]
pub fn parity_ddd_method() -> MethodDescriptor {
    MethodDescriptor {
        modifiers: Modifiers::new(Modifiers::PRIVATE | Modifiers::STATIC),
    }
}
