use pinyin::ToPinyinMulti;
use std::fs;
use std::path::PathBuf;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tauri::{
    menu::{MenuBuilder, MenuItemBuilder},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    Emitter, Manager, WindowEvent,
};
use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState};

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

// 打开文件夹（使用系统文件管理器）
#[tauri::command]
fn open_directory(path: String) -> Result<(), String> {
    #[cfg(target_os = "windows")]
    {
        std::process::Command::new("explorer")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("打开目录失败: {}", e))?;
    }

    #[cfg(target_os = "macos")]
    {
        std::process::Command::new("open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("打开目录失败: {}", e))?;
    }

    #[cfg(target_os = "linux")]
    {
        std::process::Command::new("xdg-open")
            .arg(&path)
            .spawn()
            .map_err(|e| format!("打开目录失败: {}", e))?;
    }

    Ok(())
}

// 读取 markdown 文件
#[tauri::command]
fn read_note(filename: String) -> Result<String, String> {
    let file_path = PathBuf::from(&filename);

    if !file_path.exists() {
        return Ok(String::new());
    }

    let content = fs::read_to_string(&file_path).map_err(|e| format!("读取文件失败: {}", e))?;
    Ok(content)
}

// 保存 markdown 文件
#[tauri::command]
fn save_note(filename: String, content: String) -> Result<(), String> {
    let file_path = PathBuf::from(&filename);

    fs::write(&file_path, content).map_err(|e| format!("保存文件失败: {}", e))?;
    Ok(())
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

// 将中文字符串转换为拼音全拼（小写，无分隔符）
// 为了处理多音字，返回多个可能的拼音组合
fn to_pinyin_full_multi(text: &str) -> Vec<String> {
    let mut current_results = vec![String::new()];

    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            // ASCII 字母保留（转小写）
            let lower = c.to_lowercase().to_string();
            for result in current_results.iter_mut() {
                result.push_str(&lower);
            }
        } else if c.is_ascii() {
            // 其他 ASCII 字符保留
            for result in current_results.iter_mut() {
                result.push(c);
            }
        } else {
            // 中文字符：获取所有可能的拼音（使用 ToPinyinMulti）
            if let Some(pinyin_multi) = c.to_pinyin_multi() {
                let mut new_results = Vec::new();
                for pinyin_option in pinyin_multi {
                    for base in &current_results {
                        let mut new_result = base.clone();
                        new_result.push_str(pinyin_option.plain());
                        new_results.push(new_result);
                    }
                }
                if !new_results.is_empty() {
                    current_results = new_results;
                }
            }
        }
    }

    current_results
        .into_iter()
        .map(|s| s.to_lowercase())
        .collect()
}

// 将中文字符串转换为拼音首字母缩写（小写）
// 为了处理多音字，返回多个可能的首字母组合
fn to_pinyin_initials_multi(text: &str) -> Vec<String> {
    let mut current_results = vec![String::new()];

    for c in text.chars() {
        if c.is_ascii_alphabetic() {
            // ASCII 字母保留（转小写）
            let lower = c.to_lowercase().to_string();
            for result in current_results.iter_mut() {
                result.push_str(&lower);
            }
        } else if c.is_ascii() {
            // 跳过其他 ASCII 字符
            continue;
        } else {
            // 中文字符：获取所有可能的拼音首字母（使用 ToPinyinMulti）
            if let Some(pinyin_multi) = c.to_pinyin_multi() {
                let mut new_results = Vec::new();
                for pinyin_option in pinyin_multi {
                    if let Some(first_char) = pinyin_option.plain().chars().next() {
                        for base in &current_results {
                            let mut new_result = base.clone();
                            new_result.push(first_char);
                            new_results.push(new_result);
                        }
                    }
                }
                if !new_results.is_empty() {
                    current_results = new_results;
                }
            }
        }
    }

    current_results
        .into_iter()
        .map(|s| s.to_lowercase())
        .collect()
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
    if n == 0 {
        return m;
    }
    if m == 0 {
        return n;
    }

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
fn search_workspaces(query: String, directories: Vec<String>) -> Result<Vec<String>, String> {
    // 如果没有配置搜索目录，返回空结果
    if directories.is_empty() {
        return Ok(Vec::new());
    }

    // 当查询为空时，直接返回全部目录（维持现有行为）
    let query_lower = query.to_lowercase();
    let query_norm = normalize(&query_lower);
    let mut results = Vec::new();
    let mut all_dirs: Vec<String> = Vec::new();

    // 遍历所有配置的搜索目录
    for workspace_dir_str in directories {
        let workspace_dir = PathBuf::from(&workspace_dir_str);

        if !workspace_dir.exists() {
            continue;
        }

        if let Ok(entries) = fs::read_dir(workspace_dir) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        if let Ok(dir_name) = entry.file_name().into_string() {
                            let name_lower = dir_name.to_lowercase();
                            let name_norm = normalize(&name_lower);

                            // 生成拼音表示（多音字版本）
                            let name_pinyin_full_multi = to_pinyin_full_multi(&dir_name);
                            let name_pinyin_initials_multi = to_pinyin_initials_multi(&dir_name);

                            // 检查是否有任何拼音版本匹配
                            let pinyin_full_match = name_pinyin_full_multi
                                .iter()
                                .any(|py| py.contains(&query_lower));
                            let pinyin_initials_match = name_pinyin_initials_multi
                                .iter()
                                .any(|py| py.contains(&query_lower));

                            // 记录全量目录，用于回退模糊匹配
                            all_dirs.push(dir_name.clone());

                            // 先尝试包含匹配（兼容去除分隔符后的包含 + 多音字拼音匹配）
                            if query.is_empty()
                                || name_lower.contains(&query_lower)
                                || (!query_norm.is_empty() && name_norm.contains(&query_norm))
                                || pinyin_full_match // 拼音全拼匹配（多音字）
                                || pinyin_initials_match
                            // 拼音首字母匹配（多音字）
                            {
                                results.push(dir_name);
                            }
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
    // 同时考虑拼音匹配的得分（支持多音字）
    let mut scored: Vec<(String, usize, usize, usize)> = Vec::new();
    for name in all_dirs.into_iter() {
        let name_lower = name.to_lowercase();
        let name_norm = normalize(&name_lower);
        let name_pinyin_full_multi = to_pinyin_full_multi(&name);
        let name_pinyin_initials_multi = to_pinyin_initials_multi(&name);

        // 计算原始名称匹配得分
        let lcs_sub = longest_common_substring_len(&name_norm, &query_norm);
        let lcs_seq = longest_common_subsequence_len(&name_norm, &query_norm);
        let edit = levenshtein_distance(&name_norm, &query_norm);

        // 计算所有拼音全拼的最佳匹配得分
        let mut best_pinyin_lcs_sub = 0;
        let mut best_pinyin_lcs_seq = 0;
        let mut best_pinyin_edit = usize::MAX;
        for pinyin in &name_pinyin_full_multi {
            let lcs_sub = longest_common_substring_len(pinyin, &query_lower);
            let lcs_seq = longest_common_subsequence_len(pinyin, &query_lower);
            let edit = levenshtein_distance(pinyin, &query_lower);

            best_pinyin_lcs_sub = best_pinyin_lcs_sub.max(lcs_sub);
            best_pinyin_lcs_seq = best_pinyin_lcs_seq.max(lcs_seq);
            best_pinyin_edit = best_pinyin_edit.min(edit);
        }

        // 计算所有拼音首字母的最佳匹配得分
        let mut best_initials_lcs_sub = 0;
        let mut best_initials_lcs_seq = 0;
        let mut best_initials_edit = usize::MAX;
        for initials in &name_pinyin_initials_multi {
            let lcs_sub = longest_common_substring_len(initials, &query_lower);
            let lcs_seq = longest_common_subsequence_len(initials, &query_lower);
            let edit = levenshtein_distance(initials, &query_lower);

            best_initials_lcs_sub = best_initials_lcs_sub.max(lcs_sub);
            best_initials_lcs_seq = best_initials_lcs_seq.max(lcs_seq);
            best_initials_edit = best_initials_edit.min(edit);
        }

        // 取所有匹配方式中的最佳得分
        let best_lcs_sub = lcs_sub.max(best_pinyin_lcs_sub).max(best_initials_lcs_sub);
        let best_lcs_seq = lcs_seq.max(best_pinyin_lcs_seq).max(best_initials_lcs_seq);
        let best_edit = edit.min(best_pinyin_edit).min(best_initials_edit);

        scored.push((name, best_lcs_sub, best_lcs_seq, best_edit));
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

// 导入笔记（打开文件选择对话框）
#[tauri::command]
async fn import_note(app: tauri::AppHandle) -> Result<String, String> {
    use tauri_plugin_dialog::DialogExt;

    let file_path = app
        .dialog()
        .file()
        .add_filter("文本文件", &["md", "txt"])
        .blocking_pick_file();

    match file_path {
        Some(path) => {
            if let Some(path_ref) = path.as_path() {
                let path_str = path_ref.to_string_lossy().to_string();
                Ok(path_str)
            } else {
                Ok(String::new())
            }
        }
        None => Ok(String::new()), // 用户取消选择
    }
}

// 创建新笔记
#[tauri::command]
fn create_note(name: String, base_dir: String) -> Result<String, String> {
    let base_path = PathBuf::from(&base_dir);

    // 确保目录存在
    if !base_path.exists() {
        fs::create_dir_all(&base_path).map_err(|e| format!("创建目录失败: {}", e))?;
    }

    let file_path = base_path.join(&name);

    // 创建空文件
    fs::write(&file_path, "").map_err(|e| format!("创建文件失败: {}", e))?;

    // 返回完整路径
    Ok(file_path.to_string_lossy().to_string())
}

// 删除笔记文件
#[tauri::command]
fn delete_note_file(path: String) -> Result<(), String> {
    let file_path = PathBuf::from(&path);

    if !file_path.exists() {
        return Err(format!("文件不存在: {}", path));
    }

    fs::remove_file(&file_path).map_err(|e| format!("删除文件失败: {}", e))?;
    Ok(())
}

// 解析快捷键字符串为 Shortcut 对象
// 输入格式: "Ctrl+Alt+D", "Alt+Space" 等
fn parse_shortcut(shortcut_str: &str) -> Result<Shortcut, String> {
    let parts: Vec<&str> = shortcut_str.split('+').map(|s| s.trim()).collect();

    if parts.is_empty() {
        return Err("快捷键字符串不能为空".to_string());
    }

    let mut modifiers = Modifiers::empty();
    let mut main_key: Option<Code> = None;

    for part in parts {
        match part {
            "Ctrl" | "Control" => modifiers |= Modifiers::CONTROL,
            "Alt" => modifiers |= Modifiers::ALT,
            "Shift" => modifiers |= Modifiers::SHIFT,
            "Meta" | "Cmd" | "Command" | "Super" => modifiers |= Modifiers::META,
            // 字母键
            "A" => main_key = Some(Code::KeyA),
            "B" => main_key = Some(Code::KeyB),
            "C" => main_key = Some(Code::KeyC),
            "D" => main_key = Some(Code::KeyD),
            "E" => main_key = Some(Code::KeyE),
            "F" => main_key = Some(Code::KeyF),
            "G" => main_key = Some(Code::KeyG),
            "H" => main_key = Some(Code::KeyH),
            "I" => main_key = Some(Code::KeyI),
            "J" => main_key = Some(Code::KeyJ),
            "K" => main_key = Some(Code::KeyK),
            "L" => main_key = Some(Code::KeyL),
            "M" => main_key = Some(Code::KeyM),
            "N" => main_key = Some(Code::KeyN),
            "O" => main_key = Some(Code::KeyO),
            "P" => main_key = Some(Code::KeyP),
            "Q" => main_key = Some(Code::KeyQ),
            "R" => main_key = Some(Code::KeyR),
            "S" => main_key = Some(Code::KeyS),
            "T" => main_key = Some(Code::KeyT),
            "U" => main_key = Some(Code::KeyU),
            "V" => main_key = Some(Code::KeyV),
            "W" => main_key = Some(Code::KeyW),
            "X" => main_key = Some(Code::KeyX),
            "Y" => main_key = Some(Code::KeyY),
            "Z" => main_key = Some(Code::KeyZ),
            // 数字键
            "0" => main_key = Some(Code::Digit0),
            "1" => main_key = Some(Code::Digit1),
            "2" => main_key = Some(Code::Digit2),
            "3" => main_key = Some(Code::Digit3),
            "4" => main_key = Some(Code::Digit4),
            "5" => main_key = Some(Code::Digit5),
            "6" => main_key = Some(Code::Digit6),
            "7" => main_key = Some(Code::Digit7),
            "8" => main_key = Some(Code::Digit8),
            "9" => main_key = Some(Code::Digit9),
            // 特殊键
            "Space" => main_key = Some(Code::Space),
            "Enter" => main_key = Some(Code::Enter),
            "Escape" | "Esc" => main_key = Some(Code::Escape),
            "Tab" => main_key = Some(Code::Tab),
            "Backspace" => main_key = Some(Code::Backspace),
            "Delete" => main_key = Some(Code::Delete),
            "Insert" => main_key = Some(Code::Insert),
            "Home" => main_key = Some(Code::Home),
            "End" => main_key = Some(Code::End),
            "PageUp" => main_key = Some(Code::PageUp),
            "PageDown" => main_key = Some(Code::PageDown),
            "ArrowUp" => main_key = Some(Code::ArrowUp),
            "ArrowDown" => main_key = Some(Code::ArrowDown),
            "ArrowLeft" => main_key = Some(Code::ArrowLeft),
            "ArrowRight" => main_key = Some(Code::ArrowRight),
            // F1-F12
            "F1" => main_key = Some(Code::F1),
            "F2" => main_key = Some(Code::F2),
            "F3" => main_key = Some(Code::F3),
            "F4" => main_key = Some(Code::F4),
            "F5" => main_key = Some(Code::F5),
            "F6" => main_key = Some(Code::F6),
            "F7" => main_key = Some(Code::F7),
            "F8" => main_key = Some(Code::F8),
            "F9" => main_key = Some(Code::F9),
            "F10" => main_key = Some(Code::F10),
            "F11" => main_key = Some(Code::F11),
            "F12" => main_key = Some(Code::F12),
            _ => return Err(format!("不支持的按键: {}", part)),
        }
    }

    let code = main_key.ok_or("快捷键必须包含一个主键".to_string())?;

    let modifiers_opt = if modifiers.is_empty() {
        None
    } else {
        Some(modifiers)
    };

    Ok(Shortcut::new(modifiers_opt, code))
}

// 更新全局快捷键
#[tauri::command]
fn update_global_shortcuts(
    app_handle: tauri::AppHandle,
    show_main: String,
    show_search: String,
) -> Result<(), String> {
    // 解析新的快捷键
    let new_main_shortcut = parse_shortcut(&show_main)?;
    let new_search_shortcut = parse_shortcut(&show_search)?;

    // 注销所有现有快捷键（简化处理：注销所有，然后重新注册）
    let _ = app_handle.global_shortcut().unregister_all();

    // 注册新的快捷键
    app_handle
        .global_shortcut()
        .register(new_main_shortcut.clone())
        .map_err(|e| format!("注册主窗口快捷键失败: {}", e))?;

    app_handle
        .global_shortcut()
        .register(new_search_shortcut.clone())
        .map_err(|e| format!("注册搜索窗口快捷键失败: {}", e))?;

    Ok(())
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
            // 跟踪主窗口焦点状态（不再在失焦时自动隐藏）
            let main_focused = Arc::new(AtomicBool::new(false));
            if let Some(main_window) = app.get_webview_window("main") {
                let focused_flag = main_focused.clone();
                main_window.on_window_event(move |event| {
                    if let WindowEvent::Focused(focused) = event {
                        focused_flag.store(*focused, Ordering::SeqCst);
                    }
                });
            }

            // 创建托盘菜单
            let show_hide = MenuItemBuilder::with_id("show_hide", "显示/隐藏窗口").build(app)?;
            let settings = MenuItemBuilder::with_id("settings", "设置").build(app)?;
            let quit = MenuItemBuilder::with_id("quit", "退出").build(app)?;

            let menu = MenuBuilder::new(app)
                .items(&[&show_hide, &settings, &quit])
                .build()?;

            // 创建系统托盘图标
            let _tray = TrayIconBuilder::new()
                .icon(app.default_window_icon().unwrap().clone())
                .menu(&menu)
                .show_menu_on_left_click(false)
                .on_menu_event(|app, event| match event.id().as_ref() {
                    "show_hide" => {
                        if let Some(window) = app.get_webview_window("main") {
                            // 使用与快捷键相同的逻辑
                            let is_visible = window.is_visible().unwrap_or(false);
                            let is_minimized = window.is_minimized().unwrap_or(false);

                            if is_visible && !is_minimized {
                                let _ = window.hide();
                            } else {
                                if is_minimized {
                                    let _ = window.unminimize();
                                }
                                let _ = window.show();
                                let _ = window.set_focus();
                            }
                        }
                    }
                    "settings" => {
                        // 检查设置窗口是否已存在
                        if let Some(settings_window) = app.get_webview_window("settings") {
                            // 如果存在，显示并聚焦
                            let _ = settings_window.show();
                            let _ = settings_window.set_focus();
                        } else {
                            // 如果不存在，发送事件给主窗口创建（不显示主窗口）
                            if let Some(main_window) = app.get_webview_window("main") {
                                let _ = main_window.emit("open-settings", ());
                            }
                        }
                    }
                    "quit" => {
                        app.exit(0);
                    }
                    _ => {}
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
                            // 使用与快捷键相同的逻辑
                            let is_visible = window.is_visible().unwrap_or(false);
                            let is_minimized = window.is_minimized().unwrap_or(false);

                            if is_visible && !is_minimized {
                                let _ = window.hide();
                            } else {
                                if is_minimized {
                                    let _ = window.unminimize();
                                }
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

            let main_focused_for_shortcut = main_focused.clone();
            app.handle().plugin(
                tauri_plugin_global_shortcut::Builder::new()
                    .with_handler(move |app_handle, received_shortcut, event| {
                        if event.state() == ShortcutState::Pressed {
                            // Alt+Space: 显示/隐藏主窗口
                            if received_shortcut == &alt_space {
                                if let Some(window) = app_handle.get_webview_window("main") {
                                    // 更可靠的状态检查：同时检查可见性和是否最小化
                                    let is_visible = window.is_visible().unwrap_or(false);
                                    let is_minimized = window.is_minimized().unwrap_or(false);
                                    let is_focused =
                                        main_focused_for_shortcut.load(Ordering::SeqCst);

                                    if is_visible && !is_minimized {
                                        // 如果已获得焦点，则隐藏；否则将其聚焦到前台
                                        if is_focused {
                                            let _ = window.hide();
                                        } else {
                                            let _ = window.show();
                                            let _ = window.set_focus();
                                        }
                                    } else {
                                        // 窗口不可见或最小化，显示并聚焦
                                        if is_minimized {
                                            let _ = window.unminimize();
                                        }
                                        let _ = window.show();
                                        let _ = window.set_focus();
                                    }
                                }
                            }
                            // Ctrl+Space: 切换搜索窗口显示/隐藏（窗口已预创建）
                            else if received_shortcut == &ctrl_space {
                                if let Some(search_window) = app_handle.get_webview_window("search")
                                {
                                    // 切换显示状态
                                    if search_window.is_visible().unwrap_or(false) {
                                        let _ = search_window.hide();
                                    } else {
                                        let _ = search_window.show();
                                        let _ = search_window.set_focus();
                                    }
                                }
                            }
                        }
                    })
                    .build(),
            )?;

            // 预创建搜索窗口（隐藏），提升首次打开速度
            use tauri::WebviewWindowBuilder;
            let search_window_result = if cfg!(debug_assertions) {
                // 开发模式
                WebviewWindowBuilder::new(app, "search", tauri::WebviewUrl::App("/#/search".into()))
            } else {
                // 生产模式
                WebviewWindowBuilder::new(app, "search", tauri::WebviewUrl::App("/".into()))
                    .initialization_script("window.location.hash = '#/search';")
            };

            if let Ok(search_window) = search_window_result
                .title("搜索工作区")
                .inner_size(650.0, 550.0)
                .resizable(false)
                .decorations(false)
                .transparent(true)
                .shadow(false) // 移除窗口阴影和边框
                .always_on_top(true)
                .skip_taskbar(true)
                .visible(false) // 默认隐藏
                .center()
                .build()
            {
                // 为搜索窗口添加失焦自动隐藏功能
                let search_window_clone = search_window.clone();
                search_window.on_window_event(move |event| {
                    if let WindowEvent::Focused(false) = event {
                        // 窗口失去焦点时自动隐藏
                        let _ = search_window_clone.hide();
                    }
                });
            }

            // 注册 Alt+Space 快捷键
            let _ = app.global_shortcut().register(alt_space);

            // 注册 Ctrl+Space 快捷键
            let _ = app.global_shortcut().register(ctrl_space);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
