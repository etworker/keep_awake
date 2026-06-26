//go:build windows

package main

import (
	"syscall"
	"unsafe"
)

func ensureSingleInstance() bool {
	kernel32 := syscall.NewLazyDLL("kernel32.dll")
	procCreateMutexW := kernel32.NewProc("CreateMutexW")
	name, _ := syscall.UTF16PtrFromString("keep-awake-6a1b2c3d")
	m, _, err := procCreateMutexW.Call(0, 1, uintptr(unsafe.Pointer(name)))
	if m == 0 {
		return false
	}
	if err == syscall.ERROR_ALREADY_EXISTS {
		return false
	}
	return true
}
