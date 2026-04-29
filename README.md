# OlaForge

[English](./README.md) | [中文](./README_zh.md)

## 简介

OlaForge 是一个**本地优先**的智能安全沙箱系统，为 AI Agent 提供安全、高效、可进化的代码执行环境。

**核心理念**：让用户在本地**一键安装、即开即用**，同时支持企业级部署。

---

## 特性

### 🏃 本地即开即用

- **一行安装**：`pip install olaforge`
- **一行启动**：`olaforge chat`
- **零依赖**：无需 Docker/KVM
- **极速启动**：< 1秒

### 🛡️ 三层安全防护

- 安装时扫描（静态 + LLM 分析）
- 执行前授权（用户确认）
- 运行时隔离（系统级沙箱）

### 🧠 智能进化

- 自动优化 prompts
- 学习用户习惯
- 记忆成功模式

### ⚡ 高性能

- 冷启动 < 100ms
- 内存开销 < 50MB
- 支持高并发

---

## 快速开始

### 安装 (5秒)

```bash
pip install olaforge
```

### 配置 (10秒)

```bash
# 配置 API Key
olaforge config --api-key sk-your-key

# 或使用本地模型
olaforge config --provider local --model llama3
```

### 使用 (5秒)

```bash
# 交互式对话
olaforge chat

# 或单次执行
olaforge run "帮我写个快速排序"

# 指定模型
olaforge chat --model claude-3-5-sonnet
```

---

## 命令一览

| 命令 | 说明 |
|------|------|
| `olaforge chat` | 启动交互式对话 |
| `olaforge run <prompt>` | 单次执行 |
| `olaforge config` | 配置 API |
| `olaforge init` | 初始化项目 |
| `olaforge status` | 查看状态 |
| `olaforge logs` | 查看日志 |

---

## 本地 vs 云端

| 模式 | 说明 | 适用 |
|------|------|------|
| **本地模式** | 本地执行，无需网络 | 个人开发、快速原型 |
| **云端模式** | 集群部署，企业级 | 大规模、高并发 |

---

## 技术栈

- **核心**: Rust (< 10MB)
- **Agent**: Python + LangChain
- **隔离**: bwrap/seccomp/namespaces
- **可选**: KVM (企业版)

---

## 文档

- [架构设计](./ARCHITECTURE.md) - 完整系统架构
- [开发指南](./DEVELOP.md) - 如何开发
- [贡献指南](./CONTRIBUTING.md) - 如何贡献

---

## License

MIT License

---

*让 AI 代码执行既安全又简单*