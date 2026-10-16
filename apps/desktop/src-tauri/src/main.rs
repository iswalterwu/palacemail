// 发布构建在 Windows 上不弹出额外的控制台窗口，请勿删除。
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    palacemail_desktop_lib::run()
}
