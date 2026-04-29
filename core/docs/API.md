# OlaForge API 文档

## 概述

OlaForge 提供两种 API 模式：
1. **CLI 模式** - 命令行交互
2. **HTTP 模式** - RESTful API 服务

## CLI 命令

### execute - 执行代码

```bash
olaforge execute --code <CODE> --language <LANG> [OPTIONS]

选项:
  --code <CODE>         要执行的代码 (必填)
  --language <LANG>    语言: python, javascript, bash, ruby, go (默认: python)
  --security <LEVEL>   安全级别: L0, L1, L2, L3 (默认: L2)
  --timeout <SEC>      超时秒数 (默认: 60)
  --no-sandbox         禁用沙箱

示例:
  olaforge execute --code "print('hello')" --language python
  olaforge execute --code "console.log(1+2)" --language javascript --security L3
```

### serve - 启动 API 服务

```bash
olaforge serve [OPTIONS]

选项:
  --port <PORT>  端口 (默认: 7860)
  --host <HOST>  绑定地址 (默认: 127.0.0.1)

示例:
  olaforge serve --port 8080
```

### webui - 启动 Web UI

```bash
olaforge webui [OPTIONS]

选项:
  --port <PORT>  端口 (默认: 8080)
  --host <HOST>  绑定地址 (默认: 127.0.0.1)

示例:
  olaforge webui --port 8080
```

### run - 运行技能

```bash
olaforge run [<SKILL_DIR>] [OPTIONS]

选项:
  <SKILL_DIR>           技能目录路径
  --input-json <JSON>   输入参数 JSON
  --goal <GOAL>         目标描述
  --allow-network       允许网络访问
  --sandbox-level <N>   沙箱级别 1-3

示例:
  olaforge run ./my-skill --input-json '{"task": "test"}'
```

### skills - 列出技能

```bash
olaforge skills
```

### config - 配置管理

```bash
olaforge config [--full]
olaforge init [--path <PATH>]
```

### 其他命令

```bash
olaforge health    # 健康检查
olaforge version   # 版本信息
```

## HTTP API

### 基础信息

```
Base URL: http://localhost:7860
Content-Type: application/json
```

### 端点列表

| 方法 | 路径 | 说明 |
|------|------|------|
| GET | `/` | API 信息 |
| GET | `/health` | 健康检查 |
| GET | `/version` | 版本信息 |
| GET | `/config` | 获取配置 |
| POST | `/execute` | 执行代码 |
| GET | `/skills` | 技能列表 |

### 执行代码

```http
POST /execute
Content-Type: application/json

{
  "code": "print('hello')",
  "language": "python",
  "security": "L2",
  "timeout": 60,
  "no_sandbox": false
}
```

**响应:**

```json
{
  "success": true,
  "output": "hello\n",
  "error": null,
  "exit_code": 0,
  "execution_time_ms": 35,
  "sandbox": {
    "enabled": true,
    "level": "L2",
    "scanned": true,
    "passed": true,
    "issues": []
  },
  "language": "python"
}
```

### 错误响应

```json
{
  "success": false,
  "output": "",
  "error": "安全扫描未通过",
  "exit_code": -1,
  "execution_time_ms": 20,
  "security_issues": [
    "[CRITICAL] 危险函数: eval()",
    "[NETWORK] requests 库需要网络权限"
  ],
  "sandbox": {
    "enabled": true,
    "level": "L3",
    "passed": false
  }
}
```

## 安全级别

### L0 - 无沙箱
- 不执行任何安全检查
- 直接运行代码
- 仅用于测试

### L1 - 基础隔离
- 基础文件系统隔离
- 最小权限原则

### L2 - 标准 (默认)
- 安全扫描
- 网络使用警告
- 推荐日常使用

### L3 - 严格
- 阻止所有网络请求
- 严格安全扫描
- 高安全要求场景

## 错误码

| 退出码 | 含义 |
|--------|------|
| 0 | 成功 |
| 1 | 一般错误 |
| 124 | 超时 |
| 127 | 命令未找到 |

## 性能

- 启动时间: < 50ms
- 代码执行开销: ~10ms
- 内存占用: ~2MB