//! YTools 核心库
//!
//! 模块化组织：
//! - notes: 笔记管理功能
//! - search: 搜索算法和工作区搜索
//! - shortcuts: 快捷键解析和管理
//! - system: 系统集成（托盘、窗口、目录打开）

mod notes;
mod search;
mod shortcuts;
mod system;

pub use notes::{create_note, import_note, read_note, save_note};
pub use search::search_workspaces;
pub use shortcuts::update_global_shortcuts;
pub use system::open_directory;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_store::Builder::default().build())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_autostart::init(
            tauri_plugin_autostart::MacosLauncher::LaunchAgent,
            None,
        ))
        .invoke_handler(tauri::generate_handler![
            read_note,
            save_note,
            search_workspaces,
            import_note,
            create_note,
            open_directory,
            update_global_shortcuts
        ])
        .setup(|app| {
            system::setup_tray(app)?;
            let _main_focused = system::setup_windows_and_shortcuts(app)?;
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
