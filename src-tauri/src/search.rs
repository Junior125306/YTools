//! 搜索模块
//! 
//! 提供工作区搜索功能，包括拼音匹配、模糊匹配等算法

use pinyin::ToPinyinMulti;
use serde::Serialize;
use std::fs;
use std::path::{Component, Path, PathBuf};

const MAX_PINYIN_VARIANTS: usize = 32;
const MAX_READINGS_PER_CHAR: usize = 2;

/// 工作区搜索命中
#[derive(Debug, Clone, Serialize, PartialEq, Eq)]
pub struct WorkspaceHit {
    pub name: String,
    pub path: String,
}

fn expand_pinyin_variants(current: &[String], additions: &[String]) -> Vec<String> {
    let mut next = Vec::new();
    for addition in additions {
        for base in current {
            if next.len() >= MAX_PINYIN_VARIANTS {
                return next;
            }
            let mut combined = base.clone();
            combined.push_str(addition);
            next.push(combined);
        }
    }
    next
}

/// 规范化字符串：转小写并移除常见分隔符，便于宽松匹配
pub(crate) fn normalize(input: &str) -> String {
    input
        .chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
        .to_lowercase()
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
                let additions: Vec<String> = pinyin_multi
                    .into_iter()
                    .take(MAX_READINGS_PER_CHAR)
                    .map(|p| p.plain().to_string())
                    .collect();
                let new_results = expand_pinyin_variants(&current_results, &additions);
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
                let additions: Vec<String> = pinyin_multi
                    .into_iter()
                    .take(MAX_READINGS_PER_CHAR)
                    .filter_map(|p| p.plain().chars().next().map(|ch| ch.to_string()))
                    .collect();
                let new_results = expand_pinyin_variants(&current_results, &additions);
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

fn is_safe_search_root(path: &Path) -> bool {
    !path.components().any(|c| matches!(c, Component::ParentDir))
}

/// 搜索工作区文件夹，返回名称与完整路径
#[tauri::command]
pub fn search_workspaces(
    query: String,
    directories: Vec<String>,
) -> Result<Vec<WorkspaceHit>, String> {
    if directories.is_empty() {
        return Ok(Vec::new());
    }

    let query_lower = query.to_lowercase();
    let query_norm = normalize(&query_lower);
    let mut results: Vec<WorkspaceHit> = Vec::new();
    let mut all_dirs: Vec<WorkspaceHit> = Vec::new();

    for workspace_dir_str in directories {
        let workspace_dir = PathBuf::from(&workspace_dir_str);
        if !is_safe_search_root(&workspace_dir) || !workspace_dir.exists() {
            continue;
        }

        if let Ok(entries) = fs::read_dir(workspace_dir) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        if let Ok(dir_name) = entry.file_name().into_string() {
                            let hit = WorkspaceHit {
                                name: dir_name.clone(),
                                path: entry.path().to_string_lossy().to_string(),
                            };
                            let name_lower = dir_name.to_lowercase();
                            let name_norm = normalize(&name_lower);

                            let name_pinyin_full_multi = to_pinyin_full_multi(&dir_name);
                            let name_pinyin_initials_multi = to_pinyin_initials_multi(&dir_name);

                            let pinyin_full_match = name_pinyin_full_multi
                                .iter()
                                .any(|py| py.contains(&query_lower));
                            let pinyin_initials_match = name_pinyin_initials_multi
                                .iter()
                                .any(|py| py.contains(&query_lower));

                            all_dirs.push(hit.clone());

                            if query.is_empty()
                                || name_lower.contains(&query_lower)
                                || (!query_norm.is_empty() && name_norm.contains(&query_norm))
                                || pinyin_full_match
                                || pinyin_initials_match
                            {
                                results.push(hit);
                            }
                        }
                    }
                }
            }
        }
    }

    if !results.is_empty() || query.is_empty() {
        results.sort_by(|a, b| a.name.cmp(&b.name).then(a.path.cmp(&b.path)));
        return Ok(results);
    }

    let mut scored: Vec<(WorkspaceHit, usize, usize, usize)> = Vec::new();
    for hit in all_dirs.into_iter() {
        let name_lower = hit.name.to_lowercase();
        let name_norm = normalize(&name_lower);
        let name_pinyin_full_multi = to_pinyin_full_multi(&hit.name);
        let name_pinyin_initials_multi = to_pinyin_initials_multi(&hit.name);

        let lcs_sub = longest_common_substring_len(&name_norm, &query_norm);
        let lcs_seq = longest_common_subsequence_len(&name_norm, &query_norm);
        let edit = levenshtein_distance(&name_norm, &query_norm);

        let mut best_pinyin_lcs_sub = 0;
        let mut best_pinyin_lcs_seq = 0;
        let mut best_pinyin_edit = usize::MAX;
        for pinyin in &name_pinyin_full_multi {
            best_pinyin_lcs_sub =
                best_pinyin_lcs_sub.max(longest_common_substring_len(pinyin, &query_lower));
            best_pinyin_lcs_seq =
                best_pinyin_lcs_seq.max(longest_common_subsequence_len(pinyin, &query_lower));
            best_pinyin_edit = best_pinyin_edit.min(levenshtein_distance(pinyin, &query_lower));
        }

        let mut best_initials_lcs_sub = 0;
        let mut best_initials_lcs_seq = 0;
        let mut best_initials_edit = usize::MAX;
        for initials in &name_pinyin_initials_multi {
            best_initials_lcs_sub =
                best_initials_lcs_sub.max(longest_common_substring_len(initials, &query_lower));
            best_initials_lcs_seq =
                best_initials_lcs_seq.max(longest_common_subsequence_len(initials, &query_lower));
            best_initials_edit =
                best_initials_edit.min(levenshtein_distance(initials, &query_lower));
        }

        let best_lcs_sub = lcs_sub.max(best_pinyin_lcs_sub).max(best_initials_lcs_sub);
        let best_lcs_seq = lcs_seq.max(best_pinyin_lcs_seq).max(best_initials_lcs_seq);
        let best_edit = edit.min(best_pinyin_edit).min(best_initials_edit);

        scored.push((hit, best_lcs_sub, best_lcs_seq, best_edit));
    }

    scored.sort_by(|a, b| {
        b.1.cmp(&a.1)
            .then(b.2.cmp(&a.2))
            .then(a.3.cmp(&b.3))
            .then(a.0.name.cmp(&b.0.name))
    });

    Ok(scored.into_iter().take(5).map(|t| t.0).collect())
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
        assert_eq!(result.unwrap(), Vec::<WorkspaceHit>::new());
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
        let results = result.unwrap();
        assert!(results.iter().any(|h| h.name == "dir1"));
        assert!(results.iter().any(|h| h.name == "dir2"));
        assert!(results.iter().any(|h| h.name == "dir3"));
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
        assert_eq!(results[0].name, "my-project");
        assert!(results[0].path.ends_with("my-project"));
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
        assert!(results.iter().any(|h| h.name == "my-project-v2"));
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
        assert!(results.iter().any(|h| h.name == "MyProject"));
    }

    #[test]
    fn test_search_workspaces_nonexistent_directory() {
        // 测试不存在的目录应该被跳过
        let result = search_workspaces(
            "test".to_string(),
            vec!["/nonexistent/path".to_string()],
        );
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), Vec::<WorkspaceHit>::new());
    }
}

