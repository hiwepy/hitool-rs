//! 分页结果 —— 对齐 Hutool `cn.hutool.db.PageResult`。

use crate::entity::Entity;

/// Hutool 风格分页结果（页码从 0 开始）。
#[derive(Debug, Clone, Default, PartialEq)]
pub struct PageResult {
    records: Vec<Entity>,
    page: u32,
    page_size: u32,
    total_page: u32,
    total: u64,
}

impl PageResult {
    /// 对齐 Java: `PageResult(int page, int pageSize, int total)`.
    #[must_use]
    pub fn new(page: u32, page_size: u32, total: u64, records: Vec<Entity>) -> Self {
        let page_size = if page_size == 0 {
            crate::hutool_page::HutoolPage::DEFAULT_PAGE_SIZE
        } else {
            page_size
        };
        let total_page = if total == 0 {
            0
        } else {
            total.div_ceil(u64::from(page_size)) as u32
        };
        Self {
            records,
            page,
            page_size,
            total_page,
            total,
        }
    }

    /// 对齐 Java: `PageResult.getTotal()`.
    #[must_use]
    pub fn total(&self) -> u64 {
        self.total
    }

    /// 对齐 Java: `PageResult.getTotalPage()`.
    #[must_use]
    pub fn total_page(&self) -> u32 {
        self.total_page
    }

    /// 对齐 Java: `PageResult.size()`.
    #[must_use]
    pub fn size(&self) -> usize {
        self.records.len()
    }

    /// 对齐 Java: `PageResult.isLast()`.
    #[must_use]
    pub fn is_last(&self) -> bool {
        self.page.saturating_add(1) >= self.total_page
    }

    /// 返回记录列表。
    #[must_use]
    pub fn records(&self) -> &[Entity] {
        &self.records
    }
}
