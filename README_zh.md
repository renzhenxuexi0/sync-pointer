# Sync Pointer

> **⚠️ 警告：该项目目前正在开发中，尚未可用。**

<div align="center">
   <img src="https://raw.githubusercontent.com/renzhenxuexi0/sync-pointer/release/public/favicon.ico" alt="Sync Pointer" width="80" />
   
   <p>
      <img src="https://img.shields.io/github/license/renzhenxuexi0/sync-pointer" alt="许可证" />
      <img src="https://img.shields.io/github/last-commit/renzhenxuexi0/sync-pointer" alt="最近提交" />
      <img src="https://img.shields.io/github/issues/renzhenxuexi0/sync-pointer" alt="问题" />
      <img src="https://img.shields.io/github/issues-pr/renzhenxuexi0/sync-pointer" alt="拉取请求" />
      <img src="https://img.shields.io/github/contributors/renzhenxuexi0/sync-pointer" alt="贡献者" />
      <img src="https://img.shields.io/github/stars/renzhenxuexi0/sync-pointer" alt="星标" />
   </p>

   <p>
      <a href="README.md">English</a> | 
      <a href="README_zh.md">中文</a>
   </p>
</div>

## 功能

- 在多台设备之间无缝切换键盘和鼠标
- 支持多种操作系统，包括 Windows、macOS 和 Linux
- 简单易用的用户界面

## 技术栈

### 前端

- **React 19**: 用于构建高效用户界面的 JavaScript 库
- **Vite**: 现代化的前端构建工具，提供极速的开发体验
- **TypeScript**: 增强的 JavaScript 类型系统
- **Ant Design 5.x**: 企业级 UI 设计语言和组件库
- **Tailwind CSS 4.x**: 实用优先的 CSS 框架
- **Valtio**: 轻量级状态管理库
- **i18next**: 强大的国际化框架
- **React Router 7**: 声明式路由解决方案
- **DND Kit**: 现代化拖放工具库

### 后端 (Rust)

- **Tauri 2.x**: 构建小巧、快速、安全的跨平台桌面应用框架
- **Tokio**: 异步运行时和网络框架
- **Serde**: 高效的序列化/反序列化库
- **rkyv**: 零拷贝反序列化框架
- **mdns-sd**: 服务发现库，用于局域网设备发现
- **spdlog-rs**: 高性能日志库
- **rust-i18n**: 国际化支持

### 插件

- **tauri-plugin-single-instance**: 确保应用只有一个实例运行
- **tauri-plugin-valtio**: 状态管理与持久化
- **tauri-plugin-autostart**: 系统启动时自动运行应用
- **tauri-plugin-window-state**: 保存和恢复窗口位置及大小
- **tauri-plugin-fs/os/dialog/opener/process**: 系统交互功能集成

## 推荐开发环境

- [VS Code](https://code.visualstudio.com/) + [Tauri 插件](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## 安装与使用

1. 克隆仓库：

   ```bash
   git clone https://github.com/renzhenxuexi0/sync-pointer.git
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

## 日志路径

| 操作系统 | 路径                                         |
| -------- | -------------------------------------------- |
| Linux    | `${configDir}/${bundleIdentifier}/logs`      |
| macOS    | `${homeDir}/Library/Logs/{bundleIdentifier}` |
| Windows  | `${configDir}/${bundleIdentifier}/logs`      |

## 贡献

欢迎贡献代码！请提交 Pull Request 或报告问题。

## 使用协议

本仓库遵循 AGPL-3.0 开源协议。

允许个人使用，如果需要商业使用，请联系作者。除非获得商业授权，否则无论以何种方式修改或者使用代码，都需要开源，并保留相关版权信息。详细内容请参见 AGPL-3.0 开源协议。
