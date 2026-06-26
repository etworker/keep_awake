//go:build windows

package main

import "syscall"

type windowsGuard struct{}

func acquireInhibit() InhibitGuard {
	const (
		esContinuous      = 0x80000000
		esSystemRequired  = 0x00000001
		esDisplayRequired = 0x00000002
	)
	kernel32 := syscall.NewLazyDLL("kernel32.dll")
	procSetThreadExecutionState := kernel32.NewProc("SetThreadExecutionState")
	ret, _, _ := procSetThreadExecutionState.Call(esContinuous | esSystemRequired | esDisplayRequired)
	if ret == 0 {
		return nil
	}
	return &windowsGuard{}
}

func (g *windowsGuard) Release() {
	kernel32 := syscall.NewLazyDLL("kernel32.dll")
	procSetThreadExecutionState := kernel32.NewProc("SetThreadExecutionState")
	procSetThreadExecutionState.Call(0x80000000) // ES_CONTINUOUS
}
