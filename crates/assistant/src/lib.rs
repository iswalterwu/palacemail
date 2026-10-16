//! `palace-assistant` —— AI 助手层。
//!
//! 职责：
//!
//! - 供应商抽象：只适配 OpenAI 兼容与 Anthropic Messages 两种线协议，
//!   按 `fast` / `smart` / `embed` 角色路由，配置写角色不写型号；
//! - 提示词注册表与确定性的上下文组装（按 token 预算从粗到细填充）；
//! - 摘要、分类、检索、起草等流水线：消费 `store` 的作业队列，结果写回派生表；
//! - 模型输出的动作只成为提议，交由统一命令层校验后执行。
//!
//! 边界规则：只依赖 `palace-store`，不碰 IMAP；永不读取原始 MIME。
//!
//! 目前仅为骨架，尚未接入任何模型。

/// 占位：返回从本层开始、自上而下的依赖链。
pub fn dependency_chain() -> Vec<&'static str> {
    let mut chain = vec!["assistant"];
    chain.extend(palace_store::dependency_chain());
    chain
}

#[cfg(test)]
mod tests {
    use super::dependency_chain;

    #[test]
    fn assistant_sits_directly_above_store() {
        assert_eq!(dependency_chain(), ["assistant", "store", "mail"]);
    }
}
