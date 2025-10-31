//! 搜索模块
//! 
//! 提供工作区搜索功能，包括拼音匹配、模糊匹配等算法

use pinyin::ToPinyinMulti;
use std::fs;
use std::path::PathBuf;

/// 规范化字符串：转小写并移除常见分隔符，便于宽松匹配
pub(crate) fn normalize(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_ascii_alphanumeric())
        .collect::<String>()
}

/// 将中文字符串转换为拼音全拼（小写，无分隔符）
/// 为了处理多音字，返回多个可能的拼音组合
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

/// 将中文字符串转换为拼音首字母缩写（小写）
/// 为了处理多音字，返回多个可能的首字母组合
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

/// 最长公共子串长度（连续匹配长度）
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

/// 最长公共子序列长度（非连续匹配总量）
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

/// Levenshtein 编辑距离（大小写与分隔符已通过 normalize 处理在外部）
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

/// 搜索工作区文件夹
#[tauri::command]
pub fn search_workspaces(query: String, directories: Vec<String>) -> Result<Vec<String>, String> {
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

/// 打开文件夹（已废弃，使用 open_directory 代替）
#[tauri::command]
pub fn open_folder(folder_name: String) -> Result<(), String> {
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

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_normalize() {
        assert_eq!(normalize("Hello-World"), "helloworld");
        assert_eq!(normalize("Test_123"), "test123");
        assert_eq!(normalize("项目-管理"), "项目管理");
        assert_eq!(normalize(""), "");
    }

    #[test]
    fn test_search_workspaces_empty_directories() {
        // 测试空目录列表
        let result = search_workspaces("test".to_string(), vec![]);
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Vec::<String>::new());
    }

    #[test]
    fn test_search_workspaces_empty_query() {
        // 创建临时目录结构
        let temp_dir = TempDir::new().unwrap();
        let workspace_dir = temp_dir.path();
        
        // 创建测试目录
        fs::create_dir_all(workspace_dir.join("dir1")).unwrap();
        fs::create_dir_all(workspace_dir.join("dir2")).unwrap();
        fs::create_dir_all(workspace_dir.join("dir3")).unwrap();

        // 测试空查询应该返回所有目录
        let result = search_workspaces(
            "".to_string(),
            vec![workspace_dir.to_string_lossy().to_string()],
        );
        assert!(result.is_ok());
        let mut results = result.unwrap();
        results.sort();
        assert!(results.contains(&"dir1".to_string()));
        assert!(results.contains(&"dir2".to_string()));
        assert!(results.contains(&"dir3".to_string()));
    }

    #[test]
    fn test_search_workspaces_exact_match() {
        // 创建临时目录结构
        let temp_dir = TempDir::new().unwrap();
        let workspace_dir = temp_dir.path();
        
        fs::create_dir_all(workspace_dir.join("my-project")).unwrap();
        fs::create_dir_all(workspace_dir.join("other-project")).unwrap();

        // 测试精确匹配
        let result = search_workspaces(
            "my-project".to_string(),
            vec![workspace_dir.to_string_lossy().to_string()],
        );
        assert!(result.is_ok());
        let results = result.unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[0], "my-project");
    }

    #[test]
    fn test_search_workspaces_partial_match() {
        // 创建临时目录结构
        let temp_dir = TempDir::new().unwrap();
        let workspace_dir = temp_dir.path();
        
        fs::create_dir_all(workspace_dir.join("my-project-v2")).unwrap();
        fs::create_dir_all(workspace_dir.join("other-project")).unwrap();

        // 测试部分匹配
        let result = search_workspaces(
            "my-project".to_string(),
            vec![workspace_dir.to_string_lossy().to_string()],
        );
        assert!(result.is_ok());
        let results = result.unwrap();
        assert!(results.contains(&"my-project-v2".to_string()));
    }

    #[test]
    fn test_search_workspaces_case_insensitive() {
        // 创建临时目录结构
        let temp_dir = TempDir::new().unwrap();
        let workspace_dir = temp_dir.path();
        
        fs::create_dir_all(workspace_dir.join("MyProject")).unwrap();

        // 测试大小写不敏感
        let result = search_workspaces(
            "myproject".to_string(),
            vec![workspace_dir.to_string_lossy().to_string()],
        );
        assert!(result.is_ok());
        let results = result.unwrap();
        assert!(results.contains(&"MyProject".to_string()));
    }

    #[test]
    fn test_search_workspaces_nonexistent_directory() {
        // 测试不存在的目录应该被跳过
        let result = search_workspaces(
            "test".to_string(),
            vec!["/nonexistent/path".to_string()],
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Vec::<String>::new());
    }
}

