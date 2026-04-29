use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use chrono::{DateTime, Utc};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionLog {
    pub id: String,
    pub timestamp: String,
    pub code: String,
    pub language: String,
    pub security_level: String,
    pub success: bool,
    pub output: String,
    pub error: Option<String>,
    pub execution_time_ms: u64,
    pub risk_level: String,
    pub issues_count: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogStats {
    pub total_executions: usize,
    pub successful: usize,
    pub failed: usize,
    pub blocked: usize,
    pub avg_execution_time_ms: u64,
}

pub struct LogManager {
    log_dir: PathBuf,
    enabled: bool,
}

impl LogManager {
    pub fn new(log_dir: Option<PathBuf>, enabled: bool) -> Self {
        let dir = log_dir.unwrap_or_else(|| {
            dirs::data_local_dir()
                .unwrap_or_else(|| PathBuf::from("."))
                .join("OlaForge")
                .join("logs")
        });
        
        if enabled && !dir.exists() {
            fs::create_dir_all(&dir).ok();
        }
        
        Self {
            log_dir: dir,
            enabled,
        }
    }
    
    pub fn log_execution(&self, log: &ExecutionLog) -> Result<(), String> {
        if !self.enabled {
            return Ok(());
        }
        
        // 按日期分目录
        let date = &log.timestamp[..10];
        let date_dir = self.log_dir.join(date);
        if !date_dir.exists() {
            fs::create_dir_all(&date_dir).map_err(|e| e.to_string())?;
        }
        
        // 写入日志文件
        let log_file = date_dir.join("executions.jsonl");
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_file)
            .map_err(|e| e.to_string())?;
        
        let json = serde_json::to_string(log).map_err(|e| e.to_string())?;
        writeln!(file, "{}", json).map_err(|e| e.to_string())?;
        
        // 同时写入人类可读的日志
        let text_file = date_dir.join("executions.log");
        let mut text = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&text_file)
            .map_err(|e| e.to_string())?;
        
        let status = if log.success { "✓" } else { "✗" };
        writeln!(
            text,
            "[{}] {} {} ({}ms) - risk:{}",
            log.timestamp, status, log.language, log.execution_time_ms, log.risk_level
        ).map_err(|e| e.to_string())?;
        
        Ok(())
    }
    
    pub fn get_logs(&self, limit: usize) -> Result<Vec<ExecutionLog>, String> {
        if !self.enabled || !self.log_dir.exists() {
            return Ok(Vec::new());
        }
        
        let mut logs = Vec::new();
        
        // 读取最近的文件
        if let Ok(entries) = fs::read_dir(&self.log_dir) {
            let mut dirs: Vec<_> = entries
                .filter_map(|e| e.ok())
                .filter(|e| e.path().is_dir())
                .collect();
            
            dirs.sort_by(|a, b| b.path().cmp(&a.path()));
            
            for dir in dirs.iter().take(7) {
                let log_file = dir.path().join("executions.jsonl");
                if log_file.exists() {
                    if let Ok(content) = fs::read_to_string(&log_file) {
                        for line in content.lines().rev() {
                            if let Ok(log) = serde_json::from_str::<ExecutionLog>(line) {
                                logs.push(log);
                                if logs.len() >= limit {
                                    break;
                                }
                            }
                        }
                    }
                }
                if logs.len() >= limit {
                    break;
                }
            }
        }
        
        logs.sort_by(|a, b| b.timestamp.cmp(&a.timestamp));
        logs.truncate(limit);
        
        Ok(logs)
    }
    
    pub fn get_stats(&self) -> Result<LogStats, String> {
        let logs = self.get_logs(10000)?;
        
        if logs.is_empty() {
            return Ok(LogStats {
                total_executions: 0,
                successful: 0,
                failed: 0,
                blocked: 0,
                avg_execution_time_ms: 0,
            });
        }
        
        let total = logs.len();
        let successful = logs.iter().filter(|l| l.success).count();
        let failed = total - successful;
        let blocked = logs.iter().filter(|l| !l.success && l.error.as_ref().map(|e| e.contains("安全扫描")).unwrap_or(false)).count();
        let avg_time: u64 = logs.iter().map(|l| l.execution_time_ms).sum::<u64>() / total as u64;
        
        Ok(LogStats {
            total_executions: total,
            successful,
            failed,
            blocked,
            avg_execution_time_ms: avg_time,
        })
    }
    
    pub fn get_log_dir(&self) -> &PathBuf {
        &self.log_dir
    }
}

lazy_static::lazy_static! {
    pub static ref LOG_MANAGER: Mutex<LogManager> = Mutex::new(LogManager::new(None, true));
}

pub fn init_log_manager(enabled: bool) {
    if let Ok(mut manager) = LOG_MANAGER.lock() {
        *manager = LogManager::new(None, enabled);
    }
}

pub fn log_execution(log: ExecutionLog) {
    if let Ok(manager) = LOG_MANAGER.lock() {
        manager.log_execution(&log).ok();
    }
}

pub fn get_recent_logs(limit: usize) -> Vec<ExecutionLog> {
    if let Ok(manager) = LOG_MANAGER.lock() {
        manager.get_logs(limit).unwrap_or_default()
    } else {
        Vec::new()
    }
}

pub fn get_log_stats() -> LogStats {
    if let Ok(manager) = LOG_MANAGER.lock() {
        manager.get_stats().unwrap_or(LogStats {
            total_executions: 0,
            successful: 0,
            failed: 0,
            blocked: 0,
            avg_execution_time_ms: 0,
        })
    } else {
        LogStats {
            total_executions: 0,
            successful: 0,
            failed: 0,
            blocked: 0,
            avg_execution_time_ms: 0,
        }
    }
}