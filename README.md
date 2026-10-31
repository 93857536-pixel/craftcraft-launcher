# CraftCraft Launcher

A modern, cross-platform Minecraft: Java Edition launcher built with **Rust + Tauri 2 + React**.
比 PCL 更轻、更快，界面更现代，且不需要你手动装 Java。

![platform](https://img.shields.io/badge/platform-macOS%20%7C%20Windows%20%7C%20Linux-6366F1)
![license](https://img.shields.io/badge/license-MIT-22C55E)
![stack](https://img.shields.io/badge/stack-Rust%20%2B%20Tauri%202%20%2B%20React-F59E0B)

---

## 为什么不用 PCL / Prism？

| | PCL | Prism | **CraftCraft** |
|---|---|---|---|
| 自动下载 Java（无需手装） | ✗ | ✗ | **✓** |
| 官方 Mojang 下载链路 | 部分 | ✓ | **✓** |
| 内嵌实时日志 + 错误高亮 | ✗ | ✗ | **✓** |
| 服务器状态探测（真实 TCP ping） | ✗ | ✗ | **✓** |
| 安装包体积 | ~ | ~ | **≈10MB（Tauri 壳）** |
| 现代暗色 UI | ✗ | ✗（Qt） | **✓** |

---

## 功能

- **实例管理** — 多实例并存，各自独立的存档 / 模组 / 配置目录
- **零手装 Java** — 内置 Temurin JDK 下载器（8 / 17 / 21），自动匹配 MC 版本所需 Java
- **真实启动链路** — 解析 Mojang version manifest → 下载 client jar / libraries / natives / assets → 组装 JVM 参数 → 拉起进程
- **离线可玩** — 自动生成离线 UUID，无需正版即可单人游戏
- **实时日志** — 内嵌日志面板，2 秒轮询，ERROR/WARN 高亮
- **服务器书签** — 保存服务器 + 真实状态探测（MOTD / 在线人数 / 版本）
- **内存与 JVM 参数** — 每实例独立配置

---

## 快速开始

### 下载安装

从 [Releases](https://github.com/93857536-pixel/craftcraft-launcher/releases) 下载：

- macOS: `CraftCraft Launcher_0.1.0_aarch64.dmg`
- Windows: `CraftCraft Launcher_0.1.0_x64-setup.exe`
- Linux: `.deb` / `.AppImage`

首次打开（未签名）macOS 需右键 → 打开。

### 从源码构建

```bash
git clone https://github.com/93857536-pixel/craftcraft-launcher
cd craftcraft-launcher

npm install
npm run tauri build     # 产物在 src-tauri/target/release/bundle/
```

开发模式：

```bash
npm run tauri dev
```

---

## 使用流程

1. **Java 页** → 点 `Download Temurin 21`（一次性，约 200MB）
2. **New** → 填名称 / 版本（如 `1.21`）/ 内存 → Create
3. **Instances** → 点 `▶ Launch`（首次会下载游戏文件，约 300MB）
4. 自动跳到 **Logs** 页看实时启动日志

---

## 架构

```
React (Vite)  ──invoke──▶  Tauri commands (18)
                              │
                    ┌─────────┴──────────┐
              services/              models/
        mojang.rs   java_manager.rs   instance.rs
        launcher.rs server_store.rs   java.rs / server.rs / settings.rs
        instance_store.rs settings_store.rs
                              │
                    <data dir>/instances.json
                              servers.json / settings.json
                              versions/ libraries/ assets/ javas/
```

数据目录：

| 平台 | 路径 |
|---|---|
| macOS | `~/Library/Application Support/CraftCraft/` |
| Windows | `%APPDATA%\CraftCraft\` |
| Linux | `~/.config/CraftCraft/` |

完整实现规格见 [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md)。

---

## 开发

```bash
# 后端编译检查
cd src-tauri && cargo check

# 端到端冒烟测试（真实下载 Java + 游戏文件 + 启动 JVM 校验 classpath）
cd src-tauri && cargo run --release --bin smoke -- 1.21

# 前端类型检查 + 构建
npm run build
```

---

## 已知限制（v0.1.0）

- 仅支持 **vanilla**（Forge / Fabric / Quilt / NeoForge 在路线图上）
- **离线模式**（离线 UUID），未接入 Microsoft 正版登录 → 无法进入正版验证服务器
- 资产下载为串行（首次较慢，后续有缓存）
- 未做代码签名 / 公证

## License

MIT © LinMinHao
