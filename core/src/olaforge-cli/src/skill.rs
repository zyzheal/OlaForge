use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Skill {
    pub name: String,
    pub version: String,
    pub description: String,
    pub entry_point: String,
    pub language: String,
    pub permissions: SkillPermissions,
    pub dependencies: Vec<String>,
    pub environment: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SkillPermissions {
    pub network: bool,
    pub filesystem: bool,
    pub shell: bool,
    pub env: Vec<String>,
}

impl Skill {
    pub fn load(path: &Path) -> Result<Self> {
        let meta_path = path.join("SKILL.md");
        let yaml_path = path.join("skill.yaml");
        let json_path = path.join("skill.json");

        // 优先级: YAML > JSON > markdown
        if yaml_path.exists() {
            let content = fs::read_to_string(&yaml_path)?;
            let skill: Skill = serde_yaml::from_str(&content)?;
            return Ok(skill);
        }

        if json_path.exists() {
            let content = fs::read_to_string(&json_path)?;
            let skill: Skill = serde_json::from_str(&content)?;
            return Ok(skill);
        }

        // 从 SKILL.md 解析
        if meta_path.exists() {
            return Self::from_markdown(path);
        }

        Err(anyhow::anyhow!("未找到技能配置文件"))
    }

    pub fn from_markdown(path: &Path) -> Result<Self> {
        let _content = fs::read_to_string(path.join("SKILL.md"))?;
        
        // 简单解析
        let name = path.file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        Ok(Skill {
            name,
            version: "1.0.0".to_string(),
            description: "从 SKILL.md 解析".to_string(),
            entry_point: "main.py".to_string(),
            language: "python".to_string(),
            permissions: SkillPermissions {
                network: false,
                filesystem: false,
                shell: false,
                env: vec![],
            },
            dependencies: vec![],
            environment: HashMap::new(),
        })
    }

    pub fn execute(&self, skill_dir: &Path, input_json: &str) -> Result<String> {
        let entry_path = skill_dir.join(&self.entry_point);
        
        if !entry_path.exists() {
            return Err(anyhow::anyhow!("入口文件不存在: {}", self.entry_point));
        }

        // 构建命令
        let mut cmd = std::process::Command::new(get_interpreter(&self.language));
        
        // 添加环境变量
        for (key, value) in &self.environment {
            cmd.env(key, value);
        }

        cmd.arg(&entry_path)
           .arg(input_json)
           .stdout(std::process::Stdio::piped())
           .stderr(std::process::Stdio::piped());

        let output = cmd.output()?;
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();
        let exit_code = output.status.code().unwrap_or(-1);

        let result = serde_json::json!({
            "success": exit_code == 0,
            "output": stdout,
            "error": if stderr.is_empty() { serde_json::Value::Null } else { serde_json::Value::String(stderr) },
            "exit_code": exit_code,
            "skill": self.name,
            "version": self.version
        });

        Ok(serde_json::to_string(&result)?)
    }
}

fn get_interpreter(language: &str) -> &str {
    match language.to_lowercase().as_str() {
        "python" | "python3" => "python3",
        "javascript" | "js" => "node",
        _ => "bash",
    }
}

pub fn list_skills(skills_dir: &Path) -> Result<Vec<Skill>> {
    let mut skills = Vec::new();

    if !skills_dir.exists() {
        return Ok(skills);
    }

    for entry in fs::read_dir(skills_dir)? {
        let entry = entry?;
        let path = entry.path();
        
        if path.is_dir() {
            if let Ok(skill) = Skill::load(&path) {
                skills.push(skill);
            }
        }
    }

    Ok(skills)
}