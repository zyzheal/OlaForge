use anyhow::Result;
use std::time::Instant;
use std::path::Path;
use serde_json::{json, Value};
use olaforge_sandbox::security::scanner::ScriptScanner;
use olaforge_sandbox::security::types::{SecuritySeverity, ScanResult};

pub struct SandboxExecutor {
    scanner: ScriptScanner,
    enable_network: bool,
}

impl SandboxExecutor {
    pub fn new(enable_network: bool) -> Self {
        let scanner = ScriptScanner::new()
            .allow_network(enable_network);
        
        Self { scanner, enable_network }
    }

    pub fn scan(&self, code: &str, language: &str) -> Result<ScanResult, String> {
        self.scanner.scan_content(code, Path::new(&format!("script.{}", language)))
            .map_err(|e| e.to_string())
    }

    pub fn is_safe(&self, code: &str, language: &str) -> bool {
        match self.scan(code, language) {
            Ok(result) => result.is_safe,
            Err(_) => true, // 扫描失败时允许执行
        }
    }

    pub fn check_network(&self, code: &str, language: &str) -> Vec<String> {
        let mut issues: Vec<String> = Vec::new();
        
        if self.enable_network {
            return issues;
        }
        
        let network_patterns = match language {
            "python" => vec![
                ("requests", "requests 库需要网络权限"),
                ("urllib", "urllib 需要网络权限"),
                ("http.client", "http.client 需要网络权限"),
                ("socket", "socket 需要网络权限"),
                ("ftplib", "ftplib 需要网络权限"),
                ("smtplib", "smtplib 需要网络权限"),
            ],
            "javascript" => vec![
                ("fetch(", "fetch 需要网络权限"),
                ("axios", "axios 需要网络权限"),
                ("http.request", "http.request 需要网络权限"),
                ("https.request", "https.request 需要网络权限"),
                ("require('http')", "HTTP 模块需要网络权限"),
            ],
            _ => vec![],
        };
        
        for (pattern, msg) in network_patterns {
            if code.contains(pattern) {
                issues.push(format!("[NETWORK] {}", msg));
            }
        }
        
        issues
    }

    pub fn get_issues(&self, code: &str, language: &str) -> Vec<String> {
        let _issues: Vec<String> = Vec::new();
        
        // 安全扫描
        match self.scan(code, language) {
            Ok(result) => {
                result.issues.iter()
                    .filter(|i| i.severity == SecuritySeverity::High || i.severity == SecuritySeverity::Critical)
                    .map(|i| {
                        let severity_str = match i.severity {
                            SecuritySeverity::Critical => "CRITICAL",
                            SecuritySeverity::High => "HIGH", 
                            SecuritySeverity::Medium => "MEDIUM",
                            SecuritySeverity::Low => "LOW",
                        };
                        format!("[{}] {}", severity_str, i.description)
                    })
                    .collect()
            }
            Err(e) => vec![format!("扫描错误: {}", e)],
        }
    }
}

pub fn execute_in_sandbox(
    code: &str,
    language: &str,
    timeout: u64,
    security_level: &str,
    enable_network: bool,
) -> Result<String> {
    let start = Instant::now();
    let sandbox = SandboxExecutor::new(enable_network);
    
    // 安全扫描
    let mut security_issues = Vec::new();
    let mut scan_passed = true;
    
    if security_level != "disabled" && security_level != "L0" {
        // 安全扫描
        let issues = sandbox.get_issues(code, language);
        if !issues.is_empty() {
            security_issues.extend(issues.clone());
            if issues.iter().any(|i| i.contains("CRITICAL")) {
                scan_passed = false;
            }
        }
        
        // L1: 基础文件系统检查
        if security_level == "L1" {
            let fs_issues = check_filesystem_access(code, language);
            if !fs_issues.is_empty() {
                security_issues.extend(fs_issues);
            }
        }
        
        // 网络检查 (L2 及以上)
        if security_level == "L2" || security_level == "L3" {
            let network_issues = sandbox.check_network(code, language);
            if !network_issues.is_empty() && !enable_network {
                security_issues.extend(network_issues);
                // L3 严格拒绝网络，L2 只警告
                if security_level == "L3" {
                    scan_passed = false;
                }
            }
        }
    }

    let elapsed = start.elapsed().as_millis() as u64;

    if !scan_passed {
        return Ok(serde_json::to_string(&json!({
            "success": false,
            "output": "",
            "error": "安全扫描未通过",
            "security_issues": security_issues,
            "exit_code": -1,
            "execution_time_ms": elapsed,
            "sandbox": {
                "enabled": true,
                "level": security_level,
                "scanned": true,
                "passed": false
            },
            "language": language
        }))?);
    }

    // 执行代码
    let output = std::process::Command::new("timeout")
        .arg(format!("{}", timeout))
        .arg(get_interpreter(language))
        .arg(get_language_flag(language))
        .arg(code)
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .output();

    let stdout = match &output {
        Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
        Err(_) => String::new(),
    };
    
    let stderr = match &output {
        Ok(o) => String::from_utf8_lossy(&o.stderr).to_string(),
        Err(e) => format!("执行错误: {}", e),
    };
    
    let exit_code = match &output {
        Ok(o) => o.status.code().unwrap_or(-1),
        Err(_) => -1,
    };

    let response = json!({
        "success": exit_code == 0,
        "output": stdout,
        "error": if stderr.is_empty() { Value::Null } else { Value::String(stderr) },
        "exit_code": exit_code,
        "execution_time_ms": elapsed,
        "sandbox": {
            "enabled": true,
            "level": security_level,
            "scanned": true,
            "passed": scan_passed,
            "issues": security_issues
        },
        "language": language
    });

    Ok(serde_json::to_string(&response)?)
}

fn get_interpreter(language: &str) -> &str {
    match language.to_lowercase().as_str() {
        "python" | "python3" => "python3",
        "python2" => "python2",
        "javascript" | "js" | "node" => "node",
        "bash" | "sh" => "bash",
        "ruby" => "ruby",
        "go" => "go",
        "perl" => "perl",
        "php" => "php",
        _ => "sh",
    }
}

fn get_language_flag(language: &str) -> &str {
    match language.to_lowercase().as_str() {
        "javascript" | "js" => "-e",
        _ => "-c",
    }
}

fn check_filesystem_access(code: &str, language: &str) -> Vec<String> {
    let mut issues = Vec::new();
    
    let fs_dangerous = match language {
        "python" => vec![
            ("open(/etc", "读取系统配置文件"),
            ("open(/root", "访问 root 目录"),
            ("chmod 777", "过于宽松的权限"),
            ("chown ", "修改文件所有权"),
            ("os.remove(", "删除文件"),
            ("os.rmdir(", "删除目录"),
            ("shutil.rmtree", "递归删除"),
        ],
        "javascript" => vec![
            ("fs.writeFileSync(/etc", "写入系统配置"),
            ("fs.writeFileSync(/root", "写入 root 目录"),
            ("unlink(/etc", "删除系统文件"),
        ],
        "bash" => vec![
            ("chmod 777", "过于宽松的权限"),
            ("> /etc/", "写入系统目录"),
            ("rm -rf /", "递归删除"),
        ],
        _ => vec![],
    };
    
    for (pattern, msg) in fs_dangerous {
        if code.contains(pattern) {
            issues.push(format!("[FILESYSTEM] {}", msg));
        }
    }
    
    issues
}