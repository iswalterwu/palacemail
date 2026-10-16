//! `palace-core` —— 内核对外 API。
//!
//! 这是 PalaceMail 内核的唯一入口：桌面壳（Tauri IPC）、未来的无头 CLI、
//! 守护进程与 MCP 服务都只通过本 crate 访问内核。职责：
//!
//! - 统一命令层：校验来自界面或助手的动作，乐观写入 `store`，记录动作日志并支持撤销；
//! - 查询与事件：向上层提供类型化的查询接口与变更事件；
//! - 组装与生命周期：初始化各层、调度作业队列。
//!
//! 边界规则：只依赖 `palace-assistant`，经由它间接到达 `store` 与 `mail`；
//! 内核可脱离界面运行。
//!
//! 目前仅为骨架：只提供版本号与依赖链两个占位函数。

/// 返回内核版本号（取自 Cargo 包版本，与工作区版本一致）。
pub fn version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// 占位：返回自 `core` 起、自上而下的完整依赖链。
pub fn dependency_chain() -> Vec<&'static str> {
    let mut chain = vec!["core"];
    chain.extend(palace_assistant::dependency_chain());
    chain
}

#[cfg(test)]
mod tests {
    use super::{dependency_chain, version};

    #[test]
    fn layers_depend_strictly_downward() {
        assert_eq!(dependency_chain(), ["core", "assistant", "store", "mail"]);
    }

    #[test]
    fn version_is_not_empty() {
        assert!(!version().is_empty());
    }
}
