package main

import (
	"os"
	"strings"
	"sync"
	"syscall"
	"unicode/utf16"
	"unsafe"
)

type Lang int

const (
	LangEn Lang = 0
	LangZh Lang = 1
)

var (
	currentLang Lang
	langOnce    sync.Once
)

func detectLang() Lang {
	langOnce.Do(func() {
		// Try LANG env var (macOS / Linux)
		if l := os.Getenv("LANG"); strings.HasPrefix(l, "zh") {
			currentLang = LangZh
			return
		}
		// Windows: read registry for locale (fast, no CMD flash)
		if s := winLocale(); strings.HasPrefix(s, "zh") {
			currentLang = LangZh
			return
		}
		currentLang = LangEn
	})
	return currentLang
}

func winLocale() string {
	k, err := syscall.UTF16PtrFromString(`Control Panel\International`)
	if err != nil {
		return ""
	}
	var h syscall.Handle
	if err := syscall.RegOpenKeyEx(syscall.HKEY_CURRENT_USER, k, 0, syscall.KEY_READ, &h); err != nil {
		return ""
	}
	defer syscall.RegCloseKey(h)

	buf := make([]uint16, 64)
	var typ uint32
	var size uint32 = uint32(len(buf) * 2)
	v, err := syscall.UTF16PtrFromString("LocaleName")
	if err != nil {
		return ""
	}
	if err := syscall.RegQueryValueEx(h, v, nil, &typ, (*byte)(unsafe.Pointer(&buf[0])), &size); err != nil {
		return ""
	}
	return string(utf16.Decode(buf[:size/2]))
}

func Tr(en, zh string) string {
	if detectLang() == LangZh {
		return zh
	}
	return en
}

const (
	TitleRunning   = "Keep Awake"
	TitleRunningZh = "保持唤醒"
	TextRunning    = "Running in background"
	TextRunningZh  = "已在后台运行"
	TitleActive    = "Keep Awake — Active"
	TitleActiveZh  = "保持唤醒 — 运行中"
	TitleStopped   = "Keep Awake — Stopped"
	TitleStoppedZh = "保持唤醒 — 已停止"
)

var (
	MenuToggle      = func() string { return Tr("Keep Awake", "保持唤醒") }
	MenuSwitchMouse = func() string { return Tr("Switch to Mouse Jiggle", "切换到鼠标微动") }
	MenuSwitchAPI   = func() string { return Tr("Switch to API Inhibit", "切换到API抑制") }
	MenuAutostart   = func() string { return Tr("Launch at Login", "开机启动") }
	MenuQuit        = func() string { return Tr("Quit", "退出") }
)
