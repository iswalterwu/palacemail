//! `palace-mail` —— 邮件协议层。
//!
//! 位于四层结构（ui → assistant → store → mail）的最底层，职责：
//!
//! - IMAP/SMTP 适配与 OAuth 登录；
//! - 文件夹增量同步（UIDVALIDITY、CONDSTORE、IDLE）；
//! - 出站队列：把发送、归档、打标等动作异步同步到服务器，失败时回滚并通知。
//!
//! 边界规则：本层不依赖任何其他 PalaceMail crate，也不知道 AI 的存在；
//! 它只向 `store` 交付原始消息，并执行 `store` 下发的出站动作。
//!
//! 目前仅为骨架，尚未实现任何协议逻辑。

/// 占位：返回从本层开始、自上而下的依赖链。
///
/// `mail` 是最底层，因此链中只有它自己。
pub fn dependency_chain() -> Vec<&'static str> {
    vec!["mail"]
}

#[cfg(test)]
mod tests {
    use super::dependency_chain;

    #[test]
    fn mail_is_the_bottom_layer() {
        assert_eq!(dependency_chain(), ["mail"]);
    }
}
