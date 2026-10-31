# CraftCraft Launcher - 设计文档 v1.0

## 一、项目概述

**CraftCraft Launcher** 是一款跨平台 Minecraft 启动器，目标是超越 PCL（Plain Craft Launcher）的体验。

- **目标平台**：macOS / Windows / Linux（三端统一体验）
- **开源协议**：MIT License
- **技术栈**：Tauri v2 + Rust + React (TypeScript)
- **项目路径**：`~/Programming/Projects/CraftCraftLauncher/`

---

## 二、核心设计理念

### 2.1 与 PCL/Prism 的差异点

| 特性 | PCL2 | Prism Launcher | CraftCraft Launcher |
|------|------|----------------|---------------------|
| UI 现代化 | ✗ | △ | ✓ 全新设计 |
| 多平台原生 | ✗ (Win only) | △ (Qt 适配一般) | ✓ (Tauri 原生) |
| Java 智能管理 | ✗ | △ | ✓ 内置 Jabba 集成 |
| 内容库系统 | ✗ | ✗ | ✓ 统一资源管理 |
| 内嵌日志面板 | ✗ | △ | ✓ 实时语法高亮 |
| 服务器工作台 | ✗ | ✗ | ✓ 收藏+历史+状态 |
| 性能仪表板 | ✗ | ✗ | ✓ 运行时监控 |
| 数据导入导出 | ✗ | △ | ✓ 一键迁移 PCL/Prism |
| 跨实例同步设置 | ✗ | ✗ | ✓ 模板复制 |

### 2.2 用户体验优先级

1. **启动速度**：秒开游戏，最小化等待
2. **零配置安装**：开箱即用，自动检测环境
3. **视觉舒适**：暗色主题，护眼不刺眼
4. **操作直觉**：少点击，多直接操作

---

## 三、架构设计

### 3.1 技术选型

```
┌─────────────────────────────────────────┐
│           Frontend (React + TS)          │
│  ┌──────────┬──────────┬──────────────┐  │
│  │ Instance │ Content  │  Settings    │  │
│  │ Manager  │  Vault   │   Panel      │  │
│  └──────────┴──────────┴──────────────┘  │
├─────────────────────────────────────────┤
│           Tauri Layer (Rust Backend)     │
│  ┌──────────┬──────────┬──────────────┐  │
│  │ Process  │  File    │   Network    │  │
│  │  Runner  │  Manager │   Fetcher    │  │
│  └──────────┴──────────┴──────────────┘  │
├─────────────────────────────────────────┤
│         Platform Abstraction Layer       │
│  ┌──────────┬──────────┬──────────────┐  │
│  │ macOS    │ Windows  │    Linux     │  │
│  │  Path    │   Path   │    Path      │  │
│  └──────────┴──────────┴──────────────┘  │
└─────────────────────────────────────────┘
```

### 3.2 目录结构

```
CraftCraftLauncher/
├── .github/
│   └── workflows/           # CI/CD 配置
│       ├── build-mac.yml
│       ├── build-windows.yml
│       └── build-linux.yml
├── src/
│   ├── main.rs              # Tauri 入口
│   ├── lib.rs               # Rust 库定义
│   ├── commands/            # Tauri 命令层
│   │   ├── instance.rs      # 实例管理
│   │   ├── java.rs          # Java 管理
│   │   ├── launcher.rs      # 启动逻辑
│   │   ├── log.rs           # 日志读取
│   │   └── content.rs       # 内容库
│   ├── models/              # 数据模型
│   │   ├── instance.rs
│   │   ├── java_version.rs
│   │   ├── server.rs
│   │   └── config.rs
│   ├── services/            # 业务逻辑
│   │   ├── instance_service.rs
│   │   ├── java_manager.rs
│   │   ├── minecraft_launcher.rs
│   │   ├── log_reader.rs
│   │   └── content_vault.rs
│   └── utils/               # 工具函数
│       ├── path.rs          # 跨平台路径处理
│       ├── process.rs       # 进程管理
│       └── filesystem.rs    # 文件操作
├── src-tauri/
│   ├── capabilities/        # Tauri 能力配置
│   ├── icons/               # 应用图标
│   ├── info.plist           # macOS 配置
│   └── tauri.conf.json      # Tauri 配置
├── src/frontend/            # React 前端
│   ├── index.html
│   ├── main.tsx
│   ├── App.tsx
│   ├── components/
│   │   ├── layout/
│   │   │   ├── Sidebar.tsx
│   │   │   ├── Header.tsx
│   │   │   └── MainLayout.tsx
│   │   ├── instance/
│   │   │   ├── InstanceCard.tsx
│   │   │   ├── InstanceList.tsx
│   │   │   ├── InstanceEditor.tsx
│   │   │   └── InstanceCreateWizard.tsx
│   │   ├── content/
│   │   │   ├── ContentVault.tsx
│   │   │   ├── ModManager.tsx
│   │   │   ├── ResourcePackPicker.tsx
│   │   │   └── SkinViewer.tsx
│   │   ├── server/
│   │   │   ├── ServerList.tsx
│   │   │   ├── ServerDetail.tsx
│   │   │   └── AddServerDialog.tsx
│   │   ├── log/
│   │   │   ├── LogPanel.tsx
│   │   │   ├── LogFilter.tsx
│   │   │   └── LogHighlighter.tsx
│   │   └── settings/
│   │       ├── GeneralSettings.tsx
│   │       ├── JavaSettings.tsx
│   │       ├── ThemeSettings.tsx
│   │       └── ImportExport.tsx
│   ├── hooks/
│   │   ├── useInstances.ts
│   │   ├── useJavaVersions.ts
│   │   ├── useLogs.ts
│   │   └── useSettings.ts
│   ├── stores/
│   │   ├── instanceStore.ts
│   │   ├── javaStore.ts
│   │   └── settingsStore.ts
│   └── types/
│       ├── instance.ts
│       ├── java.ts
│       ├── server.ts
│       └── log.ts
├── assets/
│   ├── icons/
│   └── images/
├── data/
│   └── default-config.json  # 默认配置
├── package.json
├── tsconfig.json
├── Cargo.toml
└── README.md
```

---

## 四、核心模块详细设计

### 4.1 实例管理模块 (InstanceManager)

**数据模型**：

```typescript
interface Instance {
  id: string;                    // UUID v4
  name: string;                  // 实例名称
  description?: string;          // 描述
  version: string;               // Minecraft 版本
  loader: LoaderType;            // forge/fabric/quilt/optifine
  loaderVersion: string;         // 加载器版本
  javaVersion?: string;          // Java 版本要求
  javaPath?: string;             // 指定 Java 路径
  gameDirectory: string;         // 游戏目录
  icon?: string;                 // 自定义图标路径
  customJVMArgs: string[];       // 自定义 JVM 参数
  customGameArgs: string[];      // 自定义游戏参数
  memoryMin: number;             // 最小内存 (MB)
  memoryMax: number;             // 最大内存 (MB)
  lastPlayedAt?: number;         // 最后运行时间戳
  playCount: number;             // 运行次数
  createdAt: number;             // 创建时间戳
  updatedAt: number;             // 更新时间戳
  tags: string[];                // 标签
  isActive: boolean;             // 是否激活
}

enum LoaderType {
  VANILLA = 'vanilla',
  FORGE = 'forge',
  FABRIC = 'fabric',
  QUILT = 'quilt',
  OPTIFINE = 'optifine',
  NEOFORGE = 'neoforge'
}
```

**核心功能**：
- CRUD 操作
- 批量导入（PCL/Prism JSON）
- 拖拽排序
- 实例克隆
- 批量操作（删除/移动）

---

### 4.2 Java 管理器 (JavaManager)

**数据模型**：

```typescript
interface JavaVersion {
  id: string;
  path: string;                  // Java 可执行文件路径
  version: string;               // 版本字符串
  vendor: string;                // 发行商 (Adoptium, Microsoft 等)
  architecture: 'x64' | 'aarch64' | 'x86';
  isActive: boolean;
  isSystem(): boolean;           // 是否为系统自带
  lastUsedAt?: number;
}
```

**核心功能**：
- 自动扫描已安装的 Java
- 集成 Jabba（macOS/Linux）/ SDKMAN 自动安装
- 按需下载 Temurin/Zulu JDK
- 版本兼容性检查（根据 Minecraft 版本推荐）
- 一键切换默认 Java

**Java 版本与 MC 版本对应关系**：
```
MC < 1.17  → Java 8
1.17 ≤ MC < 1.18.2 → Java 17
MC ≥ 1.18.2 → Java 17/21
```

---

### 4.3 启动器核心 (MinecraftLauncher)

**启动流程**：

```
1. 验证环境
   ├── 检查 Java 是否可用
   ├── 检查游戏目录是否存在
   └── 检查必要文件完整性

2. 资源检查
   ├── 检查资产文件
   ├── 检查 mods 文件夹
   └── 检查配置文件

3. 组装启动参数
   ├── JVM 参数（内存、GC、性能优化）
   ├── 游戏参数（版本、账号、目录）
   └── 环境变量（MOJANG_*）

4. 启动进程
   ├── 创建子进程
   ├── 重定向输出到日志文件
   └── 返回进程 PID

5. 后台监控
   ├── 监控进程状态
   ├── 实时读取日志
   └── 异常处理与提示
```

**核心代码逻辑**：

```rust
#[tauri::command]
async fn launch_instance(instance_id: String) -> Result<LaunchResult, String> {
    let instance = InstanceService::get(&instance_id)?;
    
    // 1. 解析 Java
    let java = JavaManager::resolve(&instance)?.ok_or("No Java found")?;
    
    // 2. 构建命令
    let mut cmd = Command::new(java.path);
    cmd.args(build_jvm_args(&instance))
       .arg("-jar")
       .arg(instance.game_path.join("minecraft.jar"))
       .args(instance.custom_jvm_args)
       .args(instance.custom_game_args);
    
    // 3. 设置环境变量
    cmd.env("JAVA_HOME", java.path.parent().unwrap());
    cmd.current_dir(&instance.game_directory);
    
    // 4. 启动并记录日志
    let mut child = cmd.spawn()?;
    let log_file = instance.game_path.join("logs/latest.log");
    childStdout.pipe_to_file(&log_file)?;
    
    Ok(LaunchResult { pid: child.id() })
}
```

---

### 4.4 日志面板 (LogPanel)

**功能设计**：
- 实时读取日志文件（tail -f 机制）
- 关键词高亮：`ERROR`（红）、`WARN`（黄）、`Exception`（橙）
- 时间戳格式化
- 搜索过滤
- 日志导出（.txt/.log）
- 自动滚动到底部（可关闭）

**性能优化**：
- 使用文件系统监听（inotify/fswatch）而非轮询
- 大文件流式读取，只保留最近 1000 行在内存
- WebAssembly 实现快速搜索

---

### 4.5 内容库 (ContentVault)

**目录结构**：
```
~/Library/Application Support/CraftCraft/
├── instances/
│   ├── {instance-id}/
│   │   ├── minecraft.jar
│   │   ├── mods/
│   │   ├── resourcepacks/
│   │   ├── saves/
│   │   └── configs/
│   └── ...
├── content/
│   ├── mods/              # 已下载的 mod jar
│   ├── resourcepacks/     # 资源包
│   ├── shaders/           # 光影包
│   ├── worlds/            # 存档
│   └── skins/             # 皮肤
├── javas/
│   └── {java-version}/
├── cache/
│   ├── assets/            # Mojang 资产缓存
│   └── libraries/         # 依赖库缓存
└── config.json
```

**核心功能**：
- Mod 管理：搜索/安装/更新/删除
- 资源包管理：预览/启用/禁用
- 光影管理：一键安装 OptiFine/Iris
- 皮肤管理：本地预览+在线商城
- 备份与恢复

---

### 4.6 服务器工作台 (ServerWorkbench)

**数据模型**：

```typescript
interface ServerBookmark {
  id: string;
  address: string;           // IP:端口 或域名
  name: string;
  description?: string;
  favicon?: string;          // 服务器图标
  version?: string;
  motd?: string;             // 在线时的 MOTD
  players?: {
    max: number;
    online: number;
    sample: Array<{name: string}>;
  };
  isFavorite: boolean;
  lastAccessedAt?: number;
  accessCount: number;
  tags: string[];
}
```

**核心功能**：
- 服务器列表（网格/列表视图）
- 实时状态查询（在线人数/版本）
- 收藏标记
- 最近访问历史
- 快速加入（双击或回车）
- 服务器详情面板

---

## 五、UI/UX 设计

### 5.1 整体布局

```
┌─────────────────────────────────────────────────────────┐
│  Logo  │  Search...              │  User Avatar  [⚙]  │
├─────────────────────────────────────────────────────────┤
│                                                          │
│  ┌────────┐  ┌─────────────────────────────────────┐    │
│  │        │  │                                     │    │
│  │  NAV   │  │           MAIN CONTENT              │    │
│  │        │  │                                     │    │
│  │• Inst. │  │  ┌─────────┐ ┌─────────┐ ┌──────┐ │    │
│  │• Cont. │  │  │ Card 1  │ │ Card 2  │ │ Card 3 │ │    │
│  │• Serve│  │  └─────────┘ └─────────┘ └──────┘ │    │
│  │• Log   │  │                                     │    │
│  │• Settg │  │  [ + New Instance ]                 │    │
│  │        │  │                                     │    │
│  └────────┘  └─────────────────────────────────────┘    │
│                                                          │
├─────────────────────────────────────────────────────────┤
│  Status: ● Online    Java: 21.0.1    FPS: --          │
└─────────────────────────────────────────────────────────┘
```

### 5.2 配色方案

```
主色调：
- Primary: #6366F1 (Indigo)
- Secondary: #8B5CF6 (Purple)
- Accent: #F59E0B (Amber)

背景：
- Dark: #0F172A (Slate 900)
- Surface: #1E293B (Slate 800)
- Card: #334155 (Slate 700)

文字：
- Primary: #F8FAFC (Slate 50)
- Secondary: #94A3B8 (Slate 400)
- Muted: #64748B (Slate 500)

状态色：
- Success: #22C55E (Green)
- Warning: #F59E0B (Amber)
- Error: #EF4444 (Red)
- Info: #3B82F6 (Blue)
```

### 5.3 交互设计要点

1. **启动按钮**：显眼、大尺寸、常驻可见
2. **实例卡片**：hover 效果、拖拽重排序
3. **快捷键支持**：
   - `Space` - 启动当前选中实例
   - `Ctrl+N` - 新建实例
   - `Ctrl+K` - 搜索实例
   - `Esc` - 关闭弹窗
4. **通知系统**：右下角 Toast 提示
5. **动画过渡**：页面切换平滑过渡

---

## 六、跨平台适配

### 6.1 平台差异处理

```rust
// src/utils/path.rs
use std::path::{PathBuf, Path};

pub fn get_app_data_dir() -> PathBuf {
    #[cfg(target_os = "macos")]
    {
        home_dir().unwrap().join("Library/Application Support/CraftCraft")
    }
    #[cfg(target_os = "windows")]
    {
        std::env::var("APPDATA")
            .map(|p| Path::new(&p).join("CraftCraft"))
            .unwrap_or_default()
    }
    #[cfg(target_os = "linux")]
    {
        home_dir().unwrap().join(".config/CraftCraft")
    }
}

pub fn get_java_binary_path(java_home: &Path) -> PathBuf {
    #[cfg(target_os = "windows")]
    {
        java_home.join("bin").join("java.exe")
    }
    #[cfg(not(target_os = "windows"))]
    {
        java_home.join("bin").join("java")
    }
}
```

### 6.2 安装与打包

- **macOS**：.dmg + .app（ad-hoc 签名）
- **Windows**：.exe 安装包（NSIS）
- **Linux**：.deb/.AppImage（原生）+ AUR（Arch）

---

## 七、数据持久化

### 7.1 存储方案

- **配置数据**：JSON 文件（跨平台兼容）
- **实例数据**：JSON + SQLite（实例多了用 DB）
- **缓存数据**：本地文件缓存

### 7.2 备份策略

- 自动备份到 `~/Library/Application Support/CraftCraft/backups/`
- 手动导出/导入（JSON 格式）
- 云端同步（可选，未来功能）

---

## 八、开发计划

### Phase 1: 基础框架 (Week 1)
- [ ] Tauri 项目初始化
- [ ] React 前端脚手架
- [ ] 基础布局组件
- [ ] 实例 CRUD API

### Phase 2: 核心功能 (Week 2-3)
- [ ] Java 管理器
- [ ] 启动器核心逻辑
- [ ] 日志面板
- [ ] 内容库基础

### Phase 3: 高级功能 (Week 4)
- [ ] 服务器工作台
- [ ] 性能仪表板
- [ ] 导入/导出工具
- [ ] 设置面板

### Phase 4: 打磨优化 (Week 5)
- [ ] UI 细节优化
- [ ] 性能优化
- [ ] 错误处理
- [ ] 国际化（中/英）

### Phase 5: 发布准备 (Week 6)
- [ ] 图标设计
- [ ] 打包脚本
- [ ] GitHub Release 配置
- [ ] 文档编写

---

## 九、依赖清单

### Rust 后端
```toml
[dependencies]
tauri = { version = "2", features = ["shell-open"] }
serde = { version = "1", features = ["derive"] }
serde_json = "1"
tokio = { version = "1", features = ["full"] }
reqwest = { version = "0.11", features = ["json"] }
dirs = "5"
uuid = { version = "1", features = ["v4"] }
chrono = { version = "0.4", features = ["serde"] }
fs_extra = "1"
tar = "0.4"
flate2 = "1"
zip = "0.6"
walkdir = "2"
notify = "6"  # 文件监听
anyhow = "1"
thiserror = "1"
```

### TypeScript 前端
```json
{
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "@tauri-apps/api": "^2",
    "@tauri-apps/plugin-shell": "^2",
    "@tauri-apps/plugin-dialog": "^2",
    "zustand": "^4.4.0",
    "react-router-dom": "^6.20.0",
    "dayjs": "^1.11.10",
    "clsx": "^2.0.0",
    "tailwindcss": "^3.4.0",
    "class-variance-authority": "^0.7.0",
    "lucide-react": "^0.294.0"
  },
  "devDependencies": {
    "@tauri-apps/cli": "^2",
    "@types/react": "^18.2.0",
    "@types/react-dom": "^18.2.0",
    "@vitejs/plugin-react": "^4.2.0",
    "typescript": "^5.3.0",
    "vite": "^5.0.0"
  }
}
```

---

## 十、风险提示

1. **Minecraft 协议变更**：微软认证系统可能调整，需持续关注
2. **Java 版本兼容性**：不同 MC 版本对 Java 要求不同，需做好版本映射
3. **反作弊问题**：部分服务器可能检测第三方启动器
4. **平台差异**：Windows/Linux/macOS 路径和命令不同，需充分测试
5. **法律风险**：开源项目需避免嵌入任何微软 copyrighted 内容

---

## 十一、下一步

确认本设计文档后，开始执行：

1. 初始化项目脚手架
2. 搭建基础架构
3. 实现核心功能模块

---

*设计文档版本: v1.0*
*最后更新: 2026-10-30*
