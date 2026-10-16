//! `palace-store` —— 唯一事实来源。
//!
//! 职责：
//!
//! - 规范化消息、会话、联系人：入站原始消息在一个事务内解析为结构化头部、
//!   干净正文与附件清单，并更新会话与联系人统计；
//! - 单个 SQLite 库：结构化表 + FTS5 全文（中文预分词）+ sqlite-vec 向量，
//!   原始 MIME 与附件按内容哈希落盘；
//! - 派生数据表：摘要、分类、嵌入等 AI 产物以（对象、任务、模型、提示词版本、输入哈希）为键；
//! - 作业队列与变更事件；混合检索接口（全文 + 向量 + 结构化过滤）。
//!
//! 所有跨层数据类型由本层定义。边界规则：只依赖 `palace-mail`，不碰模型调用。
//!
//! 目前仅为骨架，尚未定义任何 SQLite 模式。

/// 占位：返回从本层开始、自上而下的依赖链。
pub fn dependency_chain() -> Vec<&'static str> {
    let mut chain = vec!["store"];
    chain.extend(palace_mail::dependency_chain());
    chain
}

#[cfg(test)]
mod tests {
    use super::dependency_chain;

    #[test]
    fn store_sits_directly_above_mail() {
        assert_eq!(dependency_chain(), ["store", "mail"]);
    }
}
