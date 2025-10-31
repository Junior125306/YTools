//! 笔记管理模块
//! 
//! 提供笔记文件的创建、读取、保存、删除、导入等功能

use std::fs;
use std::path::PathBuf;

/// 获取 .ytools 目录路径
pub fn get_ytools_dir() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or("无法获取用户目录")?;
    let ytools_dir = home.join(".ytools");

    // 确保目录存在
    if !ytools_dir.exists() {
        fs::create_dir_all(&ytools_dir).map_err(|e| format!("创建目录失败: {}", e))?;
    }

    Ok(ytools_dir)
}

/// 读取 markdown 文件
#[tauri::command]
pub fn read_note(filename: String) -> Result<String, String> {
    let file_path = PathBuf::from(&filename);

    if !file_path.exists() {
        return Ok(String::new());
    }

    let content = fs::read_to_string(&file_path).map_err(|e| format!("读取文件失败: {}", e))?;
    Ok(content)
}

/// 保存 markdown 文件
#[tauri::command]
pub fn save_note(filename: String, content: String) -> Result<(), String> {
    let file_path = PathBuf::from(&filename);

    fs::write(&file_path, content).map_err(|e| format!("保存文件失败: {}", e))?;
    Ok(())
}

/// 获取所有笔记文件列表（仅限 .ytools 目录）
#[tauri::command]
pub fn list_notes() -> Result<Vec<String>, String> {
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

/// 创建新笔记
#[tauri::command]
pub fn create_note(name: String, base_dir: String) -> Result<String, String> {
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

/// 删除笔记文件
#[tauri::command]
pub fn delete_note_file(path: String) -> Result<(), String> {
    let file_path = PathBuf::from(&path);

    if !file_path.exists() {
        return Err(format!("文件不存在: {}", path));
    }

    fs::remove_file(&file_path).map_err(|e| format!("删除文件失败: {}", e))?;
    Ok(())
}

/// 导入笔记（打开文件选择对话框）
#[tauri::command]
pub async fn import_note(app: tauri::AppHandle) -> Result<String, String> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::TempDir;

    #[test]
    fn test_get_ytools_dir() {
        // 测试获取目录路径（应该成功）
        let result = get_ytools_dir();
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.exists());
        assert!(path.is_dir());
    }

    #[test]
    fn test_read_note_file_not_exists() {
        // 测试读取不存在的文件应该返回空字符串
        let result = read_note("non_existent_file.md".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn test_read_and_save_note() {
        // 创建临时目录
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test_note.md");
        let test_content = "# 测试笔记\n\n这是测试内容";

        // 先写入文件
        fs::write(&test_file, test_content).unwrap();

        // 读取文件
        let result = read_note(test_file.to_string_lossy().to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), test_content);
    }

    #[test]
    fn test_save_note() {
        // 创建临时目录
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("save_test.md");
        let test_content = "# 保存测试\n\n保存的内容";

        // 保存文件
        let result = save_note(
            test_file.to_string_lossy().to_string(),
            test_content.to_string(),
        );
        assert!(result.is_ok());

        // 验证文件内容
        let saved_content = fs::read_to_string(&test_file).unwrap();
        assert_eq!(saved_content, test_content);
    }

    #[test]
    fn test_create_note() {
        // 创建临时目录
        let temp_dir = TempDir::new().unwrap();
        let base_dir = temp_dir.path().to_string_lossy().to_string();
        let note_name = "new_note.md";

        // 创建笔记
        let result = create_note(note_name.to_string(), base_dir.clone());
        assert!(result.is_ok());

        let created_path = PathBuf::from(&result.unwrap());
        assert!(created_path.exists());
        assert!(created_path.is_file());
        assert_eq!(created_path.file_name().unwrap(), note_name);
    }

    #[test]
    fn test_create_note_creates_directory() {
        // 创建临时目录作为基础
        let temp_dir = TempDir::new().unwrap();
        let base_dir = temp_dir.path().join("subdir");
        let note_name = "note_in_subdir.md";

        // 创建笔记（目录不存在）
        let result = create_note(
            note_name.to_string(),
            base_dir.to_string_lossy().to_string(),
        );
        assert!(result.is_ok());

        // 验证目录和文件都被创建
        assert!(base_dir.exists());
        assert!(base_dir.is_dir());
        let note_path = base_dir.join(note_name);
        assert!(note_path.exists());
    }

    #[test]
    fn test_delete_note_file() {
        // 创建临时文件
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("delete_test.md");
        fs::write(&test_file, "test content").unwrap();
        assert!(test_file.exists());

        // 删除文件
        let result = delete_note_file(test_file.to_string_lossy().to_string());
        assert!(result.is_ok());

        // 验证文件已被删除
        assert!(!test_file.exists());
    }

    #[test]
    fn test_delete_note_file_not_exists() {
        // 尝试删除不存在的文件
        let result = delete_note_file("non_existent_file.md".to_string());
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("文件不存在"));
    }
}

