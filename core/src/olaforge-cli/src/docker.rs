use anyhow::Result;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct DockerConfig {
    pub image: String,
    pub memory_limit: Option<String>,
    pub cpu_limit: Option<f32>,
    pub network: bool,
}

impl Default for DockerConfig {
    fn default() -> Self {
        Self {
            image: "python:3.11-slim".to_string(),
            memory_limit: Some("512m".to_string()),
            cpu_limit: Some(1.0),
            network: false,
        }
    }
}

pub fn check_docker() -> Result<bool> {
    let output = Command::new("docker")
        .arg("--version")
        .output()?;
    
    Ok(output.status.success())
}

pub fn run_in_docker(code: &str, language: &str, config: &DockerConfig) -> Result<String> {
    let image = &config.image;
    
    let (cmd, args) = match language {
        "python" => ("python3", vec!["-c", code]),
        "javascript" => ("node", vec!["-e", code]),
        "bash" => ("bash", vec!["-c", code]),
        _ => ("sh", vec!["-c", code]),
    };

    let mut docker_cmd = Command::new("docker");
    docker_cmd.arg("run")
        .arg("--rm");
    
    if let Some(ref mem) = config.memory_limit {
        docker_cmd.arg("--memory").arg(mem);
    }
    
    if let Some(cpu) = config.cpu_limit {
        docker_cmd.arg("--cpus").arg(cpu.to_string());
    }
    
    if !config.network {
        docker_cmd.arg("--network").arg("none");
    }
    
    docker_cmd.arg(image)
        .arg(cmd)
        .args(args)
        .output()?;

    let output = docker_cmd.output()?;
    let stdout = String::from_utf8_lossy(&output.stdout).to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).to_string();
    let exit_code = output.status.code().unwrap_or(-1);

    let result = serde_json::json!({
        "success": exit_code == 0,
        "output": stdout,
        "error": if stderr.is_empty() { serde_json::Value::Null } else { serde_json::Value::String(stderr) },
        "exit_code": exit_code,
        "runtime": "docker",
        "image": image
    });

    Ok(serde_json::to_string(&result)?)
}

pub fn list_images() -> Result<Vec<String>> {
    let output = Command::new("docker")
        .arg("images")
        .arg("--format")
        .arg("{{.Repository}}:{{.Tag}}")
        .output()?;

    if !output.status.success() {
        return Ok(vec![]);
    }

    let images = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(|s| s.to_string())
        .collect();

    Ok(images)
}