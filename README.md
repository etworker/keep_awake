# Keep Awake

轻量级系统托盘工具，防止电脑自动锁屏/睡眠。提供 **Rust** 和 **Go** 两个实现版本。

## 截图

系统托盘显示太阳图标（橙色=运行中，灰色=已暂停），右键菜单切换模式和开机自启。

## 功能

| 功能 | Rust | Go |
|------|------|-----|
| 系统托盘图标 + 右键菜单 | ✓ | ✓ |
| API 模式（OS 级睡眠抑制） | ✓ | ✓ |
| 鼠标微动模式 | ✓ | ✓ |
| 开机自启 | ✓ | ✓ |
| 单实例保护 | ✓ | ✓ |
| 中/英文自动切换 | ✓ | ✓ |
| JSON 配置持久化 | ✓ | ✓ |
| 启动气泡通知 | - | ✓ |
| 程序化绘制太阳图标 | ✓ | ✓ |

## 目录结构

```
keep_awake/
├── rust/       ← Rust 实现（win/mac/linux）
│   ├── Cargo.toml
│   └── src/*.rs
├── go/         ← Go 实现（win/mac/linux）
│   ├── go.mod
│   ├── main.go, config.go, lang.go
│   ├── tray.go, icon.go, notify.go
│   ├── inhibit.go (+ _windows/_macos/_linux)
│   ├── mouse.go (+ _windows/_other)
│   ├── single_instance.go (+ _stub)
│   └── autostart.go, platform.go, winapi_common.go
├── docs/       ← 设计文档
└── .github/workflows/ci.yml
```

## 构建

### Rust

```bash
cd rust
cargo build --release
# 产物: rust/target/release/keep-awake
```

### Go

```bash
cd go
go mod tidy
go build -ldflags="-H=windowsgui" -o keep-awake .
# 产物: go/keep-awake (或 keep-awake.exe)
```

## 平台支持

| 平台 | Rust | Go |
|------|------|-----|
| Windows 10/11 | ✓ | ✓ |
| macOS | ✓ | (需测试) |
| Linux (GTK) | ✓ | (需测试) |

## 技术细节

- **睡眠抑制**: 调用 OS 原生 API（Windows `SetThreadExecutionState`、macOS `IOPMAssertionCreateWithName`、Linux D-Bus `ScreenSaver.Inhibit`）
- **鼠标微动**: 模拟鼠标轻移 1px（Windows `SetCursorPos`、macOS `osascript`、Linux `xdotool`）
- **语言检测**: Windows 读注册表 `LocaleName`，macOS/Linux 读 `$LANG`
- **托盘图标**: 程序化绘制（圆形 + 8 射线），RGBA → ICO（Go 版）或 GDI（Rust 版）
- **单实例**: Windows 命名 Mutex，其他平台可通过 lockfile 扩展
- **自启**: Windows 注册表 `/RUN`、macOS LaunchAgent plist、Linux autostart .desktop
- **二进制大小**: Rust ~500KB / Go ~5.5MB（均为 release + strip）

## License

MIT
