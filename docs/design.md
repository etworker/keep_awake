# 设计文档：Keep Awake

## 架构概览

```
keep_awake
├── src/
│   ├── main.rs          # 入口：启动托盘事件循环
│   ├── tray.rs          # 系统托盘图标与菜单管理
│   ├── inhibit.rs       # 平台相关的休眠抑制（条件编译）
│   ├── mouse.rs         # 鼠标微移回退模式
│   └── config.rs        # 配置读写（serde_json）
└── Cargo.toml
```

## 模块职责

### main.rs
- 初始化配置
- 创建系统托盘
- 启动事件循环（阻塞，程序常驻）
- 注册全局快捷键（可选）

### tray.rs
- 创建系统托盘图标（内置图标，无外部资源依赖）
- 右键菜单项：
  - 启用/停用（勾选状态）
  - 分隔线
  - 模式切换：API 抑制 / 鼠标微移
  - 分隔线
  - 开机自启（勾选状态）
  - 退出

### inhibit.rs — 平台条件编译
每个平台一个实现，通过 `#[cfg(target_os = "...")]` 隔离：

**Windows** (`cfg(windows)`)
```rust
// windows-sys: SetThreadExecutionState
// 调用 ES_CONTINUOUS | ES_SYSTEM_REQUIRED | ES_DISPLAY_REQUIRED
fn inhibit()   // 申请睡眠抑制
fn uninhibit() // 释放，恢复系统默认
```

**macOS** (`cfg(target_os = "macos")`)
```rust
// IOKit: IOPMAssertionCreateWithName / IOPMAssertionRelease
// 使用 kIOPMAssertionTypePreventUserIdleDisplaySleep
fn inhibit()
fn uninhibit()
```

**Linux** (`cfg(target_os = "linux")`)
```rust
// zbus: 通过 D-Bus 调用
// org.freedesktop.ScreenSaver.Inhibit
// org.freedesktop.login1.Manager.Inhibit
fn inhibit()
fn uninhibit()
```

### mouse.rs
- 启用时启动一个定时器
- 每 N 秒（默认 60）获取当前鼠标位置，移动 1px 再移回
- 跨平台：使用 `enigo` 库

### config.rs
```
配置文件路径（按平台约定）：
  Windows: %APPDATA%/keep-awake/config.json
  macOS:   ~/Library/Application Support/keep-awake/config.json
  Linux:   ~/.config/keep-awake/config.json
```
- 配置字段：
  - `enabled: bool` — 是否启用防锁屏
  - `mode: enum { Api, Mouse }` — 模式
  - `interval_secs: u64` — 鼠标移动间隔（仅 Mouse 模式）
  - `autostart: bool` — 是否开机自启
- 启动时读取，变化时即时写入

## 开机自启实现

| 平台 | 实现方式 |
|------|---------|
| Windows | 写注册表 `HKCU\Software\Microsoft\Windows\CurrentVersion\Run` |
| macOS | 生成 `~/Library/LaunchAgents/com.keep-awake.plist` |
| Linux | 生成 `~/.config/autostart/keep-awake.desktop` |

## 构建

```bash
# 当前平台
cargo build --release

# 交叉编译示例
cargo build --release --target x86_64-pc-windows-msvc  # Windows
cargo build --release --target x86_64-apple-darwin      # macOS
cargo build --release --target x86_64-unknown-linux-gnu # Linux
```

最终产物为单文件 `target/release/keep-awake.exe`（或同名的无扩展名文件），无外部依赖。
