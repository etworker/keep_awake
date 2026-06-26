# 技术选型

## 语言对比

| 语言 | 单文件体积 | 跨平台难度 | 开发效率 |
|------|-----------|-----------|---------|
| AutoHotkey | ~1 MB | Windows only | 高 |
| C++ | ~200 KB | 高（三套原生 API） | 低 |
| Rust | ~1-3 MB | 中（生态成熟） | 中 |
| Go | ~3-6 MB | 低（交叉编译一命令） | 高 |

## 选择 Rust 的原因

### 1. 跨平台系统托盘

Rust 生态有 `tray-icon` 库，一套 API 覆盖 Windows / macOS / Linux，无需为每个平台写不同的托盘代码。

### 2. 单文件轻量

Rust 静态编译，不依赖任何运行时（无 JVM、无解释器、无 .NET Runtime），产物体积 1-3 MB，符合轻量需求。

### 3. 平台条件编译

通过 `#[cfg(target_os = "...")]` 特性，在同一个代码库中为不同平台提供不同的休眠抑制实现，编译时自动选择：

| 平台 | 系统 API | Rust 绑定 |
|------|---------|-----------|
| Windows | `SetThreadExecutionState` | `windows-sys`（官方零开销绑定） |
| macOS | `IOPMAssertionCreateWithName` | `core-foundation` + `io-kit` |
| Linux | D-Bus ScreenSaver / logind Inhibit | `zbus`（纯 Rust D-Bus 实现） |

### 4. 未选其他语言的原因

- **Go** — 体积 3-6 MB，虽然也在可接受范围内，但比 Rust 大 2-3 倍；Rust 对零成本抽象更彻底
- **C++** — 体积最小，但跨平台托盘和电源抑制需要为三个平台分别编写和维护完全不同的代码，开发成本远高于 Rust；且内存安全问题需要额外注意
- **AutoHotkey / AutoIt** — 仅限 Windows，不满足跨平台需求
- **Python + PyInstaller** — 打包后 10-30 MB，远大于 Rust

### 5. 构建产物

```
cargo build --release
# 输出：target/release/keep-awake.exe (Windows)
#       target/release/keep-awake          (macOS / Linux)
```

单文件，无外部依赖，复制即可运行。
