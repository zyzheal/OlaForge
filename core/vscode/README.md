# OlaForge VSCode Extension

在 VSCode 中安全地运行代码。

## 功能

- **安全沙箱执行**: 代码在隔离环境中运行
- **多语言支持**: Python, JavaScript, TypeScript, Bash
- **安全级别选择**: L0-L3 四级安全
- **快捷键**: `Ctrl+Shift+R` (Mac: `Cmd+Shift+R`)

## 安装

```bash
cd vscode
npm install
npm run vscode:prepublish
```

然后在 VSCode 中按 `F5` 启动调试。

## 配置

| 设置 | 说明 | 默认值 |
|------|------|--------|
| `olaforge.binaryPath` | OlaForge 二进制路径 | `olaforge` |
| `olaforge.securityLevel` | 安全级别 (L0/L1/L2/L3) | `L2` |
| `olaforge.timeout` | 超时时间 (秒) | `60` |

## 使用方法

1. 打开代码文件
2. 按 `Ctrl+Shift+R` 或右键选择 "Run Code in OlaForge Sandbox"
3. 查看输出面板

## 安全级别

| 级别 | 说明 |
|------|------|
| L0 | 无沙箱 |
| L1 | 文件系统限制 |
| L2 | 网络警告 + 安全扫描 |
| L3 | 网络阻止 + 严格扫描 |

## 许可

MIT