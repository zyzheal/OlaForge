use std::fs;
use std::path::PathBuf;
use serde::Deserialize;

pub const DEFAULT_TEMPLATES: &[(&str, &str, &str)] = &[
    ("python-script", "Python 脚本", r#"#!/usr/bin/env python3
# Skill: {name}
# Description: {description}

def main(input: dict) -> dict:
    """主入口函数"""
    # 处理输入
    result = input.get("data", "Hello from Python skill!")
    return {"result": result, "status": "success"}

if __name__ == "__main__":
    import json
    input_data = json.loads(input())
    output = main(input_data)
    print(json.dumps(output))
"#),
    ("javascript-script", "JavaScript 脚本", r#"#!/usr/bin/env node
// Skill: {name}
// Description: {description}

function main(input) {
    // 处理输入
    const result = input.data || "Hello from JS skill!";
    return { result, status: "success" };
}

if (require.main === module) {
    const input = JSON.parse(require('fs').readFileSync(0, 'utf-8'));
    console.log(JSON.stringify(main(input)));
}
"#),
    ("bash-script", "Bash 脚本", r#"#!/bin/bash
# Skill: {name}
# Description: {description}

main() {
    local input="$1"
    echo "Received: $input"
    echo '{"result": "success", "status": "completed"}'
}

main "$@"
"#),
    ("api-tool", "API 工具", r#"#!/usr/bin/env python3
# Skill: {name}
# Description: {description}

import requests
import json

def main(input: dict) -> dict:
    """调用外部 API"""
    url = input.get("url", "https://api.example.com/data")
    method = input.get("method", "GET")
    
    try:
        if method == "GET":
            response = requests.get(url, timeout=10)
        else:
            response = requests.post(url, json=input.get("body", {}), timeout=10)
        
        return {
            "status": "success",
            "data": response.json() if response.ok else response.text,
            "code": response.status_code
        }
    except Exception as e:
        return {"status": "error", "message": str(e)}

if __name__ == "__main__":
    import sys
    input_data = json.loads(sys.stdin.read())
    print(json.dumps(main(input_data)))
"#),
    ("data-processor", "数据处理", r#"#!/usr/bin/env python3
# Skill: {name}
# Description: {description}

import json

def main(input: dict) -> dict:
    """数据处理技能"""
    data = input.get("data", [])
    operation = input.get("operation", "count")
    
    if not isinstance(data, list):
        return {"status": "error", "message": "data must be a list"}
    
    result = None
    if operation == "count":
        result = len(data)
    elif operation == "sum" and all(isinstance(x, (int, float)) for x in data):
        result = sum(data)
    elif operation == "avg" and all(isinstance(x, (int, float)) for x in data):
        result = sum(data) / len(data) if data else 0
    elif operation == "max":
        result = max(data) if data else None
    elif operation == "min":
        result = min(data) if data else None
    else:
        return {"status": "error", "message": f"Unknown operation: {operation}"}
    
    return {"result": result, "operation": operation, "status": "success"}

if __name__ == "__main__":
    input_data = json.loads(input())
    print(json.dumps(main(input_data)))
"#),
];

pub fn list_templates() -> Vec<SkillTemplate> {
    DEFAULT_TEMPLATES.iter()
        .map(|(id, name, desc)| SkillTemplate {
            id: id.to_string(),
            name: name.to_string(),
            description: desc.to_string(),
        })
        .collect()
}

pub fn create_from_template(template_id: &str, output_dir: &str, name: &str, description: &str) -> Result<String, String> {
    let template = DEFAULT_TEMPLATES.iter()
        .find(|(id, _, _)| *id == template_id)
        .ok_or_else(|| format!("模板不存在: {}", template_id))?;
    
    let output_path = PathBuf::from(output_dir);
    if !output_path.exists() {
        fs::create_dir_all(&output_path).map_err(|e| e.to_string())?;
    }
    
    // 替换占位符
    let mut content = template.2.to_string();
    content = content.replace("{name}", name);
    content = content.replace("{description}", description);
    
    // 根据模板类型确定文件扩展名
    let ext = match template_id {
        id if id.contains("python") => "py",
        id if id.contains("javascript") => "js",
        id if id.contains("bash") => "sh",
        _ => "py",
    };
    
    let file_name = format!("{}.{}", name.replace(" ", "-").to_lowercase(), ext);
    let file_path = output_path.join(&file_name);
    
    fs::write(&file_path, content).map_err(|e| e.to_string())?;
    
    // 创建 SKILL.md
    let skill_md = format!(r#"# {}

{}

## 输入

```json
{{
  "data": "输入数据"
}}
```

## 输出

```json
{{
  "result": "处理结果",
  "status": "success"
}}
```

## 使用

```bash
olaforge run {} --input-json '{{"data": "test"}}'
```
"#,
        name, description, name.replace(" ", "-").to_lowercase()
    );
    
    let md_path = output_path.join("SKILL.md");
    fs::write(&md_path, skill_md).map_err(|e| e.to_string())?;
    
    // 设置执行权限
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(mut perms) = fs::metadata(&file_path).map(|m| m.permissions()) {
            perms.set_mode(0o755);
            fs::set_permissions(&file_path, perms).ok();
        }
    }
    
    Ok(format!("技能已创建: {}", file_path.display()))
}

#[derive(Debug, Clone, serde::Serialize, Deserialize)]
pub struct SkillTemplate {
    pub id: String,
    pub name: String,
    pub description: String,
}