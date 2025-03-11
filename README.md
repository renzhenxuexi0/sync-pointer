# Tauri + React + Typescript

This template should help get you started developing with Tauri, React and Typescript in Vite.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)
- **Next.js**: 一个用于构建现代 Web 应用的 React 框架。
- **Tauri**: 一个用于构建小巧、快速、安全的桌面应用的框架。

## 功能

- 在多台设备之间无缝切换键盘和鼠标。
- 支持多种操作系统，包括 Windows、macOS 和 Linux。
- 简单易用的用户界面。

## 日志路径

日志文件位置因操作系统而异：

- Linux: `${configDir}/${bundleIdentifier}/logs`
- macOS: `${homeDir}/Library/Logs/{bundleIdentifier}`
- Windows: `${configDir}/${bundleIdentifier}/logs`

## 安装与使用

1. 克隆仓库：

   ```bash
   git clone https://github.com/your-repo/sync-pointer.git
   cd sync-pointer
   ```

2. 安装依赖：

   ```bash
   bun i
   ```

3. 运行开发服务器：

   ```bash
   bun run dev
   ```

4. 构建桌面应用：
   ```bash
   bun run tauri build
   ```

## 贡献

欢迎贡献代码！请提交 Pull Request 或报告问题。

## 使用协议

本仓库遵循 AGPL-3.0 开源协议。

允许个人使用，如果需要商业使用，请联系作者。除非获得商业授权，否则无论以何种方式修改或者使用代码，都需要开源，并保留相关版权信息。详细内容请参见 AGPL-3.0 开源协议。
