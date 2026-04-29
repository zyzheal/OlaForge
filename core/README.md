# OlaForge

智能安全沙箱系统 - 安全的代码执行环境

## 特性

- 🔒 **多级沙箱** - L0-L3 安全级别
- 🛡️ **安全扫描** - 代码执行前安全检查
- 🌐 **网络控制** - 阻止危险网络请求
- 🚀 **多语言支持** - Python, JavaScript, Bash, Go, Ruby 等
- 🎨 **Web UI** - 图形化控制台
- ⚡ **API 服务** - RESTful 接口

## 快速开始

### 安装

```bash
# 克隆项目
git clone https://github.com/zyzheal/OlaForge.git
cd OlaForge/core

# 构建
cargo build --release

# 或者使用安装脚本
chmod +x install.sh
./install.sh
```

### 基本使用

```bash
# 执行代码
olaforge execute --code "print('hello world')" --language python

# 指定安全级别
olaforge execute --code "print(1+2)" --security L2

# 启动 Web UI
olaforge webui --port 8080

# 启动 API 服务
olaforge serve --port 7860

# 查看配置
olaforge config

# 列出技能
olaforge skills
```

### 安全级别

| 级别 | 说明 |
|------|------|
| L0 | 无沙箱 |
| L1 | 基础隔离 |
| L2 | 标准 (推荐) |
| L3 | 严格 (阻止网络) |

## API

### 执行代码

```bash
curl -X POST http://localhost:7860/execute \
  -H "Content-Type: application/json" \
  -d '{
    "code": "print(1+2)",
    "language": "python",
    "security": "L2"
  }'
```

### 响应格式

```json
{
  "success": true,
  "output": "3\n",
  "error": null,
  "exit_code": 0,
  "execution_time_ms": 35,
  "sandbox": {
    "enabled": true,
    "level": "L2",
    "scanned": true,
    "passed": true,
    "issues": []
  }
}
```

## 配置文件

默认路径: `~/.olaforge/config.yaml`

```yaml
version: "1.0.0"
sandbox:
  level: L2
  enabled: true
  allow_network: false
  timeout_seconds: 60
execution:
  default_language: python
api:
  host: "127.0.0.1"
  port: 7860
```

## 架构

基于 SkillLite 和 CubeSandbox 代码复用:

- `olaforge-core` - 核心配置
- `olaforge-sandbox` - 沙箱隔离 (SkillLite)
- `olaforge-executor` - 执行器 (SkillLite)
- `olaforge-evolution` - 技能进化 (SkillLite)
- `olaforge-cli` - CLI 入口

## 许可证

MIT License