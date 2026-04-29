#!/bin/bash

set -e

echo "🚀 OlaForge 安装脚本"
echo "===================="

# 检查 Rust
if ! command -v rustc &> /dev/null; then
    echo "❌ Rust 未安装"
    echo "请先安装 Rust: curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
    exit 1
fi

echo "✅ Rust 已安装: $(rustc --version)"

# 检查依赖
echo "📦 检查依赖..."

# 检查 Python
if command -v python3 &> /dev/null; then
    echo "  ✅ Python3: $(python3 --version)"
else
    echo "  ⚠️  Python3 未安装 (可选)"
fi

# 检查 Node.js
if command -v node &> /dev/null; then
    echo "  ✅ Node.js: $(node --version)"
else
    echo "  ⚠️  Node.js 未安装 (可选)"
fi

# 检查 bubblewrap (Linux)
if command -v bwrap &> /dev/null; then
    echo "  ✅ bubblewrap: 已安装"
elif [ "$(uname)" = "Linux" ]; then
    echo "  ⚠️  bubblewrap 未安装 (Linux 沙箱需要)"
    echo "     Ubuntu/Debian: sudo apt install bubblewrap"
    echo "     Fedora: sudo dnf install bubblewrap"
fi

# 构建项目
echo ""
echo "🔨 构建 OlaForge..."
cd "$(dirname "$0")"

# 使用 release 模式构建
cargo build --release

# 创建 bin 目录
mkdir -p ~/.local/bin

# 复制二进制文件
cp target/release/olaforge ~/.local/bin/

# 添加到 PATH (如果需要)
SHELL_RC="$HOME/.bashrc"
if [ -f "$HOME/.zshrc" ]; then
    SHELL_RC="$HOME/.zshrc"
fi

if ! grep -q "olaforge" "$SHELL_RC" 2>/dev/null; then
    echo "" >> "$SHELL_RC"
    echo "# OlaForge" >> "$SHELL_RC"
    echo 'export PATH="$HOME/.local/bin:$PATH"' >> "$SHELL_RC"
    echo "✅ 已添加 ~/.local/bin 到 PATH"
fi

echo ""
echo "===================="
echo "🎉 安装完成!"
echo ""
echo "使用方式:"
echo "  olaforge --help           # 查看帮助"
echo "  olaforge execute          # 执行代码"
echo "  olaforge webui            # 启动 Web UI"
echo "  olaforge serve            # 启动 API"
echo ""
echo "示例:"
echo "  olaforge execute --code \"print('hello')\" --language python"
echo "  olaforge webui --port 8080"
echo ""
echo "📖 文档: https://github.com/zyzheal/OlaForge"