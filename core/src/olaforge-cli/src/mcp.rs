use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::io::{Read, Write};
use std::process::{Command, Stdio};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpTool {
    pub name: String,
    pub description: String,
    pub input_schema: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpRequest {
    pub jsonrpc: String,
    pub id: Option<usize>,
    pub method: String,
    pub params: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpResponse {
    pub jsonrpc: String,
    pub id: Option<usize>,
    pub result: Option<serde_json::Value>,
    pub error: Option<McpError>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct McpError {
    pub code: i32,
    pub message: String,
    pub data: Option<serde_json::Value>,
}

pub fn get_mcp_tools() -> Vec<McpTool> {
    vec![
        McpTool {
            name: "execute_code".to_string(),
            description: "在沙箱中安全地执行代码".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "code": {
                        "type": "string",
                        "description": "要执行的代码"
                    },
                    "language": {
                        "type": "string",
                        "enum": ["python", "javascript", "bash", "ruby", "go", "perl"],
                        "description": "编程语言"
                    },
                    "security": {
                        "type": "string",
                        "enum": ["L0", "L1", "L2", "L3"],
                        "description": "安全级别",
                        "default": "L2"
                    },
                    "timeout": {
                        "type": "number",
                        "description": "超时秒数",
                        "default": 60
                    }
                },
                "required": ["code", "language"]
            }),
        },
        McpTool {
            name: "list_skills".to_string(),
            description: "列出所有可用的技能".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {},
            }),
        },
        McpTool {
            name: "run_skill".to_string(),
            description: "执行一个预定义的技能".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "skill_dir": {
                        "type": "string",
                        "description": "技能目录路径"
                    },
                    "input_json": {
                        "type": "string",
                        "description": "输入参数 (JSON 字符串)"
                    }
                },
                "required": ["skill_dir"]
            }),
        },
        McpTool {
            name: "audit_dependencies".to_string(),
            description: "依赖安全审计 (OSV)".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "要审计的目录路径"
                    }
                },
                "required": ["path"]
            }),
        },
        McpTool {
            name: "get_logs".to_string(),
            description: "获取执行日志".to_string(),
            input_schema: serde_json::json!({
                "type": "object",
                "properties": {
                    "limit": {
                        "type": "number",
                        "description": "返回数量",
                        "default": 10
                    },
                    "stats": {
                        "type": "boolean",
                        "description": "是否返回统计信息",
                        "default": false
                    }
                },
            }),
        },
    ]
}

pub fn handle_mcp_request(request: &McpRequest) -> McpResponse {
    let id = request.id.clone();
    
    match request.method.as_str() {
        "initialize" => {
            McpResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: Some(serde_json::json!({
                    "protocolVersion": "2024-11-05",
                    "serverInfo": {
                        "name": "olaforge",
                        "version": env!("CARGO_PKG_VERSION")
                    },
                    "capabilities": {
                        "tools": {}
                    }
                })),
                error: None,
            }
        }
        
        "tools/list" => {
            McpResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: Some(serde_json::json!({
                    "tools": get_mcp_tools()
                })),
                error: None,
            }
        }
        
        "tools/call" => {
            if let Some(params) = &request.params {
                let tool_name = params.get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("");
                let arguments = params.get("arguments")
                    .and_then(|v| v.as_object())
                    .map(|m| serde_json::Value::Object(m.clone()))
                    .unwrap_or(serde_json::Value::Null);
                
                let result = execute_tool(tool_name, &arguments);
                
                McpResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: Some(serde_json::json!({
                        "content": [
                            {
                                "type": "text",
                                "text": result
                            }
                        ]
                    })),
                    error: None,
                }
            } else {
                McpResponse {
                    jsonrpc: "2.0".to_string(),
                    id,
                    result: None,
                    error: Some(McpError {
                        code: -32602,
                        message: "Invalid params".to_string(),
                        data: None,
                    }),
                }
            }
        }
        
        _ => {
            McpResponse {
                jsonrpc: "2.0".to_string(),
                id,
                result: None,
                error: Some(McpError {
                    code: -32601,
                    message: format!("Method not found: {}", request.method),
                    data: None,
                }),
            }
        }
    }
}

fn execute_tool(name: &str, arguments: &serde_json::Value) -> String {
    let binary = std::env::current_exe()
        .unwrap_or_else(|_| std::path::PathBuf::from("olaforge"));
    
    match name {
        "execute_code" => {
            let code = arguments.get("code")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let language = arguments.get("language")
                .and_then(|v| v.as_str())
                .unwrap_or("python");
            let security = arguments.get("security")
                .and_then(|v| v.as_str())
                .unwrap_or("L2");
            let timeout = arguments.get("timeout")
                .and_then(|v| v.as_u64())
                .unwrap_or(60);
            
            let output = Command::new(&binary)
                .args(["execute", "--code", code, "--language", language, 
                       "--security", security, "--timeout", &timeout.to_string()])
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output();
            
            match output {
                Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
                Err(e) => format!("{{\"error\":\"{}\"}}", e),
            }
        }
        
        "list_skills" => {
            let output = Command::new(&binary)
                .arg("skills")
                .stdout(Stdio::piped())
                .output();
            
            match output {
                Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
                Err(e) => format!("{{\"error\":\"{}\"}}", e),
            }
        }
        
        "run_skill" => {
            let skill_dir = arguments.get("skill_dir")
                .and_then(|v| v.as_str())
                .unwrap_or("");
            let input_json = arguments.get("input_json")
                .and_then(|v| v.as_str());
            
            let mut cmd = Command::new(&binary);
            cmd.arg("run").arg(skill_dir);
            if let Some(input) = input_json {
                cmd.arg("--input-json").arg(input);
            }
            
            let output = cmd.stdout(Stdio::piped()).output();
            
            match output {
                Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
                Err(e) => format!("{{\"error\":\"{}\"}}", e),
            }
        }
        
        "audit_dependencies" => {
            let path = arguments.get("path")
                .and_then(|v| v.as_str())
                .unwrap_or(".");
            
            let output = Command::new(&binary)
                .args(["audit", "--path", path])
                .stdout(Stdio::piped())
                .output();
            
            match output {
                Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
                Err(e) => format!("{{\"error\":\"{}\"}}", e),
            }
        }
        
        "get_logs" => {
            let limit = arguments.get("limit")
                .and_then(|v| v.as_u64())
                .unwrap_or(10) as usize;
            let stats = arguments.get("stats")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            
            let output = if stats {
                Command::new(&binary)
                    .args(["logs", "--stats"])
                    .stdout(Stdio::piped())
                    .output()
            } else {
                let limit_str = limit.to_string();
                Command::new(&binary)
                    .args(["logs", "--limit", &limit_str])
                    .stdout(Stdio::piped())
                    .output()
            };
            
            match output {
                Ok(o) => String::from_utf8_lossy(&o.stdout).to_string(),
                Err(e) => format!("{{\"error\":\"{}\"}}", e),
            }
        }
        
        _ => format!("{{\"error\":\"Unknown tool: {}\"}}", name),
    }
}

pub fn run_mcp_server() -> std::io::Result<()> {
    let stdin = std::io::stdin();
    let mut stdout = std::io::stdout();
    
    let mut buffer = String::new();
    
    loop {
        buffer.clear();
        
        // 读取 JSON-RPC 消息长度 (简单实现)
        match stdin.read_line(&mut buffer) {
            Ok(0) => break,  // EOF
            Ok(_) => {
                // 解析请求
                if let Ok(request) = serde_json::from_str::<McpRequest>(&buffer) {
                    let response = handle_mcp_request(&request);
                    
                    // 发送响应
                    if let Ok(response_str) = serde_json::to_string(&response) {
                        println!("{}", response_str);
                        stdout.write_all(response_str.as_bytes())?;
                        stdout.write_all(b"\n")?;
                        stdout.flush()?;
                    }
                }
            }
            Err(e) => {
                eprintln!("读取错误: {}", e);
                break;
            }
        }
    }
    
    Ok(())
}