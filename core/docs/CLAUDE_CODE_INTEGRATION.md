# OlaForge 与 Claude Code 集成指南

**目标**: 让 Claude Code 可以安全地使用 OlaForge 执行代码

---

## 集成模式

### 模式1: 通过 Bash 命令调用 (最简单)

Claude Code 可以直接调用 OlaForge 作为安全的代码执行后端：

```bash
# 在 Claude Code 对话中，可以直接使用：
olaforge execute --code "排序算法代码" --language python
```

**优势**: 无需配置，直接使用

---

### 模式2: 配置自定义工具 (推荐)

通过 Claude Code 的工具配置，让 AI 自动调用 OlaForge：

#### 步骤1: 创建 OlaForge 工具脚本

```bash
#!/bin/bash
# 保存为 ~/olaforge-tool

CODE="$1"
LANGUAGE="${2:-python}"
SECURITY="${3:-L2}"

~/OlaForge/core/target/release/olaforge execute \
  --code "$CODE" \
  --language "$LANGUAGE" \
  --security "$SECURITY" \
  --timeout 60
```

```bash
chmod +x ~/olaforge-tool
```

#### 步骤2: 配置 Claude Code

在项目根目录创建 `CLAUDE.md`：

```markdown
# 项目工具

## 代码执行

你可以使用 `olaforge` 命令安全地执行代码：

- 语言支持: python, javascript, bash, ruby, go
- 安全级别: L0 (无限制), L1 (基础), L2 (标准), L3 (严格)

示例:
- Python: `olaforge execute --code "print('hello')" --language python`
- JavaScript: `olaforge execute --code "console.log(1+2)" --language javascript`
- Bash: `olaforge execute --code "echo hello" --language bash`

安全模式:
- L2 (默认): 安全扫描 + 警告
- L3: 阻止网络请求
```

---

### 模式3: MCP 协议集成 (高级)

#### 步骤1: 启动 OlaForge API 服务

```bash
olaforge serve --port 7860
```

#### 步骤2: MCP 工具定义

创建 `mcp-tools.md` 让 Claude Code 知道可用的工具：

```markdown
# MCP 工具定义

## 可用工具

### execute_code
在沙箱中安全执行代码

参数:
- code: string (必填) - 要执行的代码
- language: string (可选, 默认: python) - 语言
- security: string (可选, 默认: L2) - 安全级别

返回:
- success: boolean
- output: string (执行结果)
- error: string? (错误信息)
- execution_time_ms: number

示例请求:
curl -X POST http://localhost:7860/execute \
  -H "Content-Type: application/json" \
  -d '{"code":"print(1+2)","language":"python"}'
```

---

## 实际使用示例

### 场景1: 让 Claude Code 执行代码

```
用户: 写一个快速排序算法并测试

Claude Code:
我来写一个快速排序算法并使用 OlaForge 在沙箱中执行：

[执行命令]
olaforge execute --code "
def quicksort(arr):
    if len(arr) <= 1:
        return arr
    pivot = arr[len(arr) // 2]
    left = [x for x in arr if x < pivot]
    middle = [x for x in arr if x == pivot]
    right = [x for x in arr if x > pivot]
    return quicksort(left) + middle + quicksort(right)

print(quicksort([3,6,8,10,1,2,1]))
" --language python

[结果]
[1, 1, 2, 3, 6, 8, 10]
```

### 场景2: 限制资源使用

```
用户: 计算 Fibonacci(1000)

Claude Code:
这个计算很耗时，让我用 L2 安全模式执行：

olaforge execute --code "
def fib(n):
    if n <= 1:
        return n
    return fib(n-1) + fib(n-2)

print(fib(30))
" --language python --timeout 10

[结果]
832040
```

---

## 安全配置

### 为 Claude Code 配置严格模式

在项目 CLAUDE.md 中添加:

```markdown
## 安全策略

本项目使用 OlaForge L3 模式执行所有代码：
- 禁止网络请求
- 禁止访问系统文件
- 禁止危险系统调用

执行任何代码前必须使用 L3 或更高安全级别。
```

---

## 常见问题

### Q: Claude Code 如何安装 OlaForge?

A: 
```bash
# 方式1: 直接使用二进制
cp ~/OlaForge/core/target/release/olaforge /usr/local/bin/

# 方式2: 添加到 PATH
echo 'export PATH="$PATH:$HOME/OlaForge/core/target/release"' >> ~/.bashrc
```

### Q: 执行超时怎么办?

A: 使用 `--timeout` 参数:
```bash
olaforge execute --code "while True: pass" --timeout 5
# 5秒后自动终止
```

### Q: 如何查看执行日志?

A:
```bash
# 启用详细输出
olaforge --verbose execute --code "print(1)"
```

---

## 集成架构图

```
┌─────────────────────────────────────────────────┐
│            Claude Code (VSCode)                 │
│                                                 │
│  用户请求: "写个排序并执行"                       │
│         │                                       │
│         ▼                                       │
│  ┌─────────────────┐                           │
│  │ 生成代码 + 调用  │                          │
│  │   OlaForge      │                          │
│  └────────┬────────┘                           │
└───────────┼────────────────────────────────────┘
            │ bash/curl
            ▼
┌─────────────────────────────────────────────────┐
│         OlaForge 沙箱                           │
│                                                 │
│  1. 安全扫描 (检测危险代码)                      │
│  2. 网络检查 (L2/L3)                           │
│  3. 资源限制 (超时/内存)                        │
│  4. 执行代码                                    │
│  5. 返回结果                                    │
└─────────────────────────────────────────────────┘
```

---

## 下一步

1. **测试集成**: 在 Claude Code 中尝试使用 `olaforge` 命令
2. **配置工具**: 创建项目专属的 CLAUDE.md
3. **安全策略**: 根据需求设置 L0-L3 安全级别

---

**集成完成!** 🎉