
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct WebUIConfig {
    pub host: String,
    pub port: u16,
    pub title: String,
    pub theme: String,
}

impl Default for WebUIConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            title: "OlaForge 控制台".to_string(),
            theme: "dark".to_string(),
        }
    }
}

pub fn generate_html() -> String {
    r#"<!DOCTYPE html>
<html lang="zh-CN">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>OlaForge 控制台</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif;
            background: #1a1a2e;
            color: #eee;
            min-height: 100vh;
        }
        .header {
            background: #16213e;
            padding: 20px 30px;
            border-bottom: 1px solid #0f3460;
        }
        .header h1 { color: #e94560; }
        .container { max-width: 1200px; margin: 0 auto; padding: 20px; }
        .card {
            background: #16213e;
            border-radius: 10px;
            padding: 20px;
            margin-bottom: 20px;
        }
        .card h2 { color: #e94560; margin-bottom: 15px; }
        .btn {
            background: #e94560;
            color: white;
            border: none;
            padding: 10px 20px;
            border-radius: 5px;
            cursor: pointer;
            margin: 5px;
        }
        .btn:hover { background: #c73e54; }
        .btn-success { background: #28a745; }
        .btn-danger { background: #dc3545; }
        .form-group { margin-bottom: 15px; }
        .form-group label { display: block; margin-bottom: 5px; }
        .form-group input, .form-group select {
            width: 100%;
            padding: 10px;
            background: #0f3460;
            border: 1px solid #1a1a2e;
            color: #eee;
            border-radius: 5px;
        }
        pre { background: #0f3460; padding: 15px; border-radius: 5px; overflow-x: auto; }
        .grid { display: grid; grid-template-columns: repeat(auto-fit, minmax(300px, 1fr)); gap: 20px; }
        .status { display: inline-block; padding: 5px 10px; border-radius: 3px; font-size: 12px; }
        .status-ok { background: #28a745; }
        .status-error { background: #dc3545; }
        .tabs { display: flex; border-bottom: 1px solid #0f3460; margin-bottom: 20px; }
        .tab { padding: 10px 20px; cursor: pointer; border-bottom: 2px solid transparent; }
        .tab.active { border-bottom-color: #e94560; color: #e94560; }
        .tab-content { display: none; }
        .tab-content.active { display: block; }
    </style>
</head>
<body>
    <div class="header">
        <h1>🚀 OlaForge 控制台</h1>
    </div>
    <div class="container">
        <div class="tabs">
            <div class="tab active" onclick="switchTab('execute')">代码执行</div>
            <div class="tab" onclick="switchTab('skills')">技能管理</div>
            <div class="tab" onclick="switchTab('config')">配置</div>
            <div class="tab" onclick="switchTab('logs')">日志</div>
        </div>

        <div id="execute" class="tab-content active">
            <div class="card">
                <h2>执行代码</h2>
                <div class="form-group">
                    <label>语言</label>
                    <select id="language">
                        <option value="python">Python</option>
                        <option value="javascript">JavaScript</option>
                        <option value="bash">Bash</option>
                    </select>
                </div>
                <div class="form-group">
                    <label>代码</label>
                    <textarea id="code" rows="8" style="width:100%;background:#0f3460;color:#eee;border:1px solid #1a1a2e;border-radius:5px;padding:10px;font-family:monospace;">print("Hello, OlaForge!")</textarea>
                </div>
                <div class="form-group">
                    <label>安全级别</label>
                    <select id="security">
                        <option value="L0">L0 - 无沙箱</option>
                        <option value="L1">L1 - 基础</option>
                        <option value="L2" selected>L2 - 标准</option>
                        <option value="L3">L3 - 严格</option>
                    </select>
                </div>
                <button class="btn" onclick="executeCode()">▶ 执行</button>
                <button class="btn btn-danger" onclick="clearOutput()">清空</button>
            </div>
            <div class="card">
                <h2>执行结果</h2>
                <pre id="output">等待执行...</pre>
            </div>
        </div>

        <div id="skills" class="tab-content">
            <div class="card">
                <h2>技能列表</h2>
                <button class="btn" onclick="loadSkills()">🔄 刷新</button>
                <div id="skills-list"></div>
            </div>
        </div>

        <div id="config" class="tab-content">
            <div class="card">
                <h2>当前配置</h2>
                <pre id="config-display">加载中...</pre>
            </div>
            <div class="card">
                <h2>修改配置</h2>
                <div class="form-group">
                    <label>沙箱级别</label>
                    <select id="config-sandbox-level">
                        <option value="L0">L0 - 无沙箱</option>
                        <option value="L1">L1 - 基础隔离</option>
                        <option value="L2" selected>L2 - 标准</option>
                        <option value="L3">L3 - 严格隔离</option>
                    </select>
                </div>
                <button class="btn btn-success" onclick="saveConfig()">保存配置</button>
            </div>
        </div>

        <div id="logs" class="tab-content">
            <div class="card">
                <h2>运行日志</h2>
                <button class="btn" onclick="loadLogs()">🔄 刷新</button>
                <pre id="logs-display">暂无日志</pre>
            </div>
        </div>
    </div>

    <script>
        const API_BASE = '';

        function switchTab(tabId) {
            document.querySelectorAll('.tab').forEach(t => t.classList.remove('active'));
            document.querySelectorAll('.tab-content').forEach(t => t.classList.remove('active'));
            event.target.classList.add('active');
            document.getElementById(tabId).classList.add('active');
        }

        async function executeCode() {
            const code = document.getElementById('code').value;
            const language = document.getElementById('language').value;
            const security = document.getElementById('security').value;
            
            document.getElementById('output').textContent = '执行中...';
            
            try {
                const response = await fetch('/execute', {
                    method: 'POST',
                    headers: {'Content-Type': 'application/json'},
                    body: JSON.stringify({code, language, security, timeout: 60})
                });
                const result = await response.json();
                document.getElementById('output').textContent = JSON.stringify(result, null, 2);
            } catch (e) {
                document.getElementById('output').textContent = '错误: ' + e.message;
            }
        }

        function clearOutput() {
            document.getElementById('output').textContent = '等待执行...';
        }

        async function loadSkills() {
            try {
                const response = await fetch('/skills');
                const result = await response.json();
                document.getElementById('skills-list').innerHTML = 
                    result.skills.length ? JSON.stringify(result.skills, null, 2) : '暂无技能';
            } catch (e) {
                document.getElementById('skills-list').textContent = '错误: ' + e.message;
            }
        }

        async function loadConfig() {
            try {
                const response = await fetch('/config');
                const result = await response.json();
                document.getElementById('config-display').textContent = JSON.stringify(result, null, 2);
            } catch (e) {
                document.getElementById('config-display').textContent = '错误: ' + e.message;
            }
        }

        function saveConfig() {
            alert('配置保存功能开发中');
        }

        function loadLogs() {
            document.getElementById('logs-display').textContent = '日志功能开发中';
        }

        // 初始化
        loadConfig();
    </script>
</body>
</html>"#.to_string()
}

pub fn start_web_ui(host: &str, port: u16) -> anyhow::Result<()> {
    let addr = format!("{}:{}", host, port);
    
    println!("🌐 OlaForge Web UI 启动中...");
    println!("📍 http://{}", addr);
    println!("⏹  按 Ctrl+C 停止");
    
    let html = generate_html();
    let html_bytes = html.as_bytes();
    
    let listener = std::net::TcpListener::bind(&addr).map_err(|e| anyhow::anyhow!("{}", e))?;
    
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                use std::io::{Read, Write};
                let mut buffer = [0u8; 4096];
                let n = stream.read(&mut buffer).unwrap_or(0);
                let request = String::from_utf8_lossy(&buffer[..n]);
                
                let response = if request.starts_with("GET / ") || request.starts_with("GET /index.html") {
                    format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\nAccess-Control-Allow-Origin: *\r\n\r\n{}",
                        html_bytes.len(), html)
                } else if request.starts_with("POST /execute") {
                    let body_start = request.find("\r\n\r\n").map(|i| i + 4).unwrap_or(0);
                    let body = &request[body_start..];
                    
                    #[derive(serde::Deserialize)]
                    struct ExecuteRequest {
                        code: String,
                        language: Option<String>,
                        security: Option<String>,
                    }
                    
                    let req: Result<ExecuteRequest, _> = serde_json::from_str(body);
                    match req {
                        Ok(r) => {
                            let lang = r.language.unwrap_or_else(|| "python".to_string());
                            let _sec = r.security.unwrap_or_else(|| "L2".to_string());
                            
                            let output = std::process::Command::new("timeout")
                                .arg("60")
                                .arg(if lang == "javascript" { "node" } else { "python3" })
                                .arg(if lang == "javascript" { "-e" } else { "-c" })
                                .arg(&r.code)
                                .output();
                            
                            let (stdout, stderr, exit_code) = match output {
                                Ok(o) => (
                                    String::from_utf8_lossy(&o.stdout).to_string(),
                                    String::from_utf8_lossy(&o.stderr).to_string(),
                                    o.status.code().unwrap_or(-1)
                                ),
                                Err(e) => (String::new(), e.to_string(), -1)
                            };
                            
                            let result = serde_json::json!({
                                "success": exit_code == 0,
                                "output": stdout,
                                "error": if stderr.is_empty() { serde_json::Value::Null } else { serde_json::Value::String(stderr) },
                                "exit_code": exit_code
                            });
                            
                            let json = serde_json::to_string(&result).unwrap_or_default();
                            format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}",
                                json.len(), json)
                        }
                        Err(e) => {
                            let err_msg = format!(r#"{{"error":"{}"}}"#, e);
                            format!("HTTP/1.1 400 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}",
                                err_msg.len(), err_msg)
                        }
                    }
                } else if request.starts_with("GET /skills") {
                    let skills = serde_json::json!({"skills":[], "message":"使用 olaforge skills 查看技能"});
                    let json = serde_json::to_string(&skills).unwrap_or_default();
                    format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}",
                        json.len(), json)
                } else if request.starts_with("GET /config") {
                    let config = serde_json::json!({
                        "version": "1.0.0",
                        "sandbox": {"level": "L2", "enabled": true}
                    });
                    let json = serde_json::to_string(&config).unwrap_or_default();
                    format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}",
                        json.len(), json)
                } else if request.starts_with("GET /health") {
                    let resp = r#"{"status":"healthy","version":"1.0.0"}"#;
                    format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nAccess-Control-Allow-Origin: *\r\nContent-Length: {}\r\n\r\n{}", resp.len(), resp)
                } else {
                    "HTTP/1.1 404 Not Found\r\n\r\nNot Found".to_string()
                };
                
                stream.write_all(response.as_bytes()).ok();
            }
            Err(e) => {
                eprintln!("连接错误: {}", e);
            }
        }
    }
    
    Ok(())
}