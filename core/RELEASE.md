# OlaForge v1.0.0 发布说明

**发布日期**: 2026-04-29

---

## 版本信息

- **版本**: v1.0.0
- **二进制大小**: 2.9MB
- **代码行数**: ~1854 (CLI) + ~16000 (复用)
- **测试**: 93+ 通过

---

## 核心功能

| 功能 | 状态 |
|------|------|
| 代码执行 (Python/JS/Bash/Go/Ruby) | ✅ |
| 安全沙箱 (L0-L3) | ✅ |
| 安全扫描 | ✅ |
| 网络控制 | ✅ |
| 技能系统 | ✅ |
| Web UI | ✅ |
| API 服务 | ✅ |
| Docker 支持 | ✅ |
| AI 对话 | ✅ |

---

## CLI 命令

```bash
olaforge chat           # AI 聊天模式
olaforge execute       # 代码执行
olaforge run           # 技能运行
olaforge skills        # 技能列表
olaforge docker        # Docker执行
olaforge images        # 镜像列表
olaforge webui         # Web UI
olaforge serve         # API 服务
olaforge init          # 配置初始化
olaforge config        # 配置查看
olaforge health        # 健康检查
olaforge version       # 版本信息
```

---

## 安全分级

| 级别 | 功能 |
|------|------|
| L0 | 无沙箱 |
| L1 | 文件系统限制 |
| L2 | 网络警告 + 安全扫描 |
| L3 | 网络阻止 + 严格扫描 |

---

## 安装

```bash
# 直接使用
./target/release/olaforge --help

# 或添加 PATH
export PATH=$PATH:~/OlaForge/core/target/release
```

---

## 特性亮点

- 🚀 超轻量: 2.9MB 二进制
- ⚡ 快速启动: <50ms
- 🔒 多级安全: L0-L3 分级
- 📦 开箱即用: 零配置
- 🔄 代码复用: 基于 SkillLite 85%+

---

## 技术栈

- **语言**: Rust
- **复用**: SkillLite (沙箱/执行器/安全扫描)
- **目标**: 本地优先 + 云端扩展

---

## 项目位置

```
~/OlaForge/
├── core/               # 主项目
│   ├── target/release/olaforge  # 二进制
│   └── docs/           # 文档
└── ARCHITECTURE.md     # 架构文档
```

---

**OlaForge v1.0.0 发布完成!** 🎉