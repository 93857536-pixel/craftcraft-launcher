# CraftCraft Launcher

> 新一代跨平台 Minecraft 启动器，超越 PCL 的体验。

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](LICENSE)
[![macOS](https://img.shields.io/badge/macOS-%23000000.svg?style=flat&logo=apple&logoColor=white)](#)
[![Windows](https://img.shields.io/badge/Windows-0078D6?style=flat&logo=windows&logoColor=white)](#)
[![Linux](https://img.shields.io/badge/Linux-FCC624?style=flat&logo=linux&logoColor=black)](#)

## ✨ 特性

- 🚀 **极速启动** - 秒开游戏，最小化等待
- 📦 **智能 Java 管理** - 一键安装/切换 Java 版本
- 🎨 **现代化 UI** - 暗色主题，视觉舒适
- 📂 **内容库系统** - Mods、资源包、光影统一管理
- 📝 **内嵌日志面板** - 实时查看、关键词高亮、搜索过滤
- 🖥️ **性能仪表板** - 内存、CPU、FPS 实时监控
- 🌐 **服务器工作台** - 收藏、历史、一键加入
- 🔄 **一键导入** - 从 PCL/Prism 迁移数据
- 🎮 **多实例管理** - 拖拽排序，快速克隆
- 🔒 **完全开源** - MIT 协议，自由修改

## 📦 安装

### macOS
```bash
brew install --cask craftcraft
```
或从 [Releases](https://github.com/linminhao/craftcraft-launcher/releases) 下载 `.dmg`

### Windows
从 [Releases](https://github.com/linminhao/craftcraft-launcher/releases) 下载 `.exe` 安装包

### Linux (Ubuntu/Debian)
```bash
wget https://github.com/linminhao/craftcraft-launcher/releases/download/v0.1.0/craftcraft_0.1.0_amd64.deb
sudo dpkg -i craftcraft_0.1.0_amd64.deb
```

## 🚀 快速开始

1. 安装 CraftCraft Launcher
2. 首次启动时会自动扫描系统 Java
3. 点击 "+" 创建新实例
4. 选择 Minecraft 版本和加载器
5. 点击 "启动" 开始游戏

## 🛠️ 开发

### 环境要求
- Node.js >= 18
- Rust >= 1.70
- Cargo

### 克隆项目
```bash
git clone https://github.com/linminhao/craftcraft-launcher.git
cd craftcraft-launcher
```

### 安装依赖
```bash
# 前端依赖
npm install

# Rust 依赖会自动安装
```

### 开发模式
```bash
npm run tauri dev
```

### 构建发布版本
```bash
npm run tauri build
```

构建产物位于 `src-tauri/target/release/bundle/`

## 📁 项目结构

```
CraftCraftLauncher/
├── src/                    # Rust 后端
│   ├── commands/          # Tauri 命令
│   ├── models/            # 数据模型
│   ├── services/          # 业务逻辑
│   └── utils/             # 工具函数
├── src-tauri/             # Tauri 配置
└── src/frontend/          # React 前端
    ├── components/        # UI 组件
    ├── hooks/             # 自定义 Hooks
    └── stores/            # 状态管理
```

## 🤝 贡献

欢迎提交 Issue 和 Pull Request！

1. Fork 本仓库
2. 创建特性分支 (`git checkout -b feature/AmazingFeature`)
3. 提交更改 (`git commit -m 'Add some AmazingFeature'`)
4. 推送到分支 (`git push origin feature/AmazingFeature`)
5. 开启 Pull Request

## 📄 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件

## 🙏 致谢

- [Minecraft](https://www.minecraft.net/) - Mojang Studios
- [Prism Launcher](https://prismlauncher.org/) - 开源启动器参考
- [PCL2](https://hmks.github.io/PCL2/) - 中文启动器参考
- [Tauri](https://tauri.app/) - 跨平台框架
- [React](https://reactjs.org/) - 前端框架

## 📊 统计

![GitHub stars](https://img.shields.io/github/stars/linminhao/craftcraft-launcher?style=social)
![GitHub forks](https://img.shields.io/github/forks/linminhao/craftcraft-launcher?style=social)
![GitHub issues](https://img.shields.io/github/issues/linminhao/craftcraft-launcher)
![GitHub license](https://img.shields.io/github/license/linminhao/craftcraft-launcher)

---

Made with ❤️ by [LinMinHao](https://github.com/linminhao)
