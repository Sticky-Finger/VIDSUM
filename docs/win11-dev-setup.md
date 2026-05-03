# Windows 11 开发环境搭建指南

## 前置条件
- 已安装 Node.js v18+、pnpm v8.0+、Rust
- 已安装 scoop（Windows 包管理器）或使用 Windows 自带的 winget

## 常见问题与解决方案

### 1. 缺少 libclang

**错误信息**：`Unable to find libclang`

**解决办法**：
```bash
scoop install llvm
```

### 2. 缺少 cmake

**错误信息**：编译 whisper.cpp 时提示找不到 cmake

**解决办法**：
```bash
cargo clean
scoop install cmake
```

### 3. 缺少 whisper 模型文件

**错误信息**：models/ 文件夹下缺少模型文件

**解决办法**：
1. 访问 https://huggingface.co/ggerganov/whisper.cpp/tree/main
2. 下载所需模型（推荐 `ggml-tiny.bin`）
3. 将模型文件放到 `src-tauri/models/` 目录下

## 验证

完成上述步骤后，运行：
```bash
cd src-tauri
cargo check
```
应能成功通过编译检查。
