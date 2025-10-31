//! 快捷键管理模块
//! 
//! 提供快捷键字符串解析和全局快捷键注册功能

use tauri_plugin_global_shortcut::{Code, GlobalShortcutExt, Modifiers, Shortcut};

/// 解析快捷键字符串为 Shortcut 对象
/// 输入格式: "Ctrl+Alt+D", "Alt+Space" 等
pub fn parse_shortcut(shortcut_str: &str) -> Result<Shortcut, String> {
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

/// 更新全局快捷键
#[tauri::command]
pub fn update_global_shortcuts(
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_shortcut_simple() {
        // 测试简单的快捷键
        let result = parse_shortcut("Ctrl+S");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_shortcut_alt_space() {
        // 测试 Alt+Space
        let result = parse_shortcut("Alt+Space");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_shortcut_ctrl_space() {
        // 测试 Ctrl+Space
        let result = parse_shortcut("Ctrl+Space");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_shortcut_multiple_modifiers() {
        // 测试多个修饰键
        let result = parse_shortcut("Ctrl+Alt+D");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_shortcut_all_modifiers() {
        // 测试所有修饰键
        let result = parse_shortcut("Ctrl+Alt+Shift+Meta+A");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_shortcut_with_spaces() {
        // 测试带空格的快捷键
        let result = parse_shortcut("Ctrl + Alt + D");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_shortcut_control_alias() {
        // 测试 Control 别名
        let result1 = parse_shortcut("Ctrl+S");
        let result2 = parse_shortcut("Control+S");
        assert!(result1.is_ok());
        assert!(result2.is_ok());
    }

    #[test]
    fn test_parse_shortcut_meta_aliases() {
        // 测试 Meta 的各种别名
        let aliases = vec!["Meta+S", "Cmd+S", "Command+S", "Super+S"];
        for alias in aliases {
            let result = parse_shortcut(alias);
            assert!(result.is_ok(), "Failed to parse: {}", alias);
        }
    }

    #[test]
    fn test_parse_shortcut_escape_alias() {
        // 测试 Escape 的别名
        let result1 = parse_shortcut("Escape");
        let result2 = parse_shortcut("Esc");
        assert!(result1.is_ok());
        assert!(result2.is_ok());
    }

    #[test]
    fn test_parse_shortcut_numbers() {
        // 测试数字键
        for i in 0..=9 {
            let shortcut_str = format!("Ctrl+{}", i);
            let result = parse_shortcut(&shortcut_str);
            assert!(result.is_ok(), "Failed to parse: {}", shortcut_str);
        }
    }

    #[test]
    fn test_parse_shortcut_function_keys() {
        // 测试功能键 F1-F12
        for i in 1..=12 {
            let shortcut_str = format!("Ctrl+F{}", i);
            let result = parse_shortcut(&shortcut_str);
            assert!(result.is_ok(), "Failed to parse: {}", shortcut_str);
        }
    }

    #[test]
    fn test_parse_shortcut_arrow_keys() {
        // 测试方向键
        let arrows = vec!["ArrowUp", "ArrowDown", "ArrowLeft", "ArrowRight"];
        for arrow in arrows {
            let shortcut_str = format!("Ctrl+{}", arrow);
            let result = parse_shortcut(&shortcut_str);
            assert!(result.is_ok(), "Failed to parse: {}", shortcut_str);
        }
    }

    #[test]
    fn test_parse_shortcut_empty_string() {
        // 测试空字符串
        let result = parse_shortcut("");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("不能为空"));
    }

    #[test]
    fn test_parse_shortcut_no_main_key() {
        // 测试只有修饰键没有主键
        let result = parse_shortcut("Ctrl+Alt");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("必须包含一个主键"));
    }

    #[test]
    fn test_parse_shortcut_invalid_key() {
        // 测试无效的按键
        let result = parse_shortcut("Ctrl+InvalidKey");
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("不支持的按键"));
    }

    #[test]
    fn test_parse_shortcut_only_main_key() {
        // 测试只有主键没有修饰键
        let result = parse_shortcut("S");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parse_shortcut_letters() {
        // 测试所有字母键
        for c in b'A'..=b'Z' {
            let letter = char::from(c);
            let shortcut_str = format!("Ctrl+{}", letter);
            let result = parse_shortcut(&shortcut_str);
            assert!(result.is_ok(), "Failed to parse: {}", shortcut_str);
        }
    }
}

