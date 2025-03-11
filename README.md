# Sync Pointer

> **⚠️ WARNING: This project is currently under development and not yet ready for use.**

<div align="center">
   <img src="https://raw.githubusercontent.com/renzhenxuexi0/sync-pointer/release/public/favicon.ico" alt="Sync Pointer" width="80" />
   
   <p>
      <img src="https://img.shields.io/github/license/renzhenxuexi0/sync-pointer" alt="License" />
      <img src="https://img.shields.io/github/last-commit/renzhenxuexi0/sync-pointer" alt="Last Commit" />
      <img src="https://img.shields.io/github/issues/renzhenxuexi0/sync-pointer" alt="Issues" />
      <img src="https://img.shields.io/github/issues-pr/renzhenxuexi0/sync-pointer" alt="Pull Requests" />
      <img src="https://img.shields.io/github/contributors/renzhenxuexi0/sync-pointer" alt="Contributors" />
      <img src="https://img.shields.io/github/stars/renzhenxuexi0/sync-pointer" alt="Stars" />
   </p>

   <p>
      <a href="README.md">English</a> | 
      <a href="README_zh.md">中文</a>
   </p>
</div>

## Features

- Seamlessly switch keyboard and mouse between multiple devices
- Support for multiple operating systems, including Windows, macOS, and Linux
- Simple and intuitive user interface

## Tech Stack

### Frontend

- **React 19**: JavaScript library for building efficient user interfaces
- **Vite**: Modern frontend build tool that provides an extremely fast development experience
- **TypeScript**: Enhanced JavaScript with type system
- **Ant Design 5.x**: Enterprise-class UI design language and React components
- **Tailwind CSS 4.x**: Utility-first CSS framework
- **Valtio**: Lightweight state management library
- **i18next**: Powerful internationalization framework
- **React Router 7**: Declarative routing solution
- **DND Kit**: Modern drag-and-drop toolkit

### Backend (Rust)

- **Tauri 2.x**: Framework for building small, fast, and secure desktop applications
- **Tokio**: Asynchronous runtime and networking framework
- **Serde**: Efficient serialization/deserialization library
- **rkyv**: Zero-copy deserialization framework
- **mdns-sd**: Service discovery library for local network device discovery
- **spdlog-rs**: High-performance logging library
- **rust-i18n**: Internationalization support

### Plugins

- **tauri-plugin-single-instance**: Ensures only one instance of the app is running
- **tauri-plugin-valtio**: State management and persistence
- **tauri-plugin-autostart**: Auto-launch app on system startup
- **tauri-plugin-window-state**: Save and restore window position and size
- **tauri-plugin-fs/os/dialog/opener/process**: System interaction functionality

## Recommended Development Environment

- [VS Code](https://code.visualstudio.com/) + [Tauri Extension](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)

## Installation and Usage

1. Clone the repository:

   ```bash
   git clone https://github.com/renzhenxuexi0/sync-pointer.git
   cd sync-pointer
   ```

2. Install dependencies:

   ```bash
   bun i
   ```

3. Run development server:

   ```bash
   bun run dev
   ```

4. Build the desktop app:
   ```bash
   bun run tauri build
   ```

## Log Paths

| OS      | Path                                         |
| ------- | -------------------------------------------- |
| Linux   | `${configDir}/${bundleIdentifier}/logs`      |
| macOS   | `${homeDir}/Library/Logs/{bundleIdentifier}` |
| Windows | `${configDir}/${bundleIdentifier}/logs`      |

## Contribution

Contributions are welcome! Please submit a Pull Request or report issues.

## License

This repository is licensed under AGPL-3.0.

Personal use is permitted. For commercial use, please contact the author. Unless commercially licensed, any modifications or usage of the code requires open-sourcing and maintaining copyright notices. See the AGPL-3.0 license for details.
