# 共享键鼠软件

该项目是一个使用 Next.js 和 Tauri 构建的共享键鼠软件。它允许用户在多台设备之间共享键盘和鼠标，提高工作效率。

## 技术栈

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

## 许可证

该项目使用 MIT 许可证。