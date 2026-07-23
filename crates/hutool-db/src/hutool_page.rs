//! 分页请求 —— 对齐 Hutool `cn.hutool.db.Page`（0-based page number）。

use crate::sql::order::Order;

/// Hutool 风格分页对象（页码从 0 开始）。
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HutoolPage {
    page_number: u32,
    page_size: u32,
    orders: Vec<Order>,
}

impl HutoolPage {
    /// 默认每页条数 —— 对齐 Hutool `Page.DEFAULT_PAGE_SIZE`。
    pub const DEFAULT_PAGE_SIZE: u32 = 20;

    /// 对齐 Java: `Page.of(int pageNumber, int pageSize)`.
    #[must_use]
    pub fn of(page_number: u32, page_size: u32) -> Self {
        Self {
            page_number,
            page_size: if page_size == 0 {
                Self::DEFAULT_PAGE_SIZE
            } else {
                page_size
            },
            orders: Vec::new(),
        }
    }

    /// 对齐 Java: `Page.addOrder(Order...)`.
    pub fn add_order(&mut self, order: Order) -> &mut Self {
        self.orders.push(order);
        self
    }

    /// 对齐 Java: `Page.getOrders()`.
    #[must_use]
    pub fn orders(&self) -> &[Order] {
        &self.orders
    }

    /// 对齐 Java: `Page.getPageNumber()`.
    #[must_use]
    pub fn page_number(&self) -> u32 {
        self.page_number
    }

    /// 对齐 Java: `Page.getPageSize()`.
    #[must_use]
    pub fn page_size(&self) -> u32 {
        self.page_size
    }

    /// 对齐 Java: `Page.getStartEnd()` —— `[start, end]` 右开区间 end 为 offset+size。
    #[must_use]
    pub fn start_end(&self) -> [u32; 2] {
        let start = self.page_number.saturating_mul(self.page_size);
        [start, start + self.page_size]
    }
}
