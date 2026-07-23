//! 对齐: `cn.hutool.core.date.chinese.LunarFestival`

#![allow(dead_code)]

use crate::Result;

/// 农历节日表（月,日）→ 名称。
fn festivals() -> &'static [((i32, i32), &'static str)] {
    &[
        ((1, 1), "春节"),
        ((1, 15), "元宵节 上元节"),
        ((5, 5), "端午节 端阳节"),
        ((7, 7), "七夕"),
        ((8, 15), "中秋节"),
        ((9, 9), "重阳节"),
        ((12, 8), "腊八节"),
        ((12, 23), "小年"),
        ((12, 30), "除夕"),
    ]
}

/// 对齐 Java: `cn.hutool.core.date.chinese.LunarFestival`
#[derive(Debug, Clone, Copy, Default)]
pub struct LunarFestival;

impl LunarFestival {
    /// 获取节日（多个用逗号连接；简化表覆盖常用节日）。
    pub fn get_festivals(year: i32, month: i32, day: i32) -> String {
        let _ = year;
        let mut found = Vec::new();
        for ((m, d), name) in festivals() {
            if *m == month && *d == day {
                found.push(*name);
            }
        }
        // 除夕：腊月最后一天
        if month == 12 {
            let last = crate::date::chinese::lunar_info::LunarInfo::month_days(year, 12);
            if day == last && !found.iter().any(|n| n.contains("除夕")) {
                // only if day is last day of 腊月
                if day == 29 || day == 30 {
                    if festivals().iter().any(|((m, d), _)| *m == 12 && *d == 30) && day != 30 {
                        // 腊月廿九也可能是除夕
                        found.push("除夕");
                    }
                }
            }
        }
        found.join(",")
    }

    /// 兼容 sentinel。
    pub fn sentinel() -> Result<()> {
        Ok(())
    }
}
