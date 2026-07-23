//! 对齐: `cn.hutool.core.date.DateRange`
//! 来源: hutool-core DateRange（按 DateField 步进的日期区间）

use crate::date::date_field::DateField;
use crate::date::date_time::DateTime;

/// 对齐 Java: `cn.hutool.core.date.DateRange`
#[derive(Debug, Clone)]
pub struct DateRange {
    start: DateTime,
    end: DateTime,
    field: DateField,
    step: i64,
    include_start: bool,
    include_end: bool,
}

impl DateRange {
    /// 对齐 Java: `DateRange(Date, Date, DateField)` — 步长 1，含起止。
    pub fn new(start: DateTime, end: DateTime, field: DateField) -> Self {
        Self::with_step(start, end, field, 1)
    }

    /// 对齐 Java: `DateRange(Date, Date, DateField, int)`
    pub fn with_step(start: DateTime, end: DateTime, field: DateField, step: i64) -> Self {
        Self::with_includes(start, end, field, step, true, true)
    }

    /// 对齐 Java: `DateRange(Date, Date, DateField, int, boolean, boolean)`
    pub fn with_includes(
        start: DateTime,
        end: DateTime,
        field: DateField,
        step: i64,
        include_start: bool,
        include_end: bool,
    ) -> Self {
        Self {
            start,
            end,
            field,
            step,
            include_start,
            include_end,
        }
    }

    /// 起始日期。
    pub fn start(&self) -> DateTime {
        self.start
    }

    /// 结束日期。
    pub fn end(&self) -> DateTime {
        self.end
    }

    /// 步进字段。
    pub fn field(&self) -> DateField {
        self.field
    }

    /// 步进数。
    pub fn step(&self) -> i64 {
        self.step
    }

    /// 转为列表（对齐 Hutool Range 迭代语义；`step <= 0` 仅返回起点或空）。
    pub fn to_list(&self) -> Vec<DateTime> {
        // issue#3783：step <= 0 时步进器返回 null，仅可能保留起点
        if self.step <= 0 {
            return if self.include_start {
                vec![self.start]
            } else {
                Vec::new()
            };
        }

        let mut out = Vec::new();
        if self.include_start {
            out.push(self.start);
        }

        let mut index = 0i64;
        loop {
            index += 1;
            let next = self.start.offset_new(self.field, index * self.step);
            if next.is_after(self.end) {
                break;
            }
            if next == self.end && !self.include_end {
                break;
            }
            out.push(next);
            if out.len() > 10_000 {
                break;
            }
        }
        out
    }
}
