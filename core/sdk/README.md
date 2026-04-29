# OlaForge SDK

Python 和 JavaScript SDK 用于与 OlaForge 安全沙箱交互。

## 安装

### Python SDK

```bash
cd sdk/python
pip install -e .
```

或直接使用:

```python
from olaforge import OlaForge

client = OlaForge()
result = client.execute("print('hello')")
print(result.output)
```

### JavaScript SDK

```bash
cd sdk/javascript
npm install olaforge
```

或直接引用:

```javascript
const { OlaForge } = require('./src/index.js');

const client = new OlaForge();
const result = client.execute("console.log('hello')", "javascript");
console.log(result.output);
```

## 快速开始

### Python

```python
from olaforge import OlaForge

# 创建客户端
client = OlaForge()

# 执行代码 (安全模式)
result = client.execute(
    code="print(1 + 2)",
    language="python",
    security="L2"  # L0/L1/L2/L3
)

print(f"成功: {result.success}")
print(f"输出: {result.output}")
print(f"风险级别: {result.risk_level}")

# 列出技能
for skill in client.list_skills():
    print(f"- {skill.name}: {skill.description}")

# 获取日志统计
stats = client.get_log_stats()
print(f"总执行: {stats.total_executions}")
print(f"成功: {stats.successful}")
```

### JavaScript

```javascript
const { OlaForge } = require('olaforge');

const client = new OlaForge();

// 执行代码
const result = client.execute(
  "console.log('hello')",
  "javascript",
  { security: "L2" }
);

console.log(`成功: ${result.success}`);
console.log(`输出: ${result.output}`);

// 列出技能
const skills = client.listSkills();
skills.forEach(s => console.log(`- ${s.name}: ${s.description}`));
```

## API 参考

### OlaForge 客户端

| 方法 | 说明 |
|------|------|
| `execute(code, language, options)` | 在沙箱中执行代码 |
| `runSkill(skillDir, options)` | 运行技能 |
| `listSkills()` | 列出可用技能 |
| `audit(path, format)` | 依赖安全审计 |
| `getLogs(limit)` | 获取执行日志 |
| `getLogStats()` | 获取日志统计 |
| `healthCheck()` | 健康检查 |
| `version()` | 获取版本 |
| `chat(prompt, options)` | AI 对话 |

### 安全级别

| 级别 | 说明 |
|------|------|
| L0 | 无沙箱 |
| L1 | 文件系统限制 |
| L2 | 网络警告 + 安全扫描 |
| L3 | 网络阻止 + 严格扫描 |

## 支持的语言

- Python
- JavaScript
- Bash
- Ruby
- Go
- Perl

## 许可

MIT