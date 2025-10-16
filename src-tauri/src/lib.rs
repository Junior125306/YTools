use tauri::{
    Manager,
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};
use std::fs;
use std::path::PathBuf;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

// 获取 .ytools 目录路径
fn get_ytools_dir() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or("无法获取用户目录")?;
    let ytools_dir = home.join(".ytools");
    
    // 确保目录存在
    if !ytools_dir.exists() {
        fs::create_dir_all(&ytools_dir).map_err(|e| format!("创建目录失败: {}", e))?;
    }
    
    Ok(ytools_dir)
}

// 读取 markdown 文件
#[tauri::command]
fn read_note(filename: String) -> Result<String, String> {
    let ytools_dir = get_ytools_dir()?;
    let file_path = ytools_dir.join(&filename);
    
    if !file_path.exists() {
        return Ok(String::new());
    }
    
    fs::read_to_string(file_path).map_err(|e| format!("读取文件失败: {}", e))
}

// 保存 markdown 文件
#[tauri::command]
fn save_note(filename: String, content: String) -> Result<(), String> {
    let ytools_dir = get_ytools_dir()?;
    let file_path = ytools_dir.join(&filename);
    
    fs::write(file_path, content).map_err(|e| format!("保存文件失败: {}", e))
}

// 获取所有笔记文件列表
#[tauri::command]
fn list_notes() -> Result<Vec<String>, String> {
    let ytools_dir = get_ytools_dir()?;
    
    let mut notes = Vec::new();
    if let Ok(entries) = fs::read_dir(ytools_dir) {
        for entry in entries.flatten() {
            if let Ok(file_name) = entry.file_name().into_string() {
                if file_name.ends_with(".md") {
                    notes.push(file_name);
                }
            }
        }
    }
    
    notes.sort();
    Ok(notes)
}

// 规范化字符串：转小写并移除常见分隔符，便于宽松匹配
fn normalize(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect::<String>()
}

// 最长公共子串长度（连续匹配长度）
fn longest_common_substring_len(a: &str, b: &str) -> usize {
    if a.is_empty() || b.is_empty() {
        return 0;
    }
    let ac: Vec<char> = a.chars().collect();
    let bc: Vec<char> = b.chars().collect();
    let mut dp = vec![0usize; bc.len() + 1];
    let mut best = 0usize;
    for i in 1..=ac.len() {
        let mut prev = 0usize;
        for j in 1..=bc.len() {
            let tmp = dp[j];
            if ac[i - 1] == bc[j - 1] {
                dp[j] = prev + 1;
                if dp[j] > best {
                    best = dp[j];
                }
            } else {
                dp[j] = 0;
            }
            prev = tmp;
        }
    }
    best
}

// 最长公共子序列长度（非连续匹配总量）
fn longest_common_subsequence_len(a: &str, b: &str) -> usize {
    if a.is_empty() || b.is_empty() {
        return 0;
    }
    let ac: Vec<char> = a.chars().collect();
    let bc: Vec<char> = b.chars().collect();
    let mut prev = vec![0usize; bc.len() + 1];
    let mut curr = vec![0usize; bc.len() + 1];
    for i in 1..=ac.len() {
        for j in 1..=bc.len() {
            curr[j] = if ac[i - 1] == bc[j - 1] {
                prev[j - 1] + 1
            } else {
                prev[j].max(curr[j - 1])
            };
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[bc.len()]
}

// Levenshtein 编辑距离（大小写与分隔符已通过 normalize 处理在外部）
fn levenshtein_distance(a: &str, b: &str) -> usize {
    let ac: Vec<char> = a.chars().collect();
    let bc: Vec<char> = b.chars().collect();
    let n = ac.len();
    let m = bc.len();
    if n == 0 { return m; }
    if m == 0 { return n; }

    let mut prev: Vec<usize> = (0..=m).collect();
    let mut curr: Vec<usize> = vec![0; m + 1];
    for i in 1..=n {
        curr[0] = i;
        for j in 1..=m {
            let cost = if ac[i - 1] == bc[j - 1] { 0 } else { 1 };
            curr[j] = (prev[j] + 1) // 删除
                .min(curr[j - 1] + 1) // 插入
                .min(prev[j - 1] + cost); // 替换
        }
        std::mem::swap(&mut prev, &mut curr);
    }
    prev[m]
}

// 搜索工作区文件夹
#[tauri::command]
fn search_workspaces(query: String) -> Result<Vec<String>, String> {
    let workspace_dir = PathBuf::from("D:\\workspaces");
    
    if !workspace_dir.exists() {
        return Ok(Vec::new());
    }
    
    // 当查询为空时，直接返回全部目录（维持现有行为）
    let query_lower = query.to_lowercase();
    let query_norm = normalize(&query_lower);
    let mut results = Vec::new();
    let mut all_dirs: Vec<String> = Vec::new();
    
    if let Ok(entries) = fs::read_dir(workspace_dir) {
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                if metadata.is_dir() {
                    if let Ok(dir_name) = entry.file_name().into_string() {
                        let name_lower = dir_name.to_lowercase();
                        let name_norm = normalize(&name_lower);

                        // 记录全量目录，用于回退模糊匹配
                        all_dirs.push(dir_name.clone());

                        // 先尝试包含匹配（兼容去除分隔符后的包含）
                        if query.is_empty()
                            || name_lower.contains(&query_lower)
                            || (!query_norm.is_empty() && name_norm.contains(&query_norm))
                        {
                            results.push(dir_name);
                        }
                    }
                }
            }
        }
    }
    
    // 如果包含匹配已经有结果，按字典序返回
    if !results.is_empty() || query.is_empty() {
        results.sort();
        return Ok(results);
    }

    // 回退：基于最长连续匹配（最长公共子串）、总匹配字符数（LCS）与编辑距离进行排序，取前5
    let mut scored: Vec<(String, usize, usize, usize)> = Vec::new();
    for name in all_dirs.into_iter() {
        let name_lower = name.to_lowercase();
        let name_norm = normalize(&name_lower);

        let lcs_sub = longest_common_substring_len(&name_norm, &query_norm);
        let lcs_seq = longest_common_subsequence_len(&name_norm, &query_norm);
        let edit = levenshtein_distance(&name_norm, &query_norm);

        scored.push((name, lcs_sub, lcs_seq, edit));
    }

    scored.sort_by(|a, b| {
        // 优先：最长连续匹配降序，其次：总匹配字符数降序，其次：编辑距离升序，最后：名称字典序
        b.1.cmp(&a.1)
            .then(b.2.cmp(&a.2))
            .then(a.3.cmp(&b.3))
            .then(a.0.cmp(&b.0))
    });

    let top5: Vec<String> = scored.into_iter().take(5).map(|t| t.0).collect();
    Ok(top5)
}

// 打开文件夹
#[tauri::command]
fn open_folder(folder_name: String) -> Result<(), String> {
    let folder_path = PathBuf::from("D:\\workspaces").join(&folder_name);
    
    if !folder_path.exists() {
        return Err(format!("文件夹不存在: {}", folder_name));
    }
    
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(folder_path)
            .spawn()
            .map_err(|e| format!("打开文件夹失败: {}", e))?;
    }
    
    #[cfg(not(target_os = "windows"))]
    {
        std::process::Command::new("open")
            .arg(folder_path)
            .spawn()
            .map_err(|e| format!("打开文件夹失败: {}", e))?;
    }
    
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .invoke_handler(tauri::generate_handler![
            greet, 
            read_note, 
            save_note, 
            list_notes,
            search_workspaces,
            open_folder
        ])
        .setup(|app| {
            // 创建托盘菜单
            let show_hide = MenuItemBuilder::with_id("show_hide", "显示/隐藏窗口")
                .build(app)?;
            let quit = MenuItemBuilder::with_id("quit", "退出")
                .build(app)?;
            
            let menu = MenuBuilder::new(app)
                .items(&[&show_hide, &quit])
                .build()?;

            // 创建系统托盘图标
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| {
                    match event.id().as_ref() {
                        "show_hide" => {
                            if let Some(window) = app.get_webview_window("main") {
                                if window.is_visible().unwrap_or(false) {
                                    let _ = window.hide();
                                } else {
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                }
                            }
                        }
                        "quit" => {
                            app.exit(0);
                        }
                        _ => {}
                    }
                })
                .on_tray_icon_event(|tray, event| {
                    // 处理托盘图标点击事件
                    if let TrayIconEvent::Click {
                        button: MouseButton::Left,
                        button_state: MouseButtonState::Up,
                        ..
                    } = event
                    {
                        let app = tray.app_handle();
                        if let Some(window) = app.get_webview_window("main") {
                            if window.is_visible().unwrap_or(false) {
                                let _ = window.hide();
                            } else {
                                let _ = window.unminimize();
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                })
                .build(app)?;

            // 设置全局快捷键
            let alt_space = Shortcut::new(Some(Modifiers::ALT), Code::Space);
            let ctrl_space = Shortcut::new(Some(Modifiers::CONTROL), Code::Space);
            
            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new()
                    .with_handler(move |app_handle, received_shortcut, event| {
                        if event.state() == ShortcutState::Pressed {
                            // Alt+Space: 显示/隐藏主窗口
                            if received_shortcut == &alt_space {
                                if let Some(window) = app_handle.get_webview_window("main") {
                                    if window.is_visible().unwrap_or(false) {
                                        let _ = window.hide();
                                    } else {
                                        let _ = window.unminimize();
                                        let _ = window.show();
                                        let _ = window.set_focus();
                                    }
                                }
                            }
                            // Ctrl+Space: 切换搜索窗口显示/隐藏
                            else if received_shortcut == &ctrl_space {
                                if let Some(search_window) = app_handle.get_webview_window("search") {
                                    // 搜索窗口已存在，切换显示状态
                                    if search_window.is_visible().unwrap_or(false) {
                                        let _ = search_window.hide();
                                    } else {
                                        let _ = search_window.show();
                                        let _ = search_window.set_focus();
                                    }
                                } else {
                                    // 搜索窗口不存在，创建它
                                    use tauri::WebviewWindowBuilder;
                                    
                                    let result = if cfg!(debug_assertions) {
                                        // 开发模式：使用路由 /#/search
                                        WebviewWindowBuilder::new(
                                            app_handle,
                                            "search",
                                            tauri::WebviewUrl::External("http://localhost:1420/#/search".parse().unwrap())
                                        )
                                    } else {
                                        // 生产模式：使用路由 /#/search
                                        WebviewWindowBuilder::new(
                                            app_handle,
                                            "search",
                                            tauri::WebviewUrl::App("index.html#/search".into())
                                        )
                                    }
                                    .title("搜索工作区")
                                    .inner_size(600.0, 400.0)
                                    .decorations(false)     // 移除窗口装饰
                                    .always_on_top(true)
                                    .center()
                                    .resizable(false)
                                    .skip_taskbar(true)
                                    .visible(false)
                                    .transparent(true)      // 设置窗口透明
                                    .shadow(false)          // 移除窗口阴影和边框
                                    .build();
                                    
                                    if let Ok(window) = result {
                                        let _ = window.show();
                                        let _ = window.set_focus();
                                    } else {
                                        eprintln!("❌ 创建搜索窗口失败");
                                    }
                                }
                            }
                        }
                    })
                    .build(),
            )?;
            
            // 注册 Alt+Space 快捷键
            match app.global_shortcut().register(alt_space) {
                Ok(_) => {
                    println!("✅ 全局快捷键已注册: Alt+Space (显示/隐藏窗口)");
                }
                Err(e) => {
                    eprintln!("⚠️  Alt+Space 注册失败: {}", e);
                    eprintln!("   可使用托盘图标唤出窗口");
                }
            }
            
            // 注册 Ctrl+Space 快捷键
            match app.global_shortcut().register(ctrl_space) {
                Ok(_) => {
                    println!("✅ 全局快捷键已注册: Ctrl+Space (搜索工作区)");
                }
                Err(e) => {
                    eprintln!("⚠️  Ctrl+Space 注册失败: {}", e);
                }
            }

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
