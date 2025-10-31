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

// 重新导出模块中的公开函数，供 Tauri 命令使用
pub use notes::{
    create_note, delete_note_file, import_note, list_notes, read_note, save_note,
};
pub use search::{open_folder, search_workspaces};
pub use shortcuts::update_global_shortcuts;
pub use system::open_directory;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

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
            greet,
            read_note,
            save_note,
            list_notes,
            search_workspaces,
            open_folder,
            import_note,
            create_note,
            delete_note_file,
            open_directory,
            update_global_shortcuts
        ])
        .setup(|app| {
            // 初始化系统托盘
            system::setup_tray(app)?;

            // 初始化窗口管理和全局快捷键
            let _main_focused = system::setup_windows_and_shortcuts(app)?;

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
