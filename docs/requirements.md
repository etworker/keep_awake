# 需求文档：Keep Awake — 防锁屏/防睡眠工具

## 背景

当前系统没有管理员权限，无法通过修改屏保/电源设置来阻止系统锁屏。需要一个用户态的工具来维持系统不锁屏。

## 核心机制

**主要方案：调用 OS 电源管理 API 阻止系统进入休眠/关屏状态。**

- Windows: `SetThreadExecutionState(ES_CONTINUOUS | ES_SYSTEM_REQUIRED | ES_DISPLAY_REQUIRED)` / `PowerSetRequest`
- macOS: `IOPMAssertionCreateWithName(kIOPMAssertionTypePreventUserIdleDisplaySleep, ...)`
- Linux: D-Bus `org.freedesktop.ScreenSaver.Inhibit`（屏保）+ `systemd-logind Inhibit`（睡眠）

这些 API 不需要管理员权限，媒体播放器（VLC、Spotify 等）也使用相同方案。

**回退方案：** 若 API 抑制不可用，则每隔 N 秒将鼠标移动 1 像素再移回原位，模拟用户活动。

## 功能需求

| 需求 | 说明 |
|------|------|
| 防锁屏/防睡眠 | 默认通过系统 API 抑制空闲休眠；鼠标移动作为可选的回退模式 |
| 系统托盘图标 | 运行后在系统托盘中显示图标，提供右键菜单 |
| 启用/停用 | 菜单中可开关防锁屏功能 |
| 模式切换 | 可选择「API 抑制」或「鼠标微移」两种模式 |
| 退出 | 完全退出程序 |
| 开机自启 | 支持配置是否开机自启动（默认不生效，需用户手动开启） |
| 配置持久化 | 开关状态、模式、间隔时间、开机自启等设置保存在本地 |
| 支持跨平台 | Windows / macOS / Linux |

## 技术约束

- 无需管理员权限即可运行
- 单文件可执行程序，轻量级，常驻后台
- 内存占用尽量小（< 50 MB）
- 启动后默认不启用防锁屏，除非用户配置过

## 非功能需求

- 启动迅速，无感知
- 开机自启配置通过注册表（Windows）/ LaunchAgents（macOS）/ autostart desktop file（Linux）实现
- 无需安装额外运行时依赖


