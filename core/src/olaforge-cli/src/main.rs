use clap::{Parser, Subcommand};
use anyhow::Result;
use std::process::Command;
use std::time::Instant;
use serde_json::{json, Value};
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::PathBuf;

mod config;
mod sandbox;
mod skill;
mod webui;
mod chat;
mod docker;
use config::Config;
use sandbox::execute_in_sandbox;
use skill::{Skill, list_skills};
use webui::start_web_ui;
use chat::run_interactive;
use docker::{run_in_docker, list_images, check_docker, DockerConfig};

#[derive(Parser)]
#[command(name = "olaforge")]
#[command(about = "OlaForge - 智能安全沙箱系统")]
#[command(long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,

    #[arg(short, long)]
    verbose: bool,

    /// 配置文件路径
    #[arg(long)]
    config: Option<PathBuf>,
}

#[derive(Subcommand)]
enum Commands {
    /// 聊天模式 (AI 助手)
    Chat {
        /// 系统提示词
        #[arg(short, long)]
        system: Option<String>,

        /// 使用模型
        #[arg(short, long, default_value = "gpt-3.5-turbo")]
        model: String,

        /// 非交互模式 (单轮对话)
        #[arg(short, long)]
        prompt: Option<String>,
    },

    /// 执行代码
    Execute {
        #[arg(short, long)]
        code: String,

        #[arg(short, long, default_value = "python")]
        language: String,

        #[arg(short, long, default_value = "L2")]
        security: String,

        #[arg(short, long, default_value_t = 60)]
        timeout: u64,

        #[arg(long, default_value_t = false)]
        no_sandbox: bool,
    },

    /// 运行技能
    Run {
        #[arg(value_name = "SKILL_DIR")]
        skill_dir: Option<String>,

        #[arg(value_name = "INPUT_JSON")]
        input_json: Option<String>,

        #[arg(long)]
        goal: Option<String>,

        #[arg(long)]
        allow_network: bool,

        #[arg(long)]
        sandbox_level: Option<u8>,
    },

    /// 列出可用技能
    Skills,

    /// Docker 容器中执行代码
    Docker {
        #[arg(short, long)]
        code: String,

        #[arg(short, long, default_value = "python")]
        language: String,

        #[arg(short, long)]
        image: Option<String>,

        #[arg(long)]
        network: bool,
    },

    /// 查看 Docker 镜像
    Images,

    /// 启动 Web UI
    Webui {
        #[arg(short, long, default_value_t = 8080)]
        port: u16,

        #[arg(long, default_value = "127.0.0.1")]
        host: String,
    },

    /// 启动 API 服务
    Serve {
        #[arg(short, long, default_value_t = 7860)]
        port: u16,

        #[arg(long, default_value = "127.0.0.1")]
        host: String,
    },

    /// 初始化配置文件
    Init {
        /// 输出路径
        #[arg(short, long)]
        path: Option<PathBuf>,
    },

    /// 显示当前配置
    Config {
        /// 显示完整配置
        #[arg(short, long)]
        full: bool,
    },

    /// 健康检查
    Health,

    /// 版本信息
    Version,
}

fn get_interpreter(language: &str, config: &Config) -> String {
    if let Some(ref python_path) = config.execution.python_path {
        if language == "python" || language == "python3" {
            return python_path.clone();
        }
    }
    if let Some(ref node_path) = config.execution.node_path {
        if language == "javascript" || language == "js" || language == "node" {
            return node_path.clone();
        }
    }

    match language.to_lowercase().as_str() {
        "python" | "python3" => "python3",
        "python2" => "python2",
        "javascript" | "js" | "node" => "node",
        "bash" | "sh" => "bash",
        "ruby" => "ruby",
        "go" => "go",
        "rust" => "rustc",
        "perl" => "perl",
        "php" => "php",
        _ => "sh",
    }.to_string()
}

fn execute_code(code: &str, language: &str, timeout: u64, no_sandbox: bool, config: &Config) -> Result<String> {
    let interpreter = get_interpreter(language, config);
    let start = Instant::now();

    let effective_timeout = if timeout > 0 { timeout } else { config.sandbox.timeout_seconds };

    let output = if effective_timeout > 0 {
        let interpreter_ref = &interpreter;
        let result = Command::new("timeout")
            .arg(format!("{}", effective_timeout))
            .arg(interpreter_ref)
            .arg(if language == "javascript" || language == "js" { "-e" } else { "-c" })
            .arg(code)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .output();
        
        match result {
            Ok(o) => o,
            Err(_) => {
                Command::new(&interpreter)
                    .arg("-c")
                    .arg(code)
                    .stdout(std::process::Stdio::piped())
                    .stderr(std::process::Stdio::piped())
                    .output()?
            }
        }
    } else {
        Command::new(&interpreter)
            .arg(if language == "javascript" || language == "js" { "-e" } else { "-c" })
            .arg(code)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::piped())
            .output()?
    };

    let elapsed = start.elapsed().as_millis() as u64;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let exit_code = output.status.code().unwrap_or(-1);

    let response = json!({
        "success": exit_code == 0,
        "output": stdout,
        "error": if stderr.is_empty() { Value::Null } else { Value::String(stderr) },
        "exit_code": exit_code,
        "execution_time_ms": elapsed,
        "sandbox": {
            "enabled": !no_sandbox && config.sandbox.enabled,
            "level": if no_sandbox || !config.sandbox.enabled { "disabled" } else { &config.sandbox.level }
        },
        "language": language
    });

    Ok(serde_json::to_string(&response)?)
}

fn handle_request(request: &str) -> String {
    let lines: Vec<&str> = request.lines().collect();
    if lines.is_empty() {
        return "HTTP/1.1 400 Bad Request\r\n\r\n{\"error\":\"Empty request\"}".to_string();
    }

    let request_line = lines.first().unwrap();
    let parts: Vec<&str> = request_line.split_whitespace().collect();
    
    if parts.len() < 2 {
        return "HTTP/1.1 400 Bad Request\r\n\r\n".to_string();
    }

    let method = parts[0];
    let path = parts[1];

    let body_start = request.find("\r\n\r\n").map(|i| i + 4).unwrap_or(0);
    let body = &request[body_start..];

    match (method, path) {
        ("GET", "/") => {
            r#"HTTP/1.1 200 OK\r
Content-Type: application/json\r
Access-Control-Allow-Origin: *\r
\r
{
    "name": "OlaForge API",
    "version": "1.0.0",
    "endpoints": {
        "POST /execute": "执行代码",
        "GET /health": "健康检查",
        "GET /version": "版本信息",
        "GET /config": "获取配置"
    }
}"#.to_string()
        }
        
        ("GET", "/health") => {
            r#"HTTP/1.1 200 OK\r
Content-Type: application/json\r
Access-Control-Allow-Origin: *\r
\r
{"status":"healthy","version":"1.0.0"}"#.to_string()
        }
        
        ("GET", "/version") => {
            format!(r#"HTTP/1.1 200 OK\r
Content-Type: application/json\r
Access-Control-Allow-Origin: *\r
\r
{}"#, r#"{"version":"1.0.0","name":"OlaForge"}"#)
        }
        
        ("GET", "/config") => {
            let config = Config::default();
            let json = serde_json::to_string(&config).unwrap_or_default();
            format!(r#"HTTP/1.1 200 OK\r
Content-Type: application/json\r
Access-Control-Allow-Origin: *\r
Content-Length: {}\r
\r
{}"#, json.len(), json)
        }
        
        ("POST", "/execute") => {
            #[derive(serde::Deserialize)]
            struct ExecuteRequest {
                code: String,
                language: Option<String>,
                timeout: Option<u64>,
                no_sandbox: Option<bool>,
            }

            let config = Config::default();
            let req: Result<ExecuteRequest, _> = serde_json::from_str(body);
            
            match req {
                Ok(r) => {
                    let language = r.language.unwrap_or_else(|| config.execution.default_language.clone());
                    let timeout = r.timeout.unwrap_or(config.sandbox.timeout_seconds);
                    let no_sandbox = r.no_sandbox.unwrap_or(!config.sandbox.enabled);
                    
                    match execute_code(&r.code, &language, timeout, no_sandbox, &config) {
                        Ok(result) => {
                            format!(r#"HTTP/1.1 200 OK\r
Content-Type: application/json\r
Access-Control-Allow-Origin: *\r
Content-Length: {}\r
\r
{}"#, result.len(), result)
                        }
                        Err(e) => {
                            format!(r#"HTTP/1.1 500 Internal Server Error\r
Content-Type: application/json\r
Access-Control-Allow-Origin: *\r
\r
{{"error":"{}"}}"#, e)
                        }
                    }
                }
                Err(e) => {
                    format!(r#"HTTP/1.1 400 Bad Request\r
Content-Type: application/json\r
Access-Control-Allow-Origin: *\r
\r
{{"error":"Invalid JSON: {}"}}"#, e)
                }
            }
        }
        
        _ => {
            format!(r#"HTTP/1.1 404 Not Found\r
Content-Type: application/json\r
Access-Control-Allow-Origin: *\r
\r
{{"error":"Not found: {}"}}"#, path)
        }
    }
}

fn run_server(host: &str, port: u16) -> Result<()> {
    let addr = format!("{}:{}", host, port);
    
    println!("🚀 OlaForge API 服务启动中...");
    println!("📍 http://{}", addr);
    println!("📚 API 文档: http://{}/", addr);
    println!("❤️  健康检查: http://{}/health", addr);
    println!("⏹  按 Ctrl+C 停止服务");
    
    let listener = TcpListener::bind(&addr)?;
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buffer = [0u8; 4096];
                let n = stream.read(&mut buffer).unwrap_or(0);
                let request = String::from_utf8_lossy(&buffer[..n]);
                
                let response = handle_request(&request);
                stream.write_all(response.as_bytes()).ok();
            }
            Err(e) => {
                eprintln!("连接错误: {}", e);
            }
        }
    }
    
    Ok(())
}

fn main_cli() -> Result<()> {
    let cli = Cli::parse();
    
    let config = Config::load(&cli.config).unwrap_or_default();

    match cli.command {
        Commands::Chat { system, model, prompt } => {
            if let Some(p) = prompt {
                // 单轮对话模式
                let mut session = chat::ChatSession::new(model);
                if let Some(ref sys) = system {
                    session.add_system_prompt(sys);
                }
                match session.send_message(&p) {
                    Ok(resp) => print!("{}", resp),
                    Err(e) => print!("{{\"error\":\"{}\"}}", e),
                }
            } else {
                // 交互模式
                run_interactive(system.as_deref())?;
            }
        }
        
        Commands::Execute { code, language, security, timeout, no_sandbox } => {
            if cli.verbose {
                eprintln!("执行: {} (安全: {})", language, security);
                eprintln!("超时: {}s", timeout);
                eprintln!("沙箱: {}", if no_sandbox { "禁用" } else { "启用" });
            }
            
            let effective_security = if no_sandbox { "disabled".to_string() } else { security };
            let result = execute_in_sandbox(
                &code,
                &language,
                timeout,
                &effective_security,
                config.sandbox.allow_network
            )?;
            print!("{}", result);
        }
        
        Commands::Run { skill_dir, input_json, goal, allow_network: _, sandbox_level: _ } => {
            if cli.verbose {
                eprintln!("运行技能: {:?}", skill_dir);
                eprintln!("目标: {:?}", goal);
            }
            
            let input = input_json.unwrap_or_else(|| r#"{"goal":"test"}"#.to_string());
            
            if let Some(dir) = skill_dir {
                let path = PathBuf::from(&dir);
                match Skill::load(&path) {
                    Ok(skill) => {
                        match skill.execute(&path, &input) {
                            Ok(result) => print!("{}", result),
                            Err(e) => print!("{{\"error\":\"{}\"}}", e),
                        }
                    }
                    Err(e) => print!("{{\"error\":\"加载技能失败: {}\"}}", e),
                }
            } else {
                // 列出可用技能
                let skills_dir = PathBuf::from(".skills");
                match list_skills(&skills_dir) {
                    Ok(skills) => {
                        let list: Vec<_> = skills.iter().map(|s| serde_json::json!({
                            "name": s.name,
                            "version": s.version,
                            "description": s.description
                        })).collect();
                        print!("{}", serde_json::to_string(&list).unwrap_or_default());
                    }
                    Err(e) => print!("{{\"error\":\"{}\"}}", e),
                }
            }
        }

        Commands::Skills => {
            let skills_dir = PathBuf::from(".skills");
            match list_skills(&skills_dir) {
                Ok(skills) => {
                    if skills.is_empty() {
                        print!("{{\"skills\":[],\"message\":\"未找到技能，使用 olaforge run <skill_dir> 指定技能目录\"}}");
                    } else {
                        let list: Vec<_> = skills.iter().map(|s| serde_json::json!({
                            "name": s.name,
                            "version": s.version,
                            "description": s.description,
                            "language": s.language,
                            "entry_point": s.entry_point
                        })).collect();
                        print!("{{\"skills\":{}}}", serde_json::to_string(&list).unwrap_or_default());
                    }
                }
                Err(e) => print!("{{\"error\":\"{}\"}}", e),
            }
        }
        
        Commands::Docker { code, language, image, network } => {
            if !check_docker()? {
                print!("{{\"error\":\"Docker 未安装或未运行\"}}");
            } else {
                let docker_config = DockerConfig {
                    image: image.unwrap_or_else(|| "python:3.11-slim".to_string()),
                    memory_limit: Some("512m".to_string()),
                    cpu_limit: Some(1.0),
                    network,
                };
                match run_in_docker(&code, &language, &docker_config) {
                    Ok(result) => print!("{}", result),
                    Err(e) => print!("{{\"error\":\"{}\"}}", e),
                }
            }
        }
        
        Commands::Images => {
            match list_images() {
                Ok(images) => {
                    print!("{{\"images\":{}}}", serde_json::to_string(&images).unwrap_or_default());
                }
                Err(e) => print!("{{\"error\":\"{}\"}}", e),
            }
        }
        
        Commands::Webui { port, host } => {
            start_web_ui(&host, port)?;
        }
        
        Commands::Serve { port, host } => {
            run_server(&host, port)?;
        }
        
        Commands::Init { path } => {
            let init_path = path.unwrap_or_else(|| Config::default_path().unwrap_or_default());
            let config = Config::default();
            config.save(&init_path).map_err(|e| anyhow::anyhow!("{}", e))?;
            print!("配置文件已创建: {}", init_path.display());
        }
        
        Commands::Config { full } => {
            if full {
                let json = serde_json::to_string_pretty(&config)?;
                print!("{}", json);
            } else {
                print!("版本: {}", config.version);
                println!("沙箱级别: {}", config.sandbox.level);
                println!("超时: {}s", config.sandbox.timeout_seconds);
                println!("默认语言: {}", config.execution.default_language);
                println!("API端口: {}", config.api.port);
            }
        }
        
        Commands::Health => {
            print!("{{\"status\":\"healthy\",\"version\":\"1.0.0\"}}");
        }
        
        Commands::Version => {
            println!("OlaForge v1.0.0");
            println!("  - Core: v{}", env!("CARGO_PKG_VERSION"));
            println!("  - Sandbox: v{}", env!("CARGO_PKG_VERSION"));
            println!("  - Executor: v{}", env!("CARGO_PKG_VERSION"));
            println!("  - Evolution: v{}", env!("CARGO_PKG_VERSION"));
            println!();
            println!("基于 SkillLite 代码复用");
        }
    }

    Ok(())
}

fn main() {
    if let Err(e) = main_cli() {
        let exit_code = if e.to_string().contains("超时") {
            124  // 标准超时退出码
        } else if e.to_string().contains("权限") {
            77  // 权限拒绝
        } else if e.to_string().contains("未找到") {
            127  // 命令未找到
        } else {
            1   // 一般错误
        };
        
        eprintln!("❌ 错误: {}", e);
        std::process::exit(exit_code);
    }
}