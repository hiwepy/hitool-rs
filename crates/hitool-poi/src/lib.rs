//! `hitool-poi` — 对齐 Hutool `cn.hutool.poi.*` 的 Office 文档处理模块。
//!
//! ## 当前状态
//!
//! 🟡 **占位骨架**：本 crate 提供完整的 API 形状（78 个 `.rs` 文件对应 78 个 Java `.java`），
//! 但所有方法实现为 `unimplemented!()` 桩。等待以下引擎完成后填充实现：
//!
//! - `easyexcel-rs`：Excel 读写（对应 `cn.hutool.poi.excel.*`）
//! - `easydoc-rs`：Word 读写（对应 `cn.hutool.poi.word.*`）
//! - `easyofd-rs`：OFD 写入（对应 `cn.hutool.poi.ofd.*`）
//! - `easypdf-rs`：PDF 处理（对应 `cn.hutool.poi.pdf.*`）
//!
//! ## 模块结构
//!
//! ```text
//! hitool-poi/src/
//! ├── lib.rs                    ← 本文件（目录索引）
//! ├── excel/                    ← cn.hutool.poi.excel
//! │   ├── excel_util.rs         ← ExcelUtil（静态门面）
//! │   ├── excel_reader.rs       ← ExcelReader
//! │   ├── excel_writer.rs       ← ExcelWriter
//! │   ├── big_excel_writer.rs   ← BigExcelWriter（SXSSF）
//! │   ├── cell/                 ← Cell 相关接口和值类型
//! │   │   ├── setters/          ← CellSetter 实现
//! │   │   └── values/           ← CellValue 实现
//! │   ├── editors/              ← CellEditor 实现
//! │   ├── reader/               ← SheetReader 实现
//! │   ├── sax/                  ← SAX 流式读取
//! │   │   └── handler/          ← RowHandler 实现
//! │   └── style/                ← 样式工具
//! ├── exceptions/               ← POIException
//! ├── ofd/                      ← OfdWriter
//! └── word/                     ← Word07Writer / WordUtil
//! ```

#![forbid(unsafe_code)]

pub mod excel;
pub mod exceptions;
pub mod ofd;
pub mod word;

// 顶层 POI 类
mod global_poi_config;
mod poi_checker;

pub use global_poi_config::GlobalPoiConfig;
pub use poi_checker::PoiChecker;