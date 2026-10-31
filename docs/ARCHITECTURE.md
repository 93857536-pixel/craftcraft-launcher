# CraftCraft Launcher — 架构与实现规格文档

> 版本 v1.1 · 2026-10-31
> 本文档是**唯一实现基准**：照着做即可复现/继续开发整个项目。
> 当前进度：Rust 后端 100% 完成（cargo check 通过），前端完成 spec 层（types.ts + api.ts），页面组件未写。

---

## 1. 产品目标

跨平台（macOS/Windows/Linux）Minecraft Java 版启动器，体验优于 PCL：

| 能力 | PCL | Prism | CraftCraft |
|---|---|---|---|
| 免装 Java（自动下载 Temurin） | ✗ | ✗ | ✓ |
| 真实 Mojang 官方下载链路 | 半 | ✓ | ✓ |
| 内嵌实时日志 | ✗ | ✗ | ✓ |
| 服务器状态探测（真实 ping） | ✗ | 无 | ✓ |
| 现代 UI（暗色、卡片化） | 旧 | 旧 Qt | ✓ |
| 体积（Tauri vs Electron） | ~ | ~ | ~5MB 壳 |

设计原则：
1. 启动路径极简：选版本 → 点启动 → 自动下载缺什么 → 拉起游戏。
2. 数据全本地：实例/服务器/设置都是 JSON 文件，无数据库、无账号体系（首版 vanilla 免登录，离线/假账号可玩，在线需要 MS 认证——v2）。
3. 零黑盒：JVM 参数可看可改，日志全量落盘。

---

## 2. 技术栈

| 层 | 选型 | 版本 | 理由 |
|---|---|---|---|
| 桌面壳 | Tauri | 2.x | 体积小、原生 webview、Rust 后端 |
| 后端 | Rust | 2021 edition | 进程/网络/文件操作用 Rust 最稳 |
| 前端 | React + TS | 18 / 5.x | 生态、开发速度 |
| 构建 | Vite | 5.x | 快 |
| HTTP | reqwest | 0.12 | Mojang/Adoptium 下载 |
| 持久化 | JSON 文件 | — | 首版够用，路径见 §4 |

依赖清单（src-tauri/Cargo.toml）：
```
tauri 2, tauri-plugin-dialog 2, tauri-plugin-shell 2
serde 1 (derive), serde_json 1
reqwest 0.12 (stream), tokio 1 (macros+rt)
uuid 1 (v4), chrono 0.4, anyhow 1, dirs 6
```
前端依赖：react, react-dom, @tauri-apps/api ^2（仅 `invoke`，不用 shell 插件）。

---

## 3. 目录结构（现状 + 目标）

```
CraftCraftLauncher/
├── index.html                  # 前端入口
├── vite.config.ts              # 5173 固定端口，outDir=dist
├── tsconfig.json
├── package.json
├── src/                        # 前端
│   ├── main.tsx                # React 挂载（待写）
│   ├── App.tsx                 # 布局 + 5 tab 切换（待写）
│   ├── api.ts                  # ✓ 已写：全部 18 个 invoke 封装
│   ├── types.ts                # ✓ 已写：全部 TS 类型（与 Rust 镜像）
│   ├── style.css               # 全局暗色主题（待写）
│   └── components/
│       ├── InstancesPage.tsx   # 待写
│       ├── WizardPage.tsx      # 待写
│       ├── JavaPage.tsx        # 待写
│       ├── LogPage.tsx         # 待写
│       ├── ServersPage.tsx     # 待写
│       └── SettingsPage.tsx    # 待写
├── src-tauri/
│   ├── Cargo.toml              # ✓（crate 名 craftcraft，lib 名 craftcraft_lib）
│   ├── tauri.conf.json         # ✓ identifier=com.linminhao.craftcraft
│   ├── build.rs                # ✓
│   ├── icons/                  # ✓ png×5 + icns + ico（icns 为手写容器生成）
│   └── src/
│       ├── main.rs             # ✓ 调 craftcraft_lib::run()
│       ├── lib.rs              # ✓ Builder：注册 4 个 State + 18 个 command
│       ├── commands/
│       │   ├── mod.rs          # ✓ re-export
│       │   └── instance.rs     # ✓ 全部 18 个 #[tauri::command]（集中一处，简单）
│       ├── models/             # ✓ instance.rs / java.rs / server.rs / settings.rs
│       ├── services/
│       │   ├── mojang.rs       # ✓ Mojang API 客户端
│       │   ├── java_manager.rs # ✓ Java 发现 + Temurin 下载
│       │   ├── launcher.rs     # ✓ 参数组装 + spawn + 日志 tail
│       │   ├── instance_store.rs # ✓ 实例 CRUD
│       │   ├── server_store.rs  # ✓ 服务器 + 真实 ping 协议
│       │   └── settings_store.rs # ✓ 设置
│       └── utils/path.rs       # ✓ 跨平台数据目录
└── docs/
    ├── design.md               # 早期概念设计（已被本文档取代）
    ├── status.md / SUMMARY.md  # 旧进度记录
    └── ARCHITECTURE.md         # 本文档
```

---

## 4. 数据模型（Rust ↔ TS 镜像）

持久化根目录（`utils::path::get_app_data_dir`）：

| 平台 | 路径 |
|---|---|
| macOS | `~/Library/Application Support/CraftCraft/` |
| Windows | `%APPDATA%\CraftCraft\` |
| Linux | `~/.config/CraftCraft/` |

布局：
```
<data>/
├── instances.json          # Vec<Instance>
├── servers.json            # Vec<ServerBookmark>
├── settings.json           # AppSettings
├── versions/<ver>/<ver>.json + minecraft_<ver>.jar
├── libraries/<maven-path>  # 依赖 jar
├── assets/{indexes,objects}
├── javas/temurin-<ver>/    # 托管 JDK（tar 解压）
└── instances/<uuid>/      # 每实例 game directory（saves/config/mods 落这里）
```

### 4.1 Instance（models/instance.rs）
```rust
pub struct Instance {
    pub id: String,              // uuid v4
    pub name: String,
    pub version: String,         // "1.21" / "1.20.4" / "26.3"（Mojang 版本 id）
    pub loader: LoaderType,      // vanilla|forge|fabric|quilt|neoforge（首版只实现 vanilla）
    pub loader_version: Option<String>,
    pub java_version: Option<String>,  // 期望大版本 "21"/"17"/"8"，解析时前缀匹配
    pub java_path: Option<String>,     // 显式指定 java 二进制则优先
    pub game_directory: String,   // 绝对路径（create_instance 时自动分配 <data>/instances/<id>）
    pub memory_min_mb: u64,       // 默认 2048
    pub memory_max_mb: u64,       // 默认 6144
    pub jvm_extra_args: Vec<String>,
    pub account_name: String,     // 首版离线账号名，默认 "Player"
    pub last_played_at: Option<u64>, // unix ts
    pub play_count: u64,
    pub created_at: u64,
    pub tags: Vec<String>,
}
pub struct InstanceUpdates {      // 全 Option 部分更新；update_instance 走它
    name, version, loader, java_path, java_version,
    memory_min_mb, memory_max_mb, account_name, jvm_extra_args,
    #[serde(skip)] last_played_touch: bool,  // 内部标记，launch 成功后置 true
}
```
TS 侧（types.ts）一一对应，`Option<T>` → `T | null`。

### 4.2 JavaVersion（models/java.rs）
```rust
pub struct JavaVersion {
    pub id: String, path: String, version: String,
    pub vendor: String, architecture: String,
    pub is_managed: bool,   // true = 本启动器下载的 Temurin
}
pub struct JavaRecommendation {  // 静态映射表
    mc_version, recommended_java, minimum_java, explanation
}
```
映射表（`JavaRecommendation::get_for_version`）：
```
mc < 1.17        → 推荐 8  （最低 8）
1.17 ≤ mc <1.18.2 → 推荐 17（最低 17）
mc ≥ 1.18.2      → 推荐 21（最低 17）
```

### 4.3 ServerBookmark / ServerStatus（models/server.rs）
```rust
pub struct ServerBookmark { id, address, name, description, is_favorite, last_accessed_at, access_count }
pub struct ServerStatus  { reachable, motd, online, max, version, error }  // query_server 返回
```

### 4.4 AppSettings（models/settings.rs）
```rust
pub struct AppSettings {
    theme: String,                 // "dark"|"light"（默认 dark）
    java_home_override: Option<String>,
    default_memory_min_mb: u64,    // 2048
    default_memory_max_mb: u64,    // 6144
    open_log_on_launch: bool,      // 启动后自动切到日志页（默认 true）
}
```

### 4.5 启动结果
```rust
pub struct LaunchResult { success: bool, pid: Option<u32>, message: String }
```

---

## 5. 后端架构

### 5.1 依赖方向（无环）
```
commands/instance.rs  →  services/*（业务）
services/launcher.rs  →  services/mojang.rs + java_manager.rs + utils
services/*            →  models/* + utils/path
lib.rs 组装：manage(InstanceStore) manage(LauncherState) manage(SettingsStore) manage(ServerStore)
```
`LauncherState` 是全局启动器状态（持有 `MojangClient` + 运行中 pid 列表），所有下载/启动经它。

### 5.2 Tauri 命令 API（18 个，全部在 commands/instance.rs）

| 命令 | 参数 | 返回 | 说明 |
|---|---|---|---|
| `get_instances` | – | `Vec<Instance>`（经 InstanceList 包装） | 读 instances.json |
| `create_instance` | name, version, loader?, account_name? | `Instance` | 自动分配 game dir + 落盘 |
| `update_instance` | id, changes(InstanceUpdates) | `Instance` | 部分更新 |
| `delete_instance` | id | – | 保留磁盘 game dir（首版不做级联删除） |
| `launch_instance` | id | `LaunchResult` | 异步：解析 Java→下载→组装→spawn；成功后 touch_play |
| `read_log_tail` | instanceId, lines | `Vec<String>` | 读 <game dir>/logs/latest.log 尾 N 行 |
| `list_minecraft_versions` | – | `Vec<{id,type,url}>` | Mojang version manifest 过滤非 release 也返回，前端过滤 |
| `list_java` | – | `Vec<JavaVersion>` | PATH + /Library/Java + 托管目录 |
| `install_java` | version | `JavaVersion` | 下 Adoptium tarball → 解压到 javas/ |
| `recommend_java` | mcVersion | `JavaRecommendation` | 纯静态表 |
| `list_servers` | – | `Vec<ServerBookmark>` | |
| `add_server` | address, name | `ServerBookmark` | |
| `remove_server` | id | – | |
| `toggle_server_favorite` | id | `Option<ServerBookmark>` | |
| `query_server` | address | `ServerStatus` | **真实 TCP ping**（§7.3），8s 超时 |
| `get_settings` | – | `AppSettings` | |
| `save_settings` | settings | – | |

前端调用统一走 `src/api.ts` 的 `invoke` 封装（已完成，函数名一一对应）。

### 5.3 MojangClient（services/mojang.rs）关键方法
```
fetch_version_list()        GET piston-meta.mojang.com/mc/game/version_manifest_v2.json
get_version_json(ver)       GET launchermeta.mojang.com/mc/game/download/{ver}.json，落缓存
ensure_client_jar(ver)      下载 minecraft_{ver}.jar
ensure_libraries(json)      按 libraries[] 逐个下到 libraries/，返回 classpath Vec<PathBuf>
ensure_assets(json)         按 asset_index.url 下索引 → objects/ 逐个补
```
实现要点：
- `VersionJson.arguments` 是**新旧混合格式**：旧版 `arguments.game` 是 `Vec<serde_json::Value>`（字符串或带 rules 的对象数组），新版同样。统一用 `render_value` 渲染（§7.2）。
- 库下载 URL 优先 `lib.downloads.artifact.url`，回退 `lib.url`。
- 跳过当前 OS 之外的 natives（`natives-macos/windows/linux` 前缀 + rules.os.name 双重判断）。
- 全部幂等：文件已存在即跳过（可断点续用的粗粒度缓存）。

---

## 6. 启动流水线（launch_instance 内部）

```
1. 解析 Java
   instance.java_path? → 用它
   否则 JavaManager::list() 按 java_version 前缀匹配 → 取 is_managed 兜底
   都没有 → LaunchResult{success:false, message:"no Java found..."}（前端引导去 Java 页装）
2. 下载游戏文件（MojangClient）
   version json → client jar → libraries → assets（幂等）
   任一步网络失败 → success:false，message 带具体原因
3. 组装参数（build_launch_args）
   jvm: -Xms/-Xmx（来自 instance 内存配置）+ -XX:MaxMetaspaceSize=max/4
        + version_json.arguments.jvm（render_value）
        + instance.jvm_extra_args
   classpath: client_jar : libs...（mac/linux 冒号；Windows 需 ';' —— 见 §10 风险）
   main_class: version_json.main_class
   game args: version_json.arguments.game（render_value）
     补默认 --gameDir / --assetsDir / --assetIndex / --username
4. spawn
   stdout → /dev/null（Mojang 客户端日志在 stderr）
   stderr → <game dir>/logs/latest.log
   current_dir = game dir
5. 记 pid 进 LauncherState.running；emit "instance:launched" 事件
6. 成功后 update_instance(last_played_touch=true)
```
读回路径：前端 Log 页轮询 `read_log_tail(id, 300)`（首版 2s 轮询，v2 可换 Tauri 事件流式）。

---

## 7. 关键算法与协议

### 7.1 占位符渲染 render_value（launcher.rs）
输入 `serde_json::Value`（旧格式是字符串，新格式是对象数组），输出最终参数字符串：
- 纯字符串：把 `${classpath}`、`${version}`、`${game_directory}`、`${assets_directory}`、`${auth_player_name}`、`${classpath_separator}`、`${user_type}`、`${natives_directory}`、`${legacy_user_hash}`、`${xplatform}` 替换为实际值。
- 新格式数组：每个元素若是对象，先过 `rules`（action=allow 且 os.name==当前 OS 才保留），再取 `value`（字符串直接取；数组拼接）。
- 过滤掉空串。

### 7.2 版本 JSON 关键字段
```json
{ "id": "1.21", "main_class": "net.minecraft.client.main.Main",
  "arguments": { "game": [ ... ], "jvm": [ ... ] },
  "libraries": [ { "name": "group:artifact:version[:classifier]", "downloads": { "artifact": { "url": "..." } } } ],
  "asset_index": { "id": "x", "url": "..." },
  "download": { "client": { "url": "..." } } }
```
Rust 侧对应 `VersionJson` 等结构（services/mojang.rs 已有）。

### 7.3 服务器状态探测（server_store.rs，已实现）
现代协议（≥1.7）：
```
发送1: [varint(24)][0x00][varint(47)][host][port_be][0x01]
发送2: [varint(1)][0x00]
接收:  [varint(0)][varint(json_len)][json]
json:  { "description": "...", "players": {"online":N,"max":M}, "version": {"name":"1.21"} }
```
解析防御：`read_varint` 返回 `(consumed_bytes, value)`；越界/超时统一 `reachable:false + error`。8s 读超时。

### 7.4 Temurin 下载（java_manager.rs，已实现）
```
GET https://api.adoptium.net/v3/binary/latest/{ver}/ga/{os}/{arch}/jdk/hotspot/normal/eclipse
os: mac|win|linux；arch: aarch64|x64（本机 arm64→aarch64）
tar 解到 javas/ → 找到 jdk-* 目录 rename 为 temurin-{ver} → 返回 JavaVersion(is_managed=true)
```

---

## 8. 前端实现规格

### 8.1 架构
- 无路由库：App.tsx 用 `useState<Tab>` 切换 6 个页面（Instances/Wizard/Java/Logs/Servers/Settings），左侧竖排 nav。
- 无状态库：数据都来自 invoke，每页自己 `useEffect` 拉取；`api.ts` 是唯一数据通道。
- 样式：单 `style.css`，CSS 变量暗色主题（背景 #0F172A，卡片 #1E293B，强调 #6366F1，成功 #22C55E，错误 #EF4444），卡片圆角 12px。
- 轮询：Log 页 2s 一次 `readLogTail`；启动中时显示 spinner。

### 8.2 组件树
```
App
├─ Nav（6 个 tab 按钮）
├─ InstancesPage
│  ├─ InstanceCard × N（名称/版本/加载器/内存/最后玩过；按钮：▶启动、⚙编辑、🗑删除）
│  └─ 顶部「新建实例」按钮 → Wizard
├─ WizardPage（分 3 步）
│  ① 选版本（listMinecraftVersions，可搜索，过滤 type=release 默认勾选）+ 加载器下拉
│  ② 选 Java（listJava 列表 + recommendJava 提示 + 「下载 Temurin {ver}」按钮调 installJava）
│  ③ 内存滑块 min/max + 账号名 → createInstance → 回列表
├─ JavaPage
│  已装 Java 表格（version/vendor/path/托管标记）+ 一键下载区（8/17/21 三个按钮）
├─ LogPage
│  选实例 → 终端样式 <pre>（monospace，深色），ERROR/Exception 行红色高亮（正则 split）
│  按钮：刷新、清空提示（read_log_tail 只读）
├─ ServersPage
│  列表（name/address/❤收藏）+ 添加表单（address+name → addServer）
│  每行「测速」按钮 → queryServer → 显示 MOTD/在线人数/版本/延迟状态
└─ SettingsPage
   theme、默认内存、open_log_on_launch 开关 → saveSettings
```

### 8.3 页面细节规格
- **InstancesPage**：启动按钮 loading 态；LaunchResult.success=false 时顶部红条显示 message（通常是"no Java"→ 附「去装 Java」链接切 tab）。启动成功且 open_log_on_launch=true 自动切 Log 页选中该实例。
- **WizardPage**：版本列表长，做 300ms 防抖搜索过滤；选中版本后调 recommendJava 显示「推荐 Java 21（最低 17）」。下载 Temurin 显示进度态（无进度条，只 spinner + 完成后刷新 listJava）。
- **LogPage**：高亮规则：`/ERROR|Exception|Caused by/` 命中整行 #EF4444；`/WARN/` #F59E0B。空日志显示占位文案。
- **ServersPage**：queryServer 是同步长调用（最多 8s），按钮禁用等待；失败显示 error 字段。
- **启动确认**：所有「启动」按钮直接执行（游戏启动不是危险操作），但「删除实例」需 confirm。

### 8.4 关键代码骨架（写组件时直接抄）
```tsx
// App.tsx
const TABS = ['Instances','New','Java','Logs','Servers','Settings'] as const;
export default function App() {
  const [tab, setTab] = useState<typeof TABS[number]>('Instances');
  return <div className="app">
    <Nav tabs={TABS} active={tab} onChange={setTab} />
    {tab==='Instances' && <InstancesPage onLaunch={(id)=>{if(settings.open_log_on_launch){setTab('Logs');setSelected(id);}} />} />}
    {tab==='New' && <WizardPage onDone={()=>setTab('Instances')} />}
    {tab==='Java' && <JavaPage />}
    {tab==='Logs' && <LogPage />}
    {tab==='Servers' && <ServersPage />}
    {tab==='Settings' && <SettingsPage />}
  </div>;
}
```
```tsx
// LogPage.tsx 轮询模式
useEffect(() => {
  if (!sel) return;
  const t = setInterval(async () => {
    setLines(await readLogTail(sel, 300).catch(() => []));
  }, 2000);
  return () => clearInterval(t);
}, [sel]);
```
```css
/* style.css 变量 */
:root{--bg:#0F172A;--card:#1E293B;--card2:#334155;--text:#F8FAFC;--muted:#94A3B8;
      --accent:#6366F1;--ok:#22C55E;--warn:#F59E0B;--err:#EF4444;
      --mono:ui-monospace,SFMono-Regular,Menlo,monospace}
```

### 8.5 类型对齐注意
Rust 的 `Option<T>` 序列化为 `null`，TS 用 `T | null`；`LoaderType` 序列化为小写字符串。`MojangVersionInfo.type` 在 Rust 里是 `r#type`，JSON 键就是 `type`。

---

## 9. 构建与打包

```bash
cd CraftCraftLauncher
npm install                     # 前端依赖
cd src-tauri && cargo check    # 后端验证（已通过）
cd ..
npm run tauri build            # 出 .app/.dmg（macOS）、.msi/.nsis（Win）、.deb/.AppImage（Linux）
                               # 内部流程：beforeBuildCommand=npm run build → dist/ → 嵌入
```
- `npm run build` = `tsc && vite build`（产物 dist/ 已验证 142KB JS）。
- 图标已生成（png/icns/ico，Tauri CLI 构建时按需校验）。
- 签名：首版 ad-hoc / 不签名，README 说明首次打开右键。
- 版本号三处同步：src-tauri/tauri.conf.json、src-tauri/Cargo.toml、package.json。

---

## 10. 已知风险与 TODO（v1.1）

| # | 风险 | 处置 |
|---|---|---|
| 1 | classpath 分隔符：mac/linux `:`、Windows `;` | 在 build_launch_args 用 `if cfg!(windows) {";"} else {":"}` 取 sep 传 ctx（当前硬编码 ":"，需补） |
| 2 | 仅 vanilla；Forge/Fabric loader 注入 | v2：按 loader 追加 main_class/额外 lib（Fabric 装 loader jar 进 classpath） |
| 3 | 无在线认证（MS token）→ 联机进不去 | v2 加 MS 登录（浏览器授权码流程）；首版离线/本地单可玩 |
| 4 | 日志轮询非流式 | 首版可接受；v2 用 Tauri channel 推 |
| 5 | 旧版游戏（≤1.6）用 `--assetsDir <dir>/<index>` 拼接 | 1.21+ 验证为主；旧版 assets 路径拼接为 `assets_dir/<asset_index>` 需注意 |
| 6 | icns 是手写容器生成（iconutil 本机故障） | 已验证 macOS 能渲染；Windows .ico 走 PIL 正常 |

---

## 11. 验证清单（交付前逐条过）

- [ ] `cargo check` 0 error（当前通过）
- [ ] `npm run build` 出 dist/（当前通过）
- [ ] 前端 6 页组件写完并编译通过
- [ ] `npm run tauri dev`：跑起来，建实例 → 下 1.21 → 装 Temurin 21 → 启动游戏画面出现
- [ ] Log 页能看到 MC 启动日志、ERROR 高亮
- [ ] Servers 页对一个真实 MC 服务器 ping 出 MOTD/人数
- [ ] `npm run tauri build` 出 .app，双击可运行
- [ ] 杀游戏后重启启动器，实例数据仍在（instances.json 验证）

---

## 12. 已完成 vs 待做（接手者路线）

已做：目录骨架、全部 Rust 后端（含真实下载/启动/协议实现）、图标、前端 types.ts + api.ts。
待做（按序）：
1. src/main.tsx + style.css + App.tsx（§8.2 树）
2. 六个页面组件（§8.3 规格 + §8.4 骨架）
3. 修 classpath 分隔符（§10-1）
4. tauri dev 实测启动 MC（§11 第 4 条）
5. tauri build 出包 → GitHub Release（按用户惯例：必须 dmg 可安装包 + 建仓 About 描述）
