//! 笔记管理模块
//!
//! 提供笔记文件的创建、读取、保存、导入等功能（删除仅在前端移除引用）

use std::fs;
use std::path::{Component, Path, PathBuf};

/// 获取用户目录下的 .ytools 路径
#[allow(dead_code)]
pub fn get_ytools_dir() -> Result<PathBuf, String> {
    let home = dirs::home_dir().ok_or("无法获取用户目录")?;
    let ytools_dir = home.join(".ytools");

    if !ytools_dir.exists() {
        fs::create_dir_all(&ytools_dir).map_err(|e| format!("创建目录失败: {}", e))?;
    }

    Ok(ytools_dir)
}

fn has_parent_dir(path: &Path) -> bool {
    path.components().any(|c| matches!(c, Component::ParentDir))
}

fn is_allowed_note_file(path: &Path) -> bool {
    if has_parent_dir(path) {
        return false;
    }
    matches!(
        path.extension()
            .and_then(|s| s.to_str())
            .map(|s| s.to_ascii_lowercase())
            .as_deref(),
        Some("md") | Some("txt")
    )
}

fn is_allowed_directory(path: &Path) -> bool {
    !has_parent_dir(path)
}

/// 读取笔记文件
#[tauri::command]
pub fn read_note(filename: String) -> Result<String, String> {
    let file_path = PathBuf::from(&filename);
    if !is_allowed_note_file(&file_path) {
        return Err("不允许的笔记路径".to_string());
    }

    if !file_path.exists() {
        return Ok(String::new());
    }

    fs::read_to_string(&file_path).map_err(|e| format!("读取文件失败: {}", e))
}

/// 保存笔记文件
#[tauri::command]
pub fn save_note(filename: String, content: String) -> Result<(), String> {
    let file_path = PathBuf::from(&filename);
    if !is_allowed_note_file(&file_path) {
        return Err("不允许的笔记路径".to_string());
    }

    fs::write(&file_path, content).map_err(|e| format!("保存文件失败: {}", e))?;
    Ok(())
}

/// 创建新笔记
#[tauri::command]
pub fn create_note(name: String, base_dir: String) -> Result<String, String> {
    if name.contains("..") || name.contains('/') || name.contains('\\') {
        return Err("非法的笔记名称".to_string());
    }

    let base_path = PathBuf::from(&base_dir);
    if !is_allowed_directory(&base_path) {
        return Err("不允许的笔记目录".to_string());
    }

    if !base_path.exists() {
        fs::create_dir_all(&base_path).map_err(|e| format!("创建目录失败: {}", e))?;
    }

    let file_path = base_path.join(&name);
    if !is_allowed_note_file(&file_path) {
        return Err("仅支持 .md 或 .txt 笔记".to_string());
    }

    fs::write(&file_path, "").map_err(|e| format!("创建文件失败: {}", e))?;
    Ok(file_path.to_string_lossy().to_string())
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
                if !is_allowed_note_file(path_ref) {
                    return Err("仅支持导入 .md 或 .txt 文件".to_string());
                }
                Ok(path_ref.to_string_lossy().to_string())
            } else {
                Ok(String::new())
            }
        }
        None => Ok(String::new()),
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
        let result = get_ytools_dir();
        assert!(result.is_ok());
        let path = result.unwrap();
        assert!(path.exists());
        assert!(path.is_dir());
    }

    #[test]
    fn test_read_note_file_not_exists() {
        let result = read_note("non_existent_file.md".to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "");
    }

    #[test]
    fn test_read_note_rejects_parent_dir() {
        let result = read_note("../secret.md".to_string());
        assert!(result.is_err());
    }

    #[test]
    fn test_read_and_save_note() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("test_note.md");
        let test_content = "# 测试笔记\n\n这是测试内容";
        fs::write(&test_file, test_content).unwrap();

        let result = read_note(test_file.to_string_lossy().to_string());
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), test_content);
    }

    #[test]
    fn test_save_note() {
        let temp_dir = TempDir::new().unwrap();
        let test_file = temp_dir.path().join("save_test.md");
        let test_content = "# 保存测试\n\n保存的内容";

        let result = save_note(
            test_file.to_string_lossy().to_string(),
            test_content.to_string(),
        );
        assert!(result.is_ok());

        let saved_content = fs::read_to_string(&test_file).unwrap();
        assert_eq!(saved_content, test_content);
    }

    #[test]
    fn test_create_note() {
        let temp_dir = TempDir::new().unwrap();
        let base_dir = temp_dir.path().to_string_lossy().to_string();
        let note_name = "new_note.md";

        let result = create_note(note_name.to_string(), base_dir.clone());
        assert!(result.is_ok());

        let created_path = PathBuf::from(&result.unwrap());
        assert!(created_path.exists());
        assert!(created_path.is_file());
        assert_eq!(created_path.file_name().unwrap(), note_name);
    }

    #[test]
    fn test_create_note_creates_directory() {
        let temp_dir = TempDir::new().unwrap();
        let base_dir = temp_dir.path().join("subdir");
        let note_name = "note_in_subdir.md";

        let result = create_note(
            note_name.to_string(),
            base_dir.to_string_lossy().to_string(),
        );
        assert!(result.is_ok());

        assert!(base_dir.exists());
        assert!(base_dir.is_dir());
        let note_path = base_dir.join(note_name);
        assert!(note_path.exists());
    }

    #[test]
    fn test_create_note_rejects_path_in_name() {
        let temp_dir = TempDir::new().unwrap();
        let result = create_note(
            "../escape.md".to_string(),
            temp_dir.path().to_string_lossy().to_string(),
        );
        assert!(result.is_err());
    }
}
