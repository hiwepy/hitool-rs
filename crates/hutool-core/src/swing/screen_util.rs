//! 屏幕工具 —— 对齐 `cn.hutool.core.swing.ScreenUtil`
//!
//! 来源: hutool-core/src/main/java/cn/hutool/core/swing/ScreenUtil.java
//!
//! Hutool 的 `ScreenUtil` 通过 `java.awt.Toolkit` 查询屏幕尺寸,
//! 并通过 `Robot.createScreenCapture` 进行屏幕截图。
//!
//! # Rust 化迁移
//!
//! Java 的 `Toolkit.getDefaultToolkit().getScreenSize()` → Rust [`xcap::Monitor`]
//! (跨平台屏幕枚举与捕获)。
//!
//! Java 的 `BufferedImage` → Rust `image::RgbaImage`(xcap 的原生产物)。
//!
//! Java 的 `java.awt.Rectangle` → Rust [`ScreenRect`]。
//!
//! # 多显示器注意
//!
//! Hutool 只返回"主屏幕"信息,`xcap` 提供所有显示器枚举,Rust 版本通过
//! [`ScreenUtil::primary_monitor`] 选择主显示器,其余显示器可由调用方自行枚举。

use crate::swing::{Result, SwingError};
use image::RgbaImage;
use std::path::Path;

/// 屏幕矩形区域,对应 Java `java.awt.Rectangle`。
///
/// 对齐 Java: `java.awt.Rectangle` 在 ScreenUtil/RobotUtil 路径中的用法
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ScreenRect {
    /// 矩形左上角 X 坐标(像素,相对于显示器虚拟坐标)。
    pub x: i32,
    /// 矩形左上角 Y 坐标(像素,相对于显示器虚拟坐标)。
    pub y: i32,
    /// 矩形宽度(像素)。
    pub width: u32,
    /// 矩形高度(像素)。
    pub height: u32,
}

impl ScreenRect {
    /// 创建一个从原点开始的指定宽高的矩形。
    ///
    /// 对齐 Java: `new Rectangle(int width, int height)`
    #[must_use]
    pub const fn new(width: u32, height: u32) -> Self {
        Self {
            x: 0,
            y: 0,
            width,
            height,
        }
    }

    /// 创建一个指定位置和尺寸的矩形。
    ///
    /// 对齐 Java: `new Rectangle(int x, int y, int width, int height)`
    #[must_use]
    pub const fn with_origin(x: i32, y: i32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

/// Screen facade.
///
/// 对齐 Java: `cn.hutool.core.swing.ScreenUtil`
pub struct ScreenUtil;

impl ScreenUtil {
    /// 获取主显示器宽度(像素)。
    ///
    /// 对齐 Java: `ScreenUtil.getWidth()`
    pub fn get_width() -> Result<u32> {
        let monitor = Self::primary_monitor()?;
        Ok(monitor.width().map_err(screen_err)?)
    }

    /// 获取主显示器高度(像素)。
    ///
    /// 对齐 Java: `ScreenUtil.getHeight()`
    pub fn get_height() -> Result<u32> {
        let monitor = Self::primary_monitor()?;
        Ok(monitor.height().map_err(screen_err)?)
    }

    /// 获取主显示器的矩形区域。
    ///
    /// 对齐 Java: `ScreenUtil.getRectangle()`
    pub fn get_rectangle() -> Result<ScreenRect> {
        let monitor = Self::primary_monitor()?;
        let x = monitor.x().map_err(screen_err)? as i32;
        let y = monitor.y().map_err(screen_err)? as i32;
        let width = monitor.width().map_err(screen_err)?;
        let height = monitor.height().map_err(screen_err)?;
        Ok(ScreenRect::with_origin(x, y, width, height))
    }

    /// 截取主屏幕全屏。
    ///
    /// 对齐 Java: `ScreenUtil.captureScreen()` → `BufferedImage`
    pub fn capture_screen() -> Result<RgbaImage> {
        let monitor = Self::primary_monitor()?;
        monitor
            .capture_image()
            .map_err(|error| SwingError::Screen(error.to_string()))
    }

    /// 截取主屏幕全屏到文件。
    ///
    /// 对齐 Java: `ScreenUtil.captureScreen(File outFile)`
    pub fn capture_screen_to_file(out_file: impl AsRef<Path>) -> Result<()> {
        let image = Self::capture_screen()?;
        image
            .save(out_file.as_ref())
            .map_err(|error| SwingError::Image(error.to_string()))
    }

    /// 截取主屏幕的指定矩形区域。
    ///
    /// 对齐 Java: `ScreenUtil.captureScreen(Rectangle screenRect)` → `BufferedImage`
    ///
    /// # 注意
    ///
    /// `xcap` 的 `Monitor::capture_image` 不直接支持区域参数,Rust 版本
    /// 先截全屏再裁剪。`x`/`y` 是相对于显示器虚拟坐标的偏移。
    pub fn capture_screen_region(rect: ScreenRect) -> Result<RgbaImage> {
        let full = Self::capture_screen()?;
        let monitor = Self::primary_monitor()?;
        let monitor_x = monitor.x().map_err(screen_err)? as i32;
        let monitor_y = monitor.y().map_err(screen_err)? as i32;
        // 把全局坐标转换为相对于显示器原点的坐标
        let rel_x = rect.x.saturating_sub(monitor_x).max(0) as u32;
        let rel_y = rect.y.saturating_sub(monitor_y).max(0) as u32;
        // 用 image crate 裁剪
        let sub = image::imageops::crop_imm(&full, rel_x, rel_y, rect.width, rect.height).to_image();
        Ok(sub)
    }

    /// 截取主屏幕的指定矩形区域到文件。
    ///
    /// 对齐 Java: `ScreenUtil.captureScreen(Rectangle screenRect, File outFile)`
    pub fn capture_screen_region_to_file(rect: ScreenRect, out_file: impl AsRef<Path>) -> Result<()> {
        let image = Self::capture_screen_region(rect)?;
        image
            .save(out_file.as_ref())
            .map_err(|error| SwingError::Image(error.to_string()))
    }

    /// 枚举所有显示器,返回第一个标记为 primary 的显示器。
    /// xcap 不显式暴露 primary 标记,这里按惯例返回 `Monitor::all()` 的第一个。
    fn primary_monitor() -> Result<xcap::Monitor> {
        let monitors = xcap::Monitor::all().map_err(screen_err)?;
        monitors
            .into_iter()
            .next()
            .ok_or_else(|| SwingError::Screen("no monitor available".into()))
    }
}

/// 屏幕错误别名。
pub type ScreenError = SwingError;

fn screen_err(error: xcap::XCapError) -> SwingError {
    SwingError::Screen(error.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn screen_rect_new_aligns_with_java_constructor() {
        let rect = ScreenRect::new(1920, 1080);
        assert_eq!(rect.x, 0);
        assert_eq!(rect.y, 0);
        assert_eq!(rect.width, 1920);
        assert_eq!(rect.height, 1080);
    }

    #[test]
    fn screen_rect_with_origin_aligns_with_java_full_constructor() {
        let rect = ScreenRect::with_origin(-100, 200, 800, 600);
        assert_eq!(rect.x, -100);
        assert_eq!(rect.y, 200);
        assert_eq!(rect.width, 800);
        assert_eq!(rect.height, 600);
    }
}