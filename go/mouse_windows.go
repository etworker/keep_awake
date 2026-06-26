//go:build windows

package main

import (
	"unsafe"
)

type point struct {
	x, y int32
}

var (
	procGetCursorPos = user32.NewProc("GetCursorPos")
	procSetCursorPos = user32.NewProc("SetCursorPos")
)

func platformJiggle() func() {
	return func() {
		var pt point
		procGetCursorPos.Call(uintptr(unsafe.Pointer(&pt)))
		procSetCursorPos.Call(uintptr(pt.x+1), uintptr(pt.y))
		procSetCursorPos.Call(uintptr(pt.x), uintptr(pt.y))
	}
}
