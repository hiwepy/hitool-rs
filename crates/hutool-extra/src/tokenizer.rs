//! 中文分词 facade，对齐 hutool 的 `cn.hutool.extra.tokenizer.*`。
//!
//! 提供 TokenizerEngine trait + TokenizerResult 抽象 + Word trait + TokenizerUtil 静态门面。
//! 各具体分词引擎（Ansj/HanLP/IKAnalyzer/Jcseg/Jieba/MMSeg/MynNLP/Word/Analysis）
//! 需要外部 Java crate，属于 unsafe-to-copy，暂不在 hutool-extra 中实现。

use crate::HutoolException;

/// 分词结果接口，对齐 `cn.hutool.extra.tokenizer.Result`。
///
/// 注意：模块内部使用 `TokenizerResult` 而非 `Result`，避免与 `std::result::Result` 冲突。
pub trait TokenizerResult: Send + Sync {
    /// 是否还有下一个词
    fn has_next(&self) -> bool;

    /// 获取下一个词
    fn next_word(&self) -> Option<Box<dyn Word>>;
}

/// 抽象分词结果，对齐 `cn.hutool.extra.tokenizer.AbstractResult`。
pub trait AbstractResult: TokenizerResult {}

/// 词接口，对齐 `cn.hutool.extra.tokenizer.Word`。
pub trait Word: Send + Sync + std::fmt::Debug {
    /// 获取词文本
    fn get_text(&self) -> String;

    /// 获取起始位置
    fn get_start_offset(&self) -> i32;

    /// 获取结束位置
    fn get_end_offset(&self) -> i32;

    /// 转字符串
    fn to_string_repr(&self) -> String {
        self.get_text()
    }
}

/// 分词引擎抽象，对齐 `cn.hutool.extra.tokenizer.TokenizerEngine`。
pub trait TokenizerEngine: Send + Sync {
    /// 对文本进行分词
    fn parse(&self, text: &str) -> std::result::Result<Box<dyn TokenizerResult>, HutoolException>;
}

/// 分词工具类，对齐 `cn.hutool.extra.tokenizer.TokenizerUtil`。
pub struct TokenizerUtil;

impl TokenizerUtil {
    /// 对齐 `TokenizerUtil.createEngine()`：根据默认配置创建分词引擎
    pub fn create_engine() -> std::result::Result<Box<dyn TokenizerEngine>, HutoolException> {
        Err(HutoolException::Message(
            "TokenizerUtil::create_engine requires a concrete TokenizerEngine implementation".into(),
        ))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tokenizer_util_create_engine_not_implemented() {
        let r = TokenizerUtil::create_engine();
        assert!(r.is_err());
    }
}