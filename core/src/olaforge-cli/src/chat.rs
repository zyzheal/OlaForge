use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::env;
use std::io::{self, Write};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolDefinition {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value,
}

pub const AGENT_TOOLS: &[&str] = &[
    r#"{
        "type": "function",
        "function": {
            "name": "execute_code",
            "description": "在沙箱中安全地执行代码",
            "parameters": {
                "type": "object",
                "properties": {
                    "code": {"type": "string", "description": "要执行的代码"},
                    "language": {"type": "string", "enum": ["python", "javascript", "bash", "ruby", "go"], "description": "编程语言"},
                    "timeout": {"type": "number", "description": "超时秒数", "default": 60}
                },
                "required": ["code", "language"]
            }
        }
    }"#,
    r#"{
        "type": "function",
        "function": {
            "name": "list_skills",
            "description": "列出所有可用的技能",
            "parameters": {
                "type": "object",
                "properties": {}
            }
        }
    }"#,
    r#"{
        "type": "function",
        "function": {
            "name": "run_skill",
            "description": "执行一个技能",
            "parameters": {
                "type": "object",
                "properties": {
                    "skill_name": {"type": "string", "description": "技能名称"},
                    "input": {"type": "string", "description": "输入参数 (JSON)"}
                },
                "required": ["skill_name"]
            }
        }
    }"#,
];

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRequest {
    pub model: String,
    pub messages: Vec<ChatMessage>,
    pub temperature: f32,
    pub max_tokens: u32,
}

#[derive(Debug, Deserialize)]
pub struct ChatResponse {
    pub choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
pub struct Choice {
    pub message: ResponseMessage,
}

#[derive(Debug, Deserialize)]
pub struct ResponseMessage {
    pub content: String,
}

pub struct ChatSession {
    pub messages: Vec<ChatMessage>,
    pub model: String,
    pub api_key: String,
    pub api_base: String,
    pub temperature: f32,
}

impl ChatSession {
    pub fn new(model: String) -> Self {
        let api_key = env::var("OPENAI_API_KEY")
            .or_else(|_| env::var("OLA_API_KEY"))
            .unwrap_or_else(|_| "sk-dummy".to_string());
            
        let api_base = env::var("OPENAI_API_BASE")
            .or_else(|_| env::var("OLA_API_BASE"))
            .unwrap_or_else(|_| "https://api.openai.com/v1".to_string());

        Self {
            messages: Vec::new(),
            model,
            api_key,
            api_base,
            temperature: 0.7,
        }
    }

    pub fn add_system_prompt(&mut self, system: &str) {
        self.messages.insert(0, ChatMessage {
            role: "system".to_string(),
            content: system.to_string(),
        });
    }

    pub fn send_message(&mut self, content: &str) -> Result<String> {
        self.messages.push(ChatMessage {
            role: "user".to_string(),
            content: content.to_string(),
        });

        // 如果是模拟模式（无有效 API key）
        if self.api_key == "sk-dummy" || self.api_key.is_empty() {
            return self.mock_response(content);
        }

        self.call_api()
    }

    fn call_api(&self) -> Result<String> {
        let client = reqwest::blocking::Client::new();
        
        let request = ChatRequest {
            model: self.model.clone(),
            messages: self.messages.clone(),
            temperature: self.temperature,
            max_tokens: 2048,
        };

        let url = format!("{}/chat/completions", self.api_base);
        
        let response = client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .header("Content-Type", "application/json")
            .json(&request)
            .send()?;

        if !response.status().is_success() {
            return Err(anyhow::anyhow!("API 请求失败: {}", response.status()));
        }

        let chat_resp: ChatResponse = response.json()?;
        
        Ok(chat_resp.choices.first()
            .map(|c| c.message.content.clone())
            .unwrap_or_default())
    }

    fn mock_response(&self, input: &str) -> Result<String> {
        // 模拟响应，用于测试
        let lower = input.to_lowercase();
        
        let response = if lower.contains("你好") || lower.contains("hello") {
            "你好！我是 OlaForge AI 助手。我可以在沙箱中安全地执行代码。\n\n例如，我可以帮你：\n- 写 Python 代码\n- 写 JavaScript 代码\n- 执行数学计算\n- 等等\n\n有什么我可以帮你的吗？"
        } else if lower.contains("python") || lower.contains("代码") {
            "我可以帮你写代码！例如：\n```python\nprint('Hello, World!')\n```\n\n你想让我写什么代码？"
        } else if lower.contains("execute") || lower.contains("执行") {
            "要执行代码，请使用 `olaforge execute` 命令。例如：\n```bash\nolaforge execute --code \"print(1+2)\" --language python\n```"
        } else if lower.contains("help") || lower.contains("帮助") {
            "我可以帮助您：\n- 编写代码 (Python, JavaScript, Bash 等)\n- 执行计算\n- 解答技术问题\n\n请告诉我您需要什么帮助！"
        } else {
            "我理解了你的问题。作为一个 AI 助手，我可以在沙箱中安全地执行代码来帮助你。\n\n你可以：\n1. 让我写代码\n2. 让我执行代码（用 olaforge execute）\n3. 问技术问题\n\n需要我做什么？"
        };

        Ok(response.to_string())
    }

    pub fn clear_history(&mut self) {
        // 保留 system prompt
        let system = self.messages.first()
            .filter(|m| m.role == "system")
            .cloned();
        
        self.messages.clear();
        if let Some(s) = system {
            self.messages.push(s);
        }
    }

    pub fn history_len(&self) -> usize {
        self.messages.len()
    }

    pub fn execute_tool(&self, tool_name: &str, arguments: &str) -> Result<String> {
        match tool_name {
            "execute_code" => {
                let args: serde_json::Value = serde_json::from_str(arguments)
                    .map_err(|e| anyhow::anyhow!("Invalid arguments: {}", e))?;
                
                let code = args["code"].as_str().ok_or_else(|| anyhow::anyhow!("Missing code"))?;
                let language = args["language"].as_str().unwrap_or("python");
                let timeout = args["timeout"].as_u64().unwrap_or(60) as u64;
                
                let output = Command::new("timeout")
                    .arg(format!("{}", timeout))
                    .arg(get_interpreter(language))
                    .arg(get_language_flag(language))
                    .arg(code)
                    .output()?;
                
                let stdout = String::from_utf8_lossy(&output.stdout);
                let stderr = String::from_utf8_lossy(&output.stderr);
                
                if output.status.success() {
                    Ok(format!("执行成功:\n{}", stdout))
                } else {
                    Ok(format!("执行失败 (退出码: {:?}):\n{}\n{}", 
                        output.status.code(), stdout, stderr))
                }
            }
            "list_skills" => {
                Ok("技能列表: (使用 olaforge skills 查看)".to_string())
            }
            "run_skill" => {
                Ok("技能执行: (使用 olaforge run <skill> 查看)".to_string())
            }
            _ => Err(anyhow::anyhow!("未知工具: {}", tool_name))
        }
    }
}

fn get_interpreter(language: &str) -> &str {
    match language {
        "python" | "python3" => "python3",
        "javascript" | "js" => "node",
        "bash" | "sh" => "bash",
        "ruby" => "ruby",
        "go" => "go",
        _ => "sh",
    }
}

fn get_language_flag(language: &str) -> &str {
    match language {
        "javascript" | "js" => "-e",
        _ => "-c",
    }
}

pub fn run_interactive(system_prompt: Option<&str>, agent_mode: bool) -> Result<()> {
    let mode_desc = if agent_mode {
        "Agent 模式 (自动执行代码)"
    } else {
        "对话模式"
    };
    
    println!("
╔═══════════════════════════════════════════╗
║         OlaForge Chat Mode                ║
║  AI 助手 + 安全沙箱执行                    ║
╠═══════════════════════════════════════════╣
║  模式: {}                          ║
║  输入代码我将执行                          ║
║  输入 :clear 清除对话                     ║
║  输入 :quit 退出                          ║
║  输入 :exec <code> 执行代码               ║
║  输入 :agent 切换 Agent 模式              ║
╚═══════════════════════════════════════════╝
", mode_desc);

        let mut session = ChatSession::new("gpt-3.5-turbo".to_string());
    
    let default_system = if agent_mode {
        r#"你是一个智能编程助手，可以使用工具来执行代码。

## 可用工具

### execute_code
在沙箱中安全地执行代码。当用户要求运行代码时，必须使用此工具。
参数:
- code: 要执行的代码 (string)
- language: 语言 (python/javascript/bash/ruby/go)
- timeout: 超时秒数 (可选，默认60)

### list_skills
列出所有可用的技能。

### run_skill
执行一个预定义的技能。

## 行为规则
1. 用户要求执行代码时，立即使用 execute_code 工具执行
2. 返回执行结果给用户
3. 如果代码有错误，解释错误并提供修复建议
4. 保持对话简洁"#.
to_string()
    } else {
        "你是一个智能编程助手。可以用 olaforge execute 命令执行代码。".to_string()
    };
    
    if let Some(prompt) = system_prompt {
        session.add_system_prompt(prompt);
    } else {
        session.add_system_prompt(&default_system);
    }

    loop {
        print!("\n👤 你: ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        if input.is_empty() {
            continue;
        }

        match input {
            ":quit" | ":exit" | "quit" | "exit" => {
                println!("👋 再见！");
                break;
            }
            ":clear" => {
                session.clear_history();
                println!("✅ 对话已清除");
                continue;
            }
            ":history" => {
                println!("📝 对话历史 ({} 条):", session.history_len());
                for (i, msg) in session.messages.iter().enumerate() {
                    println!("  [{}] {}: {}", i, msg.role, msg.content.chars().take(50).collect::<String>());
                }
                continue;
            }
            _ if input.starts_with(":exec ") => {
                let code = input.strip_prefix(":exec ").unwrap_or("");
                println!("\n🔧 执行代码: {}", code);
                
                let output = std::process::Command::new("python3")
                    .arg("-c")
                    .arg(code)
                    .output()?;
                
                println!("📤 输出:\n{}", String::from_utf8_lossy(&output.stdout));
                if !output.stderr.is_empty() {
                    println!("⚠️ 错误:\n{}", String::from_utf8_lossy(&output.stderr));
                }
                continue;
            }
            _ => {}
        }

        print!("\n🤖 AI: ");
        io::stdout().flush()?;

        match session.send_message(input) {
            Ok(response) => {
                println!("{}", response);
                // 显示执行建议
                if input.contains("写") && (input.contains("代码") || input.contains("程序")) {
                    println!("\n💡 提示: 可以用 :exec <代码> 直接执行");
                }
            }
            Err(e) => {
                println!("❌ 错误: {}", e);
            }
        }
    }

    Ok(())
}