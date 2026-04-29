use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub version: String,
    pub sandbox: SandboxConfig,
    pub execution: ExecutionConfig,
    pub api: ApiConfig,
    pub logging: LoggingConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SandboxConfig {
    pub level: String,
    pub enabled: bool,
    pub allow_network: bool,
    pub timeout_seconds: u64,
    pub memory_limit_mb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionConfig {
    pub default_language: String,
    pub python_path: Option<String>,
    pub node_path: Option<String>,
    pub allowed_languages: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiConfig {
    pub host: String,
    pub port: u16,
    pub cors_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingConfig {
    pub level: String,
    pub file: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            version: "1.0.0".to_string(),
            sandbox: SandboxConfig {
                level: "L2".to_string(),
                enabled: true,
                allow_network: false,
                timeout_seconds: 60,
                memory_limit_mb: 256,
            },
            execution: ExecutionConfig {
                default_language: "python".to_string(),
                python_path: None,
                node_path: None,
                allowed_languages: vec![
                    "python".to_string(),
                    "python3".to_string(),
                    "javascript".to_string(),
                    "node".to_string(),
                    "bash".to_string(),
                    "sh".to_string(),
                ],
            },
            api: ApiConfig {
                host: "127.0.0.1".to_string(),
                port: 7860,
                cors_enabled: true,
            },
            logging: LoggingConfig {
                level: "info".to_string(),
                file: None,
            },
        }
    }
}

impl Config {
    pub fn load(path: &Option<PathBuf>) -> Result<Self, String> {
        let config_path = match path {
            Some(p) => p.clone(),
            None => Self::default_path()?,
        };

        if !config_path.exists() {
            return Ok(Config::default());
        }

        let content = fs::read_to_string(&config_path)
            .map_err(|e| format!("读取配置文件失败: {}", e))?;

        let ext = config_path.extension()
            .and_then(|e| e.to_str())
            .unwrap_or("yaml");

        match ext {
            "yaml" | "yml" => {
                serde_yaml::from_str(&content)
                    .map_err(|e| format!("YAML 解析失败: {}", e))
            }
            "json" => {
                serde_json::from_str(&content)
                    .map_err(|e| format!("JSON 解析失败: {}", e))
            }
            _ => Err("不支持的配置文件格式".to_string()),
        }
    }

    pub fn save(&self, path: &PathBuf) -> Result<(), String> {
        let content = serde_yaml::to_string(self)
            .map_err(|e| format!("序列化失败: {}", e))?;

        fs::write(path, content)
            .map_err(|e| format!("写入配置文件失败: {}", e))
    }

    pub fn default_path() -> Result<PathBuf, String> {
        let home = dirs::home_dir()
            .ok_or("无法获取用户主目录")?;

        Ok(home.join(".olaforge").join("config.yaml"))
    }

    pub fn init_default() -> Result<PathBuf, String> {
        let path = Self::default_path()?;
        
        if path.exists() {
            return Ok(path);
        }

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .map_err(|e| format!("创建配置目录失败: {}", e))?;
        }

        let config = Config::default();
        config.save(&path)?;
        
        Ok(path)
    }
}