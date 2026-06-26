//go:build windows

package main

import "syscall"

var (
	user32               = syscall.NewLazyDLL("user32.dll")
	shell32              = syscall.NewLazyDLL("shell32.dll")
	procFindWindowW      = user32.NewProc("FindWindowW")
	procShellNotifyIconW = shell32.NewProc("Shell_NotifyIconW")
)
