#!/bin/bash

# 脚本功能：清理旧的 Rust-Python 模块并重新构建（maturin develop）
# 使用方法：./reinstall_dev.sh

set -e  # 遇到错误立即退出

# 1. 此处输入模块名称
MODULE_NAME="sm83_kernel"

# 2. 清理旧模块（强制卸载）
echo "正在卸载旧模块: $MODULE_NAME..."
pip uninstall -y "$MODULE_NAME" || echo "未安装旧模块（无需卸载）"

# 3. 清理可能的残留文件（如 .pyc 缓存）
echo "清理 Python 缓存..."
find . -name "*.pyc" -delete
find . -name "__pycache__" -exec rm -rf {} +

# 4. 重新构建并安装开发版本
echo "运行 maturin develop..."
maturin develop --release

# 5. 验证安装
echo "验证模块是否可导入..."
python -c "import $MODULE_NAME; print('操作成功完成！')"
