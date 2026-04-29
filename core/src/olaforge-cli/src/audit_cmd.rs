use std::path::PathBuf;
use olaforge_sandbox::security::dependency_audit::{audit_skill_dependencies, format_audit_result, format_audit_result_json};

pub fn run_audit(path: Option<&str>, format: &str) -> Result<String, String> {
    let skill_dir = if let Some(p) = path {
        PathBuf::from(p)
    } else {
        // 默认当前目录
        std::env::current_dir().map_err(|e| e.to_string())?
    };

    if !skill_dir.exists() {
        return Err(format!("路径不存在: {}", skill_dir.display()));
    }

    if !skill_dir.is_dir() {
        return Err(format!("路径不是目录: {}", skill_dir.display()));
    }

    let result = audit_skill_dependencies(&skill_dir, None)
        .map_err(|e| format!("审计失败: {}", e))?;

    match format {
        "text" | "txt" => Ok(format_audit_result(&result)),
        _ => Ok(format_audit_result_json(&result)),
    }
}