//! PalaceMail 桌面应用的 Tauri 壳。
//!
//! 这一层只负责创建窗口、注册 IPC 命令，并把界面的调用转交给 `palace-core`；
//! 不包含任何邮件或 AI 逻辑。

/// 示例命令：返回内核版本号，供界面显示。
#[tauri::command]
fn kernel_version() -> String {
    palace_core::version().to_owned()
}

/// 构建并运行 Tauri 应用。
pub fn run() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![kernel_version])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
