# OlaForge OpenAPI 3.0 规范

本文件包含 OlaForge REST API 的完整 OpenAPI 3.0 定义。

## 快速使用

```bash
# 生成客户端代码
npm install @openapitools/openapi-generator-cli
openapi-generator generate -i openapi.yaml -g python -o ./sdk/python

# 生成 TypeScript
openapi-generator generate -i openapi.yaml -g typescript-fetch -o ./sdk/typescript
```

## API 端点总览

| 方法 | 路径 | 描述 | 标签 |
|------|------|------|------|
| POST | /chat | 发送对话请求 | chat |
| POST | /chat/stream | 流式对话 | chat |
| POST | /execute | 执行代码 | execute |
| GET | /skills | 获取 Skill 列表 | skill |
| POST | /skills | 添加 Skill | skill |
| GET | /skills/{name} | 获取 Skill 详情 | skill |
| DELETE | /skills/{name} | 删除 Skill | skill |
| POST | /skills/{name}/run | 执行 Skill | skill |
| GET | /status | 获取系统状态 | system |
| GET | /config | 获取配置 | system |
| PUT | /config | 更新配置 | system |
| GET | /metrics | 获取监控指标 | system |
| GET | /health | 健康检查 | system |

## 认证方式

### API Key
```bash
curl -H "X-API-Key: your-api-key" http://localhost:7860/api/v1/status
```

### Bearer Token
```bash
curl -H "Authorization: Bearer your-jwt-token" http://localhost:7860/api/v1/status
```

## 示例

### 执行 Python 代码
```bash
curl -X POST http://localhost:7860/api/v1/execute \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-key" \
  -d '{
    "code": "print(sum([1,2,3,4,5]))",
    "language": "python",
    "security": "L2"
  }'
```

响应:
```json
{
  "id": "exec_abc123",
  "output": "15",
  "error": null,
  "exit_code": 0,
  "execution_time_ms": 156,
  "memory_used_mb": 45
}
```

### 对话
```bash
curl -X POST http://localhost:7860/api/v1/chat \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-key" \
  -d '{
    "message": "写一个快速排序",
    "model": "claude-3-5-sonnet",
    "security": "L2"
  }'
```

响应:
```json
{
  "id": "msg_xyz789",
  "text": "def quick_sort(arr):\n    if len(arr) <= 1:\n        return arr\n    ...",
  "usage": {
    "input_tokens": 120,
    "output_tokens": 380,
    "total_tokens": 500
  }
}
```

### 流式对话
```bash
curl -X POST http://localhost:7860/api/v1/chat/stream \
  -H "Content-Type: application/json" \
  -H "X-API-Key: your-key" \
  -d '{
    "message": "讲个故事",
    "stream": true
  }' \
  -N
```

SSE 响应:
```
data: {"type": "chunk", "text": "很久"}
data: {"type": "chunk", "text": "以前"}
data: {"type": "done"}
```

### 错误响应

限流:
```json
{
  "error": {
    "code": "OLA-000-5",
    "message": "Rate limit exceeded",
    "detail": "请求频率 60/min, 当前 61/min"
  },
  "retry_after": 15
}
```

安全拦截:
```json
{
  "error": {
    "code": "OLA-300-3",
    "message": "危险操作被拦截",
    "detail": "命令 'rm -rf /' 被安全策略拦截"
  }
}
```

---

版本: 1.0.0
最后更新: 2026-04-29